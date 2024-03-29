use std::{collections::HashMap, hash::Hash};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;


#[derive(Parser)]
#[grammar="src/lang/data_lang/data_lang.pest"]
struct DataLangParser;

//type Modules = HashMap<String, Module>;

#[derive(Debug)]
pub enum Node {
    File(String, Box<Node>),
    Module(String, Vec<Node>),
    TypeAlias(String, Box<Node>),
    TypeName(String),
    Entity(String, Box<Node>),
    EntityDef{extends: Option<String>, attrs: HashMap<String, Node>},
    Attr(String, Box<Node>),
}

pub fn parse_file(unparsed_file: &str, file_name: &str) -> () {
    let mut pairs = DataLangParser::parse(Rule::file, unparsed_file).unwrap();
    println!("pairs: {:#?}", pairs);
    let file: Node = pairs.next().unwrap().node(file_name);
    let module = if let Node::File(_, node) = file {
        node.to_hm()
    } else {unreachable!()};
    println!("{:#?}", module);
}

trait GimbalPair {
    fn tos(&self) -> String;
    fn node(self, fname: &str) -> Node;
}


impl GimbalPair for Pair<'_, Rule> {
    fn tos(&self) -> String {
        self.as_str().to_string()
    }

    fn node(self, fname: &str) -> Node {
        match self.as_rule() {
            Rule::file => Node::file(self, fname),
            Rule::type_def => Node::type_def(self, fname),
            Rule::entity => Node::entity(self, fname),
            Rule::type_alias => Node::type_alias(self, fname),
            Rule::entity_def => Node::entity_def(self, fname),
            Rule::type_name => Node::type_name(self, fname),
            Rule::attr => Node::attr(self, fname),
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


    fn to_hm(self) -> (String, HashMap<String, Node>) {
        match self {
            Node::Module(s, nodes) => (s, Node::nodes_to_hm(nodes)),
            _ => unreachable!(),
        }
    }

    fn nodes_to_hm(nodes: Vec<Node>) -> HashMap<String, Node>{
        nodes.into_iter().map(|x| x.node_to_tuple()).collect()
    }

    fn node_to_tuple(self) -> (String, Node) {
        match self {
            Node::TypeAlias(name, node) => (name, *node),
            Node::Entity(name, node) => (name, *node),
            Node::Attr(name, node) => (name, *node),
            _ => unreachable!(),
        }
    }



    fn file(pair: Pair<Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        let module_name = pairs.next().unwrap().into_inner().next().unwrap().tos();
        let nodes = pairs.map(|x| x.node(fname)).collect();
        Node::File(fname.to_string(), Node::Module(module_name, nodes).bx())
    }

    fn type_def(pair: Pair<Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        pairs.next().unwrap().node(fname)
    }

    fn entity(pair: Pair<Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        let entity = pairs.next().unwrap();
        println!("entity: {:#?}", entity);
        let entity_name = entity.clone().as_str().to_string();
        Node::Entity(entity_name, pairs.next().unwrap().node(fname).bx())
    }


    fn type_alias(pair: Pair<Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        let alias_name = pairs.next().unwrap().tos();
        let aliased_type = pairs.next().unwrap().node(fname);
        Node::TypeAlias(alias_name, aliased_type.bx())
    }

    fn type_name(pair: Pair<Rule>, _: &str) -> Node {
        let name = pair.tos();
        Node::TypeName(name)
    }
    
    fn entity_def(pair: Pair<'_, Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        let extends = pairs.next().map(|x| x.tos());
        let attrs_option = pairs.next().map(|x| x.into_inner().map(|y| y.node(fname)).collect::<Vec<Node>>());
        let attrs = match attrs_option {
            Some(v) => v,
            None => Vec::new(),
        };
        Node::EntityDef{extends, attrs: Node::nodes_to_hm(attrs)}
    }
    
    fn attr(pair: Pair<'_, Rule>, fname: &str) -> Node {
        let mut pairs = pair.into_inner();
        let name = pairs.next().unwrap().tos();
        let attr_type = pairs.next().unwrap().node(fname);
        Node::Attr(name, attr_type.bx())
    }

  

}

