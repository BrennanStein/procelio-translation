// Procelio Localization Tool
// Copyright Brennan Stein 2020
use std::fs;
use std::collections::HashSet;
use std::path::Path;
use super::utils;
use crate::json::localization::{LanguageConfig, TextElement, lang_image_size};

fn create_new_lang(dirpath: &std::path::Path, lang_name: &str, map: utils::Mapping) {
    let mut elements = Vec::new();
    for elem in map.field_to_enum {
        elements.push(TextElement::new(elem.0));
    }
    let config = LanguageConfig {
        anglicized_name: lang_name.to_string(),
        native_name: "".to_string(),
        authors: "".to_string(),
        version: (0, 0, 1),
        language_elements: elements,
        language_image: Vec::new()
    };
    let path = dirpath.join(utils::LANGUAGE_FILE_NAME);
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    eprintln!("Writing serialize to {:#?}", path.display());
    let res = std::fs::write(path, serialized);
    if let Err(e) = res {
        eprintln!("Failed to create localization file: {:#?}", e);
        return;
    }

    let imgpath = dirpath.join(utils::LANGUAGE_IMAGE_NAME);
    let imgsize = lang_image_size();
    let mut img = image::ImageBuffer::new(imgsize.0.into(), imgsize.1.into());
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([0 as u8, 0 as u8, 0 as u8, 0 as u8]);
    }
    let save = img.save(imgpath);
    if let Err(e) = save {
        eprintln!("Failed to create localization image: {:#?}", e);
        return;
    }
}

fn update_existing_lang(dirpath: &std::path::Path, mut config: LanguageConfig, map: utils::Mapping) {
    let mut elems = map.field_to_enum;
    let mut gone = HashSet::new();
    for (index, exist) in config.language_elements.iter().enumerate() {
        if let None = elems.remove(&exist.field_name) {
            gone.insert(index);
        }
    }

    for index in gone {
        config.language_elements.remove(index);
    }

    for remaining in elems {
        config.language_elements.push(TextElement::new(remaining.0));
    }

    let path = dirpath.join(utils::LANGUAGE_FILE_NAME);
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    let res = std::fs::write(path, serialized);
    if let Err(e) = res {
        eprintln!("Failed to write updated elements to file: {:#?}", e);
        return;
    }
}

pub fn create_localization(mut args: std::env::Args) {
    let lang = args.next();
    if lang.is_none() {
        eprintln!("No language provided!");
        eprintln!("Usage: localetool.exe create ANGLICIZED_LANG [path/to/files/folder]");
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
    let res = fs::create_dir_all(&newlang);
    if let Err(e) = res {
        eprintln!("Could not create directory: {:#?}", e);
        return;
    }

    let data = utils::load_language(&newlang);
    match data {
        Err(_) => { println!("Generating new language file..."); create_new_lang(&newlang, &lang, mapping); }
        Ok(c) => { println!("Updating existing language file..."); update_existing_lang(&newlang, c, mapping); }
    }

    println!();
    println!();
    println!("File generation in {} successful!", newlang.display());
}