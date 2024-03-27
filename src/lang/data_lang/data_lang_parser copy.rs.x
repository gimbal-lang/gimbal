use std::{collections::HashMap, fs, hash::Hash, path::Path};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;


#[derive(Parser)]
#[grammar="src/lang/data_lang/data_lang.pest"]
pub struct DataLangParser;

type Modules = HashMap<String, Module>;
#[derive(Debug, Clone)]
pub enum DataType {
    PersistentEntity{def: Box<EntityDefHashMap>, attrs: Box<AttrsHashMap>}
}

#[derive(Debug, Clone)]
pub struct  Module {
    pub name: String,
    pub data_types: Box<DataTypesHM>
}


pub type DataTypesHM = HashMap<String,  DataType>;

pub fn parse_gmd_files(app_path: &Path) -> Modules  {
    let paths = fs::read_dir(app_path).unwrap();
    let mut module_list: Vec<Module> = Vec::new();
    let mut modules: Modules = HashMap::new();
    for file_dir in paths {
        let path = file_dir.unwrap().path();
        if path.is_file() && path.extension().is_some_and(|x| x=="gmd") {
            let unparsed_file = fs::read_to_string(&path).expect("file not found");
            let module = parse_data_lang( &unparsed_file);
            let name = module.name.to_string();
            modules.
        }
    };

    for module in module_list.into_iter() {
        modules.insert(module.name.clone(), module_list.into_iter().filter(|x| x.name == module.name));
    }
    println!("files: {:#?}", module_list);
    let modules = HashMap::new();
    // let modules = merge_modules(&mut files);
    // println!("modules: {:#?}", &modules);
    modules
}

// fn _merge_modules(module_list: &mut Vec<Module>) -> Modules {
//     let mut modules: Modules = HashMap::new();
//     for module in module_list {
//         modules.insert(module.name, )
//     }
//     modules
// }

fn parse_data_lang(unparsed_file: &str) -> Module {
    let mut module_name: String = String::new();
    let mut data_types = DataTypesHM::new();
    let pairs = DataLangParser::parse(Rule::file, unparsed_file).expect("unsuccesful parse").next().unwrap().into_inner();
    for p in pairs {
        match p.as_rule() {
            Rule::statement => {
                let statement = p.into_inner().next().unwrap();
                //println!("*statement: {:#?}\n\n", statement);
                match statement.as_rule() {
                    Rule::persistent_entity => data_types.extend(HashMap::from([parse_entity(statement)])),
                    _ => unreachable!(),
                };

            }
            Rule::module => module_name = parse_module(p),
            _ => ()
        };
    };
    let module = Module{name: module_name, data_types: Box::new(data_types)};
    module
}

fn parse_module(pair: Pair<'_, Rule>) -> String {
    pair.into_inner().next().unwrap().as_str().to_string()
}


fn parse_entity(pair: Pair<Rule>) -> (String, DataType) {
    let mut entity_def_hm: EntityDefHashMap = HashMap::new();
    let mut attrs_hm: AttrsHashMap = HashMap::new(); 
    for pairs in pair.into_inner() {
        match pairs.as_rule() {
            Rule::entity_def => entity_def_hm = parse_entity_def(pairs),
            Rule::attrs => attrs_hm = parse_attrs(pairs),
            _ => unreachable!(),
        };
    };
    (entity_def_hm["entity_name"].clone(), DataType::PersistentEntity { def: Box::new(entity_def_hm), attrs: Box::new(attrs_hm) })
}

type EntityDefHashMap = HashMap<String, String>;
fn parse_entity_def(pair: Pair<Rule>) -> EntityDefHashMap {
    let mut hash_map: EntityDefHashMap = HashMap::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::entity_name => hash_map.insert("entity_name".to_string(), pair.as_str().to_string()),
            Rule::entity_type => hash_map.insert("entity_type".to_string(), pair.as_str().to_string()),
            _ => unreachable!(),
        };
    };
    hash_map
}

type AttrsHashMap = HashMap<String, Box<AttrHashMap>>;
fn parse_attrs(pair: Pair<Rule>) -> AttrsHashMap {
    let mut attrs_hm: AttrsHashMap = HashMap::new();
    let mut attr_hm: AttrHashMap = HashMap::new();
    for p in pair.into_inner() {
        attr_hm.extend(parse_attr(p));
    }
    attrs_hm.insert("attrs".to_string(), Box::new(attr_hm));
    attrs_hm
}

type AttrHashMap = HashMap<String, String>; 
fn parse_attr(pair: Pair<Rule>) -> AttrHashMap {
    let mut attr_hm: AttrHashMap = HashMap::new();
        let mut attr_name: &str = "";
        let mut attr_type: &str = "";
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::attr_name => attr_name = p.as_str(),
            Rule::attr_type => attr_type = p.as_str(),
            _ => unreachable!(),
        };
        attr_hm.insert(attr_name.to_string(), attr_type.to_string());
    };
    attr_hm
}

// pub struct DataStatement {
//     keyword: String,
//     name: String,
//     qualifiers: Vec<String>
// }

// #[derive(Debug)]
// enum Statement {
//     Entity{_name: String, _qualifiers: Vec<String>},
// }

// struct EntityDef {
//     name: String,
//     entity_type: String,
// }
