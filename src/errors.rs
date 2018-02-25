// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;

use regex::Error as RegexError;

#[derive(Debug)]
pub enum LocalizationError {
    RegexError(String, RegexError),
    UnvalidLocale(String),
}

unsafe impl Send for LocalizationError {}
unsafe impl Sync for LocalizationError {}

impl fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LocalizationError::RegexError(ref desc, _) => {
                write!(f, "Regex error: {}", desc)
            },
            &LocalizationError::UnvalidLocale(ref desc) => {
                write!(f, "Unvalid locale: {}", desc)
            },
        }
    }
}

impl Error for LocalizationError {
    fn description(&self) -> &str {
        match self {
            &LocalizationError::RegexError(_, _) => {
                "RegexError"
            },
            &LocalizationError::UnvalidLocale(_) => {
                "UnvalidLocale"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LocalizationError::RegexError(_, ref regex_error) => {
                Some(regex_error)
            },
            &LocalizationError::UnvalidLocale(_) => {
                None
            },
        }
    }
}

pub type LocalizationResult<T> = Result<T, LocalizationError>;

impl From<RegexError> for LocalizationError {
    fn from(error: RegexError) -> Self {
        LocalizationError::RegexError(String::from("Error while parsing a regular expression."), error)
    }
}