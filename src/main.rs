use std::{fs, path::Path};

use lang::data_lang::data_lang_parser::parse_file;

mod lang;
fn main() {
    let app_path = Path::new("tests/example/person.gmd");
    let files = parse_file(&fs::read_to_string(app_path).expect("file not found"), app_path.to_str().unwrap());
    println!("files: {:#?}", files);
}


