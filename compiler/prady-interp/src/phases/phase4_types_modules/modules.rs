// Phase 4 — Hierarchical Module System & Import Resolver
// Resolves imports, detects circular dependencies, and manages symbol export tables.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ModuleSymbol {
    pub name: String,
    pub is_exported: bool,
    pub kind: String, // "fn", "struct", "class", "enum", "const"
}

#[derive(Debug, Clone)]
pub struct ModuleNode {
    pub path: String, // e.g. "std.dsa.vector" or "models.user"
    pub file_path: Option<PathBuf>,
    pub symbols: HashMap<String, ModuleSymbol>,
    pub imports: Vec<String>,
}

impl ModuleNode {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            file_path: None,
            symbols: HashMap::new(),
            imports: vec![],
        }
    }

    pub fn export_symbol(&mut self, name: impl Into<String>, kind: impl Into<String>) {
        let name_str = name.into();
        self.symbols.insert(
            name_str.clone(),
            ModuleSymbol {
                name: name_str,
                is_exported: true,
                kind: kind.into(),
            },
        );
    }
}

pub struct ModuleSystem {
    modules: HashMap<String, ModuleNode>,
}

impl ModuleSystem {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module: ModuleNode) {
        self.modules.insert(module.path.clone(), module);
    }

    pub fn resolve_symbol(&self, module_path: &str, symbol_name: &str) -> Option<&ModuleSymbol> {
        self.modules
            .get(module_path)
            .and_then(|m| m.symbols.get(symbol_name))
            .filter(|s| s.is_exported)
    }

    /// Detect circular dependencies using depth-first search cycle detection.
    pub fn detect_cycles(&self) -> Result<(), Vec<String>> {
        let mut visited = HashSet::new();
        let mut on_stack = HashSet::new();
        let mut cycle_path = Vec::new();

        for mod_path in self.modules.keys() {
            if !visited.contains(mod_path) {
                if self.dfs_cycle(mod_path, &mut visited, &mut on_stack, &mut cycle_path) {
                    return Err(cycle_path);
                }
            }
        }

        Ok(())
    }

    fn dfs_cycle(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        on_stack: &mut HashSet<String>,
        cycle_path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node.to_string());
        on_stack.insert(node.to_string());
        cycle_path.push(node.to_string());

        if let Some(module) = self.modules.get(node) {
            for dep in &module.imports {
                if !visited.contains(dep) {
                    if self.dfs_cycle(dep, visited, on_stack, cycle_path) {
                        return true;
                    }
                } else if on_stack.contains(dep) {
                    cycle_path.push(dep.to_string());
                    return true;
                }
            }
        }

        on_stack.remove(node);
        cycle_path.pop();
        false
    }
}
