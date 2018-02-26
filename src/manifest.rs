// Copyright 2017 - 2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut};
use std::io::Write;
use errors::{LocalizationError, LocalizationResult};
use serde_json;

#[derive(Debug)]
pub struct ManifestMap(HashMap<String, Manifest>);

impl Default for ManifestMap {
    fn default() -> Self {
        ManifestMap(HashMap::new())
    }
}

impl ManifestMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_manifest<S, M>(&mut self, key: S, manifest: M) -> Option<Manifest> where
        S: Into<String>,
        M: Into<Manifest>,
    {
        self.0.insert(key.into(), manifest.into())
    }

    pub fn get_mut<S>(&mut self, key: S) -> LocalizationResult<&mut Manifest> where
        S: AsRef<str>
    {
        match self.0.get_mut(key.as_ref()) {
            Some(manifest) => {
                Ok(manifest)
            },
            None => {
                Err(LocalizationError::ManifestMapError(format!("The manifest mapped to the language {} could not be found !", key.as_ref())))
            }
        }
    }

    pub fn get<S>(&self, key: S) -> LocalizationResult<&Manifest> where
        S: AsRef<str>,
    {
        match self.0.get(key.as_ref()) {
            Some(manifest) => {
                Ok(manifest)
            },
            None => {
                Err(LocalizationError::ManifestMapError(format!("The manifest mapped to the language {} could not be found !", key.as_ref())))
            }
        }
    }

    pub fn count(&self) -> usize {
        self.0.iter().count()
    }

    pub fn iter(&self) -> Iter<String, Manifest> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<String, Manifest> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Serialize)]
pub struct Manifest(HashMap<String, String>);

impl Default for Manifest {
    fn default() -> Self {
        Manifest(HashMap::new())
    }
}

impl Manifest {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn save_to_json<W: Write>(&self, writer: &mut W) -> LocalizationResult<()> {
        let json_string = serde_json::to_string_pretty(&self).map_err(|json_error| {
            LocalizationError::from(json_error)
        })?;

        writer.write_all(json_string.as_bytes()).map_err(|io_error| {
            LocalizationError::from(io_error)
        })
    }

    pub fn insert<S>(&mut self, key: S, value: S) -> Option<String> where
        S: Into<String>
    {
        self.0.insert(key.into(), value.into())
    }
}

#[derive(Debug)]
pub struct LangMapper(HashMap<u8, String>);

impl Default for LangMapper {
    fn default() -> Self {
        LangMapper(HashMap::new())
    }
}

impl LangMapper {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_mapping<S>(&mut self, key: u8, lang: S) -> Option<String> where
        S: Into<String>
    {
        self.0.insert(key, lang.into())
    }

    pub fn get_mapping(&self, key: u8) -> LocalizationResult<&str> {
        match self.0.get(&key) {
            Some(lang) => {
                Ok(lang.as_str())
            },
            None => {
                Err(LocalizationError::LanguageMappingError(format!("Could not find a language mapped to the index {} !", key)))
            },
        }
    }
}