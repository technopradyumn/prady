// Phase 3 — LLVM Native Code Generation & Executable Linking
// This module is the entry point for the LLVM backend.
// It translates Prady's typed IR into LLVM IR and compiles to native executables.
//
// Architecture:
//   AST (typed) → Lower to PradyIR → Emit LLVM IR → LLVM Optimization → Link → Binary

pub mod codegen;
pub mod ir;
pub mod link;
pub mod lower;
pub mod optimize;
pub mod target;

pub use codegen::CodeGenerator;
pub use ir::{IrBasicBlock, IrFunction, IrInstr, IrModule, IrType, IrValue, PradyIr};
pub use lower::Lowerer;
pub use target::TargetTriple;
