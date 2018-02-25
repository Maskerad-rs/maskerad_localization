// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*
    Process:
    - Get the arguments passed to the program.
    - Check them
    - Begin to parse the csv
    - Check that it has the right form (matrix-like)
    - Read the first column
    - check that the (0, 0) emplacement is empty.
    - Check that all langs are in a valid locale format (from<str> + regex)
    - create a JSONManifest (holding an hashmap and serializable to JSON) for each lang
    - Read each rows to get the key and all the variants
    - populate those JSON manifest.
    - Save them in the correct file hierarchy ([Output dir]/localization/{locale}/localization.json
*/

//Manifest -> Serializable to JSON

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate regex;

pub mod locale;
pub mod errors;
pub mod manifest;
pub mod csv_structure;