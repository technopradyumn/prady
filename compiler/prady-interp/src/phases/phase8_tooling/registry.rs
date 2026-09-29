// Phase 8 — Central Package Registry Client
// Handles package search, tarball downloads, and index updates.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegistryPackageInfo {
    pub name: String,
    pub description: String,
    pub latest_version: String,
    pub versions: Vec<String>,
    pub download_url: String,
}

pub struct PackageRegistryClient {
    pub registry_url: String,
    index: HashMap<String, RegistryPackageInfo>,
}

impl PackageRegistryClient {
    pub fn new(registry_url: impl Into<String>) -> Self {
        let mut client = Self {
            registry_url: registry_url.into(),
            index: HashMap::new(),
        };

        // Seed with core official packages
        client.register_seed_package(
            "prady-http",
            "High performance HTTP server and client for Prady",
            "1.0.0",
        );
        client.register_seed_package(
            "prady-json",
            "Fast JSON serialization and deserialization library",
            "1.0.0",
        );
        client.register_seed_package("prady-test", "Unit testing and assertion toolkit", "1.0.0");

        client
    }

    fn register_seed_package(&mut self, name: &str, desc: &str, ver: &str) {
        self.index.insert(
            name.to_string(),
            RegistryPackageInfo {
                name: name.to_string(),
                description: desc.to_string(),
                latest_version: ver.to_string(),
                versions: vec![ver.to_string()],
                download_url: format!("{}/packages/{name}/{ver}.tar.gz", self.registry_url),
            },
        );
    }

    pub fn search(&self, query: &str) -> Vec<&RegistryPackageInfo> {
        self.index
            .values()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }

    pub fn get_package(&self, name: &str) -> Option<&RegistryPackageInfo> {
        self.index.get(name)
    }
}
