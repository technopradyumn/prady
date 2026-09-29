// Phase 6 — Interfaces, Traits & Dynamic VTables
// Implements trait declarations, interface contracts, and runtime vtable dispatch.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: String,
    pub param_types: Vec<String>,
    pub return_type: Option<String>,
    pub has_default_impl: bool,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub methods: HashMap<String, TraitMethod>,
}

impl TraitDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            methods: HashMap::new(),
        }
    }

    pub fn add_method(&mut self, method: TraitMethod) {
        self.methods.insert(method.name.clone(), method);
    }
}

/// Runtime representation of a virtual method table.
#[derive(Debug, Clone)]
pub struct VTable {
    pub trait_name: String,
    pub implementing_type: String,
    pub function_pointers: HashMap<String, usize>, // method_name -> fn index
}

impl VTable {
    pub fn new(trait_name: impl Into<String>, implementing_type: impl Into<String>) -> Self {
        Self {
            trait_name: trait_name.into(),
            implementing_type: implementing_type.into(),
            function_pointers: HashMap::new(),
        }
    }

    pub fn bind(&mut self, method_name: impl Into<String>, fn_ptr: usize) {
        self.function_pointers.insert(method_name.into(), fn_ptr);
    }

    pub fn lookup(&self, method_name: &str) -> Option<usize> {
        self.function_pointers.get(method_name).copied()
    }
}
