use std::fmt::Display;

use crate::icu::status::ICUStatus;

#[derive(Debug)]
pub enum Error {
    InvalidStatus(ICUStatus),
    UnexpectedNullPtr,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidStatus(s) => s.fmt(f),
            Error::UnexpectedNullPtr => write!(f, "Unexpected null pointer"),
        }
    }
}
