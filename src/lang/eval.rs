mod exp;
mod core;
mod defs;

use self::exp::{Exp, Value};
use self::defs::{Defs};

use crate::util::{IndexTree};



fn eval(tree: &IndexTree<Exp>, idx: usize, params: Option<&Vec<&Value>>, defs: &Defs) -> Result<Value, EvalError> {
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
  use super::defs::*;
  use super::exp::*;

  pub(super) fn test_defs() -> Defs {
    let t1 = Type::Int;
    let t2 = Type::Int;
    let types = vec![t1, t2];

    let mut fn_tree = IndexTree::new();
    fn_tree.add_node(None, Exp::BVariable(BVariable::new(1)));
    let fn_body = FnBody::new(fn_tree);
    
    let sig = Signature::new(Type::Int, types, fn_body);

    let fn_def = FnDef::new("second", vec![sig]);

    let mut defs = Defs::new();
    defs.insert(fn_def).unwrap();
    defs
  }

  #[test]
  fn run_second() {
        let defs = test_defs();
      
        let v1 = Value::new_int(1);
        let v2 = Value::new_int(3);
        let params = vec![&v1, &v2];

        let fn_app = FnApp::new("second");

        let mut tree = IndexTree::new();
        tree.add_node(None, Exp::FnApp(fn_app));

        let result = eval(&tree, 0, Some(&params), &defs).unwrap();
        assert_eq!(3, result.int_value().unwrap());

    

  }

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