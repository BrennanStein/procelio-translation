use std::vec::Vec;
use serde::{Serialize, Deserialize};
fn whitefont() -> (u8, u8, u8) {
    (255, 255, 255)
}

pub fn lang_image_size() -> (u16, u16) {
    (48, 24)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub anglicized_name: String,
    pub native_name: String,
    #[serde(skip)] 
    pub language_image: Vec<u8>, // RGBA in row-major order
    pub language_elements: Vec<TextElement>
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub field_name: String,
    pub field_value: String,
    #[serde(default)]
    pub text_size: u32,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default="whitefont")]
    pub text_color: (u8, u8, u8),
}

impl TextElement {
    pub fn new(name: String) -> TextElement {
        TextElement {
            field_name: name,
            field_value: "".to_string(),
            text_size: 0,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            text_color: whitefont()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuildLocalizationConfig {
    pub enum_file: String, 
    pub output_folder: String,
    pub languages: std::vec::Vec<String>
}