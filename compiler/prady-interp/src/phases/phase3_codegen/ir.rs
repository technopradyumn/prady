// Phase 3 — Prady Intermediate Representation (PradyIR)
// A simple, flat IR used as the bridge between typed AST and LLVM IR.
// Inspired by LLVM IR semantics but Prady-specific.

use std::collections::HashMap;

// ─── Value Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Bool,
    Void,
    Ptr(Box<IrType>),
    Array(Box<IrType>, usize),
    Struct(String),
    Function(Vec<IrType>, Box<IrType>),
}

// ─── IR Value ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum IrValue {
    Const(i64),
    ConstF(f64),
    ConstBool(bool),
    Null,
    Undef,
    Reg(String),       // %name
    Global(String),    // @name
    Label(String),
}

// ─── IR Instructions ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum IrInstr {
    // Arithmetic
    Add  { dest: String, ty: IrType, lhs: IrValue, rhs: IrValue },
    Sub  { dest: String, ty: IrType, lhs: IrValue, rhs: IrValue },
    Mul  { dest: String, ty: IrType, lhs: IrValue, rhs: IrValue },
    Div  { dest: String, ty: IrType, lhs: IrValue, rhs: IrValue },
    Rem  { dest: String, ty: IrType, lhs: IrValue, rhs: IrValue },

    // Comparison
    ICmp { dest: String, pred: CmpPred, ty: IrType, lhs: IrValue, rhs: IrValue },
    FCmp { dest: String, pred: CmpPred, ty: IrType, lhs: IrValue, rhs: IrValue },

    // Memory
    Alloca { dest: String, ty: IrType },
    Load   { dest: String, ty: IrType, ptr: IrValue },
    Store  { ty: IrType, val: IrValue, ptr: IrValue },
    GEP    { dest: String, ty: IrType, ptr: IrValue, indices: Vec<IrValue> },

    // Control flow
    Br    { cond: IrValue, then_label: String, else_label: String },
    Jump  { label: String },
    Ret   { ty: IrType, val: Option<IrValue> },

    // Function calls
    Call  { dest: Option<String>, func: String, args: Vec<(IrType, IrValue)>, ret_ty: IrType },

    // Type conversions
    Trunc  { dest: String, from: IrType, val: IrValue, to: IrType },
    ZExt   { dest: String, from: IrType, val: IrValue, to: IrType },
    SExt   { dest: String, from: IrType, val: IrValue, to: IrType },
    Bitcast{ dest: String, from: IrType, val: IrValue, to: IrType },
    FPToSI { dest: String, from: IrType, val: IrValue, to: IrType },
    SIToFP { dest: String, from: IrType, val: IrValue, to: IrType },

    // Phi node (for SSA form)
    Phi    { dest: String, ty: IrType, incoming: Vec<(IrValue, String)> },

    // Unreachable / No-op
    Unreachable,
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpPred {
    Eq, Ne,
    Lt, Le,
    Gt, Ge,
}

// ─── IR Basic Block ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IrBasicBlock {
    pub label: String,
    pub instrs: Vec<IrInstr>,
}

impl IrBasicBlock {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), instrs: vec![] }
    }

    pub fn push(&mut self, instr: IrInstr) {
        self.instrs.push(instr);
    }

    pub fn is_terminated(&self) -> bool {
        matches!(
            self.instrs.last(),
            Some(IrInstr::Ret { .. } | IrInstr::Br { .. } | IrInstr::Jump { .. } | IrInstr::Unreachable)
        )
    }
}

// ─── IR Function ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub ret_ty: IrType,
    pub blocks: Vec<IrBasicBlock>,
    pub is_external: bool,
}

impl IrFunction {
    pub fn new(name: impl Into<String>, params: Vec<(String, IrType)>, ret_ty: IrType) -> Self {
        Self {
            name: name.into(),
            params,
            ret_ty,
            blocks: vec![],
            is_external: false,
        }
    }

    pub fn entry_block(&self) -> Option<&IrBasicBlock> {
        self.blocks.first()
    }
}

// ─── IR Module ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct IrModule {
    pub name: String,
    pub functions: Vec<IrFunction>,
    pub globals: HashMap<String, (IrType, Option<IrValue>)>,
    pub struct_defs: HashMap<String, Vec<(String, IrType)>>,
    pub target_triple: String,
}

impl IrModule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: vec![],
            globals: HashMap::new(),
            struct_defs: HashMap::new(),
            target_triple: String::from("x86_64-pc-linux-gnu"),
        }
    }

    pub fn add_function(&mut self, func: IrFunction) {
        self.functions.push(func);
    }

    pub fn define_global(&mut self, name: String, ty: IrType, init: Option<IrValue>) {
        self.globals.insert(name, (ty, init));
    }
}

pub type PradyIr = IrModule;
