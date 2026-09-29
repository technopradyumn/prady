// Phase 8 — Package Manager, Registry, Formatter, and Linter
// Module root

pub mod package_manager;
pub mod registry;
pub mod formatter;
pub mod linter;

pub use formatter::CodeFormatter;
pub use linter::{LintDiagnostic, LintSeverity, Linter};
pub use package_manager::{DepSource, Lockfile, LockfileEntry, Manifest};
pub use registry::{PackageRegistryClient, RegistryPackageInfo};
