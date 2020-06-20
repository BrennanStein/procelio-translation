// Procelio Localization Tool
// Copyright Brennan Stein 2020
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use image::{imageops, Pixel};
use crate::json;
pub const LANGUAGE_FILE_NAME: &str = "language.json";
pub const LANGUAGE_IMAGE_NAME: &str = "image.png";
pub const CONFIG_FILE_NAME: &str = "config.json";
pub struct Mapping {
    pub field_to_enum: HashMap<String, i32>
}
impl Mapping {
    pub fn new(map: HashMap<String, i32>) -> Mapping {
        Mapping { field_to_enum: map }
    }

    pub fn get(&self, val: &str) -> Option<&i32> {
        self.field_to_enum.get(val)
    }
}

/// Load the mapping, given the path to the root localization folder
pub fn load_mapping(files_folder: &std::path::Path) -> Result<Mapping, Box<dyn std::error::Error>> {
    let config = load_config(files_folder);
    match config {
        Err(e) => Err(e),
        Ok(o) => load_mapping_with_config(files_folder, &o)
    }
}

pub fn load_mapping_with_config(files_folder: &std::path::Path, config: &json::localization::BuildLocalizationConfig) -> Result<Mapping, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(files_folder.join(&config.enum_file));
    if contents.is_err() {
        eprintln!("Error loading localization mapping {:#?}", contents);
        return Err(contents.unwrap_err().into());
    }
    let contents = contents?;
    let mut contents = contents.split(|c|c=='{' || c=='}');
    contents.next();
    let contents = contents.next().unwrap_or("ERR");
    let split = contents.split(',');
    let mut index = 0;
    let mut map = HashMap::new();
    for tok in split {
        if tok.contains('=') {
            let mut toksplit = tok.split('=');
            let key = toksplit.next();
            let val = toksplit.next();
            if let Some(k) = key {
                if let Some(v) = val {
                    index = v.trim().parse()?;
                    map.insert(k.trim().to_string(), index);
                    index += 1;
                }
            }
        } else {
            map.insert(tok.trim().to_string(), index);
            index += 1;
        }
    }
    println!("{} text mappings loaded", map.len());
    Ok(Mapping::new(map))
}

pub fn load_config(files_folder: &std::path::Path) -> Result<json::localization::BuildLocalizationConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new(&files_folder).join(CONFIG_FILE_NAME);
    let file = File::open(&config_path);
    if file.is_err() {
        eprintln!("Could not open localization config file {:#?} {:#?}", &config_path.display(), file);
        return Err(file.err().unwrap().into());
    }
    let config = serde_json::from_reader(BufReader::new(file.unwrap()));
    if config.is_err() {
        let err = config.err();
        eprintln!("Could not parse localization config file {:#?}", &err);
        return Err(err.unwrap().into());
    }
    let config: json::localization::BuildLocalizationConfig = config.unwrap();
    Ok(config)
}

pub fn load_language(lang_folder: &std::path::Path) -> Result<json::localization::LanguageConfig, Box<dyn std::error::Error>> {
    let file = File::open(&lang_folder.join(LANGUAGE_FILE_NAME));
    if file.is_err() {
        eprintln!("Could not open language file {:#?}", &lang_folder.display());
        return Err(file.err().unwrap().into());
    }
    let lang = serde_json::from_reader(BufReader::new(file.unwrap()));
    if lang.is_err() {
        let err = lang.err();
        eprintln!("Could not parse localization config file {:#?}", &err);
        return Err(err.unwrap().into());
    }
    let mut lang: json::localization::LanguageConfig = lang.unwrap();
    let img = image::open(&lang_folder.join(LANGUAGE_IMAGE_NAME));
    let siz = json::localization::lang_image_size();
    lang.language_image.reserve((siz.0*siz.1).into());
        match img {
            Err(_) => {
                for _ in 0..siz.0*siz.1 {
                    lang.language_image.push(0); // R
                    lang.language_image.push(0); // G
                    lang.language_image.push(0); // B
                    lang.language_image.push(0); // A
                }
            },
            Ok(pic) => {
                let buf = pic;
                let buf = imageops::resize(&buf, siz.0.into(), siz.1.into(), imageops::FilterType::Nearest);
                for x in 0..siz.0 {
                    for y in 0..siz.1 {
                        let pix = buf.get_pixel(x.into(), y.into()).to_rgba();
                        lang.language_image.push(pix[0]); // R
                        lang.language_image.push(pix[1]); // G
                        lang.language_image.push(pix[2]); // B
                        lang.language_image.push(pix[3]); // A
                    }
                }
            }
        };
    Ok(lang)
}