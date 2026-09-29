// Phase 6 — Architecture & Quality Policy Engine
// Enforces Clean Architecture layer boundaries (e.g. Domain -> Application -> Infrastructure).
// Prevents dependency inversions and verifies declared `architecture { layer ... }` blocks.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchViolation {
    pub from_layer: String,
    pub to_layer: String,
    pub file: String,
    pub line: usize,
    pub rule_description: String,
}

#[derive(Debug, Clone)]
pub struct LayerPolicy {
    pub name: String,
    pub allowed_dependencies: HashSet<String>,
    pub forbidden_dependencies: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct ArchitecturePolicyEngine {
    pub name: String,
    pub layers: HashMap<String, LayerPolicy>,
    pub file_layer_mapping: HashMap<String, String>, // file_path -> layer_name
}

impl ArchitecturePolicyEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layers: HashMap::new(),
            file_layer_mapping: HashMap::new(),
        }
    }

    pub fn add_layer(&mut self, name: impl Into<String>) {
        let name_str = name.into();
        self.layers.insert(
            name_str.clone(),
            LayerPolicy {
                name: name_str,
                allowed_dependencies: HashSet::new(),
                forbidden_dependencies: HashSet::new(),
            },
        );
    }

    pub fn allow_flow(&mut self, from: &str, to: &str) {
        if let Some(layer) = self.layers.get_mut(from) {
            layer.allowed_dependencies.insert(to.to_string());
        }
    }

    pub fn deny_flow(&mut self, from: &str, to: &str) {
        if let Some(layer) = self.layers.get_mut(from) {
            layer.forbidden_dependencies.insert(to.to_string());
        }
    }

    pub fn map_file_to_layer(&mut self, file_path: impl Into<String>, layer: impl Into<String>) {
        self.file_layer_mapping
            .insert(file_path.into(), layer.into());
    }

    /// Validate an import between two files according to the architecture policy.
    pub fn check_import(
        &self,
        importer_file: &str,
        imported_file: &str,
        line: usize,
    ) -> Result<(), ArchViolation> {
        let from_layer = self.file_layer_mapping.get(importer_file);
        let to_layer = self.file_layer_mapping.get(imported_file);

        if let (Some(from), Some(to)) = (from_layer, to_layer) {
            if from == to {
                return Ok(());
            }

            if let Some(policy) = self.layers.get(from) {
                if policy.forbidden_dependencies.contains(to) {
                    return Err(ArchViolation {
                        from_layer: from.clone(),
                        to_layer: to.clone(),
                        file: importer_file.to_string(),
                        line,
                        rule_description: format!("Architecture policy '{}' strictly forbids layer '{}' from depending on layer '{}'", self.name, from, to),
                    });
                }

                if !policy.allowed_dependencies.is_empty()
                    && !policy.allowed_dependencies.contains(to)
                {
                    return Err(ArchViolation {
                        from_layer: from.clone(),
                        to_layer: to.clone(),
                        file: importer_file.to_string(),
                        line,
                        rule_description: format!("Architecture policy '{}': dependency from layer '{}' to layer '{}' is not explicitly permitted", self.name, from, to),
                    });
                }
            }
        }

        Ok(())
    }
}
