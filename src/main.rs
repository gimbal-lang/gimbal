use std::path::Path;

use lang::data_lang::data_lang_parser::parse_gmd_files;

mod lang;
fn main() {
    let app_path = Path::new("tests/example");
    let _modules = parse_gmd_files(app_path);
    
}


