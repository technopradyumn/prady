// Phase 6 — Object-Oriented Programming (Classes & Inheritance)
// Implements class definitions, single inheritance, field layouts, and virtual method dispatch.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
    pub is_abstract: bool,
    pub is_override: bool,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: HashMap<String, String>, // field_name -> type
    pub methods: HashMap<String, MethodDef>,
    pub is_abstract: bool,
}

impl ClassDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            super_class: None,
            interfaces: vec![],
            fields: HashMap::new(),
            methods: HashMap::new(),
            is_abstract: false,
        }
    }

    pub fn add_field(&mut self, name: impl Into<String>, ty: impl Into<String>) {
        self.fields.insert(name.into(), ty.into());
    }

    pub fn add_method(&mut self, method: MethodDef) {
        self.methods.insert(method.name.clone(), method);
    }
}

pub struct ClassHierarchy {
    classes: HashMap<String, ClassDef>,
}

impl ClassHierarchy {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    pub fn register(&mut self, class_def: ClassDef) {
        self.classes.insert(class_def.name.clone(), class_def);
    }

    /// Resolve a method via inheritance chain (virtual dispatch).
    pub fn resolve_method(&self, class_name: &str, method_name: &str) -> Option<(&ClassDef, &MethodDef)> {
        let mut curr = Some(class_name);
        while let Some(name) = curr {
            if let Some(def) = self.classes.get(name) {
                if let Some(m) = def.methods.get(method_name) {
                    return Some((def, m));
                }
                curr = def.super_class.as_deref();
            } else {
                break;
            }
        }
        None
    }

    /// Check if class `sub` is a subtype of `base` (direct or indirect).
    pub fn is_subclass(&self, sub: &str, base: &str) -> bool {
        if sub == base {
            return true;
        }
        let mut curr = self.classes.get(sub).and_then(|c| c.super_class.as_deref());
        while let Some(parent) = curr {
            if parent == base {
                return true;
            }
            curr = self.classes.get(parent).and_then(|c| c.super_class.as_deref());
        }
        false
    }
}
