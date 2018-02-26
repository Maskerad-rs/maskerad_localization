// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;

use regex::Error as RegexError;
use csv::Error as CSVError;
use serde_json::Error as JSONError;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum LocalizationError {
    RegexError(String, RegexError),
    UnvalidLocale(String),
    CSVError(String, CSVError),
    HeaderError(String),
    JSONError(String, JSONError),
    IOError(String, IOError),
    LanguageMappingError(String),
    ManifestMapError(String),
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
            &LocalizationError::CSVError(ref desc, _) => {
                write!(f, "CSV error: {}", desc)
            },
            &LocalizationError::HeaderError(ref desc) => {
                write!(f, "Header error: {}", desc)
            },
            &LocalizationError::JSONError(ref desc, _) => {
                write!(f, "JSON error: {}", desc)
            },
            &LocalizationError::IOError(ref desc, _) => {
                write!(f, "I/O error: {}", desc)
            },
            &LocalizationError::LanguageMappingError(ref desc) => {
                write!(f, "Language mapping error: {}", desc)
            },
            &LocalizationError::ManifestMapError(ref desc) => {
                write!(f, "Manifest map error: {}", desc)
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
            &LocalizationError::CSVError(_, _) => {
                "CSVError"
            },
            &LocalizationError::HeaderError(_) => {
                "HeaderError"
            },
            &LocalizationError::JSONError(_, _) => {
                "JSONError"
            },
            &LocalizationError::IOError(_, _) => {
                "IOError"
            },
            &LocalizationError::LanguageMappingError(_) => {
                "LanguageMappingError"
            },
            &LocalizationError::ManifestMapError(_) => {
                "ManifestMapError"
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
            &LocalizationError::CSVError(_, ref csv_error) => {
                Some(csv_error)
            },
            &LocalizationError::HeaderError(_) => {
                None
            },
            &LocalizationError::JSONError(_, ref json_error) => {
                Some(json_error)
            },
            &LocalizationError::IOError(_, ref io_error) => {
                Some(io_error)
            },
            &LocalizationError::LanguageMappingError(_) => {
                None
            },
            &LocalizationError::ManifestMapError(_) => {
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

impl From<CSVError> for LocalizationError {
    fn from(error: CSVError) -> Self {
        LocalizationError::CSVError(String::from("Error while reading a CSV file."), error)
    }
}

impl From<JSONError> for LocalizationError {
    fn from(error: JSONError) -> Self {
        LocalizationError::JSONError(String::from("Error while serializing a data structure to a JSON representation."), error)
    }
}

impl From<IOError> for LocalizationError {
    fn from(error: IOError) -> Self {
        LocalizationError::IOError(String::from("Error while doing I/O operations."), error)
    }
}