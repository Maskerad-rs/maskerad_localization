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