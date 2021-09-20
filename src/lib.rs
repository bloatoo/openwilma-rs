use serde_json::Value;
use std::collections::HashMap;
use reqwest::{cookie::Jar, Url, cookie::Cookie, redirect::Policy, Client};
use std::sync::Arc;

mod profile;
mod schedule;
mod parser;

use profile::Profile;
use schedule::Schedule;

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

        let builder = reqwest::Client::builder().redirect(Policy::none());
        let client = builder.cookie_provider(jar).build()?;

        let res = client.get(url.clone())
            .send()
            .await?
            .text()
            .await?;

        let mut lines = res.split("\n");
        
        let identity = parser::parse_identity(&mut lines);

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
        let formkey = parser::parse_formkey(&mut lines);

        let profile = Profile::new(name, school, formkey);

        Ok(profile)
    }

    pub async fn schedule(&self) -> Result<Schedule, Box<dyn std::error::Error>> {
        let profile = self.profile().await?;

        let url = &format!("{}schedule/export/students/{}", self.base_url.clone(), profile.user_id());
        
        let res = self.client.get(url)
            .send()
            .await?
            .text()
            .await?;

        let result = Schedule::from_json(&res).unwrap();
        Ok(result)
    }
}

fn fix_url(prev: &str) -> String {
    let mut new: String = prev.into();

    if !new.ends_with("/") {
        new += "/"
    }

    new
}

#[cfg(test)]
mod tests {
    use crate::OpenWilma;

    pub const UNIT_TEST_USERNAME: &str = "";
    pub const UNIT_TEST_PASSWORD: &str = "";
    pub const UNIT_TEST_SERVER: &str = "";

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn login() {
        let openwilma = OpenWilma::connect(
            UNIT_TEST_USERNAME,
            UNIT_TEST_PASSWORD,
            UNIT_TEST_SERVER
        )
            .await
            .unwrap();

        let profile = openwilma.profile().await.unwrap();

        dbg!("{}", profile.name());
        dbg!("{}", profile.school());
        dbg!("{}", profile.formkey());

        assert!(!profile.name().is_empty());
        assert!(!profile.school().is_empty());
        assert_eq!(profile.formkey().split(":").count(), 3);
    }

    #[tokio::test]
    async fn schedule() {
        let openwilma = OpenWilma::connect(
            UNIT_TEST_USERNAME, 
            UNIT_TEST_PASSWORD, 
            UNIT_TEST_SERVER
        )
            .await
            .unwrap();

        let schedule = openwilma.schedule().await.unwrap();
        
        let reservations = schedule.reservations();

        for reserv in reservations {
            println!("reservation: {} ({}-{})", reserv.caption(), reserv.start(), reserv.end());
            println!("{:#?}", reserv);
        }

        assert!(!reservations.is_empty());
    }
}
