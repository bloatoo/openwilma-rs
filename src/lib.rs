use serde_json::Value;
use std::collections::HashMap;
use reqwest::{cookie::Jar, Url, cookie::Cookie, redirect::Policy, Client};
use html_parser::{Dom, Node, Error as HTMLError, Element};
use std::sync::Arc;
use scraper::{Html, Selector};

mod profile;
mod parser;

use profile::Profile;

pub enum ParseType {
    Attribute(String),
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

        let mut url = fix_url(server);

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

        let jar = Arc::new(Jar::default());
        let cookie_url = url.clone().parse::<Url>()?;
        jar.add_cookie_str(&format!("Wilma2SID={}", cookie.value()), &cookie_url);

        let builder = reqwest::Client::builder();
        let client = builder.cookie_provider(jar).build()?;

        let res = client.get(url.clone())
            .send()
            .await?
            .text()
            .await?;

        let mut lines = res.split("\n");
        
        let line = lines.find(|l| l.contains("text-style-link")).unwrap();

        let identity = parse_identity(line);

        url += &identity;

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

        let mut lines = res.split("\n");

        let name = parser::parse_name(&mut lines);
        let school = parser::parse_school(&mut lines);
        let formkey = find_prop_and_parse("formkey", &mut lines)?;

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

fn parse_identity(line: &str) -> String {
    let fragment = Html::parse_fragment(line);
    let selector = Selector::parse("a").unwrap();
    let stuff = fragment.select(&selector).next().unwrap();
    let mut identity = stuff.value().attr("href").unwrap().to_string();
    identity.remove(0);
    identity
}

fn find_prop_and_parse<'a, T>(prop: &str, original: &mut T) -> Result<String, HTMLError>
    where T: Iterator<Item = &'a str>
{
    let line = original.find(|line| line.contains(prop)).unwrap();
    println!("{}", line);
    /*let element = Html::parse_fragment(line);
    let selector = Selector::parse("a").unwrap();

    let elem = element.select(&selector).next().unwrap();
    let text = elem.text().collect::<Vec<_>>();
    println!("{}", text.join(" "));*/
    
    let element = Dom::parse(line)?;

    match element.children.get(0) {
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

    /*if let Node::Element(elem) = &Dom::parse(line)?.children[0] {
        if let Node::Text(text) = &elem.children[0] {
            return Ok(text.into());
        }
    }*/
}

fn parse_attribute(elem: &Element, attr: &str) -> String
{
    match &elem.children.get(0) {
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
