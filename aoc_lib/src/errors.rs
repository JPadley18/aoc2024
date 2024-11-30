use thiserror::Error;
use reqwest::Error as ReqwestError;
use std::io::ErrorKind;

#[derive(Error, Debug)]
pub enum AocLibError {
    #[error("invalid session token format")]
    BadSessionToken,

    #[error("no puzzle input is available for day {0} of {1}")]
    PuzzleNotFound(i32, i32),

    #[error("session token cannot be empty")]
    SessionTokenEmpty,

    #[error("session token is invalid")]
    SessionTokenInvalid,

    #[error("puzzle day {0} is out of bounds (1-25)")]
    PuzzleDayOutOfBounds(i32),

    #[error("advent of code server error")]
    ApiServerError,

    #[error("puzzle year cannot be zero")]
    PuzzleYearIsZero,

    #[error("API request failed {0}")]
    RequestFailed(ReqwestError),

    #[error("environment variable `AOC_SESSION` is unset")]
    SessionEnvVarUnset,

    #[error("permission denied to access cache location: {0}")]
    CachePermissionDenied(String),

    #[error("cache read failed for reason: {0}")]
    CacheReadFailed(ErrorKind),

    #[error("cache write failed for reason: {0}")]
    CacheWriteFailed(ErrorKind)
}
