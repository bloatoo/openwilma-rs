use serde_json::{Value, json};
use std::collections::HashMap;
use reqwest::{cookie::Jar, Url, cookie::Cookie, redirect::Policy, Client};
use html_parser::{Dom, Node};
use std::sync::Arc;

pub struct OpenWilma {
    base_url: String,
    client: Client,
}

impl OpenWilma {
    pub async fn connect(email: &str, passwd: &str, server: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = reqwest::Client::builder().redirect(Policy::none());
        let client = builder.build()?;

        let url = server.to_owned();

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
            base_url: server.to_string(),
        })
    }

    pub async fn name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let res = self.client.get(self.base_url.clone())
            .send()
            .await?
            .text()
            .await?;

        for line in res.split("\n") {
            if line.contains("teacher") {
                if let Node::Element(elem) = &Dom::parse(line)?.children[0] {
                    if let Node::Text(name) = &elem.children[0] {
                        return Ok(Some(name.into()));
                    }
                }
            }
        }

        return Ok(None);
    }
}

fn fix_url(prev: &str) -> String {
    todo!()
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
        let openwilma = OpenWilma::connect("email", "password", "server")
            .await
            .unwrap();

        let name = openwilma.name().await.unwrap();

        assert_eq!(name.unwrap().is_empty(), false);
    }
}
