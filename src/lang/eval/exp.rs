use super::{EvalError, eval};
use super::defs::{Defs};

#[derive(Debug, Clone)]
pub(super) enum Exp {
    Value(Value),
    FnApp(FnApp),
    Variable(String),
    BVariable(BVariable),
}

impl Exp {
    fn to_value(&self) -> Result<&Value, EvalError> {
        match self {
            Exp::Value(v) => Ok(v),
            _ => Err(EvalError::InternalError)
        }
    }

    fn to_fn_app(&self) -> Result<&FnApp, EvalError> {
        match self {
            Exp::FnApp(v) => Ok(v),
            _ => Err(EvalError::InternalError)
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct BVariable {
  index: usize    
}

impl BVariable {
    pub(super) fn eval(&self, params: &Vec<&Value>) -> Value {
        params[self.index].clone()
    }

    pub(super) fn new(index: usize) -> BVariable {
        BVariable{index}
    }
}

#[derive(Debug, Clone)]
pub(super) struct FnApp {
    name: String
}

impl FnApp {
    pub(super) fn new(name: &str) -> Self {
        FnApp{name: name.to_string()}
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn eval(&self, params: &Vec<&Value>, defs: &Defs) -> Result<Value, EvalError> {
        let param_types:Vec<&Type> = params.iter().map(|p| p.as_type()).collect();
        let fn_def = defs.get_fn_def(self)?;
        let signature = fn_def.get_signature(&param_types)?;
        eval(&signature.fn_body().body(), 0, Some(params), defs)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(super) enum Type {
    Int,
    Float,
    String
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Value{
    value: String,
    value_type: Type
}

impl Value {
    pub(super) fn as_type(&self) -> &Type {
        &self.value_type
    }

    fn is_num(&self) -> bool {
        *self.as_type() == Type::Int || *self.as_type() == Type::Float
    }

    pub(super)fn int_value(&self) -> Result<i128, EvalError> {
        if self.value_type == Type::Int {
            match self.value.parse::<i128>() {
                Ok(i) => Ok(i),
                Err(_) => Err(EvalError::InternalError)
            }
        } else {
            Err(EvalError::TypeError)
        }
    }

    pub(super) fn new_int(value: i128) -> Value {
        Value{value: value.to_string(), value_type: Type::Int}
    }

    fn float_value(&self) -> Result<f64, EvalError> {
        if self.value_type == Type::Float {
            match self.value.parse::<f64>() {
                Ok(f) => Ok(f),
                Err(_) => Err(EvalError::InternalError)
            }
        } else {
            Err(EvalError::TypeError)
        }
    }

    fn new_float(value: f64) -> Value {
        Value{ value: value.to_string(), value_type: Type::Float}
    }
}

#[cfg(test)]
mod tests {
  use super::{Value, BVariable, FnApp};
  use super::super::tests::{test_defs};


  #[test]
  fn bvarialbe_eval() {
      let v1 = Value::new_int(1);
      let v2 = Value::new_int(2);
      let params = vec![&v1, &v2];
      let b_var = BVariable{index: 1};
      let eval = b_var.eval(&params);
      assert_eq!(2, eval.int_value().unwrap());
  }

  #[test]
  fn fn_app_eval() {
      let defs = test_defs();

      let v1 = Value::new_int(1);
      let v2 = Value::new_int(2);
      let params = vec![&v1, &v2];

      let fn_app = FnApp::new("second");
      let result = fn_app.eval(&params, &defs).unwrap();
      assert_eq!(2, result.int_value().unwrap());
  }
}
