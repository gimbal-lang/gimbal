//#![allow(dead_code, unused_attributes)]
use std::{collections::HashMap, ffi::OsStr, fmt::Debug, fs, hash::Hash, path::PathBuf};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub use super::error::{Error, GimbalResult};


#[derive(Parser)]
#[grammar="src/lang/lang.pest"]
struct DataLangParser;


type PestError = pest::error::Error<Rule>;
type NonLeafNodeMap = HashMap<String, Node>;
pub type ModuleMap =  HashMap<String, NonLeafNodeMap>;

pub type ModuleTuple = (String, NonLeafNodeMap);




#[derive(Debug)]
pub enum Node {
    File(Box<Node>),
    Module(OfModule),
    NonLeafNode(OfNonLeafNode),
    TypeName(String),
    Entity{extends: Option<String>, attrs: HashMap<String, Node>},
}



#[derive(Debug)]
pub struct OfModule{ _s: String, _h: HashMap<String, Node> }


#[derive(Debug)]
pub struct OfNonLeafNode{ s: String, n: Box<Node>}

impl OfNonLeafNode {
    fn to_tuple(self) -> (String, Node) {
        (self.s, *self.n)
    }
}

// pub fn parse_app(app_path: &PathBuf) -> Result<App, Error>  {
//     let paths_to_parse = paths_to_parse(app_path)?;
//     let modules: Vec<ModuleTuple>  = paths_to_parse.into_iter().map(|y| Ok(parse_path(&y))?).collect::<Result<Vec<ModuleTuple>, Error>>()?;
//     //println!("modules: {:#?}", modules);
//     let mut module_map: ModuleMap = HashMap::new();
//     for module in modules {
//             let existing_module = module_map.remove(&module.0);
//         if let Some(mut em) = existing_module {
//             em.extend(module.1);
//             module_map.insert(module.0, em)
//         } else {
//             module_map.insert(module.0, module.1)
//         };
//     };
//     Ok(App::load_module_map(module_map))
// }



fn paths_to_parse(app_path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let read_dir = fs::read_dir(app_path).togr(app_path)?;
    Ok(read_dir 
    .map(|res| res.ok())
    .filter(|dir_opt| dir_opt.is_some())
    .map(|dir_opt| dir_opt.unwrap().path())
    .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("gmd")))
    .collect::<Vec<PathBuf>>())
}

pub fn parse_path(path: &PathBuf) -> Result<ModuleTuple, Error> {
    parse_source(&fs::read_to_string(&path).togr(path)?).togr(path)
}

pub fn parse_source(source_code: &str) -> Result<ModuleTuple, PestError> {
    let mut pairs = DataLangParser::parse(Rule::file, source_code)?;
    let file: Node = pairs.next().unwrap().node();
    let module = if let Node::File(n) = file {
       if let Node::Module(of_module) = *n {
        (of_module._s, of_module._h)
       } else {
        unreachable!()
       }
    } else {
        unreachable!()
    };
    Ok(module)
}

trait GimbalPair {
    fn tos(&self) -> String;
    fn node(self) -> Node;
}


impl GimbalPair for Pair<'_, Rule> {
    fn tos(&self) -> String {
        self.as_str().to_string()
    }

    fn node(self) -> Node {
        match self.as_rule() {
            Rule::file => Node::file(self),
            Rule::type_def => Node::type_def(self),
            Rule::entity_def => Node::entity_def(self),
            Rule::type_alias => Node::type_alias(self),
            Rule::entity => Node::entity(self),
            Rule::type_name => Node::type_name(self),
            Rule::attr => Node::attr(self),
            _ => unreachable!(),
        }
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

    fn file(pair: Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let module_name = pairs.next().unwrap().into_inner().next().unwrap().tos();
        let nodes = pairs.map(|x| x.node()).map(|n|{
            if let Node::NonLeafNode(of_nln) = n {
                (of_nln.s, *of_nln.n)
            } else {
                unreachable!()
            }
        } 
        ).collect();
        Node::File(Node::Module(OfModule{ _s: module_name, _h: nodes }).bx())
    }

    fn type_def(pair: Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        pairs.next().unwrap().node()
    }

    fn entity_def(pair: Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let entity = pairs.next().unwrap();
        // println!("entity: {:#?}", entity);
        let entity_name = entity.clone().as_str().to_string();
        Node::NonLeafNode(OfNonLeafNode{ s: entity_name, n: pairs.next().unwrap().node().bx()})
    }


    fn type_alias(pair: Pair<Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let alias_name = pairs.next().unwrap().tos();
        let aliased_type = pairs.next().unwrap().node();
        Node::NonLeafNode(OfNonLeafNode{ s: alias_name, n: aliased_type.bx()})
    }

    fn type_name(pair: Pair<Rule>) -> Node {
        let name = pair.tos();
        Node::TypeName(name)
    }
    
    fn entity(pair: Pair<'_, Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let extends = pairs.next().map(|x| x.tos());
        let attrs_option = pairs.next().map(|x| x.into_inner().map(|y| y.node()).collect::<Vec<Node>>());
        let attrs = match attrs_option {
            Some(v) => v,
            None => Vec::new(),
        };
        Node::Entity{extends, attrs: attrs.into_iter().map(|a| {
            if let Node::NonLeafNode(of_nln) = a {
                of_nln.to_tuple()
            } else {
                unreachable!()
            }
        }).into_iter().collect()}
    }
    
    fn attr(pair: Pair<'_, Rule>) -> Node {
        let mut pairs = pair.into_inner();
        let name = pairs.next().unwrap().tos();
        let attr_type = pairs.next().unwrap().node();
        Node::NonLeafNode(OfNonLeafNode{ s: name, n: attr_type.bx()})
    }

  

}

