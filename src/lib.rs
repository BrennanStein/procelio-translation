// Procelio Translation Tool
// Copyright Brennan Stein 2020
mod tools;
mod json;
pub fn libmain() {
    let mut args = std::env::args();
    args.next(); // consume name
    let command = args.next();
    if let Some(val) = command {
        match val.as_str() {
            "create" => { tools::create::create_translation(args); },
            "validate" => { tools::validate::validate_translation(args); },
            "generate" => { tools::generate::build_translation_files(args); },
            _ => {
                println!("TranslationTool.exe [create|validate|generate] args");
            }
        }
    } else {
        println!("TranslationTool.exe [create|validate|generate] args");
    }
}