use crate::lang::parser::ModuleMap;




#[derive(Debug, Default)]
pub struct App {
    module_map: Option<ModuleMap>
}

impl App {

    pub fn new() -> Self {
        App{ module_map: None}
    }

    pub fn load(module_map: ModuleMap) -> Self {
        App{ module_map: Some(module_map)}
    }

    pub fn module_exists(&self, module: &str) -> Option<Error> {
        match &self.module_map {
            None => Some(Error::NoApp),
            Some(m) => match m.contains_key(module) {
                false => Some(Error::NoModule(module.to_string())),
                true => None
            }
        }
    }

}

#[derive(Debug)]
pub enum Error {
    NoApp,
    NoModule(String),
}

impl Error {
    pub fn repl_msg(&self) -> String {
        match self {
            Error::NoApp => "No application loaded".to_string(),
            Error::NoModule(s) => format!("Module `{}` not found", s),
        }
    }
}