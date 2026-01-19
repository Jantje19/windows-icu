use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    string::FromUtf8Error,
};

use windows::Win32::Globalization::{U_ZERO_ERROR, UErrorCode, u_errorName};

#[derive(Debug)]
pub struct ICUStatus {
    code: UErrorCode,
}

impl ICUStatus {
    pub fn new() -> Self {
        ICUStatus { code: U_ZERO_ERROR }
    }

    pub fn is_error(&self) -> bool {
        self.code != U_ZERO_ERROR
    }

    pub fn error_message(&self) -> Result<String, FromUtf8Error> {
        unsafe { u_errorName(self.code).to_string() }
    }
}

impl Deref for ICUStatus {
    type Target = UErrorCode;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl DerefMut for ICUStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.code
    }
}

impl From<UErrorCode> for ICUStatus {
    fn from(code: UErrorCode) -> Self {
        ICUStatus { code }
    }
}

impl Display for ICUStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message().unwrap())
    }
}
