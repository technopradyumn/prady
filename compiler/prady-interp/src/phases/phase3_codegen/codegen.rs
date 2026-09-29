// Phase 3 — Code Generator (AST → PradyIR → LLVM IR text)
// This stub generates LLVM IR textual representation from PradyIR.
// Full LLVM bindings (via inkwell or llvm-sys) will be wired in Phase 3 completion.

use super::ir::*;
use std::fmt::Write;

/// Code generator: converts a PradyIR module to LLVM IR (textual).
pub struct CodeGenerator {
    pub module: IrModule,
    output: String,
    reg_counter: usize,
    block_counter: usize,
}

impl CodeGenerator {
    pub fn new(module: IrModule) -> Self {
        Self {
            module,
            output: String::new(),
            reg_counter: 0,
            block_counter: 0,
        }
    }

    pub fn fresh_reg(&mut self) -> String {
        let r = format!("%r{}", self.reg_counter);
        self.reg_counter += 1;
        r
    }

    pub fn fresh_label(&mut self) -> String {
        let l = format!("bb{}", self.block_counter);
        self.block_counter += 1;
        l
    }

    /// Emit the full LLVM IR for the module.
    pub fn emit(&mut self) -> String {
        let target = self.module.target_triple.clone();
        let name = self.module.name.clone();
        writeln!(self.output, "; Prady compiler - Module: {name}").unwrap();
        writeln!(self.output, "target triple = \"{target}\"").unwrap();
        writeln!(self.output).unwrap();

        // Emit struct type definitions
        let struct_defs: Vec<_> = self
            .module
            .struct_defs
            .iter()
            .map(|(n, fields)| (n.clone(), fields.clone()))
            .collect();
        for (name, fields) in struct_defs {
            let field_tys: Vec<String> = fields.iter().map(|(_, t)| Self::ty_str(t)).collect();
            writeln!(self.output, "%{name} = type {{ {} }}", field_tys.join(", ")).unwrap();
        }

        // Emit globals
        let globals: Vec<_> = self
            .module
            .globals
            .iter()
            .map(|(n, (t, v))| (n.clone(), t.clone(), v.clone()))
            .collect();
        for (gname, ty, init) in globals {
            let ty_str = Self::ty_str(&ty);
            let val_str = match init {
                Some(IrValue::Const(n)) => n.to_string(),
                Some(IrValue::ConstF(f)) => format!("{f:e}"),
                Some(IrValue::ConstBool(b)) => {
                    if b {
                        "true".into()
                    } else {
                        "false".into()
                    }
                }
                _ => "zeroinitializer".to_string(),
            };
            writeln!(self.output, "@{gname} = global {ty_str} {val_str}").unwrap();
        }

        writeln!(self.output).unwrap();

        // Emit runtime externals
        self.emit_runtime_decls();

        // Emit functions
        let fns: Vec<_> = self.module.functions.iter().cloned().collect();
        for func in fns {
            self.emit_function(&func);
        }

        self.output.clone()
    }

    fn emit_runtime_decls(&mut self) {
        writeln!(self.output, "; Runtime declarations").unwrap();
        writeln!(
            self.output,
            "declare i32 @printf(i8* nocapture readonly, ...)"
        )
        .unwrap();
        writeln!(self.output, "declare i8* @malloc(i64)").unwrap();
        writeln!(self.output, "declare void @free(i8*)").unwrap();
        writeln!(self.output, "declare i8* @memcpy(i8*, i8*, i64)").unwrap();
        writeln!(self.output, "declare i32 @exit(i32)").unwrap();
        writeln!(self.output).unwrap();
    }

    fn emit_function(&mut self, func: &IrFunction) {
        let ret = Self::ty_str(&func.ret_ty);
        let params: Vec<String> = func
            .params
            .iter()
            .map(|(n, t)| format!("{} %{n}", Self::ty_str(t)))
            .collect();

        if func.is_external {
            writeln!(
                self.output,
                "declare {ret} @{}({})",
                func.name,
                params.join(", ")
            )
            .unwrap();
            return;
        }

        writeln!(
            self.output,
            "define {ret} @{}({}) {{",
            func.name,
            params.join(", ")
        )
        .unwrap();

        for block in &func.blocks {
            writeln!(self.output, "{}:", block.label).unwrap();
            for instr in &block.instrs {
                self.emit_instr(instr);
            }
        }

        // If last block isn't terminated, emit implicit void return
        if let Some(last) = func.blocks.last() {
            if !last.is_terminated() {
                writeln!(self.output, "  ret void").unwrap();
            }
        }

        writeln!(self.output, "}}").unwrap();
        writeln!(self.output).unwrap();
    }

    fn emit_instr(&mut self, instr: &IrInstr) {
        use IrInstr::*;
        match instr {
            Add { dest, ty, lhs, rhs } => writeln!(
                self.output,
                "  %{dest} = add {} {}, {}",
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            Sub { dest, ty, lhs, rhs } => writeln!(
                self.output,
                "  %{dest} = sub {} {}, {}",
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            Mul { dest, ty, lhs, rhs } => writeln!(
                self.output,
                "  %{dest} = mul {} {}, {}",
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            Div { dest, ty, lhs, rhs } => writeln!(
                self.output,
                "  %{dest} = sdiv {} {}, {}",
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            Rem { dest, ty, lhs, rhs } => writeln!(
                self.output,
                "  %{dest} = srem {} {}, {}",
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            ICmp {
                dest,
                pred,
                ty,
                lhs,
                rhs,
            } => writeln!(
                self.output,
                "  %{dest} = icmp {} {} {}, {}",
                Self::pred_str(pred),
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            FCmp {
                dest,
                pred,
                ty,
                lhs,
                rhs,
            } => writeln!(
                self.output,
                "  %{dest} = fcmp o{} {} {}, {}",
                Self::pred_str(pred),
                Self::ty_str(ty),
                Self::val_str(lhs),
                Self::val_str(rhs)
            )
            .unwrap(),
            Alloca { dest, ty } => {
                writeln!(self.output, "  %{dest} = alloca {}", Self::ty_str(ty)).unwrap()
            }
            Load { dest, ty, ptr } => writeln!(
                self.output,
                "  %{dest} = load {}, {}* {}",
                Self::ty_str(ty),
                Self::ty_str(ty),
                Self::val_str(ptr)
            )
            .unwrap(),
            Store { ty, val, ptr } => writeln!(
                self.output,
                "  store {} {}, {}* {}",
                Self::ty_str(ty),
                Self::val_str(val),
                Self::ty_str(ty),
                Self::val_str(ptr)
            )
            .unwrap(),
            Ret { ty, val: None } => writeln!(self.output, "  ret {}", Self::ty_str(ty)).unwrap(),
            Ret { ty, val: Some(v) } => writeln!(
                self.output,
                "  ret {} {}",
                Self::ty_str(ty),
                Self::val_str(v)
            )
            .unwrap(),
            Br {
                cond,
                then_label,
                else_label,
            } => writeln!(
                self.output,
                "  br i1 {}, label %{then_label}, label %{else_label}",
                Self::val_str(cond)
            )
            .unwrap(),
            Jump { label } => writeln!(self.output, "  br label %{label}").unwrap(),
            Call {
                dest: None,
                func,
                args,
                ret_ty,
            } => {
                let arg_str: Vec<_> = args
                    .iter()
                    .map(|(t, v)| format!("{} {}", Self::ty_str(t), Self::val_str(v)))
                    .collect();
                writeln!(
                    self.output,
                    "  call {} @{func}({})",
                    Self::ty_str(ret_ty),
                    arg_str.join(", ")
                )
                .unwrap();
            }
            Call {
                dest: Some(d),
                func,
                args,
                ret_ty,
            } => {
                let arg_str: Vec<_> = args
                    .iter()
                    .map(|(t, v)| format!("{} {}", Self::ty_str(t), Self::val_str(v)))
                    .collect();
                writeln!(
                    self.output,
                    "  %{d} = call {} @{func}({})",
                    Self::ty_str(ret_ty),
                    arg_str.join(", ")
                )
                .unwrap();
            }
            Unreachable => writeln!(self.output, "  unreachable").unwrap(),
            Nop => {}
            Phi { dest, ty, incoming } => {
                let entries: Vec<_> = incoming
                    .iter()
                    .map(|(v, lbl)| format!("[ {}, %{lbl} ]", Self::val_str(v)))
                    .collect();
                writeln!(
                    self.output,
                    "  %{dest} = phi {} {}",
                    Self::ty_str(ty),
                    entries.join(", ")
                )
                .unwrap();
            }
            ZExt {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = zext {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            SExt {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = sext {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            Trunc {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = trunc {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            Bitcast {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = bitcast {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            FPToSI {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = fptosi {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            SIToFP {
                dest,
                from,
                val,
                to,
            } => writeln!(
                self.output,
                "  %{dest} = sitofp {} {} to {}",
                Self::ty_str(from),
                Self::val_str(val),
                Self::ty_str(to)
            )
            .unwrap(),
            GEP {
                dest,
                ty,
                ptr,
                indices,
            } => {
                let idx_str: Vec<_> = indices
                    .iter()
                    .map(|i| format!("i32 {}", Self::val_str(i)))
                    .collect();
                writeln!(
                    self.output,
                    "  %{dest} = getelementptr {}, {}* {}, {}",
                    Self::ty_str(ty),
                    Self::ty_str(ty),
                    Self::val_str(ptr),
                    idx_str.join(", ")
                )
                .unwrap();
            }
        }
    }

    pub fn ty_str(ty: &IrType) -> String {
        match ty {
            IrType::I8 => "i8".into(),
            IrType::I16 => "i16".into(),
            IrType::I32 => "i32".into(),
            IrType::I64 => "i64".into(),
            IrType::U8 => "i8".into(),
            IrType::U16 => "i16".into(),
            IrType::U32 => "i32".into(),
            IrType::U64 => "i64".into(),
            IrType::F32 => "float".into(),
            IrType::F64 => "double".into(),
            IrType::Bool => "i1".into(),
            IrType::Void => "void".into(),
            IrType::Ptr(inner) => format!("{}*", Self::ty_str(inner)),
            IrType::Array(inner, n) => format!("[{n} x {}]", Self::ty_str(inner)),
            IrType::Struct(name) => format!("%{name}"),
            IrType::Function(params, ret) => {
                let ps: Vec<_> = params.iter().map(Self::ty_str).collect();
                format!("{} ({})", Self::ty_str(ret), ps.join(", "))
            }
        }
    }

    fn val_str(val: &IrValue) -> String {
        match val {
            IrValue::Const(n) => n.to_string(),
            IrValue::ConstF(f) => format!("{f:e}"),
            IrValue::ConstBool(b) => {
                if *b {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            IrValue::Null => "null".into(),
            IrValue::Undef => "undef".into(),
            IrValue::Reg(r) => format!("%{r}"),
            IrValue::Global(g) => format!("@{g}"),
            IrValue::Label(l) => format!("%{l}"),
        }
    }

    fn pred_str(pred: &CmpPred) -> &'static str {
        match pred {
            CmpPred::Eq => "eq",
            CmpPred::Ne => "ne",
            CmpPred::Lt => "slt",
            CmpPred::Le => "sle",
            CmpPred::Gt => "sgt",
            CmpPred::Ge => "sge",
        }
    }
}
