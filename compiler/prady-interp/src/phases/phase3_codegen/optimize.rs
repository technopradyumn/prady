// Phase 3 — LLVM & PradyIR Optimization Pipeline
// Provides optimization passes on PradyIR modules before LLVM emission.

use super::ir::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    O0, // No optimization (fast compile)
    O1, // Basic optimization
    O2, // Standard release optimization
    O3, // Aggressive optimization
    Oz, // Optimize for binary size
}

pub struct Optimizer {
    pub level: OptLevel,
}

impl Optimizer {
    pub fn new(level: OptLevel) -> Self {
        Self { level }
    }

    /// Run the optimization pipeline on a module.
    pub fn run(&self, module: &mut IrModule) {
        if self.level == OptLevel::O0 {
            return;
        }

        for func in &mut module.functions {
            if func.is_external {
                continue;
            }

            self.constant_fold(func);
            self.eliminate_dead_code(func);
            self.clean_empty_blocks(func);
        }
    }

    /// Simple constant folding pass on IR instructions.
    fn constant_fold(&self, func: &mut IrFunction) {
        for block in &mut func.blocks {
            for instr in &mut block.instrs {
                match instr {
                    IrInstr::Add { ty: IrType::I64, lhs: IrValue::Const(a), rhs: IrValue::Const(b), .. } => {
                        let sum = a.wrapping_add(*b);
                        *instr = IrInstr::Add {
                            dest: match instr {
                                IrInstr::Add { dest, .. } => dest.clone(),
                                _ => unreachable!(),
                            },
                            ty: IrType::I64,
                            lhs: IrValue::Const(sum),
                            rhs: IrValue::Const(0),
                        };
                    }
                    IrInstr::Mul { ty: IrType::I64, lhs: IrValue::Const(a), rhs: IrValue::Const(b), .. } => {
                        let prod = a.wrapping_mul(*b);
                        *instr = IrInstr::Add {
                            dest: match instr {
                                IrInstr::Mul { dest, .. } => dest.clone(),
                                _ => unreachable!(),
                            },
                            ty: IrType::I64,
                            lhs: IrValue::Const(prod),
                            rhs: IrValue::Const(0),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    /// Eliminate unreachable blocks and instructions after unconditional returns or jumps.
    fn eliminate_dead_code(&self, func: &mut IrFunction) {
        for block in &mut func.blocks {
            let mut cut_index = None;
            for (idx, instr) in block.instrs.iter().enumerate() {
                if matches!(instr, IrInstr::Ret { .. } | IrInstr::Jump { .. } | IrInstr::Unreachable) {
                    cut_index = Some(idx + 1);
                    break;
                }
            }
            if let Some(cut) = cut_index {
                block.instrs.truncate(cut);
            }
        }
    }

    /// Remove empty or redundant blocks.
    fn clean_empty_blocks(&self, func: &mut IrFunction) {
        func.blocks.retain(|b| !b.instrs.is_empty() || b.label == "entry");
    }
}
