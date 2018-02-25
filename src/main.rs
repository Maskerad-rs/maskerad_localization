// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad_localization;


use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::BufReader;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let csv_file = &arguments[1];
    let dest_dir = &arguments[2];

    let fullpath_file = fs::canonicalize(Path::new(csv_file.as_str())).expect(format!("Could not create the full path of the file {}", csv_file).as_str());
    let mut dest_dir = fs::canonicalize(Path::new(dest_dir.as_str())).expect(format!("Could not create the full path of the directory {}", dest_dir).as_str());
    dest_dir.push("localization");

    println!("Reading file at path: \n{}", fullpath_file.display());
    println!("\nThe JSON files will be output in the directory: \n{}", dest_dir.display());

    let file = fs::File::open(fullpath_file.as_path()).expect(format!("Could not open the file at path {}", fullpath_file.as_path().display()).as_str());
    //let csv_reader = csv::Reader::from_reader(file);

    /*
        make a lib.rs and create some structures to make the program ezsier to use and read.

        https://godot.readthedocs.io/en/latest/tutorials/misc/locales.html#doc-locales
        https://godot.readthedocs.io/en/latest/tutorials/misc/internationalizing_games.html
        https://godot.readthedocs.io/en/latest/getting_started/workflow/assets/importing_translations.html#doc-importing-translations
    */
}