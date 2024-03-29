use std::{collections::HashMap, fs, hash::Hash, path::Path};

use pest::{iterators::{Pair, Pairs}, Parser};
use pest_derive::Parser;


#[derive(Parser)]
#[grammar="src/lang/data_lang/data_lang.pest"]
pub struct DataLangParser;

type Modules = HashMap<String, Module>;

#[derive(Debug)]
enum Node {

    // file = {  module_def  ~ (data_type)+  }
    File(Box<Node>, Vec<Node>),

    // module_def = { "module" ~ module_name ~ ";" }
    ModuleDef(String),

    //data_type = { (persistent_entity | string_type ) ~ ";"}
    DataType(Box<Node>),

    // string_type = { "String" }
    StringType,

    // persistent_entity = { "persistent" ~ entity_def ~ attrs?}
    PersistantEntity(Box<Node>, Box<Option<Node>>),

    //entity_def = { entity_name ~ (extends)?}
    EntityDef(Box<Node>, Box<Option<Node>>),

    // attrs = { "{" ~ attr ~ ("," ~ attr)* ~ ","? ~ "}" }
    Attrs(Vec<Node>),

    //attr = { attr_name ~":"~ data_type }
    Attr(Box<Node>, Box<Node>),

    // attr_name = ${ ASCII_ALPHA_LOWER ~ (ASCII_ALPHA_LOWER | "_")* }
    AttrName(String),

    // entity_name = ${ ASCII_ALPHA_UPPER ~ ASCII_ALPHA+  }
    EntityName(String),

    // module_name = ${ ASCII_ALPHA_LOWER ~ (ASCII_ALPHA_LOWER | "_")* }
    ModuleName(String),

    // qualified_datatype_name = {  module_name ~ "." ~ entity_name }
    QualifiedDataTypeName(Box<Node>, Box<Node>),

    //extends = { "extends" ~ qualified_datatype_name }
    Extends(Box<Node>),
}

#[derive(Debug)]
pub struct  Module {
    pub name: String,
    pub data_types: Box<DataTypes>
}

#[derive(Debug)]
pub enum DataType {
    PersistentEntity{def: Box<EntityDef>, attrs: Box<Attrs>}
}

#[derive(Debug)]
struct EntityDef {
    entity_name: String,
    extends: Box<DataTypeQualifiedName>
}

pub type DataTypes = HashMap<String,  DataType>;
type Attrs = HashMap<String, Box<AttrHash>>;


#[derive(Debug)]
struct DataTypeQualifiedName {
    module_name: String, type_name: String
}

pub fn parse_gmd_files(app_path: &Path) -> Modules  {
    let paths = fs::read_dir(app_path).expect("Project folder not found");
    let mut modules: Modules = HashMap::new();
    for file_dir in paths {
        let path = file_dir.unwrap().path();
        if path.is_file() && path.extension().is_some_and(|x| x=="gmd") {
            let unparsed_file = fs::read_to_string(&path).expect("file not found");
            let module = parse_data_lang(&unparsed_file);
            let name = module.name.to_string();
            let previous_module = modules.remove(&name);
            match previous_module {
                Some(pm) => {
                    let merged_data_types = module.data_types.into_iter().chain(pm.data_types.into_iter()).map(|x| x).collect();
                    modules.insert(name.clone(), Module{name: name.clone(), data_types: Box::new(merged_data_types)});

                },
                None => {modules.insert(name.clone(), module);},
            };
        }
    };

    println!("modules: {:#?}", modules);
    modules
}


fn parse_data_lang(unparsed_file: &str) -> Module {
    let mut module_name: String = String::new();
    let mut data_types = DataTypes::new();
    let pairs = DataLangParser::parse(Rule::file, unparsed_file).expect("unsuccesful parse").next().unwrap().into_inner();
    for p in pairs {
        match p.as_rule() {
            Rule::statement => {
                let statement = p.into_inner().next().unwrap();
                //println!("*statement: {:#?}\n\n", statement);
                match statement.as_rule() {
                    Rule::persistent_entity => data_types.extend(HashMap::from([parse_persistent_entity(statement)])),
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


fn parse_persistent_entity(pair: Pair<Rule>) -> (String, DataType) {
    let mut entity_def = EntityDef::new();
    for pairs in pair.into_inner() {
        match pairs.as_rule() {
            Rule::entity_def => entity_def = parse_entity_def(pairs),
            Rule::attrs => attrs_hm = parse_attrs(pairs),
            _ => unreachable!(),
        };
    };
    (entity_def_hm["entity_name"].clone(), DataType::PersistentEntity { def: Box::new(entity_def_hm), attrs: Box::new(attrs_hm) })
}

fn parse_entity_def(pair: Pair<Rule>) -> EntityDef {

    // let mut hash_map: HashMap<&str, &str> = HashMap::new();

    // let nodes: Vec<Node> = pair.into_inner().map(f)

    // for pair in pair.into_inner() {
    //     match pair.as_rule() {
    //         Rule::entity_name => hash_map.insert("entity_name".to_string(), pair.as_str().to_string()),
    //         Rule::extends => hash_map.insert("extends".to_string(), pair.into_inner().next().unwrap().as_str().to_string()),
    //         _ => unreachable!(),
    //     };
    // };
    // EntityDef { entity_name: hash_map["entity_name"].to_string(), extends: hash_map["extends"].to_string() }
}

trait Stringable {
    fn tos(&self) -> String;
}

impl Stringable for Pair<'_, Rule> {
    fn tos(&self) -> String {
        self.as_str().to_string()
    }
}

trait Boxable {
    fn bx(self) -> Box<Self>;
}

impl Boxable for Node {
    fn bx(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T> Boxable for Option<T> {
    fn bx(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Node {
    fn bx(self) -> Box<Node> {
        Box::new(self)
    }

    fn node(pair: &Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::file => Node::file(&pair),
            Rule::module_def => Node::module_def(&pair),
            Rule::data_type => Node::data_type(&pair),
            Rule::persistent_entity => Node::persistent_entity(&pair),
            Rule::string_type => Node::string_type(&pair),
            Rule::entity_def => Node::entity_def(&pair),
            Rule::attrs => Node::attrs(&pair),
            Rule::entity_name => Node::entity_name(&pair),
            Rule::extends => Node::extends(&pair.into_inner()),
        }
    }

    fn required(pairs: &Pairs<Rule>, n: usize) -> Node{
        Node::node(&pairs.nth(0).unwrap())
    }

    fn optional(pairs: &Pairs<Rule>, n: usize) -> Option<Node> {
        pairs.nth(0).map(|x| Node::node(&x)) 
    }

    fn file(pair: &Pair<Rule>) -> Node {
        let pairs = pair.into_inner();
        let module_def = Node::required(&pairs, 0).bx();
        let data_types = Node::optional(&pairs, 1);
        Node::File(module_def, data_types)
    }

    fn module_def(pair: &Pair<Rule>) -> Node {
        Node::ModuleDef(pair.tos())
    }

    fn data_type(pair: &Pair<Rule>) -> Node {
        let pairs = pair.into_inner();
        let 
        Node::DataType(Node::node())
    }

    fn string_type(pair: &Pair<Rule>) -> Node {
        Node::StringType
    }

    fn attrs(pair: &Pair<Rule>) -> Node {
        let pairs = pair.into_inner();
        Node::Attrs(pairs.into_iter().map(|x| Node::node(&x)).collect())
    }

    fn attr(pairs: &Pairs<Rule>) -> Node {
        let attr_name = Node::required(&pairs, 0).bx();
        let data_type = Node::required(&pairs, 1).bx(); 
        Node::Attr(attr_name, data_type)
    }

    fn persistent_entity(pair: &Pair<Rule>) -> Node {
        let pairs = pair.into_inner();
        let entity_def = Node::required(&pairs, 0).bx();
        let attrs = Node::optional(&pairs, 1).bx();
        Node::PersistantEntity(entity_def, attrs)
    }

    fn entity_def(pair: &Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let entity_name = Node::required(&pairs, 0).bx();
        let extends = Node::optional(&pairs, 1).bx(); 
        Node::EntityDef(entity_name, extends)
    }

    fn extends(pairs: &Pairs<Rule>) -> Node {
        Node::Extends(Node::qualified_data_type_name(&pairs).bx())
    }

    fn qualified_data_type_name(pair: &Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let module_name = Node::required(&pairs, 0).bx();
        let data_type_name = Node::required(&pairs, 1).bx();
        Node::QualifiedDataTypeName(module_name, data_type_name)
    }

    fn entity_name(pair: &Pair<Rule>) -> Node {
        Node::EntityName(pair.tos())
    }

    fn module_name(pair: &Pair<Rule>) -> Node {
        Node::ModuleName(pair.tos())
    }

}


fn parse_entity_qualified_name(pair: Pair<Rule>) -> DataTypeQualifiedName {
    let names: HashMap<&str, &str> = pair.into_inner().map(|p| {
        match p.as_rule() {
            Rule::module_name => ("module_name", p.as_str()),
            Rule::entity_name => ("entity_name", p.as_str()),
            _ => unreachable!(),
        }
    }).collect();
    DataTypeQualifiedName {module_name: names["module_name"].to_string(), type_name: names["object_name"].to_string()}
}

fn parse_attrs(pair: Pair<Rule>) -> Attrs {
    let mut attrs_hm: Attrs = HashMap::new();
    let mut attr_hm: AttrHash = HashMap::new();
    for p in pair.into_inner() {
        attr_hm.extend(parse_attr(p));
    }
    attrs_hm.insert("attrs".to_string(), Box::new(attr_hm));
    attrs_hm
}

type AttrHash = HashMap<String, String>; 
fn parse_attr(pair: Pair<Rule>) -> AttrHash {
    let mut attr_hm: AttrHash = HashMap::new();
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

