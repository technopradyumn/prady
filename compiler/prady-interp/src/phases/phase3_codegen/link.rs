// Phase 3 — Executable Linking & Toolchain Integration
// Coordinates invoking system linkers (LLD, MSVC link.exe, GCC, Clang) to produce native binaries.

use super::target::{Os, TargetTriple};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct LinkerConfig {
    pub target: TargetTriple,
    pub output_path: PathBuf,
    pub libraries: Vec<String>,
    pub library_paths: Vec<PathBuf>,
    pub optimize: bool,
    pub debug_symbols: bool,
}

impl LinkerConfig {
    pub fn new(output_path: impl Into<PathBuf>, target: TargetTriple) -> Self {
        Self {
            target,
            output_path: output_path.into(),
            libraries: vec![],
            library_paths: vec![],
            optimize: false,
            debug_symbols: true,
        }
    }

    pub fn add_lib(&mut self, lib: impl Into<String>) {
        self.libraries.push(lib.into());
    }

    pub fn add_lib_path(&mut self, path: impl Into<PathBuf>) {
        self.library_paths.push(path.into());
    }
}

pub struct Linker {
    pub config: LinkerConfig,
}

impl Linker {
    pub fn new(config: LinkerConfig) -> Self {
        Self { config }
    }

    /// Link an object file or LLVM bitcode into a final executable.
    pub fn link(&self, object_file: &Path) -> Result<PathBuf, String> {
        let out = &self.config.output_path;

        match self.config.target.os {
            Os::Windows => self.link_windows_msvc(object_file, out),
            Os::Linux => self.link_unix_gcc_or_clang("clang", object_file, out),
            Os::MacOS => self.link_unix_gcc_or_clang("clang", object_file, out),
            Os::Wasi => self.link_wasi(object_file, out),
        }
    }

    fn link_windows_msvc(&self, object_file: &Path, out: &Path) -> Result<PathBuf, String> {
        // Try clang or lld-link or link.exe
        let linkers = ["clang", "lld-link", "link.exe", "gcc"];
        for lld in &linkers {
            let mut cmd = Command::new(lld);
            cmd.arg(object_file);
            cmd.arg(format!("-o{}", out.display()));

            for lib in &self.config.libraries {
                cmd.arg(format!("-l{lib}"));
            }

            if let Ok(status) = cmd.status() {
                if status.success() {
                    return Ok(out.to_path_buf());
                }
            }
        }

        // If system linker is not available, report clear error
        Err(format!(
            "Failed to locate a suitable linker (clang, lld-link, link.exe, gcc) to link '{}'. Please install Visual Studio Build Tools or LLVM.",
            out.display()
        ))
    }

    fn link_unix_gcc_or_clang(
        &self,
        default_tool: &str,
        object_file: &Path,
        out: &Path,
    ) -> Result<PathBuf, String> {
        let tools = [default_tool, "gcc", "clang", "ld"];
        for tool in &tools {
            let mut cmd = Command::new(tool);
            cmd.arg(object_file);
            cmd.arg("-o").arg(out);

            for lib in &self.config.libraries {
                cmd.arg(format!("-l{lib}"));
            }

            if let Ok(status) = cmd.status() {
                if status.success() {
                    return Ok(out.to_path_buf());
                }
            }
        }

        Err(format!(
            "Failed to invoke native linker ({default_tool}/gcc/ld) to link '{}'. Ensure gcc or clang is in PATH.",
            out.display()
        ))
    }

    fn link_wasi(&self, object_file: &Path, out: &Path) -> Result<PathBuf, String> {
        let mut cmd = Command::new("wasm-ld");
        cmd.arg(object_file);
        cmd.arg("-o").arg(out);

        match cmd.status() {
            Ok(status) if status.success() => Ok(out.to_path_buf()),
            _ => Err("Failed to invoke wasm-ld for WebAssembly link target. Ensure wasi-sdk is installed.".to_string()),
        }
    }
}
