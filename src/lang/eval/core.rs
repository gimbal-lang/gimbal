
use super::{EvalError};
use super::exp::{FnApp, Value, Type};

pub(super) fn eval_core(fn_app: &FnApp, params: &Vec<&Value>) -> Result<Value, EvalError> {
    let types: Vec<&Type> = params.iter().map(|p| p.as_type()).collect();
    println!("^^^^^ {:?}", &fn_app);
    match fn_app.name() {
        "+" => {
            if types == vec![&Type::Int, &Type::Int] {
                Ok(plus_int(params))
            } else {
                Err(EvalError::UnknownFnSignature) 
            }
        },
        _ => Err(EvalError::UnknownFn)
    }
}

fn plus_int(params: &Vec<&Value>) -> Value {
    Value::new_int(params[0].int_value().unwrap() + params[1].int_value().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::exp::{Value};


    #[test]
    fn test_plus_int() {
        let p1 = Value::new_int(1);
        let p2 = Value::new_int(3);
        let params = vec![&p1, &p2];
        let fn_app = FnApp::new("+");
        let result = eval_core(&fn_app, &params).unwrap();
        assert_eq!(4, result.int_value().unwrap());
    }
}