use std::{collections::HashMap, hash::Hash, path::Path};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;


#[derive(Parser)]
#[grammar="src/lang/data_lang/data_lang.pest"]
pub struct DataLangParser;

#[derive(Debug)]
enum DataType<'a> {
    PersistentEntity{_def: EntityDefHashMap<'a>, _attrs: AttrsHashMap<'a>}
}

#[derive(Debug)]
struct Module<'a> {
    _name: &'a str,
    _data_types: DataTypes<'a>
}

type DataTypes<'a> = HashMap<&'a str, Box<DataType<'a>>>;

pub fn parse_data_lang(_module_path: &Path, unparsed_file: &str)  {
    let mut module_name: &str = "";
    let mut data_types: DataTypes = HashMap::new();
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
    let module = Module{_name: module_name, _data_types: data_types};
    println!("module: {:#?}", module);
}

fn parse_module(pair: Pair<'_, Rule>) -> &str {
    pair.into_inner().next().unwrap().as_str() 
}


fn parse_entity(pair: Pair<Rule>) -> (&str, Box<DataType>) {
    let mut entity_def_hm: EntityDefHashMap = HashMap::new();
    let mut attrs_hm: AttrsHashMap = HashMap::new(); 
    for pairs in pair.into_inner() {
        match pairs.as_rule() {
            Rule::entity_def => entity_def_hm = parse_entity_def(pairs),
            Rule::attrs => attrs_hm = parse_attrs(pairs),
            _ => unreachable!(),
        };
    };
    (entity_def_hm["entity_name"], Box::new(DataType::PersistentEntity { _def: entity_def_hm, _attrs: attrs_hm }))
}

type EntityDefHashMap<'a> = HashMap<&'a str, &'a str>;
fn parse_entity_def(pair: Pair<Rule>) -> EntityDefHashMap {
    let mut hash_map: EntityDefHashMap = HashMap::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::entity_name => hash_map.insert("entity_name", pair.as_str()),
            Rule::entity_type => hash_map.insert("entity_type", pair.as_str()),
            _ => unreachable!(),
        };
    };
    hash_map
}

type AttrsHashMap<'a> = HashMap<&'a str, AttrHashMap<'a>>;
fn parse_attrs(pair: Pair<Rule>) -> AttrsHashMap {
    let mut attrs_hm: AttrsHashMap = HashMap::new();
    let mut attr_hm: AttrHashMap = HashMap::new();
    for p in pair.into_inner() {
        attr_hm.extend(parse_attr(p));
    }
    attrs_hm.insert("attrs", attr_hm);
    attrs_hm
}

type AttrHashMap<'a> = HashMap<&'a str, &'a str>; 
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
        attr_hm.insert(attr_name, attr_type);
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
