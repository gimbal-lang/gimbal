use std::fs;
use std::path::Path;
use crate::lang::data_lang::data_lang_parser::parse_data_lang;
mod lang;
fn main() {
    let module_path = Path::new("tests/example/person.gmd");
    let unparsed_file = fs::read_to_string(module_path).expect("file not found");
    parse_data_lang(&module_path, &unparsed_file);
    
}