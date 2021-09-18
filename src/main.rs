use serde_json::{Value, json};
use std::collections::HashMap;
use reqwest::{cookie::Jar, Url, cookie::Cookie, redirect::Policy};
use html_parser::{Dom, Node};
use std::sync::Arc;

const BASE_URL: &str = "https://example.inschool.fi/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = reqwest::Client::builder().redirect(Policy::none());
    let client = builder.build()?;

    let url = BASE_URL.to_owned();

    let _wilmas = reqwest::get("https://www.starsoft.fi/wilmat/wilmat.json")
        .await?
        .text()
        .await?;

    let res = reqwest::get(url.clone() + "index_json")
        .await?
        .text()
        .await?;

    let res_json: Value = serde_json::from_str(&res)?;

    let session_id = res_json.get("SessionID").unwrap().as_str().unwrap();

    let mut info: HashMap<&str, &str> = HashMap::new();
    info.insert("Login", "email");
    info.insert("Password", "password");
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

    for line in res.split("\n") {
        if line.contains("teacher") {
            if let Node::Element(elem) = &Dom::parse(line)?.children[0] {
                if let Node::Text(name) = &elem.children[0] {
                    println!("{}", name);
                }
            }
        }
    }

    Ok(())
}
