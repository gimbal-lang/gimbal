mod lang;
mod repl;
mod app;

fn main() {
    // let app_path: PathBuf = Path::new("tests/example/").to_path_buf();
    // let module_map = parse_app(&app_path);
    // match module_map {
    //     Ok(m) => println!("modules: {:#?}", m),
    //     Err(e) => println!("{}", e),
    // }
    match repl::run() {
        Ok(()) => {},
        Err(err) => println!("{}", err),
    }
}


