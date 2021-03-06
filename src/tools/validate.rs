// Procelio Translation Tool
// Copyright Brennan Stein 2020
use std::collections::HashSet;
use std::path::Path;
use super::utils;
use crate::json::translation::{LanguageConfig};

// Validate that the given LanguageConfig is valid, given the current defined text elements
// Return true iff no errors (missing definitions are allowed)
// Prints all errors/missings to stderr
fn validate_lang(language: &LanguageConfig, map: &utils::Mapping) -> bool {
    let mut elements = HashSet::new();
    for elem in &map.field_to_enum {
        elements.insert(elem.0);
    }
    let mut found_elements = HashSet::new();

    let mut errors = 0;
    let mut missing = 0;
    eprintln!("============");
    if language.native_name.trim().is_empty() {
        errors += 1;
        eprintln!("Native language name empty");
    }
    if language.anglicized_name.as_bytes().len() > 254 {
        errors += 1;
        eprintln!("Anglicized name too long! Must be < 256 bytes");
    }

    for element in &language.language_elements {
        if found_elements.contains(&element.field_name) {
            errors += 1;
            eprintln!("Duplicate element: {}", element.field_name);
        }
        found_elements.insert(element.field_name.to_owned());

        if !elements.contains(&element.field_name) {
            errors += 1;
            eprintln!("Element does not exist in UI: {}", element.field_name);
        }
        elements.remove(&element.field_name);

        if element.field_value.trim().is_empty() {
            errors += 1;
            eprintln!("Text for {} is empty", element.field_name);
        }

        if element.text_size > 2048 {
            errors += 1;
            eprintln!("Font too big for {}", element.field_name);
        }
    }

    for elem in elements {
        missing += 1;
        eprintln!("Missing definition for {}", elem);
    }
    eprintln!("============");
    println!("{} errors detected", errors);
    println!("{} elements missing", missing);

    errors == 0
}

// Command line API for file translation. Given a language name, find its files and validate them
pub fn validate_translation(mut args: std::env::Args) {
    let lang = args.next();
    if lang.is_none() {
        eprintln!("No language provided!");
        eprintln!("Usage: executableName.exe validate ANGLICIZED_LANG [path/to/files/folder]");
        return;
    }
    let lang = lang.unwrap();

    let arg_folder = args.next().unwrap_or("./files".to_string());
    let arg_folder = Path::new(&arg_folder);
    let mapping = utils::load_mapping(&arg_folder);
    if mapping.is_err() {
        eprintln!("Could not load mapping of text field enum data {:#?}", mapping.err());
        return;
    }
    let mapping = mapping.unwrap();

    let newlang = arg_folder.join(&lang);

    let data = utils::load_language(&newlang);
    match data {
        Err(_) => { println!("Unable to parse JSON file..."); return; }
        Ok(c) => { 
            if lang != c.anglicized_name {
                eprintln!("Error: anglicized name must match folder name");
            } 
            else {
                validate_lang(&c, &mapping);
            }
         }
    }
}