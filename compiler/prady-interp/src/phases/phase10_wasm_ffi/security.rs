// Phase 10 — Security Hardening & Capability Sandboxing
// Implements capability-based security restricting file system, network, and system commands.

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub allow_filesystem_read: bool,
    pub allow_filesystem_write: bool,
    pub allow_network: bool,
    pub allow_subprocesses: bool,
    pub allowed_read_paths: Vec<String>,
    pub allowed_hosts: Vec<String>,
    pub max_memory_bytes: usize,
}

impl SecurityPolicy {
    /// Safe default sandbox policy (strict mode).
    pub fn strict_sandbox() -> Self {
        Self {
            allow_filesystem_read: false,
            allow_filesystem_write: false,
            allow_network: false,
            allow_subprocesses: false,
            allowed_read_paths: Vec::new(),
            allowed_hosts: Vec::new(),
            max_memory_bytes: 64 * 1024 * 1024, // 64 MB
        }
    }

    /// Full developer permissions policy.
    pub fn permissive() -> Self {
        Self {
            allow_filesystem_read: true,
            allow_filesystem_write: true,
            allow_network: true,
            allow_subprocesses: true,
            allowed_read_paths: vec!["*".to_string()],
            allowed_hosts: vec!["*".to_string()],
            max_memory_bytes: usize::MAX,
        }
    }

    pub fn check_network_access(&self, host: &str) -> Result<(), String> {
        if !self.allow_network {
            return Err(format!("Security Violation: Network access is disabled by security policy (attempted to reach '{host}')"));
        }
        if !self.allowed_hosts.iter().any(|h| h == "*" || h == host) {
            return Err(format!(
                "Security Violation: Host '{host}' is not in allowed hosts whitelist"
            ));
        }
        Ok(())
    }

    pub fn check_filesystem_write(&self, path: &str) -> Result<(), String> {
        if !self.allow_filesystem_write {
            return Err(format!("Security Violation: Filesystem write access is disabled by policy (attempted to write to '{path}')"));
        }
        Ok(())
    }
}
