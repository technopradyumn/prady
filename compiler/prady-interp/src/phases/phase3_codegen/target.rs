// Phase 3 — Target Triples & Architecture Configuration
// Defines target platforms, ABI specifications, and native linker options.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
    Wasm32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Os {
    Windows,
    MacOS,
    Linux,
    Wasi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Env {
    Msvc,
    Gnu,
    Musl,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetTriple {
    pub arch: Arch,
    pub vendor: String,
    pub os: Os,
    pub env: Env,
}

impl TargetTriple {
    pub fn host() -> Self {
        #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
        return Self {
            arch: Arch::X86_64,
            vendor: "pc".into(),
            os: Os::Windows,
            env: Env::Msvc,
        };

        #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
        return Self {
            arch: Arch::X86_64,
            vendor: "apple".into(),
            os: Os::MacOS,
            env: Env::Unknown,
        };

        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        return Self {
            arch: Arch::Aarch64,
            vendor: "apple".into(),
            os: Os::MacOS,
            env: Env::Unknown,
        };

        #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
        return Self {
            arch: Arch::X86_64,
            vendor: "unknown".into(),
            os: Os::Linux,
            env: Env::Gnu,
        };

        #[cfg(not(any(
            all(target_arch = "x86_64", target_os = "windows"),
            all(target_arch = "x86_64", target_os = "macos"),
            all(target_arch = "aarch64", target_os = "macos"),
            all(target_arch = "x86_64", target_os = "linux")
        )))]
        return Self::windows_x64();
    }

    pub fn windows_x64() -> Self {
        Self {
            arch: Arch::X86_64,
            vendor: "pc".into(),
            os: Os::Windows,
            env: Env::Msvc,
        }
    }

    pub fn linux_x64() -> Self {
        Self {
            arch: Arch::X86_64,
            vendor: "unknown".into(),
            os: Os::Linux,
            env: Env::Gnu,
        }
    }

    pub fn macos_arm64() -> Self {
        Self {
            arch: Arch::Aarch64,
            vendor: "apple".into(),
            os: Os::MacOS,
            env: Env::Unknown,
        }
    }

    pub fn wasm32_wasi() -> Self {
        Self {
            arch: Arch::Wasm32,
            vendor: "unknown".into(),
            os: Os::Wasi,
            env: Env::Unknown,
        }
    }

    pub fn executable_extension(&self) -> &'static str {
        match self.os {
            Os::Windows => "exe",
            Os::Wasi => "wasm",
            _ => "",
        }
    }

    pub fn pointer_width(&self) -> usize {
        match self.arch {
            Arch::X86_64 | Arch::Aarch64 => 64,
            Arch::Wasm32 => 32,
        }
    }
}

impl fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = match self.arch {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
            Arch::Wasm32 => "wasm32",
        };
        let os = match self.os {
            Os::Windows => "windows",
            Os::MacOS => "darwin",
            Os::Linux => "linux",
            Os::Wasi => "wasi",
        };
        let env = match self.env {
            Env::Msvc => "-msvc",
            Env::Gnu => "-gnu",
            Env::Musl => "-musl",
            Env::Unknown => "",
        };
        write!(f, "{}-{}-{}{}", arch, self.vendor, os, env)
    }
}
