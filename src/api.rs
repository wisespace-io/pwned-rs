
use std::io::{BufRead, Cursor};
use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use sha1::{Sha1};
use serde_json::{from_str};
use derive_builder::Builder;

use crate::{errors::Result, model::{Breach, Password}};

static MAIN_API_URL : &'static str = "https://haveibeenpwned.com/api/v3/";
static RANGE_API_URL : &'static str = "https://api.pwnedpasswords.com/range/";
static DEFAULT_USER_AGENT : &'static str = "wisespace-io";
static API_KEY: &'static str = "hibp-api-key";

#[derive(Builder, Debug, PartialEq)]
pub struct Pwned {
    #[builder(setter(into), default = "self.default_user_agent()")]
    pub user_agent: String,

    /// Whether the Pwned Passwords API should randomly pad responses it returns.
    /// Padding closes a small security gap; see [the original blog post] for
    /// more info.
    ///
    /// [the original blog post]: https://www.troyhunt.com/enhancing-pwned-passwords-privacy-with-padding/
    #[builder(default = "true")]
    pub pad_password_responses: bool,

    #[builder(setter(into), default = "None")]
    pub api_key: Option<String>
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
    pub async fn check_password<P>(&self, password: P) -> Result<Password>
        where P: Into<String>
    {
        let mut sha1 = Sha1::new();

        sha1.update(password.into().as_bytes());

        let hash = sha1.digest().to_string();
        let (prefix, suffix) = hash.split_at(5);
        let url = format!("{}{}", RANGE_API_URL, prefix);

        match self.get(url).await {
            Ok(answer) => {
                let cursor = Cursor::new(answer);
                for line in cursor.lines() {
                    let value = line.unwrap().to_lowercase();
                    if value.contains(suffix) {
                        let v: Vec<&str> = value.split(":").collect();
                        let count = v[1].parse::<u64>().unwrap();
                        let found = count > 0;
                        return Ok(Password {found, count});
                    }
                }
                Ok(Password {found: false, count: 0})
            },
            Err(e) => Err(e),
        }
    }

    pub async fn check_email<E>(&self, email: E) -> Result<Vec<Breach>>
        where E: Into<String>
    {
        let url = format!("{}breachedaccount/{}?truncateResponse=false", MAIN_API_URL, email.into());
        match self.get(url).await {
            Ok(answer) => {
                let breach: Vec<Breach> = from_str(answer.as_str()).unwrap();
                Ok(breach)
            },
            Err(e) => Err(e),
        }
    }

    async fn get(&self, url: String) -> Result<String> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_str(self.user_agent.as_str())?);
        if let Some(ref api_key) = self.api_key {
            custom_headers.insert(API_KEY, HeaderValue::from_str(api_key)?);
        }
        if self.pad_password_responses && url.starts_with(RANGE_API_URL) {
            custom_headers.insert("Add-Padding", HeaderValue::from_str("true")?);
        }

        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(custom_headers)
            .send().await?;

        self.handler(response).await
    }

    async fn handler(&self, response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                Ok(response.text().await?)
            },
            StatusCode::NOT_FOUND => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "The account could not be found and has therefore not been pwned").into())
            }
            status => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{:?}", status)).into())
            }
        }
    }
}