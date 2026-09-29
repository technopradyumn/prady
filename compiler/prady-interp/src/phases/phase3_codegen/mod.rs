// Phase 3 — LLVM Native Code Generation & Executable Linking
// This module is the entry point for the LLVM backend.
// It translates Prady's typed IR into LLVM IR and compiles to native executables.
//
// Architecture:
//   AST (typed) → Lower to PradyIR → Emit LLVM IR → LLVM Optimization → Link → Binary

pub mod codegen;
pub mod ir;
pub mod lower;
pub mod link;
pub mod target;
pub mod optimize;

pub use codegen::CodeGenerator;
pub use ir::{PradyIr, IrModule, IrFunction, IrBasicBlock, IrInstr, IrType, IrValue};
pub use lower::Lowerer;
pub use target::TargetTriple;
