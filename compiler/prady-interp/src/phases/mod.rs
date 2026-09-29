// Prady Language Roadmap Phases Implementation
// Provides complete systems for Phases 3 through 11.

pub mod phase3_codegen;
pub mod phase4_types_modules;
pub mod phase5_dsa;
pub mod phase6_oop_arch;
pub mod phase7_async_net;
pub mod phase8_tooling;
pub mod phase10_wasm_ffi;
pub mod phase11_stabilization;

// Convenient re-exports
pub use phase3_codegen::{CodeGenerator, IrBasicBlock, IrFunction, IrInstr, IrModule, IrType, IrValue, Lowerer, TargetTriple};
pub use phase4_types_modules::{AdtPayload, EnumInstance, EnumTypeDef, GenericSolver, MatchArm, ModuleSystem, Pattern, PatternEngine, PradyOption, PradyResult};
pub use phase5_dsa::{get_dsa_specifications, BigO, CheckedDeque, CheckedVector, DsaContract};
pub use phase6_oop_arch::{ArchitecturePolicyEngine, ClassDef, ClassHierarchy, TraitDef};
pub use phase7_async_net::{AsyncRuntime, HttpMethod, HttpRequest, HttpResponse, HttpRouter, JsonValue, ServerTemplate, TaskStatus};
pub use phase8_tooling::{CodeFormatter, LintDiagnostic, LintSeverity, Linter, Manifest, PackageRegistryClient};
pub use phase10_wasm_ffi::{BenchmarkResult, PerformanceSuite, SecurityPolicy, WasmConfig, WasmModuleEmitter};
pub use phase11_stabilization::{ConformanceSuite, ReleaseManager, ReleaseMetadata};
