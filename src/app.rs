use std::{collections::HashMap, path::PathBuf};
use crate::error::Error;
use crate::lang::parser::{self, parse_path, parse_source, ModuleMap, ModuleTuple};
use crate::path::gimbal_files_in_path;

#[derive(Debug, Default)]
pub struct App {
    modules: ModuleMap
}

impl App {

    pub fn new() -> Self {
        App{ modules: HashMap::new()}
    }

    pub fn load(app_path: &PathBuf) -> Result<Self, Error> {
        let gimbal_files = gimbal_files_in_path(app_path)?;
        let modules: Vec<ModuleTuple>  = gimbal_files.into_iter().map(|y| Ok(parse_path(&y))?).collect::<Result<Vec<ModuleTuple>, parser::Error>>()?;
        let mut app = App::new();
        app = app.merge_modules(modules)?;
        Ok(app)
    }

    pub fn patch(mut self, source_code: &str) -> Result<Self, Error> {
        let mut modules: Vec<ModuleTuple> = Vec::new();
        let module = parse_source(source_code)?;
        modules.push(module);
        self.merge_modules(modules)
    }

    fn merge_modules(mut self, modules: Vec<ModuleTuple>) -> Result<Self, Error> {
        for module in modules {
            let existing_module = self.modules.remove(&module.0);
            if let Some(mut m) = existing_module {
                m.extend(module.1);
                self.modules.insert(module.0, m);
            } else {
                self.modules.insert(module.0, module.1);
            };
        }
        Ok(self)
    }

    pub fn module_exists(&self, module: &str) -> Option<Error> {
        if self.modules.is_empty() {
            Some(Error::no_app())
        } else {
            match self.modules.get(module) {
                None => Some(Error::no_module(module)),
                Some(_) => None,
            }
        }
    }

}
