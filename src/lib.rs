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
            _ => {}
        }
    }
//    tools::localizer::build_localization_files(args);
}