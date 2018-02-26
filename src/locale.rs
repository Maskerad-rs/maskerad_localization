// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use regex::Regex;

//We want to match locales with the following forms: th, th_TH, th_TH_TH...
//The regex needed: [a-z]{2,}(_[A-Z]{2,})*$
//[a-z]{2,} -> match 2 or more characters in the range "a to z"
//_[A-Z]{2,} -> match a '_' character and 2 or more characters in the range "A to Z".
//(_[A-Z]{2,})* -> match the expression above, but 0 or more time.
//$ -> end of input.


pub fn is_valid_locale<S>(text: S) -> bool where
    S: AsRef<str>
{
    lazy_static! {
        static ref REGEX_LOCALE: Regex = Regex::new("[a-z]{2,}(_[A-Z]{2,})*$").unwrap();
    }
    REGEX_LOCALE.is_match(text.as_ref())
}

#[cfg(test)]
mod locale_tests {
    use super::*;

    #[test]
    fn match_valid_locale() {
        assert!(is_valid_locale("fr"));
        assert!(is_valid_locale("fr_FR"));
        assert!(is_valid_locale("fr_FR_FR"));
    }

    #[test]
    fn match_non_valid_locale() {
        assert!(!is_valid_locale("FR"));
        assert!(!is_valid_locale("Fr"));
        assert!(!is_valid_locale("fR"));
        assert!(!is_valid_locale("fr-FR"));
        assert!(!is_valid_locale("fr_fR"));
        assert!(!is_valid_locale("fr_FR_fR"));
    }
}