use crate::util::{IndexTree, NameMap, Named, NameMapError};

use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug)]
struct Env {
    assignments: NameMap<Assignment>
}

impl Env {
    fn get(&self, name: &str) -> Result<&Assignment, EvalError> {
        match self.assignments.get(name) {
            Some(a) => Ok(a),
            None => Err(EvalError::UnknownParam)
        }
    }
}

#[derive(Debug)]
struct Assignment {
    name: String,
    value: Value
}

impl Named for Assignment {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Assignment {

    fn value(&self) -> Value {
        self.value.clone()
    }
}

#[derive(Debug)]
struct Defs {
    fn_defs: NameMap<FnDef>
}

impl Defs {
    fn new() -> Self {
        Defs {
            fn_defs: NameMap::new()
        }
    }

    fn insert(&mut self, fn_def: FnDef) -> Result<Option<FnDef>, EvalError> {
        Ok(self.fn_defs.insert(fn_def)?)
    }

    fn core_fn_dispatch(&self, name: &Exp, param_types: &Vec<&Exp>) -> Result<&Signature, EvalError> {
        let n = match name {Exp::Application(x) => x, _ => return Err(EvalError::InternalError)};
        let pts = match exps_to_types(param_types) { Ok(x) => x, Err(e) => return Err(e) };
        let fn_def = match self.fn_defs.get(n) { Some(f) => f, None => return Err(EvalError::UnknownFn)};
        match fn_def.signatures.iter().find(|s| match_signature(&s, &pts)) {
            Some(x) => Ok(x), None => Err(EvalError::UnknownFn)
        }
    }
    
    fn fn_dispatch(&self, name: &Exp, param_types: &Vec<&Exp>, params: &Vec<&Exp>) -> Result<&FnBody, EvalError> {
        let n = match name {Exp::Application(x) => x, _ => return Err(EvalError::InternalError)};
        let pts = match exps_to_types(param_types) {Ok(x) => x, Err(e) => return Err(e)};
        match self.fn_defs.get(n) {
            Some(f) => match f.signatures.iter().find(|s| s.param_types.eq(&pts)) {
                Some(s) => match &s.fn_body {
                    Some(fnb) => Ok(fnb),
                    None => Err(EvalError::UnknownFn)
                }
                None => Err(EvalError::UnknownFn)
            },
            None => Err(EvalError::UnknownFn)
        }
    }
}

fn match_signature(s: &Signature, p: &Vec<Type>) -> bool {
    s.param_types.iter().eq(p)
}

#[derive(Debug)]
struct Signature {
    fn_type: Type,
    param_types: Vec<Type>,
    fn_body: Option<FnBody>
}

impl Signature {
    fn new(fn_type: Type, param_types: Vec<Type>, fn_body: Option<FnBody>) -> Self {
        Signature {
            fn_type,
            param_types,
            fn_body
        }
    }
}

#[derive(Debug)]
struct FnDef {
    name: String,
    signatures: Vec<Signature>
}


impl Named for FnDef {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl FnDef {
    fn new(name: &str, signatures: Vec<Signature>) -> Self {
        FnDef{
            name: name.to_string(),
            signatures 
        }
    }
}

type FnBody = IndexTree<Exp>;

fn substitute(fn_body: &FnBody, idx: usize, types: &Vec<Type>, values: &Vec<Value>) -> FnBody {
    let mut subst: IndexTree<Exp>  = IndexTree::new();
    match fn_body.node_val(idx) {
        Exp::BVariable(i) => subst.add_node(if idx == 0 {None} else {Some(idx)}, Exp::Value(values[*i].clone())),
        _ => subst.add_node(if idx == 0 {None} else {Some(idx)}, fn_body.node_val(idx).clone())
    };
    subst
}

#[derive(Debug)]
enum EvalError {
    TypeError,
    TypeMismatch,
    ParamsTypeMismatch,
    NotANumber,
    InternalError,
    NameAlreadyExists,
    UnknownFn,
    UnknownParam
}

impl From<NameMapError<FnDef>> for EvalError {
    fn from(source: NameMapError<FnDef>) -> Self {
        match source {
            NameMapError::NameAlreadyExists(i) => EvalError::NameAlreadyExists
        }
    }
}

#[derive(Debug)]
enum Token {
    Exp(Exp),
    Value(Value),
    FnDef(FnDef)
}

#[derive(Debug, Clone)]
pub enum Exp {
    Value(Value),
    Application(String),
    Variable(String),
    BVariable(usize),
}

impl Exp {
    fn to_value(&self) -> Result<Value, EvalError> {
        match self {
            Exp::Value(v) => Ok(v.clone()),
            _ => Err(EvalError::InternalError)
        }
    }

}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Int,
    Float,
    String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(String, Type);

impl Value {
    fn as_type(&self) -> Type {
        self.1.clone()
    }

    fn is_num(&self) -> bool {
        self.as_type() == Type::Int || self.as_type() == Type::Float
    }

    fn int_value(&self) -> Result<i128, EvalError> {
        if self.1 == Type::Int {
            match self.0.parse::<i128>() {
                Ok(i) => Ok(i),
                Err(e) => Err(EvalError::InternalError)
            }
        } else {
            Err(EvalError::TypeError)
        }
    }

    fn new_int(value: i128) -> Value {
        Value(value.to_string(), Type::Int)
    }

    fn float_value(&self) -> Result<f64, EvalError> {
        if self.1 == Type::Float {
            match self.0.parse::<f64>() {
                Ok(f) => Ok(f),
                Err(e) => Err(EvalError::InternalError)
            }
        } else {
            Err(EvalError::TypeError)
        }
    }

    fn new_float(value: f64) -> Value {
        Value(value.to_string(), Type::Float)
    }
}

fn exps_to_values(exps: &Vec<&Exp>) -> Result<Vec<Value>, EvalError> {
    exps.iter().map(|e| e.to_value()).collect::<Result<Vec<Value>, EvalError>>()
}

fn values_to_types(values: &Vec<Value>) -> Vec<Type> {
    values.iter().map(|v| v.as_type()).collect::<Vec<Type>>()
}

fn exps_to_types(exps: &Vec<&Exp>) -> Result<Vec<Type>, EvalError> {
    Ok(values_to_types(&exps_to_values(&exps)?))
}

fn eval(tree: &IndexTree<Exp>, idx: usize, defs: &Defs, env: &Env) -> Result<Value, EvalError> {
    match tree.node_val(idx) {
        Exp::Application(f) => {
            match f.as_str() {
                "+" => add(tree.node_val(idx), &tree.children_val(idx), defs),
                _ => eval(defs.fn_dispatch(tree.node_val(idx), &tree.children_val(idx), &tree.children_val(idx))?, 0, defs, env)
            }
        },
        _ => unreachable!()
    }
}

fn add(exp: &Exp, params: &Vec<&Exp>, defs: &Defs) -> Result<Value, EvalError> {
    let values = exps_to_values(params)?;
    let s = defs.core_fn_dispatch(exp, params)?;
    if s.param_types.eq(&vec![Type::Int, Type::Int]) {
        Ok(Value::new_int(values[0].int_value()? + values[1].int_value()?))
    } else {
        Err(EvalError::ParamsTypeMismatch)
    }
}
   

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn eval_plus() {
    let sig = Signature::new(Type::Int, vec![Type::Int, Type::Int], None);
    let fn_def = FnDef::new("+", vec![sig]);
    let mut defs = Defs::new();
    let _ = defs.insert(fn_def);
    let f = Exp::Application("+".to_string());
    let mut t = IndexTree::new();
    t.add_node(None, f);
    let i1 = Value::new_int(1);
    assert_eq!(1, i1.int_value().unwrap());
    t.add_node(Some(0), Exp::Value(i1));
    t.add_node(Some(0), Exp::Value(Value::new_int(2)));
    let e = eval(&t, 0, &defs, &Env{assignments: NameMap::new()});
    assert_eq!(Value::new_int(3), e.unwrap());
  }

  fn eval_custom_plus() {
    let mut defs = Defs::new();
    

    /* fn def for + */
    let sig = Signature::new(Type::Int, vec![Type::Int, Type::Int], None);
    let fn_def = FnDef::new("+", vec![sig]);
    let _ = defs.insert(fn_def);



    /* fn application for custom_plus */
    let c_app = Exp::Application("custom_plus".to_string());
    let mut t = IndexTree::new();
    t.add_node(None, c_app);
    let i1 = Value::new_int(1);
    assert_eq!(1, i1.int_value().unwrap());
    t.add_node(Some(0), Exp::Value(i1));
    t.add_node(Some(0), Exp::Value(Value::new_int(2)));
    
    /* fn body for custom_plus */
    let mut c_f_body = FnBody::new();
    c_f_body.add_node(None, Exp::Application("+".to_string()));
    c_f_body.add_node(Some(0), Exp::BVariable(0));
    c_f_body.add_node(Some(0), Exp::BVariable(1));


    /* fn def for custom_plus */
    let c_sig = Signature::new(Type::Int, vec![Type::Int, Type::Int], Some(c_f_body));
    let c_fn_def = FnDef::new("+", vec![c_sig]);
    let _ = defs.insert(c_fn_def);
    
    let e = eval(&t, 0, &defs, &Env{assignments: NameMap::new()});
    assert_eq!(Value::new_int(3), e.unwrap());
  }
}