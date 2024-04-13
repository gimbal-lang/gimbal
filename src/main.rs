mod lang;
mod repl;
mod app;
mod path;
mod error;

fn main() {
    match repl::run() {
        Ok(()) => {},
        Err(err) => println!("{}", err),
    }
}

trait GimbalString  {
    fn tos(&self) -> String;
}

impl GimbalString for &str {
    fn tos(&self) -> String {
        self.to_string()
    }
}

