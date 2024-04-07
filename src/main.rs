use std::path::{Path, PathBuf};

use lang::data_lang::parser::parse_app;

mod lang;
mod error;






fn main() {
    let app_path: PathBuf = Path::new("tests/example/").to_path_buf();
    let module_map = parse_app(&app_path);
    match module_map {
        Ok(m) => println!("modules: {:#?}", m),
        Err(e) => println!("{}", e),
    }
}


