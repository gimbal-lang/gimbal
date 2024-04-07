
use std::fmt;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

#[derive(Debug, Default)]
pub struct Error(String);

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



pub fn run() -> Result<(), Error>{
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(res) => {
                let input = res?;
                if &input == ":q" || &input == ":quit" {
                    break;
                }
                parse(&input);
            },
            None => {},
        }
    }
    Ok(())
}

fn parse(input: &str) {
    println!("\"{}\"", input);
}