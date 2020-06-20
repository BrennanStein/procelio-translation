// Procelio Localization Tool
// Copyright Brennan Stein 2020
use std::{fs::File, path::Path};
use std::io::BufReader;
use crate::json::localization;
use super::utils;
pub fn build_localization_files(mut args: std::env::Args) {
    let arg_folder = args.next().unwrap_or("./files".to_string());
    let config = utils::load_config(Path::new(&arg_folder));
    if let Err(e) = config {
        eprintln!("Building localization files failed {:#?}", e);
        return;
    }
    let config = config.unwrap();
    let mapping = utils::load_mapping_with_config(&Path::new(&arg_folder), &config);
    if let Err(e) = mapping {
        eprintln!("Building localization files failed {:#?}", e);
        return;
    }
    let mapping = mapping.unwrap();
}