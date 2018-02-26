// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[derive(Debug, Deserialize)]
pub struct Row {
    key: String,
    values: Vec<String>,
}

impl Row {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }
}

#[cfg(test)]
mod csv_structure_test {
    use super::*;

    #[test]
    fn row_key_values() {
        let row = Row {
            key: String::from("test_key"),
            values: vec![String::from("test1"), String::from("test2"), String::from("test3")]
        };

        assert_eq!(row.key(), "test_key");
        let mut iter = row.values().iter();
        let val1 = iter.next();
        let val2 = iter.next();
        let val3 = iter.next();
        let val4 = iter.next();

        assert!(val1.is_some());
        assert_eq!(val1.unwrap().as_str(), "test1");

        assert!(val2.is_some());
        assert_eq!(val2.unwrap().as_str(), "test2");

        assert!(val3.is_some());
        assert_eq!(val3.unwrap().as_str(), "test3");

        assert!(val4.is_none());
    }
}