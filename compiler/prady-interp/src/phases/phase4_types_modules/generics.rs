// Phase 4 — Generics & Monomorphization
// Handles generic type parameter substitution and specialized function instantiation.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteType {
    Int,
    Float,
    Bool,
    Str,
    List(Box<ConcreteType>),
    Map(Box<ConcreteType>, Box<ConcreteType>),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct GenericSignature {
    pub type_params: Vec<String>,
    pub param_types: Vec<String>, // e.g. "T", "List<T>"
    pub return_type: String,
}

pub struct GenericSolver {
    instantiations: HashMap<String, Vec<HashMap<String, ConcreteType>>>,
}

impl GenericSolver {
    pub fn new() -> Self {
        Self {
            instantiations: HashMap::new(),
        }
    }

    /// Substitute type arguments into a type expression string.
    pub fn substitute(template: &str, type_args: &HashMap<String, ConcreteType>) -> String {
        let mut result = template.to_string();
        for (param, concrete) in type_args {
            let concrete_str = match concrete {
                ConcreteType::Int => "Int",
                ConcreteType::Float => "Float",
                ConcreteType::Bool => "Bool",
                ConcreteType::Str => "String",
                ConcreteType::List(inner) => &format!("List[{:?}]", inner),
                ConcreteType::Map(k, v) => &format!("Map[{:?}, {:?}]", k, v),
                ConcreteType::Custom(name) => name.as_str(),
            };
            result = result.replace(param, concrete_str);
        }
        result
    }

    /// Register a concrete instantiation of a generic function or type.
    pub fn register_instantiation(
        &mut self,
        base_name: &str,
        type_args: HashMap<String, ConcreteType>,
    ) -> String {
        let entry = self
            .instantiations
            .entry(base_name.to_string())
            .or_default();
        if !entry.contains(&type_args) {
            entry.push(type_args.clone());
        }

        // Generate mangled name for the monomorphized entity: e.g. "identity$Int"
        let args_suffix: Vec<String> = type_args
            .iter()
            .map(|(k, v)| format!("{k}_{v:?}"))
            .collect();
        format!("{base_name}__{}", args_suffix.join("_"))
    }
}
