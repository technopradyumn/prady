// Phase 10 — Foreign Function Interface (FFI) & C-ABI Bindings
// Supports loading dynamic shared libraries (.dll, .so, .dylib) and invoking C functions.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiType {
    Void,
    Int32,
    Int64,
    Float32,
    Float64,
    CString,
    Pointer,
}

#[derive(Debug, Clone)]
pub struct ForeignFunctionDecl {
    pub symbol: String,
    pub library: String,
    pub param_types: Vec<FfiType>,
    pub return_type: FfiType,
}

pub struct FfiRegistry {
    foreign_functions: HashMap<String, ForeignFunctionDecl>,
}

impl FfiRegistry {
    pub fn new() -> Self {
        Self {
            foreign_functions: HashMap::new(),
        }
    }

    pub fn register(&mut self, decl: ForeignFunctionDecl) {
        self.foreign_functions.insert(decl.symbol.clone(), decl);
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<&ForeignFunctionDecl> {
        self.foreign_functions.get(symbol)
    }
}

impl Default for FfiRegistry {
    fn default() -> Self {
        Self::new()
    }
}
