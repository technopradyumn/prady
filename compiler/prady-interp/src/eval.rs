use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use prady_ast::*;
use prady_diagnostics::Span;

use crate::env::Environment;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub span: Option<Span>,
}

impl RuntimeError {
    pub fn new(message: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

pub enum FlowSignal {
    Error(RuntimeError),
    Return(Value),
    Break(Option<Value>),
    Continue,
}

impl From<RuntimeError> for FlowSignal {
    fn from(err: RuntimeError) -> Self {
        FlowSignal::Error(err)
    }
}

pub struct Interpreter {
    pub program: Program,
    pub functions: HashMap<String, FunctionDecl>,
    pub structs: HashMap<String, StructDecl>,
    pub classes: HashMap<String, ClassDecl>,
    pub enums: HashMap<String, EnumDecl>,
    pub output_buffer: Option<Rc<RefCell<Vec<String>>>>,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let mut functions = HashMap::new();
        let mut structs = HashMap::new();
        let mut classes = HashMap::new();
        let mut enums = HashMap::new();

        for item in &program.items {
            match item {
                Item::Function(f) => {
                    functions.insert(f.name.name.clone(), f.clone());
                }
                Item::Struct(s) => {
                    structs.insert(s.name.name.clone(), s.clone());
                }
                Item::Class(c) => {
                    classes.insert(c.name.name.clone(), c.clone());
                }
                Item::Enum(e) => {
                    enums.insert(e.name.name.clone(), e.clone());
                }
                _ => {}
            }
        }

        Self {
            program,
            functions,
            structs,
            classes,
            enums,
            output_buffer: None,
        }
    }

    pub fn with_output_buffer(mut self, buffer: Rc<RefCell<Vec<String>>>) -> Self {
        self.output_buffer = Some(buffer);
        self
    }

    fn print_output(&mut self, text: &str) {
        if let Some(ref buf) = self.output_buffer {
            buf.borrow_mut().push(text.to_string());
        } else {
            println!("{}", text);
        }
    }

    fn is_variant_name(&self, name: &str) -> bool {
        for enum_decl in self.enums.values() {
            for v in &enum_decl.variants {
                if v.name.name == name {
                    return true;
                }
            }
        }
        name == "None" || name == "Ok" || name == "Err" || name == "Some"
    }

    pub fn run_main(&mut self) -> Result<Value, RuntimeError> {
        let main_fn = match self.functions.get("main").cloned() {
            Some(f) => f,
            None => {
                return Err(RuntimeError::new(
                    "No 'main()' function found. Every Prady program starts execution in 'fn main()'.",
                    None,
                ))
            }
        };

        let global_env = Rc::new(RefCell::new(Environment::new()));
        self.execute_function(&main_fn, vec![], global_env)
    }

    pub fn execute_function(
        &mut self,
        func: &FunctionDecl,
        args: Vec<Value>,
        parent_env: Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::with_parent(parent_env)));

        for (i, param) in func.params.iter().enumerate() {
            let arg_val = args.get(i).cloned().unwrap_or(Value::Null);
            env.borrow_mut()
                .define(&param.name.name, arg_val, param.is_mut);
        }

        match self.execute_block(&func.body, env) {
            Ok(v) => Ok(v),
            Err(FlowSignal::Return(v)) => Ok(v),
            Err(FlowSignal::Error(err)) => Err(err),
            Err(FlowSignal::Break(_)) | Err(FlowSignal::Continue) => Err(RuntimeError::new(
                "Break or continue outside of loop",
                Some(func.span),
            )),
        }
    }

    pub fn execute_block(
        &mut self,
        block: &Block,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, FlowSignal> {
        let mut last_val = Value::Null;

        for stmt in &block.stmts {
            last_val = self.execute_stmt(stmt, Rc::clone(&env))?;
        }

        Ok(last_val)
    }

    pub fn execute_stmt(
        &mut self,
        stmt: &Stmt,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, FlowSignal> {
        match stmt {
            Stmt::Let(let_stmt) => {
                let init_val = if let Some(ref init_expr) = let_stmt.init {
                    self.eval_value(init_expr, Rc::clone(&env))?
                } else {
                    Value::Null
                };

                env.borrow_mut()
                    .define(&let_stmt.name.name, init_val, let_stmt.is_mut);
                Ok(Value::Null)
            }

            Stmt::Assign(assign_stmt) => {
                let val = self.eval_value(&assign_stmt.value, Rc::clone(&env))?;

                match &assign_stmt.target {
                    Expr::Ident(id) => {
                        let final_val = match assign_stmt.op {
                            AssignOp::Assign => val,
                            AssignOp::AddAssign => {
                                let cur = env.borrow().get(&id.name).unwrap_or(Value::Null);
                                self.eval_binary_op(&cur, BinaryOp::Add, &val, assign_stmt.span)?
                            }
                            AssignOp::SubAssign => {
                                let cur = env.borrow().get(&id.name).unwrap_or(Value::Null);
                                self.eval_binary_op(&cur, BinaryOp::Sub, &val, assign_stmt.span)?
                            }
                            AssignOp::MulAssign => {
                                let cur = env.borrow().get(&id.name).unwrap_or(Value::Null);
                                self.eval_binary_op(&cur, BinaryOp::Mul, &val, assign_stmt.span)?
                            }
                            AssignOp::DivAssign => {
                                let cur = env.borrow().get(&id.name).unwrap_or(Value::Null);
                                self.eval_binary_op(&cur, BinaryOp::Div, &val, assign_stmt.span)?
                            }
                            AssignOp::ModAssign => {
                                let cur = env.borrow().get(&id.name).unwrap_or(Value::Null);
                                self.eval_binary_op(&cur, BinaryOp::Mod, &val, assign_stmt.span)?
                            }
                        };

                        if let Err(err) = env.borrow_mut().assign(&id.name, final_val) {
                            return Err(RuntimeError::new(err, Some(id.span)).into());
                        }
                    }
                    Expr::Index(target_expr, idx_expr, span) => {
                        let target_val = self.eval_value(target_expr, Rc::clone(&env))?;
                        let idx_val = self.eval_value(idx_expr, Rc::clone(&env))?;

                        if let Value::Array(arr_rc) = target_val {
                            if let Value::Int(idx) = idx_val {
                                let mut arr = arr_rc.borrow_mut();
                                if idx < 0 || idx as usize >= arr.len() {
                                    return Err(RuntimeError::new(
                                        format!("Index out of bounds: index {} on array of length {}", idx, arr.len()),
                                        Some(*span),
                                    ).into());
                                }
                                let new_val = match assign_stmt.op {
                                    AssignOp::Assign => val,
                                    AssignOp::AddAssign => self.eval_binary_op(&arr[idx as usize], BinaryOp::Add, &val, *span)?,
                                    AssignOp::SubAssign => self.eval_binary_op(&arr[idx as usize], BinaryOp::Sub, &val, *span)?,
                                    AssignOp::MulAssign => self.eval_binary_op(&arr[idx as usize], BinaryOp::Mul, &val, *span)?,
                                    AssignOp::DivAssign => self.eval_binary_op(&arr[idx as usize], BinaryOp::Div, &val, *span)?,
                                    AssignOp::ModAssign => self.eval_binary_op(&arr[idx as usize], BinaryOp::Mod, &val, *span)?,
                                };
                                arr[idx as usize] = new_val;
                            } else {
                                return Err(RuntimeError::new("Array index must be an integer", Some(*span)).into());
                            }
                        } else {
                            return Err(RuntimeError::new("Cannot index non-array value", Some(*span)).into());
                        }
                    }
                    Expr::FieldAccess(target_expr, field_id, span) => {
                        let target_val = self.eval_value(target_expr, Rc::clone(&env))?;
                        if let Value::Struct { fields, .. } = target_val {
                            let mut map = fields.borrow_mut();
                            let new_val = match assign_stmt.op {
                                AssignOp::Assign => val,
                                AssignOp::AddAssign => {
                                    let cur = map.get(&field_id.name).cloned().unwrap_or(Value::Null);
                                    self.eval_binary_op(&cur, BinaryOp::Add, &val, *span)?
                                }
                                AssignOp::SubAssign => {
                                    let cur = map.get(&field_id.name).cloned().unwrap_or(Value::Null);
                                    self.eval_binary_op(&cur, BinaryOp::Sub, &val, *span)?
                                }
                                AssignOp::MulAssign => {
                                    let cur = map.get(&field_id.name).cloned().unwrap_or(Value::Null);
                                    self.eval_binary_op(&cur, BinaryOp::Mul, &val, *span)?
                                }
                                AssignOp::DivAssign => {
                                    let cur = map.get(&field_id.name).cloned().unwrap_or(Value::Null);
                                    self.eval_binary_op(&cur, BinaryOp::Div, &val, *span)?
                                }
                                AssignOp::ModAssign => {
                                    let cur = map.get(&field_id.name).cloned().unwrap_or(Value::Null);
                                    self.eval_binary_op(&cur, BinaryOp::Mod, &val, *span)?
                                }
                            };
                            map.insert(field_id.name.clone(), new_val);
                        } else {
                            return Err(RuntimeError::new("Cannot assign field on non-struct value", Some(*span)).into());
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            "Invalid assignment target",
                            Some(assign_stmt.span),
                        ).into());
                    }
                }

                Ok(Value::Null)
            }

            Stmt::Expr(expr) => self.eval_value(expr, env),

            Stmt::Return(opt_expr, _) => {
                let ret_val = if let Some(ref expr) = opt_expr {
                    self.eval_value(expr, env)?
                } else {
                    Value::Null
                };
                Err(FlowSignal::Return(ret_val))
            }

            Stmt::Break(opt_expr, _) => {
                let brk_val = if let Some(ref expr) = opt_expr {
                    Some(self.eval_value(expr, env)?)
                } else {
                    None
                };
                Err(FlowSignal::Break(brk_val))
            }

            Stmt::Continue(_) => Err(FlowSignal::Continue),

            Stmt::Using(ident, init_expr, body, _) => {
                let val = self.eval_value(init_expr, Rc::clone(&env))?;
                let block_env = Rc::new(RefCell::new(Environment::with_parent(env)));
                block_env.borrow_mut().define(&ident.name, val, false);
                self.execute_block(body, block_env)
            }
        }
    }

    pub fn eval_value(
        &mut self,
        expr: &Expr,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, FlowSignal> {
        match expr {
            Expr::Literal(lit, _) => match lit {
                Literal::Int(i) => Ok(Value::Int(*i)),
                Literal::Float(f) => Ok(Value::Float(*f)),
                Literal::String(s) => Ok(Value::String(s.clone())),
                Literal::Char(c) => Ok(Value::Char(*c)),
                Literal::Bool(b) => Ok(Value::Bool(*b)),
                Literal::Null => Ok(Value::Null),
            },

            Expr::Ident(id) => {
                if let Some(val) = env.borrow().get(&id.name) {
                    return Ok(val);
                }

                // Check enum variants without payload (e.g. Status::Pending or Pending)
                for enum_decl in self.enums.values() {
                    for variant in &enum_decl.variants {
                        if variant.name.name == id.name && variant.payload.is_none() {
                            return Ok(Value::Variant {
                                enum_name: Some(enum_decl.name.name.clone()),
                                variant_name: variant.name.name.clone(),
                                payload: vec![],
                            });
                        }
                    }
                }

                // Check None constructor
                if id.name == "None" {
                    return Ok(Value::Variant {
                        enum_name: Some("Option".to_string()),
                        variant_name: "None".to_string(),
                        payload: vec![],
                    });
                }

                Err(RuntimeError::new(
                    format!("Undefined identifier '{}'", id.name),
                    Some(id.span),
                ).into())
            }

            Expr::Binary(left, op, right, span) => {
                // Short-circuit logical operations
                if *op == BinaryOp::And {
                    let left_val = self.eval_value(left, Rc::clone(&env))?;
                    if !left_val.is_truthy() {
                        return Ok(Value::Bool(false));
                    }
                    let right_val = self.eval_value(right, env)?;
                    return Ok(Value::Bool(right_val.is_truthy()));
                }

                if *op == BinaryOp::Or {
                    let left_val = self.eval_value(left, Rc::clone(&env))?;
                    if left_val.is_truthy() {
                        return Ok(Value::Bool(true));
                    }
                    let right_val = self.eval_value(right, env)?;
                    return Ok(Value::Bool(right_val.is_truthy()));
                }

                let left_val = self.eval_value(left, Rc::clone(&env))?;
                let right_val = self.eval_value(right, env)?;
                Ok(self.eval_binary_op(&left_val, *op, &right_val, *span)?)
            }

            Expr::Unary(op, inner, span) => {
                let val = self.eval_value(inner, env)?;
                match op {
                    UnaryOp::Neg => match val {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(RuntimeError::new(
                            format!("Unary '-' cannot be applied to type {}", val.type_name()),
                            Some(*span),
                        ).into()),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                    UnaryOp::BitNot => match val {
                        Value::Int(i) => Ok(Value::Int(!i)),
                        _ => Err(RuntimeError::new(
                            format!("Bitwise '~' cannot be applied to type {}", val.type_name()),
                            Some(*span),
                        ).into()),
                    },
                }
            }

            Expr::Call(callee, args, span) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_value(arg, Rc::clone(&env))?);
                }

                if let Expr::Ident(ref id) = **callee {
                    // 1. Built-in functions
                    if id.name == "print" || id.name == "println" {
                        let formatted: Vec<String> = evaluated_args
                            .iter()
                            .map(|a| a.to_display_string())
                            .collect();
                        let line = formatted.join(" ");
                        self.print_output(&line);
                        return Ok(Value::Null);
                    }

                    if id.name == "assert" {
                        let cond = evaluated_args.first().map(|v| v.is_truthy()).unwrap_or(false);
                        if !cond {
                            let msg = evaluated_args
                                .get(1)
                                .map(|v| v.to_display_string())
                                .unwrap_or_else(|| "Assertion failed".to_string());
                            return Err(RuntimeError::new(msg, Some(*span)).into());
                        }
                        return Ok(Value::Bool(true));
                    }

                    if id.name == "len" {
                        if let Some(arg) = evaluated_args.first() {
                            match arg {
                                Value::Array(arr) => return Ok(Value::Int(arr.borrow().len() as i64)),
                                Value::String(s) => return Ok(Value::Int(s.len() as i64)),
                                _ => {
                                    return Err(RuntimeError::new(
                                        format!("'len' not supported for type {}", arg.type_name()),
                                        Some(*span),
                                    ).into())
                                }
                            }
                        }
                        return Err(RuntimeError::new("'len' requires 1 argument", Some(*span)).into());
                    }

                    if id.name == "type_of" {
                        if let Some(arg) = evaluated_args.first() {
                            return Ok(Value::String(arg.type_name().to_string()));
                        }
                        return Ok(Value::Null);
                    }

                    // Built-in Variant Constructors: Ok, Err, Some
                    if id.name == "Ok" {
                        return Ok(Value::Variant {
                            enum_name: Some("Result".to_string()),
                            variant_name: "Ok".to_string(),
                            payload: evaluated_args,
                        });
                    }

                    if id.name == "Err" {
                        return Ok(Value::Variant {
                            enum_name: Some("Result".to_string()),
                            variant_name: "Err".to_string(),
                            payload: evaluated_args,
                        });
                    }

                    if id.name == "Some" {
                        return Ok(Value::Variant {
                            enum_name: Some("Option".to_string()),
                            variant_name: "Some".to_string(),
                            payload: evaluated_args,
                        });
                    }

                    // Check Enum Variants
                    for enum_decl in self.enums.values() {
                        for variant in &enum_decl.variants {
                            if variant.name.name == id.name {
                                return Ok(Value::Variant {
                                    enum_name: Some(enum_decl.name.name.clone()),
                                    variant_name: variant.name.name.clone(),
                                    payload: evaluated_args,
                                });
                            }
                        }
                    }

                    // Check user-defined functions
                    if let Some(func_decl) = self.functions.get(&id.name).cloned() {
                        return Ok(self.execute_function(&func_decl, evaluated_args, Rc::clone(&env))?);
                    }

                    // Check Class constructor (e.g. User("John", 25) or User())
                    if let Some(class_decl) = self.classes.get(&id.name).cloned() {
                        if let Some(new_method) = class_decl.methods.iter().find(|m| m.name.name == "new").cloned() {
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            return Ok(self.execute_function(&new_method, evaluated_args, method_env)?);
                        }

                        let mut field_map = HashMap::new();
                        // First: apply default_init for each field
                        for field in &class_decl.fields {
                            let default_val = if let Some(ref init_expr) = field.default_init {
                                let tmp_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                                self.eval_value(init_expr, tmp_env).unwrap_or(Value::Null)
                            } else {
                                Value::Null
                            };
                            field_map.insert(field.name.name.clone(), default_val);
                        }
                        // Then: override with positional constructor args (if provided)
                        for (i, field) in class_decl.fields.iter().enumerate() {
                            if let Some(val) = evaluated_args.get(i) {
                                field_map.insert(field.name.name.clone(), val.clone());
                            }
                        }

                        return Ok(Value::Struct {
                            name: class_decl.name.name.clone(),
                            fields: Rc::new(RefCell::new(field_map)),
                        });
                    }

                    // Check Struct constructor (e.g. Point(10, 20))
                    if let Some(struct_decl) = self.structs.get(&id.name).cloned() {
                        if let Some(new_method) = struct_decl.methods.iter().find(|m| m.name.name == "new").cloned() {
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            return Ok(self.execute_function(&new_method, evaluated_args, method_env)?);
                        }

                        let mut field_map = HashMap::new();
                        for (i, field) in struct_decl.fields.iter().enumerate() {
                            let val = evaluated_args.get(i).cloned().unwrap_or(Value::Null);
                            field_map.insert(field.name.name.clone(), val);
                        }

                        return Ok(Value::Struct {
                            name: struct_decl.name.name.clone(),
                            fields: Rc::new(RefCell::new(field_map)),
                        });
                    }

                    return Err(RuntimeError::new(
                        format!("Undefined function or class '{}'", id.name),
                        Some(id.span),
                    ).into());
                }

                Err(RuntimeError::new("Unsupported callee expression", Some(*span)).into())
            }

            Expr::MethodCall(obj_expr, method_id, args, span) => {
                // Check if obj_expr is a Class or Struct name (static method call, e.g. User.new(...) or Math.abs(...))
                if let Expr::Ident(ref class_id) = **obj_expr {
                    if let Some(class_decl) = self.classes.get(&class_id.name).cloned() {
                        if let Some(method_decl) = class_decl.methods.iter().find(|m| m.name.name == method_id.name).cloned() {
                            let mut evaluated_args = Vec::new();
                            for arg in args {
                                evaluated_args.push(self.eval_value(arg, Rc::clone(&env))?);
                            }
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            return Ok(self.execute_function(&method_decl, evaluated_args, method_env)?);
                        }
                    }
                    if let Some(struct_decl) = self.structs.get(&class_id.name).cloned() {
                        if let Some(method_decl) = struct_decl.methods.iter().find(|m| m.name.name == method_id.name).cloned() {
                            let mut evaluated_args = Vec::new();
                            for arg in args {
                                evaluated_args.push(self.eval_value(arg, Rc::clone(&env))?);
                            }
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            return Ok(self.execute_function(&method_decl, evaluated_args, method_env)?);
                        }
                    }
                }

                let obj_val = self.eval_value(obj_expr, Rc::clone(&env))?;
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_value(arg, Rc::clone(&env))?);
                }

                // Built-in methods
                if method_id.name == "len" {
                    match &obj_val {
                        Value::Array(a) => return Ok(Value::Int(a.borrow().len() as i64)),
                        Value::String(s) => return Ok(Value::Int(s.len() as i64)),
                        _ => {}
                    }
                }

                if method_id.name == "push" {
                    if let Value::Array(a) = &obj_val {
                        for item in evaluated_args {
                            a.borrow_mut().push(item);
                        }
                        return Ok(Value::Null);
                    }
                }

                if method_id.name == "pop" {
                    if let Value::Array(a) = &obj_val {
                        return Ok(a.borrow_mut().pop().unwrap_or(Value::Null));
                    }
                }

                // Check class or struct instance methods (including inheritance)
                if let Value::Struct { ref name, ref fields } = obj_val {
                    let mut found_method = None;
                    let mut curr_class_name = Some(name.clone());
                    while let Some(cname) = curr_class_name {
                        if let Some(c) = self.classes.get(&cname) {
                            for m in &c.methods {
                                if m.name.name == method_id.name {
                                    found_method = Some(m.clone());
                                    break;
                                }
                            }
                            if found_method.is_some() {
                                break;
                            }
                            curr_class_name = c.extends.as_ref().map(|id| id.name.clone());
                        } else {
                            break;
                        }
                    }

                    if found_method.is_none() {
                        if let Some(s) = self.structs.get(name) {
                            for m in &s.methods {
                                if m.name.name == method_id.name {
                                    found_method = Some(m.clone());
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(method_decl) = found_method {
                        let method_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                        for (k, v) in fields.borrow().iter() {
                            method_env.borrow_mut().define(k, v.clone(), true);
                        }
                        // Define 'this' and 'self'
                        method_env.borrow_mut().define("this", obj_val.clone(), true);
                        method_env.borrow_mut().define("self", obj_val.clone(), true);

                        let res = self.execute_function(&method_decl, evaluated_args, Rc::clone(&method_env))?;

                        // Sync any updated field variables back to struct instance
                        let keys: Vec<String> = fields.borrow().keys().cloned().collect();
                        for k in keys {
                            if let Some(new_val) = method_env.borrow().get(&k) {
                                fields.borrow_mut().insert(k, new_val);
                            }
                        }
                        if let Some(Value::Struct { fields: updated_fields, .. }) = method_env.borrow().get("this") {
                            if !Rc::ptr_eq(fields, &updated_fields) {
                                let updates: Vec<(String, Value)> = updated_fields.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                                for (k, v) in updates {
                                    fields.borrow_mut().insert(k, v);
                                }
                            }
                        }

                        return Ok(res);
                    }
                }

                Err(RuntimeError::new(
                    format!("Method '{}' not found on {}", method_id.name, obj_val.type_name()),
                    Some(*span),
                ).into())
            }

            Expr::FieldAccess(target_expr, field_id, span) => {
                let target_val = self.eval_value(target_expr, env)?;
                if let Value::Struct { ref fields, .. } = target_val {
                    if let Some(val) = fields.borrow().get(&field_id.name) {
                        return Ok(val.clone());
                    }
                    return Err(RuntimeError::new(
                        format!("Field '{}' not found on struct", field_id.name),
                        Some(field_id.span),
                    ).into());
                }

                Err(RuntimeError::new(
                    format!("Cannot access field on type {}", target_val.type_name()),
                    Some(*span),
                ).into())
            }

            Expr::Index(target_expr, idx_expr, span) => {
                let target_val = self.eval_value(target_expr, Rc::clone(&env))?;
                let idx_val = self.eval_value(idx_expr, env)?;

                match (&target_val, &idx_val) {
                    (Value::Array(arr), Value::Int(i)) => {
                        let borrowed = arr.borrow();
                        if *i < 0 || *i as usize >= borrowed.len() {
                            return Err(RuntimeError::new(
                                format!("Index {} out of bounds for array of length {}", i, borrowed.len()),
                                Some(*span),
                            ).into());
                        }
                        Ok(borrowed[*i as usize].clone())
                    }
                    (Value::String(s), Value::Int(i)) => {
                        if *i < 0 || *i as usize >= s.len() {
                            return Err(RuntimeError::new(
                                format!("Index {} out of bounds for string of length {}", i, s.len()),
                                Some(*span),
                            ).into());
                        }
                        let ch = s.chars().nth(*i as usize).unwrap_or('\0');
                        Ok(Value::Char(ch))
                    }
                    _ => Err(RuntimeError::new(
                        format!("Cannot index {} with {}", target_val.type_name(), idx_val.type_name()),
                        Some(*span),
                    ).into()),
                }
            }

            Expr::Block(block) => {
                let block_env = Rc::new(RefCell::new(Environment::with_parent(env)));
                self.execute_block(block, block_env)
            }

            Expr::If(cond_expr, then_block, else_expr, _) => {
                let cond_val = self.eval_value(cond_expr, Rc::clone(&env))?;
                if cond_val.is_truthy() {
                    let then_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                    self.execute_block(then_block, then_env)
                } else if let Some(ref else_box) = else_expr {
                    self.eval_value(else_box, env)
                } else {
                    Ok(Value::Null)
                }
            }

            Expr::While(cond_expr, body, _) => {
                let mut guard = 0;
                while self.eval_value(cond_expr, Rc::clone(&env))?.is_truthy() {
                    guard += 1;
                    if guard > 10_000_000 {
                        return Err(RuntimeError::new("Infinite loop detected (> 10M iterations)", Some(body.span)).into());
                    }
                    let loop_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                    match self.execute_block(body, loop_env) {
                        Ok(_) | Err(FlowSignal::Continue) => continue,
                        Err(FlowSignal::Break(val)) => return Ok(val.unwrap_or(Value::Null)),
                        Err(other) => return Err(other),
                    }
                }
                Ok(Value::Null)
            }

            Expr::For(ident, iter_expr, body, span) => {
                let iter_val = self.eval_value(iter_expr, Rc::clone(&env))?;
                match iter_val {
                    Value::Array(items) => {
                        let elements = items.borrow().clone();
                        for elem in elements {
                            let loop_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            loop_env.borrow_mut().define(&ident.name, elem, false);
                            match self.execute_block(body, loop_env) {
                                Ok(_) | Err(FlowSignal::Continue) => continue,
                                Err(FlowSignal::Break(val)) => return Ok(val.unwrap_or(Value::Null)),
                                Err(other) => return Err(other),
                            }
                        }
                        Ok(Value::Null)
                    }
                    Value::Int(count) => {
                        for i in 0..count {
                            let loop_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                            loop_env.borrow_mut().define(&ident.name, Value::Int(i), false);
                            match self.execute_block(body, loop_env) {
                                Ok(_) | Err(FlowSignal::Continue) => continue,
                                Err(FlowSignal::Break(val)) => return Ok(val.unwrap_or(Value::Null)),
                                Err(other) => return Err(other),
                            }
                        }
                        Ok(Value::Null)
                    }
                    _ => Err(RuntimeError::new(
                        format!("Type {} is not iterable in 'for' loop", iter_val.type_name()),
                        Some(*span),
                    ).into()),
                }
            }

            Expr::Loop(body, _) => {
                let mut guard = 0;
                loop {
                    guard += 1;
                    if guard > 10_000_000 {
                        return Err(RuntimeError::new("Infinite loop detected (> 10M iterations)", Some(body.span)).into());
                    }
                    let loop_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&env))));
                    match self.execute_block(body, loop_env) {
                        Ok(_) | Err(FlowSignal::Continue) => continue,
                        Err(FlowSignal::Break(val)) => return Ok(val.unwrap_or(Value::Null)),
                        Err(other) => return Err(other),
                    }
                }
            }

            Expr::Match(target_expr, arms, span) => {
                let target_val = self.eval_value(target_expr, Rc::clone(&env))?;

                for arm in arms {
                    let mut arm_env = Environment::with_parent(Rc::clone(&env));
                    if self.pattern_matches(&arm.pattern, &target_val, &mut arm_env) {
                        return self.eval_value(&arm.body, Rc::new(RefCell::new(arm_env)));
                    }
                }

                Err(RuntimeError::new(
                    format!("No matching arm for value: {}", target_val),
                    Some(*span),
                ).into())
            }

            Expr::Try(inner_expr, _) => {
                let val = self.eval_value(inner_expr, env)?;
                if let Value::Variant { ref variant_name, ref payload, .. } = val {
                    if variant_name == "Err" {
                        // Return early with the Err variant
                        return Err(FlowSignal::Return(val));
                    }
                    if variant_name == "Ok" {
                        return Ok(payload.first().cloned().unwrap_or(Value::Null));
                    }
                }
                Ok(val)
            }

            Expr::ArrayLit(elements, _) => {
                let mut vals = Vec::new();
                for elem in elements {
                    vals.push(self.eval_value(elem, Rc::clone(&env))?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }

            Expr::StructLit(name_ident, field_inits, _) => {
                let mut field_map = HashMap::new();
                for (f_name, f_expr) in field_inits {
                    let f_val = self.eval_value(f_expr, Rc::clone(&env))?;
                    field_map.insert(f_name.name.clone(), f_val);
                }

                Ok(Value::Struct {
                    name: name_ident.name.clone(),
                    fields: Rc::new(RefCell::new(field_map)),
                })
            }
        }
    }

    fn pattern_matches(
        &self,
        pattern: &Pattern,
        val: &Value,
        env: &mut Environment,
    ) -> bool {
        match pattern {
            Pattern::Wildcard(_) => true,
            Pattern::Ident(id) => {
                // If it's a known enum variant name with 0 payloads (e.g. Pending)
                if self.is_variant_name(&id.name) {
                    if let Value::Variant { ref variant_name, payload, .. } = val {
                        return payload.is_empty() && variant_name == &id.name;
                    }
                    return false;
                }
                // Otherwise bind variable
                env.define(&id.name, val.clone(), false);
                true
            }
            Pattern::Literal(lit, _) => match (lit, val) {
                (Literal::Int(i), Value::Int(v)) => i == v,
                (Literal::Float(f), Value::Float(v)) => (f - v).abs() < f64::EPSILON,
                (Literal::String(s), Value::String(v)) => s == v,
                (Literal::Bool(b), Value::Bool(v)) => b == v,
                (Literal::Char(c), Value::Char(v)) => c == v,
                (Literal::Null, Value::Null) => true,
                _ => false,
            },
            Pattern::Variant(var_ident, sub_patterns, _) => {
                if let Value::Variant { ref variant_name, ref payload, .. } = val {
                    if variant_name != &var_ident.name {
                        return false;
                    }
                    if sub_patterns.len() != payload.len() {
                        return false;
                    }
                    for (sub_pat, sub_val) in sub_patterns.iter().zip(payload.iter()) {
                        if !self.pattern_matches(sub_pat, sub_val, env) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn eval_binary_op(
        &self,
        left: &Value,
        op: BinaryOp,
        right: &Value,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        match op {
            BinaryOp::Add => {
                // String concatenation
                if let Value::String(ref l) = left {
                    return Ok(Value::String(format!("{}{}", l, right.to_display_string())));
                }
                if let Value::String(ref r) = right {
                    return Ok(Value::String(format!("{}{}", left.to_display_string(), r)));
                }

                // Numeric addition
                match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                    (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
                    (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 + r)),
                    (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l + *r as f64)),
                    (Value::Array(l), Value::Array(r)) => {
                        let mut combined = l.borrow().clone();
                        combined.extend(r.borrow().clone());
                        Ok(Value::Array(Rc::new(RefCell::new(combined))))
                    }
                    _ => Err(RuntimeError::new(
                        format!("Operator '+' not supported between {} and {}", left.type_name(), right.type_name()),
                        Some(span),
                    )),
                }
            }

            BinaryOp::Sub => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 - r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l - *r as f64)),
                _ => Err(RuntimeError::new(
                    format!("Operator '-' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Mul => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Float(*l as f64 * r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l * *r as f64)),
                (Value::String(s), Value::Int(n)) => {
                    let repeat = if *n > 0 { *n as usize } else { 0 };
                    Ok(Value::String(s.repeat(repeat)))
                }
                _ => Err(RuntimeError::new(
                    format!("Operator '*' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Div => match (left, right) {
                (Value::Int(l), Value::Int(r)) => {
                    if *r == 0 {
                        return Err(RuntimeError::new("Division by zero", Some(span)));
                    }
                    Ok(Value::Int(l / r))
                }
                (Value::Float(l), Value::Float(r)) => {
                    if *r == 0.0 {
                        return Err(RuntimeError::new("Division by zero", Some(span)));
                    }
                    Ok(Value::Float(l / r))
                }
                (Value::Int(l), Value::Float(r)) => {
                    if *r == 0.0 {
                        return Err(RuntimeError::new("Division by zero", Some(span)));
                    }
                    Ok(Value::Float(*l as f64 / r))
                }
                (Value::Float(l), Value::Int(r)) => {
                    if *r == 0 {
                        return Err(RuntimeError::new("Division by zero", Some(span)));
                    }
                    Ok(Value::Float(l / *r as f64))
                }
                _ => Err(RuntimeError::new(
                    format!("Operator '/' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Mod => match (left, right) {
                (Value::Int(l), Value::Int(r)) => {
                    if *r == 0 {
                        return Err(RuntimeError::new("Modulo by zero", Some(span)));
                    }
                    Ok(Value::Int(l % r))
                }
                (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l % r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '%' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Eq => Ok(Value::Bool(left == right)),
            BinaryOp::NotEq => Ok(Value::Bool(left != right)),

            BinaryOp::Lt => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l < r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Bool((*l as f64) < *r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Bool(*l < (*r as f64))),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l < r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '<' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::LtEq => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l <= r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l <= r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Bool((*l as f64) <= *r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Bool(*l <= (*r as f64))),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l <= r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '<=' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Gt => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l > r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l > r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Bool((*l as f64) > *r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Bool(*l > (*r as f64))),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l > r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '>' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::GtEq => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l >= r)),
                (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l >= r)),
                (Value::Int(l), Value::Float(r)) => Ok(Value::Bool((*l as f64) >= *r)),
                (Value::Float(l), Value::Int(r)) => Ok(Value::Bool(*l >= (*r as f64))),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l >= r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '>=' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::BitAnd => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l & r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '&' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::BitOr => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l | r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '|' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::BitXor => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l ^ r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '^' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Shl => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l << r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '<<' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::Shr => match (left, right) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l >> r)),
                _ => Err(RuntimeError::new(
                    format!("Operator '>>' not supported between {} and {}", left.type_name(), right.type_name()),
                    Some(span),
                )),
            },

            BinaryOp::And | BinaryOp::Or => unreachable!("Logical ops handled via short-circuit"),
        }
    }
}
