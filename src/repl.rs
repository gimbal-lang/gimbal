
mod error;

use std::{io::{stdin, stdout, Write}, path::Path};

use pest::iterators::Pair;

use pest_derive::Parser;
use pest::Parser;

use crate::{app::{self, App}, lang::parser::parse_app};

use self::error::Error;


#[derive(Parser)]
#[grammar="src/repl/repl.pest"]
struct ReplParser;



pub fn run() -> Result<(), Error>{
    let mut current_app = App::new();
    let mut _current_mod: Option<String> = None;
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(res) => {
                let input = res?;
                match input.as_str() {
                    ":q" | ":quit" => break,
                    "" => {},
                    _ => 
                    match parse(&input) {
                        Ok(cmd) => match cmd {
                            Command::Load(path) => {
                                current_app = parse_app(&Path::new(&path).to_path_buf())?;
                                println!("App loaded");
                            },
                            Command::Help => println!("Type :q to quit"),
                            Command::Set { key, value } => {
                                match key {
                                    SetType::Module => {
                                        match set_module(&current_app, &value) {
                                            Err(err) => println!("{}", err.repl_msg()),
                                            Ok(m) => {
                                                _current_mod = Some(m.to_string());
                                                println!("You are now in the `{}` module", m);
                                            }
                                        };
                                    },
                                }
                            },
                            _ => println!("{:?}", cmd),
                            }
                        Err(err) => println!("{}", err),
                    }
                }
            },
            None => {},
        };
    };
    Ok(())
}

fn parse(input: &str) -> Result<Command, Error>  {
    match ReplParser::parse(Rule::line, input)?.next() {
        Some(pair) => match pair.as_rule() {
            Rule::command => {
                match pair.into_inner().next() {
                    Some(p) => p.command(),
                    None => Ok(Command::Empty),
                }
            },
            Rule::gimbal => pair.into_inner().next().unwrap().command(),
            _ => unreachable!()
        }
        None => Ok(Command::Empty),
    }
}

fn set_module<'a>(app: &App, module: &'a str) -> Result<&'a str, app::Error> {
    match app.module_exists(module) {
        None => {
            Ok(module)
        },
        Some(err) => Err(err),
    }
}

trait GimbalPair {
    fn command(self) -> Result<Command, Error>;
    fn tos(self) -> String;
    fn inu(self) -> Self;
    fn set_cmd(self) -> Command;
}

impl GimbalPair for Pair<'_, Rule> {
    fn command(self) -> Result<Command, Error> {
        match self.as_rule() {
            Rule::help => Ok(Command::Help),
            Rule::reload => Ok(Command::Reload),
            Rule::set => Ok(self.set_cmd()), 
            Rule::load => Ok(Command::Load(self.inu().tos())),
            Rule::gimbal => Ok(Command::Gimbal(self.inu().tos())),
            _ => Ok(Command::Empty)
        }
    }
    
    fn tos(self) -> String {
        self.as_str().to_string()
    }

    fn inu(self) -> Self {
        self.into_inner().next().unwrap()
    }

    fn set_cmd(self) -> Command {
        let set = self.inu();
        match set.as_rule() {
            Rule::set_mod => Command::Set{key: SetType::Module, value: set.inu().tos()},
            _ => unreachable!(),
        }
    }

}


#[derive(Debug)]
enum Command {
    Empty,
    Help,
    Load(String),
    Reload,
    Gimbal(String),
    Set{key: SetType, value: String}
}

#[derive(Debug)]
enum SetType {
    Module
}
