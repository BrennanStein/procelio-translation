// Procelio Localization Tool
// Copyright Brennan Stein 2020
use std::io::prelude::*;
use std::{fs::File, path::Path, path::PathBuf};
use crate::json::localization;
use super::utils;

fn build_file(files_folder: &Path, language: &str, config: &localization::BuildLocalizationConfig, map: &utils::Mapping) -> bool {
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

fn parse_args(mut args: std::env::Args) -> (PathBuf, Option<String>) {
    let first_arg = args.next().unwrap_or("./files".to_string());
    let second_arg = args.next();
    if let Some(path) = second_arg {
        return (PathBuf::from(&path), Some(first_arg));
    }
    (PathBuf::from(&first_arg), None)
}

pub fn build_localization_files(args: std::env::Args) {
  
    let (arg, single_lang) = parse_args(args);
    let config = utils::load_config(&arg);
    if let Err(e) = config {
        eprintln!("Building localization files failed {:#?}", e);
        return;
    }
    let config = config.unwrap();
    let mapping = utils::load_mapping_with_config(&Path::new(&arg), &config);
    if let Err(e) = mapping {
        eprintln!("Building localization files failed {:#?}", e);
        return;
    }
    let mapping = mapping.unwrap();

    if let Some(l) = single_lang {
        build_file(&arg, &l, &config, &mapping);
    } else {
        for lang in &config.languages {
            build_file(&arg, lang, &config, &mapping);
        }
    }
}