use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    vars: HashMap<String, Value>,
    mutability: HashMap<String, bool>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            vars: HashMap::new(),
            mutability: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            vars: HashMap::new(),
            mutability: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value, is_mut: bool) {
        self.vars.insert(name.to_string(), value);
        self.mutability.insert(name.to_string(), is_mut);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.vars.contains_key(name) {
            let is_mut = self.mutability.get(name).copied().unwrap_or(false);
            if !is_mut {
                return Err(format!(
                    "Cannot reassign immutable variable '{}'. Declare with 'let mut' to allow mutation.",
                    name
                ));
            }
            self.vars.insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow_mut().assign(name, value);
        }

        Err(format!("Undefined variable '{}'", name))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(val) = self.vars.get(name) {
            return Some(val.clone());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name);
        }

        None
    }
}
