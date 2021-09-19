use serde_json::Value;
use std::collections::HashMap;
use reqwest::{cookie::Jar, Url, cookie::Cookie, redirect::Policy, Client};
use html_parser::{Dom, Node, Error as HTMLError, Element};
use std::sync::Arc;

mod parseable;
mod profile;

use parseable::Parseable;
use profile::Profile;

pub enum ParseType {
    Attribute,
    Text,
}

pub struct OpenWilma {
    base_url: String,
    client: Client,
}

impl OpenWilma {
    pub async fn connect(email: &str, passwd: &str, server: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = reqwest::Client::builder().redirect(Policy::none());
        let client = builder.build()?;

        let url = fix_url(server);

        /*let _wilmas = reqwest::get("https://www.starsoft.fi/wilmat/wilmat.json")
            .await?
            .text()
            .await?;*/

        let res = reqwest::get(url.clone() + "index_json")
            .await?
            .text()
            .await?;
    
        let res_json: Value = serde_json::from_str(&res)?;
    
        let session_id = res_json.get("SessionID").unwrap().as_str().unwrap();
    
        let mut info: HashMap<&str, &str> = HashMap::new();
        info.insert("Login", email);
        info.insert("Password", passwd);
        info.insert("SESSIONID", session_id);
        info.insert("CompleteJson", "");

        let res = client.post(url.clone() + "login")
            .form(&info)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let cookies: Vec<Cookie> = res.cookies().collect();
        let cookie = cookies.iter().find(|c| c.name() == "Wilma2SID").unwrap();

        //let sid = cookie.value().to_string();
        let jar = Arc::new(Jar::default());
        let cookie_url = url.clone().parse::<Url>()?;
        jar.add_cookie_str(&format!("Wilma2SID={}", cookie.value()), &cookie_url);

        let builder = reqwest::Client::builder();
        let client = builder.cookie_provider(jar).build()?;

        Ok(Self {
            client,
            base_url: url.to_string(),
        })
    }

    pub async fn profile(&self) -> Result<Profile, Box<dyn std::error::Error>> {
        let res = self.client.get(self.base_url.clone())
            .send()
            .await?
            .text()
            .await?;

        let mut iterator = res.split("\n");

        let name = find_prop_and_parse("teacher", &mut iterator, &ParseType::Text)?;
        let school = find_prop_and_parse("school", &mut iterator, &ParseType::Text)?;
        let formkey = find_prop_and_parse("formkey", &mut iterator, &ParseType::Attribute)?;

        return Ok(Profile::new(name, school, formkey));
    }
}

fn fix_url(prev: &str) -> String {
    let mut new: String = prev.into();

    if !new.ends_with("/") {
        new += "/"
    }

    new
}

fn find_prop_and_parse<'a, T>(prop: &str, original: &mut T, parse_type: &ParseType) -> Result<String, HTMLError>
    where T: Iterator<Item = &'a str>
{
    let line = original.find(|line| line.contains(prop)).unwrap();
    let element = Dom::parse(line)?;

    match parse_type {
        ParseType::Attribute => {
            match element.first_child() {
                Some(node) => {
                    match node {
                        Node::Element(elem) => {
                            Ok(parse_attribute(elem, "value"))
                        }
                        _ => panic!("mistaken find_prop_and_parse call")
                    }
                }
                None => panic!("mistaken find_prop_and_parse call")
            }
        }
        ParseType::Text => Ok(parse_text(&element))
    }

    /*if let Node::Element(elem) = &Dom::parse(line)?.children[0] {
        if let Node::Text(text) = &elem.children[0] {
            return Ok(text.into());
        }
    }*/
}

fn parse_attribute(elem: &Element, attr: &str) -> String
{
    match &elem.first_child() {
        Some(node) => {
            match node {
                Node::Element(elem) => {
                    return parse_attribute(elem, attr);
                }
                _ => panic!("mistaken parse_attribute call")
            }
        }

        None => {
            return elem.attributes.get(attr)
                .unwrap()
                .as_ref()
                .unwrap()
                .into()
        }
    }
}
// recursive function to check for HTML content
fn parse_text<T>(elem: &T) -> String
    where T: Parseable
{
    match &elem.first_child_unchecked() {
        Node::Element(elem) => {
            return parse_text(elem);
        }

        Node::Text(text) => {
            return text.into();
        }

        // TODO: find a better solution for comments
        Node::Comment(comment) => return comment.into(),
    }
}
#[cfg(test)]
mod tests {
    use crate::OpenWilma;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn login() {
        let openwilma = OpenWilma::connect("username", "password", "server")
            .await
            .unwrap();

        let profile = openwilma.profile().await.unwrap();

        let formkey = profile.formkey().clone();

        assert_eq!(profile.name().is_empty(), false);
        assert_eq!(profile.school().is_empty(), false);
        assert_eq!(formkey.split(":").count(), 3);
    }
}
