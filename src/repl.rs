

use std::{io::{stdin, stdout, Write}, path::Path};

use pest::iterators::Pair;

use pest_derive::Parser;
use pest::Parser;

use crate::{app::App, GimbalString};
use crate::error::Error;



#[derive(Parser)]
#[grammar="src/repl/repl.pest"]
struct ReplParser;



pub fn run() -> Result<(), Error>{
    let mut current_app = App::new();
    let mut current_mod: Option<String> = None;
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
                                current_app = App::load(&Path::new(&path).to_path_buf())?;
                                println!("App loaded");
                            },
                            Command::Help => println!("Type :q to quit"),
                            Command::Set { key, value } => {
                                match key {
                                    SetType::Module => {
                                        match set_module(&current_app, &value) {
                                            Err(err) => println!("{}", err.to_string()),
                                            Ok(m) => {
                                                current_mod = Some(m.to_string());
                                                println!("You are now in the `{}` module", m);
                                            }
                                        };
                                    },
                                }
                            },
                            Command::Gimbal(code) => {
                                match &current_mod {
                                    None => println!("You are not in a module. Use the command `:set module <module_name>`"),
                                    Some(m) => {
                                        let source_code = format!("module {} {}", m, code);
                                        current_app = current_app.patch(&source_code)?;
                                        
                                    }
                                }
                            }
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
            Rule::gimbal => pair.command(),
            _ => unreachable!()
        }
        None => Ok(Command::Empty),
    }
}

fn set_module<'a>(app: &App, module: &'a str) -> Result<&'a str, Error> {
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
            Rule::gimbal => Ok(Command::Gimbal(self.tos())),
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

impl Command {
    fn help(&self) -> String {
        match self {
            Command::Help => ":help\tshows this help".tos(),
            Command::Load(_) => ":load <path>\tloads an application from <path>".tos(),
            Command::Set { key: _, value: __ } => ":set mod <module>\tsets the module you are working in".tos(),
            _ => "".tos(),
        }
    }
}

#[derive(Debug)]
enum SetType {
    Module
}
