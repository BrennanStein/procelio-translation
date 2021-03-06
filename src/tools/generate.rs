// Procelio Translation Tool
// Copyright Brennan Stein 2020
use std::io::prelude::*;
use std::{fs::File, path::Path, path::PathBuf};
use crate::json::translation;
use super::utils;

// Take the name of a language + config data, and bake that language down to the network file (see docs/translation.md)
fn build_file(files_folder: &Path, language: &str, config: &translation::BuildTranslationConfig, map: &utils::Mapping) -> bool {
    let loaded = utils::load_language(&files_folder.join(language));
    if loaded.is_err() {
        eprintln!("Failed to load {} {:#?}", language, loaded.err().unwrap());
        return false;
    }
    let loaded = loaded.unwrap();
    let bytes = loaded.compile(map);
    
    let output = files_folder.join(&config.output_folder);
    let output = output.join(language.to_owned() + ".loc");
    let file = File::create(&output);
    match file {
        Err(e) => { eprintln!("Unable to open file: {} {:#?}", output.display(), e); return false; }
        Ok(mut f) => { f.write_all(&bytes).unwrap(); }
    }
    true
}

// Parse and return (path/to/files/folder, language_to_translate)
// If no language given, will return None & default to what's in config.json
fn parse_args(mut args: std::env::Args) -> (PathBuf, Option<String>) {
    let first_arg = args.next().unwrap_or("./files".to_string());
    let second_arg = args.next();
    if let Some(path) = second_arg {
        return (PathBuf::from(&path), Some(first_arg));
    }
    (PathBuf::from(&first_arg), None)
}

// Command line interface for this. See README for purpose
pub fn build_translation_files(args: std::env::Args) {
    let (arg, single_lang) = parse_args(args);
    let config = utils::load_config(&arg);
    if let Err(e) = config {
        eprintln!("Building translation files failed {:#?}", e);
        return;
    }
    let config = config.unwrap();
    let mapping = utils::load_mapping_with_config(&Path::new(&arg), &config);
    if let Err(e) = mapping {
        eprintln!("Building translation files failed {:#?}", e);
        return;
    }
    let mapping = mapping.unwrap();

    if let Some(l) = single_lang {
        build_file(&arg, &l, &config, &mapping);
        println!("Generated {}", l);
    } else {
        for lang in &config.languages {
            build_file(&arg, lang, &config, &mapping);
            println!("Generated {}", lang);
        }
    }
}