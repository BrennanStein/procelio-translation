// Procelio Localization Tool
// Copyright Brennan Stein 2020
use std::vec::Vec;
use serde::{Serialize, Deserialize};
use std::default::Default;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TextColor {
    #[serde(flatten)]
    pub color: (u8, u8, u8)
}

impl Default for TextColor {
    fn default() -> TextColor {
        TextColor { color: (255, 255, 255) }
    }
}

fn is_default<T: PartialEq + Default>(elem: &T) -> bool {
    *elem == Default::default()
}


pub fn lang_image_size() -> (u16, u16) {
    (48, 24)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub anglicized_name: String,
    pub native_name: String,
    pub authors: String,
    pub version: (u32, u32, u32),
    #[serde(skip)] 
    pub language_image: Vec<u8>, // RGBA in row-major order
    pub language_elements: Vec<TextElement>
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub field_name: String,
    pub field_value: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub text_size: u32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub underline: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub strikethrough: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub text_color: TextColor
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
            text_color: Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuildLocalizationConfig {
    pub enum_file: String, 
    pub output_folder: String,
    pub languages: std::vec::Vec<String>
}