// Procelio Translation Tool
// Copyright Brennan Stein 2020
use std::vec::Vec;
use serde::{Serialize, Deserialize};
use std::default::Default;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TextColor {
    #[serde(default="white_text")]
    pub text_color: (u8, u8, u8)
}

// serde needs a function to get the default 
fn white_text() -> (u8, u8, u8) {(255, 255, 255)}

impl Default for TextColor {
    fn default() -> TextColor {
        TextColor { text_color: white_text() }
    }
}

// Used so serde doesn't serialize default text values (save vertical space)
fn is_default<T: PartialEq + Default>(elem: &T) -> bool {
    *elem == Default::default()
}

// the size of the langauge image (width, height). Magic numbers.
pub fn lang_image_size() -> (u16, u16) {
    (48, 24)
}

// The "full" data for a translation
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

// Serialization functions for compiling a translation
impl LanguageConfig {
    // Write a single TextElement to the file at the current offset
    fn write_elem(&self, file: &mut Cursor<Vec<u8>>, text: &TextElement, map: &crate::tools::utils::Mapping) {
        let fval = map.get(&text.field_name);
        if let None = fval {
            eprintln!("MISSING {}; SKIPPING", &text.field_name);
            return;
        }

        let fval = fval.unwrap();
        file.write_all(&i32::to_be_bytes(*fval)).unwrap();
        let translation = text.field_value.as_bytes();
        file.write_all(&u32::to_be_bytes(translation.len() as u32)).unwrap();
        file.write_all(translation).unwrap();

        file.write_all(&u16::to_be_bytes(text.text_size as u16)).unwrap();
        let mut modifications : u8 = 0;
        if text.bold {
            modifications |= 1;
        }
        if text.italic {
            modifications |= 2;
        }
        if text.underline {
            modifications |= 4;
        }
        if text.strikethrough {
            modifications |= 8;
        }
        file.write_all(&u8::to_be_bytes(modifications)).unwrap();
        file.write_all(&u8::to_be_bytes(text.text_color.text_color.0)).unwrap();
        file.write_all(&u8::to_be_bytes(text.text_color.text_color.1)).unwrap();
        file.write_all(&u8::to_be_bytes(text.text_color.text_color.2)).unwrap();
    }

    // Compile "this" down to a network-serializable form (see docs/localization.md for format)
    pub fn compile(&self, map: &crate::tools::utils::Mapping) -> Vec<u8> {
        let mut file = Cursor::new(Vec::new());
        file.write_all(&u16::to_be_bytes(0)).unwrap(); // version
        let start_offset = file.position();
        file.seek(SeekFrom::Start(8 + start_offset)).unwrap(); // two offsets
        file.write_all(&u32::to_be_bytes(self.version.0)).unwrap();
        file.write_all(&u32::to_be_bytes(self.version.1)).unwrap();
        file.write_all(&u32::to_be_bytes(self.version.2)).unwrap();
        let anam = self.anglicized_name.as_bytes();
        file.write_all(&u8::to_be_bytes(anam.len() as u8)).unwrap();
        file.write_all(anam).unwrap();
        let nnam = self.native_name.as_bytes();
        file.write_all(&u32::to_be_bytes(nnam.len() as u32)).unwrap();
        file.write_all(nnam).unwrap();
        let autt = self.authors.as_bytes();
        file.write_all(&u32::to_be_bytes(autt.len() as u32)).unwrap();
        file.write_all(autt).unwrap();
     

        let pic_start = file.position();
        file.write_all(&self.language_image).unwrap();
        let data_start = file.position();
        file.seek(SeekFrom::Start(start_offset)).unwrap();
        file.write_all(&u32::to_be_bytes(pic_start as u32)).unwrap();
        file.write_all(&u32::to_be_bytes(data_start as u32)).unwrap();
        file.seek(SeekFrom::Start(data_start)).unwrap();
  
        file.write_all(&u32::to_be_bytes(self.language_elements.len() as u32)).unwrap();
        for elem in &self.language_elements {
            self.write_elem(&mut file, &elem, map);
        }

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut out = Vec::new();
        file.read_to_end(&mut out).unwrap();
        out
    }
}

// All of the data for a single translated UI text element
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
    #[serde(flatten, default, skip_serializing_if = "is_default")]
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
pub struct BuildTranslationConfig {
    pub enum_file: String, 
    pub output_folder: String,
    pub languages: std::vec::Vec<String>
}