use prady_diagnostics::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(FunctionDecl),
    Struct(StructDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Trait(TraitDecl),
    Enum(EnumDecl),
    Architecture(ArchitectureDecl),
    Import(ImportDecl),
    Module(ModuleDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub visibility: Visibility,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSig {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub is_mut: bool,
    pub name: Ident,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub name: Ident,
    pub bounds: Vec<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<FunctionDecl>,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub extends: Option<Ident>,
    pub implements: Vec<Ident>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<FunctionDecl>,
    pub is_abstract: bool,
    pub visibility: Visibility,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub methods: Vec<FunctionSig>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub methods: Vec<FunctionSig>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: Ident,
    pub type_params: Vec<TypeParam>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub payload: Option<Vec<Type>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArchitectureDecl {
    pub name: Ident,
    pub layers: Vec<Ident>,
    pub rules: Vec<ArchRule>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArchRule {
    LayerFlow {
        from: Ident,
        to: Ident,
        span: Span,
    },
    LayerDenyImport {
        from: Ident,
        denied: Ident,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub path: Vec<Ident>,
    pub alias: Option<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub name: Ident,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub visibility: Visibility,
    pub is_const: bool,
    pub name: Ident,
    pub ty: Type,
    pub default_init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    Expr(Expr),
    Return(Option<Expr>, Span),
    Break(Option<Expr>, Span),
    Continue(Span),
    Using(Ident, Expr, Block, Span),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let(l) => l.span,
            Stmt::Assign(a) => a.span,
            Stmt::Expr(e) => e.span(),
            Stmt::Return(_, s)
            | Stmt::Break(_, s)
            | Stmt::Continue(s)
            | Stmt::Using(_, _, _, s) => *s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub is_const: bool,
    pub is_mut: bool,
    pub name: Ident,
    pub ty: Option<Type>,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt {
    pub target: Expr,
    pub op: AssignOp,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal, Span),
    Ident(Ident),
    Binary(Box<Expr>, BinaryOp, Box<Expr>, Span),
    Unary(UnaryOp, Box<Expr>, Span),
    Call(Box<Expr>, Vec<Expr>, Span),
    MethodCall(Box<Expr>, Ident, Vec<Expr>, Span),
    FieldAccess(Box<Expr>, Ident, Span),
    Index(Box<Expr>, Box<Expr>, Span),
    Block(Block),
    If(Box<Expr>, Block, Option<Box<Expr>>, Span),
    While(Box<Expr>, Block, Span),
    For(Ident, Box<Expr>, Block, Span),
    Loop(Block, Span),
    Match(Box<Expr>, Vec<MatchArm>, Span),
    Try(Box<Expr>, Span), // expr?
    ArrayLit(Vec<Expr>, Span),
    StructLit(Ident, Vec<(Ident, Expr)>, Span),
    Switch(Box<Expr>, Vec<SwitchCase>, Option<Block>, Span),
    Lambda(Vec<Param>, Option<Type>, Block, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s)
            | Expr::Binary(_, _, _, s)
            | Expr::Unary(_, _, s)
            | Expr::Call(_, _, s)
            | Expr::MethodCall(_, _, _, s)
            | Expr::FieldAccess(_, _, s)
            | Expr::Index(_, _, s)
            | Expr::If(_, _, _, s)
            | Expr::While(_, _, s)
            | Expr::For(_, _, _, s)
            | Expr::Loop(_, s)
            | Expr::Match(_, _, s)
            | Expr::Try(_, s)
            | Expr::ArrayLit(_, s)
            | Expr::StructLit(_, _, s)
            | Expr::Switch(_, _, _, s)
            | Expr::Lambda(_, _, _, s) => *s,
            Expr::Ident(id) => id.span,
            Expr::Block(b) => b.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub value: Expr,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard(Span),
    Literal(Literal, Span),
    Ident(Ident),
    Variant(Ident, Vec<Pattern>, Span),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(Ident, Vec<Type>, Span),
    Array(Box<Type>, usize, Span),
    Tuple(Vec<Type>, Span),
    Function(Vec<Type>, Box<Type>, Span),
    Unit(Span),
}

impl Type {
    pub fn span(&self) -> Span {
        match self {
            Type::Named(_, _, s)
            | Type::Array(_, _, s)
            | Type::Tuple(_, s)
            | Type::Function(_, _, s)
            | Type::Unit(s) => *s,
        }
    }
}

/// AST Formatter / Visualizer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print_program(prog: &Program) -> String {
        let mut out = String::new();
        for item in &prog.items {
            Self::print_item(item, 0, &mut out);
            out.push('\n');
        }
        out
    }

    fn indent(level: usize) -> String {
        "  ".repeat(level)
    }

    fn print_item(item: &Item, level: usize, out: &mut String) {
        let ind = Self::indent(level);
        match item {
            Item::Function(f) => {
                let async_str = if f.is_async { "async " } else { "" };
                out.push_str(&format!("{}fn {}{}(", ind, async_str, f.name));
                for (i, p) in f.params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let mut_str = if p.is_mut { "mut " } else { "" };
                    out.push_str(&format!("{}{}: {:?}", mut_str, p.name, p.ty));
                }
                out.push(')');
                if let Some(ref ret) = f.return_type {
                    out.push_str(&format!(" -> {:?}", ret));
                }
                out.push_str(" {\n");
                for s in &f.body.stmts {
                    Self::print_stmt(s, level + 1, out);
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Struct(s) => {
                out.push_str(&format!("{}struct {} {{\n", ind, s.name));
                for field in &s.fields {
                    out.push_str(&format!("{}  {}: {:?},\n", ind, field.name, field.ty));
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Class(c) => {
                let abs_str = if c.is_abstract { "abstract " } else { "" };
                let ext_str = c
                    .extends
                    .as_ref()
                    .map(|e| format!(" extends {}", e))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "{}{}{}class {}{} {{\n",
                    ind, abs_str, ext_str, c.name, ""
                ));
                for field in &c.fields {
                    out.push_str(&format!("{}  {}: {:?};\n", ind, field.name, field.ty));
                }
                for method in &c.methods {
                    Self::print_item(&Item::Function(method.clone()), level + 1, out);
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Architecture(a) => {
                out.push_str(&format!("{}architecture {} {{\n", ind, a.name));
                for layer in &a.layers {
                    out.push_str(&format!("{}  layer {};\n", ind, layer));
                }
                for rule in &a.rules {
                    match rule {
                        ArchRule::LayerFlow { from, to, .. } => {
                            out.push_str(&format!("{}  {} -> {};\n", ind, from, to));
                        }
                        ArchRule::LayerDenyImport { from, denied, .. } => {
                            out.push_str(&format!("{}  {} cannot import {};\n", ind, from, denied));
                        }
                    }
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Import(imp) => {
                let parts: Vec<String> = imp.path.iter().map(|p| p.name.clone()).collect();
                out.push_str(&format!("{}import {};\n", ind, parts.join(".")));
            }
            Item::Interface(i) => {
                out.push_str(&format!("{}interface {} {{\n", ind, i.name));
                for m in &i.methods {
                    out.push_str(&format!("{}  fn {}(...);\n", ind, m.name));
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Trait(t) => {
                out.push_str(&format!("{}trait {} {{\n", ind, t.name));
                for m in &t.methods {
                    out.push_str(&format!("{}  fn {}(...);\n", ind, m.name));
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Enum(e) => {
                out.push_str(&format!("{}enum {} {{\n", ind, e.name));
                for v in &e.variants {
                    out.push_str(&format!("{}  {},\n", ind, v.name));
                }
                out.push_str(&format!("{}}}\n", ind));
            }
            Item::Module(m) => {
                out.push_str(&format!("{}module {};\n", ind, m.name));
            }
        }
    }

    fn print_stmt(stmt: &Stmt, level: usize, out: &mut String) {
        let ind = Self::indent(level);
        match stmt {
            Stmt::Let(l) => {
                let mut_str = if l.is_mut { "mut " } else { "" };
                let ty_str =
                    l.ty.as_ref()
                        .map(|t| format!(": {:?}", t))
                        .unwrap_or_default();
                let init_str = l
                    .init
                    .as_ref()
                    .map(|e| format!(" = {:?}", e))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "{}let {}{}{}{};\n",
                    ind, mut_str, l.name, ty_str, init_str
                ));
            }
            Stmt::Assign(a) => {
                out.push_str(&format!("{}{:?} = {:?};\n", ind, a.target, a.value));
            }
            Stmt::Expr(e) => {
                out.push_str(&format!("{}{:?};\n", ind, e));
            }
            Stmt::Return(e, _) => {
                if let Some(ref val) = e {
                    out.push_str(&format!("{}return {:?};\n", ind, val));
                } else {
                    out.push_str(&format!("{}return;\n", ind));
                }
            }
            Stmt::Break(e, _) => {
                if let Some(ref val) = e {
                    out.push_str(&format!("{}break {:?};\n", ind, val));
                } else {
                    out.push_str(&format!("{}break;\n", ind));
                }
            }
            Stmt::Continue(_) => {
                out.push_str(&format!("{}continue;\n", ind));
            }
            Stmt::Using(name, init, body, _) => {
                out.push_str(&format!("{}using {} = {:?} {{\n", ind, name, init));
                for s in &body.stmts {
                    Self::print_stmt(s, level + 1, out);
                }
                out.push_str(&format!("{}}}\n", ind));
            }
        }
    }
}
