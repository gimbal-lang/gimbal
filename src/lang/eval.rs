mod exp;
mod core;
mod defs;

use self::exp::{Exp, Value};
use self::defs::{Defs};
use self::core::{eval_core};

use crate::util::{IndexTree};



fn eval(tree: &IndexTree<Exp>, idx: usize, params: Option<&Vec<&Value>>, defs: &Defs) -> Result<Value, EvalError> {
    match tree.node_val(idx) {
        Exp::BVariable(b) => Ok(b.eval(params.unwrap())),
        Exp::Value(v) => Ok(v.clone()),
        Exp::FnApp(f) => {
            let pv:Vec<Value> = eval_children(tree, idx, params, defs)?;
            let p:Vec<&Value> = pv.iter().map(|v| v).collect();
            match eval_core(f, &p) {
                Ok(v) => Ok(v),
                Err(e) => match f.eval(&p, defs) {
                    Ok(v1) => Ok(v1),
                    Err(_) => Err(e)
                }
            }
        },
        _ => Err(EvalError::InternalError)
    }
}

fn eval_children(tree: &IndexTree<Exp>, idx: usize, params: Option<&Vec<&Value>>, defs: &Defs) -> Result<Vec<Value>, EvalError> {
    Ok(tree.children_idx(idx).iter().map(|i| eval(tree, *i, params, defs)).collect::<Result<Vec<Value>, EvalError>>()?)
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
        let second_types = vec![t1, t2];

        let mut second_fn_tree = IndexTree::new();
        second_fn_tree.add_node(None, Exp::BVariable(BVariable::new(1)));
        let second_fn_body = FnBody::new(second_fn_tree);
        
        let second_sig = Signature::new(Type::Int, second_types, second_fn_body);

        let second_fn_def = FnDef::new("second", vec![second_sig]);

        let mut defs = Defs::new();
        defs.insert(second_fn_def).unwrap();

        let t3 = Type::Int;
        let t4 = Type::Int;
        let plus_types = vec![t3, t4];

        let mut plus_fn_tree = IndexTree::new();
        plus_fn_tree.add_node(None, Exp::FnApp(FnApp::new("+")));
        plus_fn_tree.add_node(Some(0), Exp::BVariable(BVariable::new(0)));
        plus_fn_tree.add_node(Some(0), Exp::BVariable(BVariable::new(1)));

        let plus_fn_body = FnBody::new(plus_fn_tree);

        let plus_sig = Signature::new(Type::Int, plus_types, plus_fn_body);
        let plus_fn_def = FnDef::new("plus", vec![plus_sig]);

        let _ = defs.insert(plus_fn_def);

        defs
    }

    pub(super) fn int_params() -> Vec<Value> {
        let v1 = Value::new_int(1);
        let v2 = Value::new_int(3);
        vec![v1, v2]

    }

  #[test]
    fn eval_second() {
        let defs = test_defs();

        let fn_app = FnApp::new("second");

        let mut tree = IndexTree::new();
        tree.add_node(None, Exp::FnApp(fn_app));
        tree.add_node(Some(0), Exp::Value(Value::new_int(1)));
        tree.add_node(Some(0), Exp::Value(Value::new_int(3)));

        let result = eval(&tree, 0, None, &defs).unwrap();
        assert_eq!(3, result.int_value().unwrap());
    }

    #[test]
    fn eval_core_plus() {
        let defs = test_defs();

        let fn_app = FnApp::new("+");

        let mut tree = IndexTree::new();
        tree.add_node(None, Exp::FnApp(fn_app));
        tree.add_node(Some(0), Exp::Value(Value::new_int(1)));
        tree.add_node(Some(0), Exp::Value(Value::new_int(3)));

        let result = eval(&tree, 0, None, &defs).unwrap();
        assert_eq!(4, result.int_value().unwrap());
    }

    #[test]
    fn eval_custom_plus() {
        let defs = test_defs();

        let fn_app = FnApp::new("plus");

        let mut tree = IndexTree::new();
        tree.add_node(None, Exp::FnApp(fn_app));
        tree.add_node(Some(0), Exp::Value(Value::new_int(1)));
        tree.add_node(Some(0), Exp::Value(Value::new_int(3)));

        let result = eval(&tree, 0, None, &defs).unwrap();
        assert_eq!(4, result.int_value().unwrap());
    }
}