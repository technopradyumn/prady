// Phase 8 — Package Manager & Dependency Resolution
// Parses `prady.toml`, resolves dependency graphs, and manages `prady.lock`.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepSource {
    Registry { version: String },
    Git { url: String, branch: Option<String> },
    Path { path: String },
}

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub dependencies: HashMap<String, DepSource>,
}

impl Manifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            authors: vec![],
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dep(&mut self, name: impl Into<String>, source: DepSource) {
        self.dependencies.insert(name.into(), source);
    }
}

#[derive(Debug, Clone)]
pub struct LockfileEntry {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Lockfile {
    pub entries: HashMap<String, LockfileEntry>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn serialize(&self) -> String {
        let mut out = String::from("# Prady lockfile - automatically generated. Do not edit.\nversion = 1\n\n");
        for (_name, entry) in &self.entries {
            out.push_str(&format!(
                "[[package]]\nname = \"{}\"\nversion = \"{}\"\nchecksum = \"{}\"\n\n",
                entry.name, entry.version, entry.checksum
            ));
        }
        out
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}
