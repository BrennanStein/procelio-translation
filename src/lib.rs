// Procelio Localization Tool
// Copyright Brennan Stein 2020
mod tools;
mod json;
pub fn libmain() {
    let mut args = std::env::args();
    args.next(); // consume name
    let command = args.next();
    if let Some(val) = command {
        match val.as_str() {
            "create" => { tools::create::create_localization(args); },
            "validate" => { tools::validate::validate_localization(args); },
            "generate" => { tools::generate::build_localization_files(args); }
            _ => {
                println!("./executableName [create|validate|generate] args");


            }
        }
    }
}