use prady_ast::*;
use prady_diagnostics::{Diagnostic, DiagnosticBag, Span};
use prady_lexer::{Token, TokenKind};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    None,
    Or,         // ||
    And,        // &&
    BitOr,      // |
    BitXor,     // ^
    BitAnd,     // &
    Equality,   // == !=
    Comparison, // < <= > >=
    Shift,      // << >>
    Term,       // + -
    Factor,     // * / %
    Unary,      // ! - ~
    Call,       // . () [] ?
}

impl Precedence {
    fn of(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::OrOr => Precedence::Or,
            TokenKind::AndAnd => Precedence::And,
            TokenKind::Pipe => Precedence::BitOr,
            TokenKind::Caret => Precedence::BitXor,
            TokenKind::Ampersand => Precedence::BitAnd,
            TokenKind::EqEq | TokenKind::NotEq => Precedence::Equality,
            TokenKind::Lt | TokenKind::LtEq | TokenKind::Gt | TokenKind::GtEq => {
                Precedence::Comparison
            }
            TokenKind::Shl | TokenKind::Shr => Precedence::Shift,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Precedence::Factor,
            TokenKind::Dot
            | TokenKind::OpenParen
            | TokenKind::OpenBracket
            | TokenKind::Question => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        self.peek_kind() == &TokenKind::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek_kind()) == std::mem::discriminant(kind)
        }
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind, message: &str) -> Option<&Token> {
        if self.check(&kind) {
            Some(self.advance())
        } else {
            let token = self.peek();
            self.diagnostics.add(
                Diagnostic::error(format!("{}, found '{}'", message, token.text), token.span)
                    .with_code("E0001")
                    .with_label(message),
            );
            None
        }
    }

    fn expect_ident(&mut self, message: &str) -> Option<Ident> {
        let token = self.peek().clone();
        if let TokenKind::Ident(ref name) = token.kind {
            self.advance();
            Some(Ident::new(name.clone(), token.span))
        } else {
            self.diagnostics.add(
                Diagnostic::error(format!("{}, found '{}'", message, token.text), token.span)
                    .with_code("E0004")
                    .with_label(message),
            );
            None
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }
            match self.peek_kind() {
                TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Const
                | TokenKind::Struct
                | TokenKind::Class
                | TokenKind::Interface
                | TokenKind::Trait
                | TokenKind::Enum
                | TokenKind::Architecture
                | TokenKind::Import
                | TokenKind::Module
                | TokenKind::Return
                | TokenKind::If
                | TokenKind::While
                | TokenKind::For => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let start_span = self.peek().span;
        let mut items = Vec::new();

        while !self.is_at_end() {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.synchronize();
            }
        }

        let end_span = self.previous().span;
        Program {
            items,
            span: start_span.merge(end_span),
        }
    }

    fn parse_item(&mut self) -> Option<Item> {
        let is_async = self.match_token(&TokenKind::Async);

        match self.peek_kind() {
            TokenKind::Fn => self.parse_function(is_async).map(Item::Function),
            TokenKind::Struct => self.parse_struct().map(Item::Struct),
            TokenKind::Class | TokenKind::Abstract => self.parse_class().map(Item::Class),
            TokenKind::Interface => self.parse_interface().map(Item::Interface),
            TokenKind::Trait => self.parse_trait().map(Item::Trait),
            TokenKind::Enum => self.parse_enum().map(Item::Enum),
            TokenKind::Architecture => self.parse_architecture().map(Item::Architecture),
            TokenKind::Import => self.parse_import().map(Item::Import),
            TokenKind::Module => self.parse_module().map(Item::Module),
            _ => {
                let token = self.peek();
                self.diagnostics.add(
                    Diagnostic::error(
                        format!("Expected top-level declaration, found '{}'", token.text),
                        token.span,
                    )
                    .with_code("E0010")
                    .with_suggestion("Declare a function with 'fn', a struct with 'struct', or an architecture block with 'architecture'"),
                );
                None
            }
        }
    }

    fn parse_function(&mut self, is_async: bool) -> Option<FunctionDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Fn, "Expected 'fn'")?;
        let name = self.expect_ident("Expected function name")?;

        let type_params = self.parse_optional_type_params();

        self.expect(TokenKind::OpenParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::CloseParen) {
            loop {
                let is_mut = self.match_token(&TokenKind::Mut);
                let p_name = self.expect_ident("Expected parameter name")?;
                self.expect(TokenKind::Colon, "Expected ':' after parameter name")?;
                let p_ty = self.parse_type()?;
                let span = p_name.span.merge(p_ty.span());
                params.push(Param {
                    is_mut,
                    name: p_name,
                    ty: p_ty,
                    span,
                });
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::CloseParen, "Expected ')' after parameters")?;

        let return_type = if self.match_token(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        let span = start_span.merge(body.span);

        Some(FunctionDecl {
            name,
            type_params,
            params,
            return_type,
            body,
            visibility: Visibility::Public,
            is_async,
            span,
        })
    }

    fn parse_optional_type_params(&mut self) -> Vec<TypeParam> {
        let mut params = Vec::new();
        if self.match_token(&TokenKind::Lt) {
            while !self.check(&TokenKind::Gt) && !self.is_at_end() {
                if let Some(ident) = self.expect_ident("Expected type parameter name") {
                    let mut bounds = Vec::new();
                    if self.match_token(&TokenKind::Colon) {
                        if let Some(bound) = self.expect_ident("Expected type constraint") {
                            bounds.push(bound);
                        }
                    }
                    let span = ident.span;
                    params.push(TypeParam {
                        name: ident,
                        bounds,
                        span,
                    });
                }
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Gt, "Expected '>' after type parameters");
        }
        params
    }

    fn parse_struct(&mut self) -> Option<StructDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Struct, "Expected 'struct'")?;
        let name = self.expect_ident("Expected struct name")?;
        let type_params = self.parse_optional_type_params();

        self.expect(TokenKind::OpenBrace, "Expected '{' before struct body")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Fn) {
                if let Some(method) = self.parse_function(false) {
                    methods.push(method);
                }
            } else {
                let f_name = self.expect_ident("Expected field name")?;
                self.expect(TokenKind::Colon, "Expected ':' after field name")?;
                let f_ty = self.parse_type()?;
                let span = f_name.span.merge(f_ty.span());
                fields.push(FieldDecl {
                    visibility: Visibility::Public,
                    is_const: false,
                    name: f_name,
                    ty: f_ty,
                    default_init: None,
                    span,
                });
                self.match_token(&TokenKind::Comma);
                self.match_token(&TokenKind::Semicolon);
            }
        }
        let end_token = self.expect(TokenKind::CloseBrace, "Expected '}' after struct body")?;
        let span = start_span.merge(end_token.span);

        Some(StructDecl {
            name,
            type_params,
            fields,
            methods,
            visibility: Visibility::Public,
            span,
        })
    }

    fn parse_class(&mut self) -> Option<ClassDecl> {
        let start_span = self.peek().span;
        let is_abstract = self.match_token(&TokenKind::Abstract);
        self.expect(TokenKind::Class, "Expected 'class'")?;
        let name = self.expect_ident("Expected class name")?;
        let type_params = self.parse_optional_type_params();

        let extends = if self.match_token(&TokenKind::Extends) {
            self.expect_ident("Expected base class name")
        } else {
            None
        };

        let mut implements = Vec::new();
        if self.match_token(&TokenKind::Implements) {
            loop {
                if let Some(iface) = self.expect_ident("Expected interface name") {
                    implements.push(iface);
                }
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenKind::OpenBrace, "Expected '{' before class body")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Fn) || self.check(&TokenKind::Async) {
                let is_async = self.match_token(&TokenKind::Async);
                if let Some(m) = self.parse_function(is_async) {
                    methods.push(m);
                }
            } else if self.check(&TokenKind::Let) || self.check(&TokenKind::Const) {
                // Support: let a: Int = 3;  or  const PI: Float = 3.14;
                let is_const = self.check(&TokenKind::Const);
                self.advance(); // consume let/const
                let is_mut = !is_const && self.match_token(&TokenKind::Mut);
                let f_name = match self.expect_ident("Expected field name after 'let'/'const'") {
                    Some(n) => n,
                    None => { self.synchronize(); continue; }
                };
                // Require type annotation
                let f_ty = if self.match_token(&TokenKind::Colon) {
                    match self.parse_type() {
                        Some(t) => t,
                        None => { self.synchronize(); continue; }
                    }
                } else {
                    // No type annotation provided — emit error and skip
                    let tok = self.peek();
                    self.diagnostics.error(
                        format!("Expected ':' and type after field '{}'", f_name.name),
                        tok.span,
                    );
                    self.synchronize();
                    continue;
                };
                // Optional default initializer
                let default_init = if self.match_token(&TokenKind::Assign) {
                    match self.parse_expr() {
                        Some(e) => Some(e),
                        None => { self.synchronize(); continue; }
                    }
                } else {
                    None
                };
                self.match_token(&TokenKind::Semicolon);
                let span = f_name.span.merge(f_ty.span());
                let _ = is_mut; // stored in is_const flag
                fields.push(FieldDecl {
                    visibility: Visibility::Public,
                    is_const,
                    name: f_name,
                    ty: f_ty,
                    default_init,
                    span,
                });
            } else {
                let f_name = self.expect_ident("Expected class member")?;
                self.expect(TokenKind::Colon, "Expected ':' after field name")?;
                let f_ty = self.parse_type()?;
                let span = f_name.span.merge(f_ty.span());
                fields.push(FieldDecl {
                    visibility: Visibility::Public,
                    is_const: false,
                    name: f_name,
                    ty: f_ty,
                    default_init: None,
                    span,
                });
                self.match_token(&TokenKind::Semicolon);
            }
        }
        let end_token = self.expect(TokenKind::CloseBrace, "Expected '}' after class body")?;
        let span = start_span.merge(end_token.span);

        Some(ClassDecl {
            name,
            type_params,
            extends,
            implements,
            fields,
            methods,
            is_abstract,
            visibility: Visibility::Public,
            span,
        })
    }

    fn parse_interface(&mut self) -> Option<InterfaceDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Interface, "Expected 'interface'")?;
        let name = self.expect_ident("Expected interface name")?;
        let type_params = self.parse_optional_type_params();

        self.expect(TokenKind::OpenBrace, "Expected '{' before interface body")?;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            let m_start = self.peek().span;
            let is_async = self.match_token(&TokenKind::Async);
            self.expect(TokenKind::Fn, "Expected 'fn' in interface")?;
            let m_name = self.expect_ident("Expected method name")?;
            let m_type_params = self.parse_optional_type_params();

            self.expect(TokenKind::OpenParen, "Expected '('")?;
            let mut params = Vec::new();
            if !self.check(&TokenKind::CloseParen) {
                loop {
                    let p_name = self.expect_ident("Expected parameter name")?;
                    self.expect(TokenKind::Colon, "Expected ':'")?;
                    let p_ty = self.parse_type()?;
                    let span = p_name.span.merge(p_ty.span());
                    params.push(Param {
                        is_mut: false,
                        name: p_name,
                        ty: p_ty,
                        span,
                    });
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.expect(TokenKind::CloseParen, "Expected ')'")?;

            let return_type = if self.match_token(&TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let end_tok =
                self.expect(TokenKind::Semicolon, "Expected ';' after interface method")?;

            methods.push(FunctionSig {
                name: m_name,
                type_params: m_type_params,
                params,
                return_type,
                is_async,
                span: m_start.merge(end_tok.span),
            });
        }
        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;

        Some(InterfaceDecl {
            name,
            type_params,
            methods,
            span: start_span.merge(end_tok.span),
        })
    }

    fn parse_trait(&mut self) -> Option<TraitDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Trait, "Expected 'trait'")?;
        let name = self.expect_ident("Expected trait name")?;
        let type_params = self.parse_optional_type_params();

        self.expect(TokenKind::OpenBrace, "Expected '{'")?;
        let mut methods = Vec::new();
        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            let m_start = self.peek().span;
            let is_async = self.match_token(&TokenKind::Async);
            self.expect(TokenKind::Fn, "Expected 'fn'")?;
            let m_name = self.expect_ident("Expected method name")?;
            let m_type_params = self.parse_optional_type_params();

            self.expect(TokenKind::OpenParen, "Expected '('")?;
            let mut params = Vec::new();
            if !self.check(&TokenKind::CloseParen) {
                loop {
                    let p_name = self.expect_ident("Expected parameter name")?;
                    self.expect(TokenKind::Colon, "Expected ':'")?;
                    let p_ty = self.parse_type()?;
                    let span = p_name.span.merge(p_ty.span());
                    params.push(Param {
                        is_mut: false,
                        name: p_name,
                        ty: p_ty,
                        span,
                    });
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.expect(TokenKind::CloseParen, "Expected ')'")?;
            let return_type = if self.match_token(&TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let end_tok = self.expect(TokenKind::Semicolon, "Expected ';'")?;
            methods.push(FunctionSig {
                name: m_name,
                type_params: m_type_params,
                params,
                return_type,
                is_async,
                span: m_start.merge(end_tok.span),
            });
        }
        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;

        Some(TraitDecl {
            name,
            type_params,
            methods,
            span: start_span.merge(end_tok.span),
        })
    }

    fn parse_enum(&mut self) -> Option<EnumDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Enum, "Expected 'enum'")?;
        let name = self.expect_ident("Expected enum name")?;
        let type_params = self.parse_optional_type_params();

        self.expect(TokenKind::OpenBrace, "Expected '{'")?;
        let mut variants = Vec::new();
        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            let v_name = self.expect_ident("Expected variant name")?;
            let payload = if self.match_token(&TokenKind::OpenParen) {
                let mut types = Vec::new();
                while !self.check(&TokenKind::CloseParen) && !self.is_at_end() {
                    types.push(self.parse_type()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(TokenKind::CloseParen, "Expected ')'")?;
                Some(types)
            } else {
                None
            };
            let span = v_name.span;
            variants.push(EnumVariant {
                name: v_name,
                payload,
                span,
            });
            self.match_token(&TokenKind::Comma);
        }
        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;

        Some(EnumDecl {
            name,
            type_params,
            variants,
            span: start_span.merge(end_tok.span),
        })
    }

    fn parse_architecture(&mut self) -> Option<ArchitectureDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Architecture, "Expected 'architecture'")?;
        let name = self.expect_ident("Expected architecture name")?;

        self.expect(
            TokenKind::OpenBrace,
            "Expected '{' before architecture block",
        )?;
        let mut layers = Vec::new();
        let mut rules = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Layer) {
                let l_name = self.expect_ident("Expected layer name")?;
                self.expect(TokenKind::Semicolon, "Expected ';' after layer")?;
                layers.push(l_name);
            } else if let Some(from) = self.expect_ident("Expected layer in architecture rule") {
                if self.match_token(&TokenKind::Arrow) {
                    let to = self.expect_ident("Expected destination layer")?;
                    let end_tok = self.expect(TokenKind::Semicolon, "Expected ';'")?;
                    let span = from.span.merge(end_tok.span);
                    rules.push(ArchRule::LayerFlow { from, to, span });
                } else if self.match_token(&TokenKind::Cannot) {
                    self.expect(TokenKind::Import, "Expected 'import' after 'cannot'")?;
                    let denied = self.expect_ident("Expected denied layer name")?;
                    let end_tok = self.expect(TokenKind::Semicolon, "Expected ';'")?;
                    let span = from.span.merge(end_tok.span);
                    rules.push(ArchRule::LayerDenyImport { from, denied, span });
                } else {
                    let tok = self.peek();
                    self.diagnostics.error(
                        format!("Unexpected token in architecture rule: '{}'", tok.text),
                        tok.span,
                    );
                    self.advance();
                }
            }
        }
        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;

        Some(ArchitectureDecl {
            name,
            layers,
            rules,
            span: start_span.merge(end_tok.span),
        })
    }

    fn parse_import(&mut self) -> Option<ImportDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Import, "Expected 'import'")?;
        let mut path = Vec::new();

        let first = self.expect_ident("Expected import path")?;
        path.push(first);

        while self.match_token(&TokenKind::Dot) {
            let segment = self.expect_ident("Expected identifier in import path")?;
            path.push(segment);
        }

        let alias = None;
        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';' after import")?;

        Some(ImportDecl {
            path,
            alias,
            span: start_span.merge(end_tok.span),
        })
    }

    fn parse_module(&mut self) -> Option<ModuleDecl> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Module, "Expected 'module'")?;
        let name = self.expect_ident("Expected module name")?;
        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';' after module name")?;
        Some(ModuleDecl {
            name,
            span: start_span.merge(end_tok.span),
        })
    }

    pub fn parse_block(&mut self) -> Option<Block> {
        let start_tok = self.expect(TokenKind::OpenBrace, "Expected '{'")?;
        let start_span = start_tok.span;
        let mut stmts = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.synchronize();
            }
        }

        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;
        let span = start_span.merge(end_tok.span);

        Some(Block { stmts, span })
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek_kind() {
            TokenKind::Let | TokenKind::Const => self.parse_let_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::Break => self.parse_break_stmt(),
            TokenKind::Continue => self.parse_continue_stmt(),
            TokenKind::Using => self.parse_using_stmt(),
            _ => self.parse_expr_or_assign_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let start_span = self.peek().span;
        let is_const = self.check(&TokenKind::Const);
        if is_const {
            self.advance(); // consume 'const'
        } else {
            self.expect(TokenKind::Let, "Expected 'let'")?;
        }
        let is_mut = !is_const && self.match_token(&TokenKind::Mut);
        let name = self.expect_ident("Expected variable name")?;

        let ty = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let init = if self.match_token(&TokenKind::Assign) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';' after let/const statement")?;
        let span = start_span.merge(end_tok.span);

        Some(Stmt::Let(LetStmt {
            is_const,
            is_mut,
            name,
            ty,
            init,
            span,
        }))
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Return, "Expected 'return'")?;

        let value = if !self.check(&TokenKind::Semicolon) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';' after return")?;
        let span = start_span.merge(end_tok.span);
        Some(Stmt::Return(value, span))
    }

    fn parse_break_stmt(&mut self) -> Option<Stmt> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Break, "Expected 'break'")?;
        let value = if !self.check(&TokenKind::Semicolon) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';'")?;
        Some(Stmt::Break(value, start_span.merge(end_tok.span)))
    }

    fn parse_continue_stmt(&mut self) -> Option<Stmt> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Continue, "Expected 'continue'")?;
        let end_tok = self.expect(TokenKind::Semicolon, "Expected ';'")?;
        Some(Stmt::Continue(start_span.merge(end_tok.span)))
    }

    fn parse_using_stmt(&mut self) -> Option<Stmt> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Using, "Expected 'using'")?;
        let name = self.expect_ident("Expected variable name")?;
        self.expect(TokenKind::Assign, "Expected '='")?;
        let init = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start_span.merge(body.span);
        Some(Stmt::Using(name, init, body, span))
    }

    fn parse_expr_or_assign_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr()?;

        // Check for assignment operator
        let assign_op = match self.peek_kind() {
            TokenKind::Assign => Some(AssignOp::Assign),
            TokenKind::PlusAssign => Some(AssignOp::AddAssign),
            TokenKind::MinusAssign => Some(AssignOp::SubAssign),
            TokenKind::StarAssign => Some(AssignOp::MulAssign),
            TokenKind::SlashAssign => Some(AssignOp::DivAssign),
            TokenKind::PercentAssign => Some(AssignOp::ModAssign),
            _ => None,
        };

        if let Some(op) = assign_op {
            self.advance();
            let value = self.parse_expr()?;
            let end_tok = self.expect(TokenKind::Semicolon, "Expected ';' after assignment")?;
            let span = expr.span().merge(end_tok.span);
            Some(Stmt::Assign(AssignStmt {
                target: expr,
                op,
                value,
                span,
            }))
        } else {
            // Expression statement
            // Statements like if/while/for/loop don't require trailing semicolon
            match &expr {
                Expr::If(..) | Expr::While(..) | Expr::For(..) | Expr::Loop(..) => {
                    self.match_token(&TokenKind::Semicolon);
                    Some(Stmt::Expr(expr))
                }
                _ => {
                    self.expect(TokenKind::Semicolon, "Expected ';' after expression")?;
                    Some(Stmt::Expr(expr))
                }
            }
        }
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_precedence(Precedence::Or)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = self.parse_prefix()?;

        while precedence <= Precedence::of(self.peek_kind()) {
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expr> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::IntLiteral(val) => {
                self.advance();
                Some(Expr::Literal(Literal::Int(*val), token.span))
            }
            TokenKind::FloatLiteral(val) => {
                self.advance();
                Some(Expr::Literal(Literal::Float(*val), token.span))
            }
            TokenKind::StringLiteral(val) => {
                self.advance();
                Some(Expr::Literal(Literal::String(val.clone()), token.span))
            }
            TokenKind::CharLiteral(val) => {
                self.advance();
                Some(Expr::Literal(Literal::Char(*val), token.span))
            }
            TokenKind::True => {
                self.advance();
                Some(Expr::Literal(Literal::Bool(true), token.span))
            }
            TokenKind::False => {
                self.advance();
                Some(Expr::Literal(Literal::Bool(false), token.span))
            }
            TokenKind::Null => {
                self.advance();
                Some(Expr::Literal(Literal::Null, token.span))
            }
            TokenKind::Ident(name) => {
                self.advance();
                let ident = Ident::new(name.clone(), token.span);

                // Check for Struct Literal: `Name { field: val, ... }`
                if self.check(&TokenKind::OpenBrace) && self.looks_like_struct_lit() {
                    self.advance(); // consume '{'
                    let mut fields = Vec::new();
                    while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
                        let f_name = self.expect_ident("Expected field name in struct literal")?;
                        let f_val = if self.match_token(&TokenKind::Colon) {
                            self.parse_expr()?
                        } else {
                            // Shorthand: Name { gateway }
                            Expr::Ident(f_name.clone())
                        };
                        fields.push((f_name, f_val));
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    let end_tok =
                        self.expect(TokenKind::CloseBrace, "Expected '}' in struct literal")?;
                    let span = ident.span.merge(end_tok.span);
                    Some(Expr::StructLit(ident, fields, span))
                } else {
                    Some(Expr::Ident(ident))
                }
            }
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_precedence(Precedence::Unary)?;
                let span = token.span.merge(expr.span());
                Some(Expr::Unary(UnaryOp::Neg, Box::new(expr), span))
            }
            TokenKind::Bang => {
                self.advance();
                let expr = self.parse_precedence(Precedence::Unary)?;
                let span = token.span.merge(expr.span());
                Some(Expr::Unary(UnaryOp::Not, Box::new(expr), span))
            }
            TokenKind::Tilde => {
                self.advance();
                let expr = self.parse_precedence(Precedence::Unary)?;
                let span = token.span.merge(expr.span());
                Some(Expr::Unary(UnaryOp::BitNot, Box::new(expr), span))
            }
            TokenKind::OpenParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::CloseParen, "Expected ')' after expression")?;
                Some(expr)
            }
            TokenKind::OpenBracket => {
                // Array literal: [1, 2, 3]
                let start_span = token.span;
                self.advance();
                let mut elements = Vec::new();
                while !self.check(&TokenKind::CloseBracket) && !self.is_at_end() {
                    elements.push(self.parse_expr()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                let end_tok = self.expect(TokenKind::CloseBracket, "Expected ']'")?;
                let span = start_span.merge(end_tok.span);
                Some(Expr::ArrayLit(elements, span))
            }
            TokenKind::If => self.parse_if_expr(),
            TokenKind::While => self.parse_while_expr(),
            TokenKind::For => self.parse_for_expr(),
            TokenKind::Loop => self.parse_loop_expr(),
            TokenKind::Match => self.parse_match_expr(),
            TokenKind::OpenBrace => {
                let block = self.parse_block()?;
                Some(Expr::Block(block))
            }
            _ => {
                self.diagnostics.add(
                    Diagnostic::error(
                        format!("Expected expression, found '{}'", token.text),
                        token.span,
                    )
                    .with_code("E0020"),
                );
                None
            }
        }
    }

    fn looks_like_struct_lit(&self) -> bool {
        if let Some(next) = self.peek_next() {
            if let TokenKind::Ident(_) = next.kind {
                // Either `Name { ident: ...` or `Name { ident, ...` or `Name { ident }`
                if let Some(after) = self.tokens.get(self.current + 2) {
                    return matches!(
                        after.kind,
                        TokenKind::Colon | TokenKind::Comma | TokenKind::CloseBrace
                    );
                }
            }
        }
        false
    }

    fn parse_infix(&mut self, left: Expr) -> Option<Expr> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::EqEq
            | TokenKind::NotEq
            | TokenKind::Lt
            | TokenKind::LtEq
            | TokenKind::Gt
            | TokenKind::GtEq
            | TokenKind::AndAnd
            | TokenKind::OrOr
            | TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Shl
            | TokenKind::Shr => {
                self.advance();
                let op = match token.kind {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Sub,
                    TokenKind::Star => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
                    TokenKind::Percent => BinaryOp::Mod,
                    TokenKind::EqEq => BinaryOp::Eq,
                    TokenKind::NotEq => BinaryOp::NotEq,
                    TokenKind::Lt => BinaryOp::Lt,
                    TokenKind::LtEq => BinaryOp::LtEq,
                    TokenKind::Gt => BinaryOp::Gt,
                    TokenKind::GtEq => BinaryOp::GtEq,
                    TokenKind::AndAnd => BinaryOp::And,
                    TokenKind::OrOr => BinaryOp::Or,
                    TokenKind::Ampersand => BinaryOp::BitAnd,
                    TokenKind::Pipe => BinaryOp::BitOr,
                    TokenKind::Caret => BinaryOp::BitXor,
                    TokenKind::Shl => BinaryOp::Shl,
                    TokenKind::Shr => BinaryOp::Shr,
                    _ => unreachable!(),
                };
                let prec = Precedence::of(&token.kind);
                let right = self.parse_precedence(prec)?;
                let span = left.span().merge(right.span());
                Some(Expr::Binary(Box::new(left), op, Box::new(right), span))
            }
            TokenKind::OpenParen => {
                // Call
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenKind::CloseParen) && !self.is_at_end() {
                    args.push(self.parse_expr()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                let end_tok = self.expect(TokenKind::CloseParen, "Expected ')' after call args")?;
                let span = left.span().merge(end_tok.span);
                Some(Expr::Call(Box::new(left), args, span))
            }
            TokenKind::Dot => {
                // Method call or field access
                self.advance();
                let member = self.expect_ident("Expected member name after '.'")?;
                if self.check(&TokenKind::OpenParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(&TokenKind::CloseParen) && !self.is_at_end() {
                        args.push(self.parse_expr()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    let end_tok = self.expect(TokenKind::CloseParen, "Expected ')'")?;
                    let span = left.span().merge(end_tok.span);
                    Some(Expr::MethodCall(Box::new(left), member, args, span))
                } else {
                    let span = left.span().merge(member.span);
                    Some(Expr::FieldAccess(Box::new(left), member, span))
                }
            }
            TokenKind::OpenBracket => {
                // Index
                self.advance();
                let index = self.parse_expr()?;
                let end_tok = self.expect(TokenKind::CloseBracket, "Expected ']'")?;
                let span = left.span().merge(end_tok.span);
                Some(Expr::Index(Box::new(left), Box::new(index), span))
            }
            TokenKind::Question => {
                // Try operator: expr?
                self.advance();
                let span = left.span().merge(token.span);
                Some(Expr::Try(Box::new(left), span))
            }
            _ => Some(left),
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        let start_span = self.peek().span;
        self.expect(TokenKind::If, "Expected 'if'")?;
        let cond = self.parse_expr()?;
        let then_branch = self.parse_block()?;

        let (else_branch, end_span) = if self.match_token(&TokenKind::Else) {
            if self.check(&TokenKind::If) {
                let else_if = self.parse_if_expr()?;
                let end = else_if.span();
                (Some(Box::new(else_if)), end)
            } else {
                let else_block = self.parse_block()?;
                let end = else_block.span;
                (Some(Box::new(Expr::Block(else_block))), end)
            }
        } else {
            (None, then_branch.span)
        };

        let span = start_span.merge(end_span);
        Some(Expr::If(Box::new(cond), then_branch, else_branch, span))
    }

    fn parse_while_expr(&mut self) -> Option<Expr> {
        let start_span = self.peek().span;
        self.expect(TokenKind::While, "Expected 'while'")?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start_span.merge(body.span);
        Some(Expr::While(Box::new(cond), body, span))
    }

    fn parse_for_expr(&mut self) -> Option<Expr> {
        let start_span = self.peek().span;
        self.expect(TokenKind::For, "Expected 'for'")?;
        let var = self.expect_ident("Expected loop variable")?;

        // Expect 'in' keyword or identifier
        if let TokenKind::Ident(ref s) = self.peek().kind {
            if s == "in" {
                self.advance();
            } else {
                self.diagnostics
                    .error("Expected 'in' in for loop", self.peek().span);
            }
        } else {
            self.diagnostics
                .error("Expected 'in' in for loop", self.peek().span);
        }

        let iter = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start_span.merge(body.span);
        Some(Expr::For(var, Box::new(iter), body, span))
    }

    fn parse_loop_expr(&mut self) -> Option<Expr> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Loop, "Expected 'loop'")?;
        let body = self.parse_block()?;
        let span = start_span.merge(body.span);
        Some(Expr::Loop(body, span))
    }

    fn parse_match_expr(&mut self) -> Option<Expr> {
        let start_span = self.peek().span;
        self.expect(TokenKind::Match, "Expected 'match'")?;
        let target = self.parse_expr()?;

        self.expect(TokenKind::OpenBrace, "Expected '{' after match target")?;
        let mut arms = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            let pat = self.parse_pattern()?;
            self.expect(TokenKind::FatArrow, "Expected '=>' after match pattern")?;
            let body = self.parse_expr()?;
            self.match_token(&TokenKind::Comma);
            let span = pat_span(&pat).merge(body.span());
            arms.push(MatchArm {
                pattern: pat,
                body,
                span,
            });
        }

        let end_tok = self.expect(TokenKind::CloseBrace, "Expected '}'")?;
        let span = start_span.merge(end_tok.span);
        Some(Expr::Match(Box::new(target), arms, span))
    }

    fn parse_pattern(&mut self) -> Option<Pattern> {
        let token = self.peek().clone();
        match &token.kind {
            TokenKind::Ident(name) => {
                if name == "_" {
                    self.advance();
                    Some(Pattern::Wildcard(token.span))
                } else {
                    self.advance();
                    let ident = Ident::new(name.clone(), token.span);
                    if self.match_token(&TokenKind::OpenParen) {
                        let mut subpats = Vec::new();
                        while !self.check(&TokenKind::CloseParen) && !self.is_at_end() {
                            subpats.push(self.parse_pattern()?);
                            if !self.match_token(&TokenKind::Comma) {
                                break;
                            }
                        }
                        let end_tok = self.expect(TokenKind::CloseParen, "Expected ')'")?;
                        let span = ident.span.merge(end_tok.span);
                        Some(Pattern::Variant(ident, subpats, span))
                    } else {
                        Some(Pattern::Ident(ident))
                    }
                }
            }
            TokenKind::IntLiteral(val) => {
                self.advance();
                Some(Pattern::Literal(Literal::Int(*val), token.span))
            }
            TokenKind::StringLiteral(val) => {
                self.advance();
                Some(Pattern::Literal(Literal::String(val.clone()), token.span))
            }
            _ => {
                self.diagnostics.error(
                    format!("Expected pattern, found '{}'", token.text),
                    token.span,
                );
                None
            }
        }
    }

    pub fn parse_type(&mut self) -> Option<Type> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Ident(name) => {
                self.advance();
                let ident = Ident::new(name.clone(), token.span);
                let mut type_args = Vec::new();

                if self.match_token(&TokenKind::Lt) {
                    while !self.check(&TokenKind::Gt) && !self.is_at_end() {
                        type_args.push(self.parse_type()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    let end_tok =
                        self.expect(TokenKind::Gt, "Expected '>' after type arguments")?;
                    let span = ident.span.merge(end_tok.span);
                    Some(Type::Named(ident, type_args, span))
                } else {
                    let span = ident.span;
                    Some(Type::Named(ident, type_args, span))
                }
            }
            TokenKind::OpenBracket => {
                // Array type: [T; N]
                let start_span = token.span;
                self.advance();
                let elem_ty = self.parse_type()?;
                self.expect(TokenKind::Semicolon, "Expected ';' in array type [T; N]")?;
                let size_tok = self.peek().clone();
                let size = if let TokenKind::IntLiteral(n) = size_tok.kind {
                    self.advance();
                    n as usize
                } else {
                    self.diagnostics
                        .error("Expected array size integer", size_tok.span);
                    0
                };
                let end_tok = self.expect(TokenKind::CloseBracket, "Expected ']'")?;
                let span = start_span.merge(end_tok.span);
                Some(Type::Array(Box::new(elem_ty), size, span))
            }
            TokenKind::OpenParen => {
                let start_span = token.span;
                self.advance();
                if self.match_token(&TokenKind::CloseParen) {
                    let span = start_span.merge(self.previous().span);
                    return Some(Type::Unit(span));
                }
                let mut types = Vec::new();
                types.push(self.parse_type()?);
                while self.match_token(&TokenKind::Comma) {
                    if self.check(&TokenKind::CloseParen) {
                        break;
                    }
                    types.push(self.parse_type()?);
                }
                let end_tok = self.expect(TokenKind::CloseParen, "Expected ')'")?;
                let span = start_span.merge(end_tok.span);
                Some(Type::Tuple(types, span))
            }
            _ => {
                self.diagnostics
                    .error(format!("Expected type, found '{}'", token.text), token.span);
                None
            }
        }
    }
}

fn pat_span(p: &Pattern) -> Span {
    match p {
        Pattern::Wildcard(s) | Pattern::Literal(_, s) | Pattern::Variant(_, _, s) => *s,
        Pattern::Ident(id) => id.span,
    }
}
