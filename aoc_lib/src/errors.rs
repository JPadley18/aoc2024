use thiserror::Error;
use reqwest::Error as ReqwestError;

#[derive(Error, Debug)]
pub enum AocLibError {
    #[error("invalid session token format")]
    BadSessionToken,

    #[error("no puzzle input is available for day {0} of {1}")]
    PuzzleNotFound(i32, i32),

    #[error("puzzle day {0} is out of bounds (1-25)")]
    PuzzleDayOutOfBounds(i32),

    #[error("puzzle year cannot be zero")]
    PuzzleYearIsZero,

    #[error("API request failed {0}")]
    RequestFailed(ReqwestError),

    #[error("environment variable `AOC_SESSION` is unset")]
    SessionEnvVarUnset
}
