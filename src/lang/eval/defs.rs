use super::{EvalError};
use super::exp::{FnApp, Type, Exp};

use crate::util::{NameMap, Named, IndexTree, NameMapError};


impl From<NameMapError<FnDef>> for EvalError {
    fn from(source: NameMapError<FnDef>) -> Self {
        match source {
            NameMapError::NameAlreadyExists(_) => EvalError::NameAlreadyExists
        }
    }
}

#[derive(Debug)]
pub(super) struct Defs {
    fn_defs: NameMap<FnDef>
}

impl Defs {
    pub(super) fn new() -> Self {
        Defs {
            fn_defs: NameMap::new()
        }
    }

    pub(super) fn insert(&mut self, fn_def: FnDef) -> Result<(), EvalError> {
        self.fn_defs.insert(fn_def)?;
        Ok(())
    }

    pub(super) fn get_fn_def(&self, fn_app: &FnApp) -> Result<&FnDef, EvalError> {
        match self.fn_defs.get(fn_app.name()) {
            Some(fd) => Ok(fd),
            None => Err(EvalError::UnknownFn)
        }
    }
}

#[derive(Debug)]
pub(super) struct Signature {
    fn_type: Type,
    param_types: Vec<Type>,
    fn_body: FnBody
}

impl Signature {
    pub(super) fn new(fn_type: Type, param_types: Vec<Type>, fn_body: FnBody) -> Self {
        Signature {
            fn_type,
            param_types,
            fn_body
        }
    }

    pub(super) fn fn_body(&self) -> &FnBody {
        &self.fn_body
    }

    fn param_types(&self) -> Vec<&Type> {
        self.param_types.iter().map(|t| t).collect()
    }
}

#[derive(Debug)]
pub(super) struct FnDef {
    name: String,
    signatures: Vec<Signature>
}


impl Named for FnDef {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl FnDef {
    pub(super) fn new(name: &str, signatures: Vec<Signature>) -> Self {
        FnDef{
            name: name.to_string(),
            signatures 
        }
    }

    pub(super) fn get_signature(&self, param_types: &Vec<&Type>) -> Result<&Signature, EvalError> {
        let signature = self.signatures.iter().find(|s| &s.param_types() == param_types);
        match signature {
            Some(s) => Ok(s),
            None => Err(EvalError::UnknownFnSignature)
        }
    }
}

#[derive(Debug)]
pub(super) struct FnBody {
    body: IndexTree<Exp>
}

impl FnBody {
    pub(super) fn new(body: IndexTree<Exp>) -> Self {
        FnBody{ body }
    }

    pub(super) fn body(&self) -> &IndexTree<Exp> {
        &self.body
    }
}



