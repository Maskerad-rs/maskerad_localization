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
    //Get all the arguments from the command line.
    let arguments: Vec<String> = env::args().collect();

    //Get a reference to the first argument: The csv file to read.
    let csv_file = &arguments[1];

    //Get a reference to the second argument: The destination directory.
    let dest_dir = &arguments[2];

    //Create the full path of the csv file to read.
    let fullpath_file = fs::canonicalize(csv_file)?;

    //Create the full path of the destination directory.
    let mut dest_dir = fs::canonicalize(dest_dir)?;

    //All the JSON files will be placed in the following directory hierarchy:
    //[destination directory]/localization/{locale}/localization.json

    //Push the localization directory to the path.
    dest_dir.push("localization");

    println!("Reading file at path: \n{}", fullpath_file.display());
    println!("The JSON files will be output in the directory: \n{}", dest_dir.display());

    //Open the file at the path given as the first argument.
    let file = fs::File::open(fullpath_file.as_path())?;

    //Create a CSV reader from the file, with custom options.

    //delimiter(b',') -> all fields must be delimited by a ','.

    //has_headers(false) -> The first line is treated as a record. If the option was
    //set to true, we would not be able to iterate over it when reading the csv file.

    //flexible(false) -> The number of fields in records (a row) are not allowed to change.
    //If row N has 5 fields, row N+1 must have 5 fields. Not 4, not 6.
    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b',').has_headers(false).flexible(false).from_reader(file);

    //Create an iterator from the CSV reader.
    let mut iter = csv_reader.deserialize();




    //Dictionary of manifests.
    //The manifests are mapped to a String representing the locale.
    let mut manifests = ManifestMap::new();

    //Language mapper.
    //The strings representing the locales are mapped to an index.
    let mut lang_mapper = LangMapper::new();

    //The counter used to map a language to an index.
    let mut counter: u8 = 0;



    //Read the header, the first line of the csv file.
    if let Some(result) = iter.next() {
      let header: Row = result?;

        //Iterate over the locales (all fields after the first).
        for lang in header.values().iter() {
            //Check that it is a valid locale.
            if locale::is_valid_locale(lang.as_str()) {
                // Add a manifest in the manifest map.
                manifests.insert_manifest(lang.as_str(), Manifest::new());

                //map the language to an index.
                lang_mapper.insert_mapping(counter, lang.as_str());

                //increment the counter.
                counter += 1;

            } else {
                return Err(LocalizationError::UnvalidLocale(format!("The text '{}' is not a valid locale form !", lang.as_str())))
            }
        }
    } else {
        return Err(LocalizationError::HeaderError(String::from("Could not find the first line of the CSV !")));
    }

    //Read all the records (rows after the first).
    for record in iter {
        //Reinitialize the counter.
        counter = 0;

        //add the ids, and the text mapped to those ids, in the appropriate manifest.
        let row: Row = record?;

        //Iterate over the text mapped to the id (the first field) of this row.
        for text in row.values().iter() {
            // Get the language of the text.
            let language = lang_mapper.get_mapping(counter)?;

            // Get the corresponding manifest.
            let corresponding_manifest = manifests.get_mut(language)?;

            // Update the manifest by inserting the id, and the text mapped to this id.
            corresponding_manifest.insert(row.key(), text.as_str());

            //Increment the counter
            counter += 1;
        }
    }

    //Iterate over the manifests.
    for data in manifests.iter() {
        //create the file at path [destination directory]/localization/{locale}/localization.json

        //[destination directory]/localization
        let mut file_path = dest_dir.clone();

        //[destination directory]/localization/{locale}
        file_path.push(data.0);

        //Create all the needed directories.
        //If they exist, do nothing.
        fs::DirBuilder::new().recursive(true).create(file_path.as_path())?;

        //Add the file's name to the path
        //[destination directory]/localization/{locale}/localization.json
        file_path.push("localization.json");

        //create the file, at the path defined above.
        let file = fs::File::create(file_path.as_path())?;

        //Wrap it in a bufwriter.
        let mut writer = BufWriter::new(file);

        //Save the manifest in JSON format, in the file created above.
        data.1.save_to_json(&mut writer)?;
    }

    // Done !
    println!("All the JSON files have been created at {}", &dest_dir.display());
    Ok(())
}