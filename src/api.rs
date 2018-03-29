use std::io::{Read, BufRead, Cursor};
use reqwest;
use reqwest::header::{Headers, UserAgent};
use sha1::{Sha1};
use errors::*;

static API_URL : &'static str = "https://api.pwnedpasswords.com/range/";
static DEFAULT_USER_AGENT : &'static str = "github.com/wisespace-io/pwned-rs";

#[derive(Debug)]
pub struct Password {
    pub found: bool,
    pub count: i32,
}

#[derive(Builder, Debug, PartialEq)]
pub struct Pwned {
    #[builder(setter(into), default = "self.default_user_agent()")]
    pub user_agent: String
}

impl PwnedBuilder {
    fn default_user_agent(&self) -> String {
        if let Some(ref user_agent) = self.user_agent {
            format!("{}", user_agent)
        } else {
            DEFAULT_USER_AGENT.to_string()
        }
    }
}

impl Pwned {
    pub fn check_password<P>(&self, password: P) -> Result<(Password)>
        where P: Into<String>
    {
        let mut sha1 = Sha1::new();

        sha1.update(password.into().as_bytes());

        let hash = sha1.digest().to_string();
        let (prefix, suffix) = hash.split_at(5);
        let url = format!("{}{}", API_URL, prefix);

        match self.get(url) {
            Ok(answer) => {
                let cursor = Cursor::new(answer);
                for line in cursor.lines() {
                    let value = line.unwrap().to_lowercase();
                    if value.contains(suffix) {
                        let v: Vec<&str> = value.split(":").collect();
                        return Ok(Password {found: true, count: v[1].parse::<i32>().unwrap()});
                    }
                }
                Ok(Password {found: false, count: 0})
            },
            Err(e) => Err(e),
        }
    }

    fn get(&self, url: String) -> Result<String> {
        let mut custon_headers = Headers::new();

        custon_headers.set(UserAgent::new(self.user_agent.clone()));

        let client = reqwest::Client::new();
        let mut response = client
            .get(url.as_str())
            .headers(custon_headers)
            .send()?;

        let mut data = String::new();
        try!(response.read_to_string(&mut data));

        Ok(data)
    }
}