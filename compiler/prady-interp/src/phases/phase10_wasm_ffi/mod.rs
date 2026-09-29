// Phase 10 — WebAssembly (WASI), FFI, Security Hardening, and Performance Suite
// Module root

pub mod wasm;
pub mod ffi;
pub mod security;
pub mod perf_suite;

pub use ffi::{FfiRegistry, FfiType, ForeignFunctionDecl};
pub use perf_suite::{BenchmarkResult, PerformanceSuite};
pub use security::SecurityPolicy;
pub use wasm::{WasmConfig, WasmModuleEmitter};
