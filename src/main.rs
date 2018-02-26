// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate maskerad_localization;
extern crate csv;

use maskerad_localization::csv_structure::Row;
use maskerad_localization::errors::{LocalizationError, LocalizationResult};
use maskerad_localization::manifest::{Manifest, ManifestMap, LangMapper};
use maskerad_localization::locale;

use std::env;
use std::fs;
use std::path::Path;
use std::io::BufWriter;
use std::process;

fn main() {
    if let Err(error) = try_main() {
        println!("An error occurred: {}", error);
        process::exit(1);
    }
}

fn try_main() -> LocalizationResult<()> {
    let arguments: Vec<String> = env::args().collect();

    let csv_file = &arguments[1];
    let dest_dir = &arguments[2];

    let fullpath_file = fs::canonicalize(Path::new(csv_file.as_str())).expect(format!("Could not create the full path of the file {}", csv_file).as_str());
    let mut dest_dir = fs::canonicalize(Path::new(dest_dir.as_str())).expect(format!("Could not create the full path of the directory {}", dest_dir).as_str());
    dest_dir.push("localization");

    println!("Reading file at path: \n{}", fullpath_file.display());
    println!("The JSON files will be output in the directory: \n{}", dest_dir.display());

    let file = fs::File::open(fullpath_file.as_path()).expect(format!("Could not open the file at path {}", fullpath_file.as_path().display()).as_str());
    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b',').has_headers(false).flexible(false).from_reader(file);
    let mut iter = csv_reader.deserialize();

    //Dictionary of manifests.
    let mut manifests = ManifestMap::new();
    //lang mapper.
    let mut lang_mapper = LangMapper::new();
    //The counter used to map a language to an index.
    let mut counter: u8 = 0;

    //Read the header
    if let Some(result) = iter.next() {
      let header: Row = result?;
        println!("header: {:?}", header);
        //if N lang, create N manifest.
        for lang in header.values().iter() {
            //Check that its a valid locale.
            if locale::is_valid_locale(lang.as_str()) {
                // Add a manifest in the manifest map.
                manifests.insert_manifest(lang.as_str(), Manifest::new());
                //map the language to an index.
                lang_mapper.insert_mapping(counter, lang.as_str());
                //increment the counter
                counter += 1;
            } else {
                return Err(LocalizationError::UnvalidLocale(format!("The text '{}' is not a valid locale form !", lang.as_str())))
            }
        }
    } else {
        return Err(LocalizationError::HeaderError(String::from("Could not find the first line of the CSV !")));
    }

    println!("Manifests: {:?}", manifests);
    println!("Lang mapper: {:?}", lang_mapper);

    //Read all the records
    for record in iter {
        counter = 0;
        //add the keys + text to the appropriate manifest.
        let row: Row = record?;
        println!("row: {:?}", row);
        for text in row.values().iter() {
            // Get the language of the text.
            let language = lang_mapper.get_mapping(counter)?;
            // Get the corresponding manifest.
            let corresponding_manifest = manifests.get_mut(language)?;
            // Update the manifest.
            corresponding_manifest.insert(row.key(), text.as_str());

            //Increment the counter
            counter += 1;
        }
    }

    println!("Manifests: {:?}", manifests);
    //Save them.
    for data in manifests.iter() {
        //create the file at path [dest_dir]/{locale}/localization.json
        let mut file_path = dest_dir.clone();

        file_path.push(data.0);
        //Create the directory
        fs::DirBuilder::new().recursive(true).create(file_path.as_path())?;
        //Add the file to the path
        file_path.push("localization.json");

        println!("{}", file_path.display());

        let file = fs::File::create(file_path.as_path())?;
        let mut writer = BufWriter::new(file);
        data.1.save_to_json(&mut writer)?;
    }

    println!("All the JSON files have been created at {}", &dest_dir.display());
    Ok(())
    /*
        make a lib.rs and create some structures to make the program easier to use and read.

        https://godot.readthedocs.io/en/latest/tutorials/misc/locales.html#doc-locales
        https://godot.readthedocs.io/en/latest/tutorials/misc/internationalizing_games.html
        https://godot.readthedocs.io/en/latest/getting_started/workflow/assets/importing_translations.html#doc-importing-translations
    */
}