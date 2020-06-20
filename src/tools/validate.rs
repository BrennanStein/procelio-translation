use std::collections::HashSet;
use std::path::Path;
use super::utils;
use crate::json::localization::{LanguageConfig};

fn validate_lang(language: &LanguageConfig, map: &utils::Mapping) -> bool {
    let mut elements = HashSet::new();
    for elem in &map.field_to_enum {
        elements.insert(elem.0);
    }
    let mut found_elements = HashSet::new();

    let mut errors = 0;
    eprintln!("============");
    if language.native_name.trim().is_empty() {
        errors += 1;
        eprintln!("Native language name empty");
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
    }

    for elem in elements {
        errors += 1;
        eprintln!("Missing definition for {}", elem);
    }
    eprintln!("============");
    println!("{} errors detected", errors);
    errors == 0
}


pub fn validate_localization(mut args: std::env::Args) {
    let lang = args.next();
    if lang.is_none() {
        eprintln!("No language provided!");
        eprintln!("Usage: localetool.exe validate ANGLICIZED_LANG [path/to/files/folder]");
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