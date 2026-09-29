// Phase 3 — AST to PradyIR Lowerer
// Translates parsed AST definitions into flat SSA-ready PradyIR representation.

use super::ir::*;
use prady_ast as ast;
use std::collections::HashMap;

pub struct Lowerer {
    module: IrModule,
    temp_counter: usize,
    var_map: HashMap<String, String>,
}

impl Lowerer {
    pub fn new(module_name: impl Into<String>) -> Self {
        Self {
            module: IrModule::new(module_name),
            temp_counter: 0,
            var_map: HashMap::new(),
        }
    }

    fn fresh_reg(&mut self) -> String {
        let r = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        r
    }

    pub fn lower_program(mut self, program: &ast::Program) -> IrModule {
        for item in &program.items {
            match item {
                ast::Item::Function(func) => self.lower_function(func),
                ast::Item::Struct(s) => self.lower_struct(s),
                _ => {}
            }
        }
        self.module
    }

    fn lower_struct(&mut self, s: &ast::StructDecl) {
        let fields: Vec<(String, IrType)> = s
            .fields
            .iter()
            .map(|f| (f.name.name.clone(), IrType::I64))
            .collect();
        self.module.struct_defs.insert(s.name.name.clone(), fields);
    }

    fn lower_function(&mut self, func: &ast::FunctionDecl) {
        let params: Vec<(String, IrType)> = func
            .params
            .iter()
            .map(|p| (p.name.name.clone(), IrType::I64))
            .collect();

        let ret_ty = if func.return_type.is_some() {
            IrType::I64
        } else {
            IrType::Void
        };

        let mut ir_func = IrFunction::new(func.name.name.clone(), params.clone(), ret_ty.clone());
        let mut entry_block = IrBasicBlock::new("entry");

        self.var_map.clear();
        for (param_name, ty) in &params {
            let alloca_reg = self.fresh_reg();
            entry_block.push(IrInstr::Alloca {
                dest: alloca_reg.clone(),
                ty: ty.clone(),
            });
            entry_block.push(IrInstr::Store {
                ty: ty.clone(),
                val: IrValue::Reg(param_name.clone()),
                ptr: IrValue::Reg(alloca_reg.clone()),
            });
            self.var_map.insert(param_name.clone(), alloca_reg);
        }

        // Lower body statements
        for stmt in &func.body.stmts {
            self.lower_stmt(stmt, &mut entry_block);
        }

        // Ensure block termination
        if !entry_block.is_terminated() {
            entry_block.push(IrInstr::Ret {
                ty: ret_ty,
                val: None,
            });
        }

        ir_func.blocks.push(entry_block);
        self.module.add_function(ir_func);
    }

    fn lower_stmt(&mut self, stmt: &ast::Stmt, block: &mut IrBasicBlock) {
        match stmt {
            ast::Stmt::Let(l) => {
                let alloca_reg = self.fresh_reg();
                block.push(IrInstr::Alloca {
                    dest: alloca_reg.clone(),
                    ty: IrType::I64,
                });
                if let Some(init) = &l.init {
                    let val = self.lower_expr(init, block);
                    block.push(IrInstr::Store {
                        ty: IrType::I64,
                        val,
                        ptr: IrValue::Reg(alloca_reg.clone()),
                    });
                }
                self.var_map.insert(l.name.name.clone(), alloca_reg);
            }
            ast::Stmt::Return(expr, _) => {
                let val = expr.as_ref().map(|e| self.lower_expr(e, block));
                block.push(IrInstr::Ret {
                    ty: if val.is_some() {
                        IrType::I64
                    } else {
                        IrType::Void
                    },
                    val,
                });
            }
            ast::Stmt::Expr(expr) => {
                self.lower_expr(expr, block);
            }
            _ => {}
        }
    }

    fn lower_expr(&mut self, expr: &ast::Expr, block: &mut IrBasicBlock) -> IrValue {
        match expr {
            ast::Expr::Literal(lit, _) => match lit {
                ast::Literal::Int(n) => IrValue::Const(*n),
                ast::Literal::Float(f) => IrValue::ConstF(*f),
                ast::Literal::Bool(b) => IrValue::ConstBool(*b),
                _ => IrValue::Const(0),
            },
            ast::Expr::Ident(id) => {
                let alloca_reg = self.var_map.get(&id.name).cloned();
                if let Some(alloca_reg) = alloca_reg {
                    let dest = self.fresh_reg();
                    block.push(IrInstr::Load {
                        dest: dest.clone(),
                        ty: IrType::I64,
                        ptr: IrValue::Reg(alloca_reg),
                    });
                    IrValue::Reg(dest)
                } else {
                    IrValue::Reg(id.name.clone())
                }
            }
            ast::Expr::Binary(lhs, op, rhs, _) => {
                let l_val = self.lower_expr(lhs, block);
                let r_val = self.lower_expr(rhs, block);
                let dest = self.fresh_reg();

                match op {
                    ast::BinaryOp::Add => {
                        block.push(IrInstr::Add {
                            dest: dest.clone(),
                            ty: IrType::I64,
                            lhs: l_val,
                            rhs: r_val,
                        });
                    }
                    ast::BinaryOp::Sub => {
                        block.push(IrInstr::Sub {
                            dest: dest.clone(),
                            ty: IrType::I64,
                            lhs: l_val,
                            rhs: r_val,
                        });
                    }
                    ast::BinaryOp::Mul => {
                        block.push(IrInstr::Mul {
                            dest: dest.clone(),
                            ty: IrType::I64,
                            lhs: l_val,
                            rhs: r_val,
                        });
                    }
                    ast::BinaryOp::Div => {
                        block.push(IrInstr::Div {
                            dest: dest.clone(),
                            ty: IrType::I64,
                            lhs: l_val,
                            rhs: r_val,
                        });
                    }
                    ast::BinaryOp::Eq => {
                        block.push(IrInstr::ICmp {
                            dest: dest.clone(),
                            pred: CmpPred::Eq,
                            ty: IrType::I64,
                            lhs: l_val,
                            rhs: r_val,
                        });
                    }
                    _ => {}
                }
                IrValue::Reg(dest)
            }
            ast::Expr::Call(callee, args, _) => {
                let func_name = match &**callee {
                    ast::Expr::Ident(id) => id.name.clone(),
                    _ => "unknown".to_string(),
                };
                let lowered_args: Vec<(IrType, IrValue)> = args
                    .iter()
                    .map(|a| (IrType::I64, self.lower_expr(a, block)))
                    .collect();

                let dest = self.fresh_reg();
                block.push(IrInstr::Call {
                    dest: Some(dest.clone()),
                    func: func_name,
                    args: lowered_args,
                    ret_ty: IrType::I64,
                });
                IrValue::Reg(dest)
            }
            _ => IrValue::Const(0),
        }
    }
}
