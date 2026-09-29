// Phase 4 — Enums & Algebraic Data Types (ADT)
// Manages enum variant representation, payloads, and discriminant tagging.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AdtPayload {
    Unit,
    Tuple(Vec<String>),
    Struct(HashMap<String, String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariantDef {
    pub name: String,
    pub tag: usize,
    pub payload: AdtPayload,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumTypeDef {
    pub name: String,
    pub type_params: Vec<String>,
    pub variants: Vec<EnumVariantDef>,
}

impl EnumTypeDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_params: vec![],
            variants: vec![],
        }
    }

    pub fn add_variant(&mut self, name: impl Into<String>, payload: AdtPayload) {
        let tag = self.variants.len();
        self.variants.push(EnumVariantDef {
            name: name.into(),
            tag,
            payload,
        });
    }

    pub fn find_variant(&self, variant_name: &str) -> Option<&EnumVariantDef> {
        self.variants.iter().find(|v| v.name == variant_name)
    }
}

/// Runtime instance of an enum value.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumInstance {
    pub enum_name: String,
    pub variant_name: String,
    pub tag: usize,
    pub payload_values: Vec<String>,
}

impl EnumInstance {
    pub fn new(
        enum_name: impl Into<String>,
        variant_name: impl Into<String>,
        tag: usize,
        payload: Vec<String>,
    ) -> Self {
        Self {
            enum_name: enum_name.into(),
            variant_name: variant_name.into(),
            tag,
            payload_values: payload,
        }
    }
}
