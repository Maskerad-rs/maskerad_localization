// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use regex::Regex;

pub fn is_valid_locale<S>(text: S) -> bool where
    S: AsRef<str>
{
    lazy_static! {
        static ref REGEX_LOCALE: Regex = Regex::new("").unwrap();
    }

    REGEX_LOCALE.is_match(text.as_ref())
}

