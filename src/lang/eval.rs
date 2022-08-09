mod exp;
mod core;
mod defs;

use self::exp::{Exp, Value};
use self::defs::{Defs, FnDef};

use crate::util::{IndexTree, NameMapError};



fn eval(tree: &IndexTree<Exp>, idx: usize, params: Option<&Vec<&Value>>, defs: &Defs) -> Result<exp::Value, EvalError> {
    let params_unwrap = match params {Some(p) => p, None => return Err(EvalError::InternalError)};

    match tree.node_val(idx) {
        Exp::BVariable(b) => Ok(b.eval(params_unwrap)),
        Exp::Value(v) => Ok(v.clone()),
        Exp::FnApp(f) => Ok(f.eval(params_unwrap, defs)?),
        _ => Err(EvalError::InternalError)
    }
}

#[derive(Debug)]
pub enum EvalError {
    TypeError,
    TypeMismatch,
    ParamsTypeMismatch,
    NotANumber,
    InternalError,
    NameAlreadyExists,
    UnknownFn,
    UnknownParam,
    UnknownFnSignature
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn eval_plus() {
      /*
    let sig = defs::Signature::new(exp::Type::Int, vec![exp::Type::Int, exp::Type::Int], None);
    let fn_def = FnDef::new("+", vec![sig]);
    let mut defs = Defs::new();
    let _ = defs.insert(fn_def);
    let f = Exp::FnApp(FnApp::new("+"));
    let mut t = IndexTree::new();
    t.add_node(None, f);
    let i1 = Value::new_int(1);
    assert_eq!(1, i1.int_value().unwrap());
    t.add_node(Some(0), Exp::Value(i1));
    t.add_node(Some(0), Exp::Value(Value::new_int(2)));
    let e = eval(&t, 0, None, &defs);
    assert_eq!(Value::new_int(3), e.unwrap());
    */
  }

  /*
  #[test]
  fn eval_custom_plus() {
    let mut defs = Defs::new();
    

    let sig = Signature::new(Type::Int, vec![Type::Int, Type::Int], None);
    let fn_def = FnDef::new("+", vec![sig]);
    let _ = defs.insert(fn_def);



    let c_app = Exp::FnApp(FnApp::new("custom_plus"));
    let mut t = IndexTree::new();
    t.add_node(None, c_app);
    let i1 = Value::new_int(1);
    assert_eq!(1, i1.int_value().unwrap());
    t.add_node(Some(0), Exp::Value(i1));
    t.add_node(Some(0), Exp::Value(Value::new_int(2)));
    
    let mut c_f_body = FnBody::new();
    c_f_body.tree.add_node(None, Exp::FnApp(FnApp::new("+")));
    c_f_body.tree.add_node(Some(0), Exp::BVariable(BVariable{index: 0}));
    c_f_body.tree.add_node(Some(0), Exp::BVariable(BVariable{index: 1}));


    let c_sig = Signature::new(Type::Int, vec![Type::Int, Type::Int], Some(c_f_body));
    let c_fn_def = FnDef::new("custom_plus", vec![c_sig]);
    let _ = defs.insert(c_fn_def);
    
    let e = eval(&t, 0, None, &defs);
    println!("{:?}", &e);
    //assert_eq!(Value::new_int(3), e.unwrap());
  }
  */
}