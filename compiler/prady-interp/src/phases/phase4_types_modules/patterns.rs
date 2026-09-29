// Phase 4 — Pattern Matching Engine & Exhaustiveness Checker
// Implements structural pattern matching with exhaustiveness analysis.

use super::enums::EnumInstance;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),
    LiteralBool(bool),
    Variable(String),
    Variant {
        enum_name: Option<String>,
        variant_name: String,
        sub_patterns: Vec<Pattern>,
    },
    Tuple(Vec<Pattern>),
    Or(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<String>, // Boolean guard expression representation
    pub action_id: usize,
}

#[derive(Debug, Clone)]
pub struct PatternEngine;

impl PatternEngine {
    /// Match a value against a pattern. Returns bound variables if match succeeds.
    pub fn try_match(
        pattern: &Pattern,
        instance: &EnumInstance,
    ) -> Option<HashMap<String, String>> {
        let mut bindings = HashMap::new();
        if Self::match_inner(pattern, instance, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    fn match_inner(
        pattern: &Pattern,
        instance: &EnumInstance,
        bindings: &mut HashMap<String, String>,
    ) -> bool {
        match pattern {
            Pattern::Wildcard => true,
            Pattern::Variable(var) => {
                bindings.insert(var.clone(), instance.variant_name.clone());
                true
            }
            Pattern::Variant {
                variant_name,
                sub_patterns,
                ..
            } => {
                if variant_name != &instance.variant_name {
                    return false;
                }
                if sub_patterns.len() > instance.payload_values.len() {
                    return false;
                }
                for (sub_pat, val) in sub_patterns.iter().zip(&instance.payload_values) {
                    match sub_pat {
                        Pattern::Wildcard => {}
                        Pattern::Variable(name) => {
                            bindings.insert(name.clone(), val.clone());
                        }
                        Pattern::LiteralString(s) if s == val => {}
                        _ => return false,
                    }
                }
                true
            }
            Pattern::Or(pats) => pats
                .iter()
                .any(|p| Self::match_inner(p, instance, bindings)),
            _ => false,
        }
    }

    /// Check if a set of arms is exhaustive for a given enum definition.
    pub fn check_exhaustiveness(arms: &[MatchArm], variants: &[String]) -> Result<(), Vec<String>> {
        let mut covered = vec![false; variants.len()];

        for arm in arms {
            match &arm.pattern {
                Pattern::Wildcard => return Ok(()),
                Pattern::Variable(_) => return Ok(()),
                Pattern::Variant { variant_name, .. } => {
                    if let Some(pos) = variants.iter().position(|v| v == variant_name) {
                        covered[pos] = true;
                    }
                }
                Pattern::Or(sub_pats) => {
                    for sp in sub_pats {
                        if let Pattern::Variant { variant_name, .. } = sp {
                            if let Some(pos) = variants.iter().position(|v| v == variant_name) {
                                covered[pos] = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let missing: Vec<String> = variants
            .iter()
            .enumerate()
            .filter(|(i, _)| !covered[*i])
            .map(|(_, name)| name.clone())
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}
