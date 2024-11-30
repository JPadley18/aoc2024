use crate::errors::AocLibError;
use std::env;
use reqwest::{StatusCode, blocking::Client, header::{HeaderValue, HeaderMap, COOKIE}};

pub struct AocClient {
    http_client: Client
}

impl AocClient {
    fn new(session_token: &str) -> Result<AocClient, AocLibError> {
        // Create a new reqwest blocking Client with the session header included
        let session_cookie = HeaderValue::from_str(&format!("session={}", session_token)).map_err(|_| AocLibError::BadSessionToken)?;
        let mut headers = HeaderMap::new();
        headers.append(COOKIE, session_cookie);

        Ok(AocClient {
            http_client: Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to initialise reqwest Client instance")
        })
    }

    fn get_input(&self, day: i32, year: i32) -> Result<String, AocLibError> {
        let result = self.http_client.get(format!("https://adventofcode.com/{}/day/{}/input", year , day))
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| match error.status() {
                Some(StatusCode::NOT_FOUND) => AocLibError::PuzzleNotFound(day, year),
                _ => AocLibError::RequestFailed(error)
            })?;
        result.text().map_err(|error| AocLibError::RequestFailed(error))
    }
}

pub struct AocRequestBuilder {
    client: AocClient,
    day: i32,
    year: i32,
}

pub fn from_session(session_token: &str) -> Result<AocRequestBuilder, AocLibError> {
    AocRequestBuilder::new(session_token)
}

pub fn from_env() -> Result<AocRequestBuilder, AocLibError> {
    let session_token = env::var("AOC_SESSION").map_err(|_| AocLibError::SessionEnvVarUnset)?;
    from_session(&session_token)
}

impl AocRequestBuilder {
    fn new(session_token: &str) -> Result<AocRequestBuilder, AocLibError> {
        let client = AocClient::new(session_token)?;
        Ok(AocRequestBuilder {
            client,
            day: 0,
            year: 0,
        })
    }

    pub fn day(self, day: i32) -> AocRequestBuilder {
        AocRequestBuilder {
            day,
            ..self
        }
    }

    pub fn year(self, year: i32) -> AocRequestBuilder {
        AocRequestBuilder {
            year,
            ..self
        }
    }

    pub fn get_input(&self) -> Result<String, AocLibError> {
        if self.year == 0 {
            Err(AocLibError::PuzzleYearIsZero)
        } else if self.day < 1 || self.day > 25 {
            Err(AocLibError::PuzzleDayOutOfBounds(self.day))
        } else {
            self.client.get_input(self.day, self.year)
        }
    }

    // fn try_cache(&self) -> Result<String, AocLibError> {
    //     // Check whether folder exists
    // }
}