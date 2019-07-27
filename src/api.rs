
use std::io::{Read, BufRead, Cursor};
use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use sha1::{Sha1};
use serde_json::{from_str};

use errors::*;
use model::*;

static MAIN_API_URL : &'static str = "https://haveibeenpwned.com/api/";
static RANGE_API_URL : &'static str = "https://api.pwnedpasswords.com/range/";
static DEFAULT_USER_AGENT : &'static str = "wisespace-io";

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
        let url = format!("{}{}", RANGE_API_URL, prefix);

        match self.get(url) {
            Ok(answer) => {
                let cursor = Cursor::new(answer);
                for line in cursor.lines() {
                    let value = line.unwrap().to_lowercase();
                    if value.contains(suffix) {
                        let v: Vec<&str> = value.split(":").collect();
                        return Ok(Password {found: true, count: v[1].parse::<u64>().unwrap()});
                    }
                }
                Ok(Password {found: false, count: 0})
            },
            Err(e) => Err(e),
        }
    }

    pub fn check_email<E>(&self, email: E) -> Result<(Vec<Breach>)>
        where E: Into<String>
    {
        let url = format!("{}breachedaccount/{}", MAIN_API_URL, email.into());

        match self.get(url) {
            Ok(answer) => {
                let breach: Vec<Breach> = from_str(answer.as_str()).unwrap();
                Ok(breach)
            },
            Err(e) => Err(e),
        }
    }

    fn get(&self, url: String) -> Result<String> {
        let mut custon_headers = HeaderMap::new();

        custon_headers.insert(USER_AGENT, HeaderValue::from_str(self.user_agent.as_str())?);
        custon_headers.insert(HeaderName::from_static("api-version"), HeaderValue::from_static("2"));

        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(custon_headers)
            .send()?;

        self.handler(response)
    }

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::OK => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                Ok(body)
            },
            StatusCode::NOT_FOUND => {
                bail!(format!("The account could not be found and has therefore not been pwned"));
            }            
            status => {
                bail!(format!("{:?}", status));
            }
        }
    }
}