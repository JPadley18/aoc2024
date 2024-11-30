use crate::errors::AocLibError;
use dirs;
use std::{env, fs, io::ErrorKind, hash::{DefaultHasher, Hash, Hasher}};
use reqwest::{StatusCode, blocking::Client, header::{HeaderValue, HeaderMap, COOKIE}};

struct AocClient {
    http_client: Client
}

impl AocClient {
    fn new(session_token: &str) -> Result<Self, AocLibError> {
        // Create a new reqwest blocking Client with the session header included
        if session_token.len() == 0 {
            Err(AocLibError::SessionTokenEmpty)
        } else {
            let session_cookie = HeaderValue::from_str(&format!("session={}", session_token)).map_err(|_| AocLibError::BadSessionToken)?;
            let mut headers = HeaderMap::new();
            headers.append(COOKIE, session_cookie);
    
            Ok(Self {
                http_client: Client::builder()
                    .default_headers(headers)
                    .build()
                    .expect("Failed to initialise reqwest Client instance")
            })
        }
    }

    fn get_input(&self, day: i32, year: i32) -> Result<String, AocLibError> {
        let result = self.http_client.get(format!("https://adventofcode.com/{}/day/{}/input", year , day))
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| match error.status() {
                Some(StatusCode::NOT_FOUND) => AocLibError::PuzzleNotFound(day, year),
                Some(StatusCode::INTERNAL_SERVER_ERROR) => AocLibError::ApiServerError,
                _ => AocLibError::RequestFailed(error)
            })?;
        result.text().map_err(|error| AocLibError::RequestFailed(error))
    }
}

pub struct AocRequestBuilder {
    client: AocClient,
    hash: u64,
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
    fn new(session_token: &str) -> Result<Self, AocLibError> {
        let client = AocClient::new(session_token)?;
        Ok(Self {
            client,
            hash: hash_str(session_token),
            day: 0,
            year: 0,
        })
    }

    pub fn day(self, day: i32) -> Self {
        Self {
            day,
            ..self
        }
    }

    pub fn year(self, year: i32) -> Self {
        Self {
            year,
            ..self
        }
    }

    pub fn get_input(&self) -> Result<String, AocLibError> {
        self.try_cache()
    }

    fn get_aoc_input(&self) -> Result<String, AocLibError> {
        if self.year == 0 {
            Err(AocLibError::PuzzleYearIsZero)
        } else if self.day < 1 || self.day > 25 {
            Err(AocLibError::PuzzleDayOutOfBounds(self.day))
        } else {
            self.client.get_input(self.day, self.year)
        }
    }

    fn calculate_cache_folder(&self) -> String {
        let base = dirs::home_dir()
        .expect("Can't find home directory")
        .into_os_string()
        .into_string()
        .expect("Can't decode home directory")
        + "/.aoc_lib/";
        format!("{}/{}/{}", base, self.hash, self.year)
    }

    fn calculate_cache_path(&self) -> String {
        format!("{}/{}.txt", self.calculate_cache_folder(), self.day)
    }

    fn try_cache(&self) -> Result<String, AocLibError> {
        // See if the file exists
        let path = self.calculate_cache_path();
        fs::read_to_string(&path)
            .or_else(|error| {
                match error.kind() {
                    ErrorKind::NotFound => self.refresh_cache(),
                    ErrorKind::PermissionDenied => Err(AocLibError::CachePermissionDenied(path)),
                    error => Err(AocLibError::CacheReadFailed(error))
                }
            })
    }

    fn refresh_cache(&self) -> Result<String, AocLibError> {
        let content = self.get_aoc_input()?;
        // Write content to cache
        fs::create_dir_all(self.calculate_cache_folder())
            .map_err(|error| AocLibError::CacheWriteFailed(error.kind()))?;
        fs::write(self.calculate_cache_path(), &content)
            .map_err(|error| AocLibError::CacheWriteFailed(error.kind()))?;
        Ok(content)
    }
}

fn hash_str(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}