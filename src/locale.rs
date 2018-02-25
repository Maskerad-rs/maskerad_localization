// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use errors::{LocalizationError, LocalizationResult};

#[derive(Debug)]
pub struct UnvalidatedLocale {
    locale: String,
}

impl<'a> From<&'a str> for UnvalidatedLocale {
    fn from(locale: &'a str) -> Self {
        UnvalidatedLocale {
            locale: locale.to_owned()
        }
    }
}

impl UnvalidatedLocale {
    pub fn validate(self) -> LocalizationResult<ValidatedLocale> {
        // the regex. (maybe cache it with lazy static or something).
        unimplemented!()
    }
}

pub struct ValidatedLocale {
    locale: String
}

