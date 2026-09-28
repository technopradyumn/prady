use prady_diagnostics::{DiagnosticBag, SourceFile, Span};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Mut,
    Class,
    Struct,
    Interface,
    Trait,
    Enum,
    Abstract,
    Extends,
    Implements,
    Return,
    If,
    Else,
    Match,
    For,
    While,
    Loop,
    Break,
    Continue,
    Import,
    Export,
    Module,
    Async,
    Await,
    Unsafe,
    Using,
    True,
    False,
    Null,
    Architecture,
    Layer,
    Cannot,

    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    // Identifiers
    Ident(String),

    // Operators
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    EqEq,          // ==
    NotEq,         // !=
    Lt,            // <
    LtEq,          // <=
    Gt,            // >
    GtEq,          // >=
    AndAnd,        // &&
    OrOr,          // ||
    Bang,          // !
    Ampersand,     // &
    Pipe,          // |
    Caret,         // ^
    Tilde,         // ~
    Shl,           // <<
    Shr,           // >>
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    Arrow,         // ->
    FatArrow,      // =>
    Question,      // ?
    Dot,           // .
    DotDot,        // ..

    // Delimiters
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    ColonColon,   // ::

    // End of file
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Interface => write!(f, "interface"),
            TokenKind::Trait => write!(f, "trait"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Abstract => write!(f, "abstract"),
            TokenKind::Extends => write!(f, "extends"),
            TokenKind::Implements => write!(f, "implements"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::For => write!(f, "for"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::Module => write!(f, "module"),
            TokenKind::Async => write!(f, "async"),
            TokenKind::Await => write!(f, "await"),
            TokenKind::Unsafe => write!(f, "unsafe"),
            TokenKind::Using => write!(f, "using"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Architecture => write!(f, "architecture"),
            TokenKind::Layer => write!(f, "layer"),
            TokenKind::Cannot => write!(f, "cannot"),
            TokenKind::IntLiteral(n) => write!(f, "{}", n),
            TokenKind::FloatLiteral(n) => write!(f, "{}", n),
            TokenKind::StringLiteral(s) => write!(f, "\"{}\"", s),
            TokenKind::CharLiteral(c) => write!(f, "'{}'", c),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::LtEq => write!(f, "<="),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::GtEq => write!(f, ">="),
            TokenKind::AndAnd => write!(f, "&&"),
            TokenKind::OrOr => write!(f, "||"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Shl => write!(f, "<<"),
            TokenKind::Shr => write!(f, ">>"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::PlusAssign => write!(f, "+="),
            TokenKind::MinusAssign => write!(f, "-="),
            TokenKind::StarAssign => write!(f, "*="),
            TokenKind::SlashAssign => write!(f, "/="),
            TokenKind::PercentAssign => write!(f, "%="),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),
            TokenKind::OpenBrace => write!(f, "{{"),
            TokenKind::CloseBrace => write!(f, "}}"),
            TokenKind::OpenBracket => write!(f, "["),
            TokenKind::CloseBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Eof => write!(f, "<EOF>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, text: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            text: text.into(),
        }
    }
}

pub struct Lexer<'a> {
    source: &'a SourceFile,
    chars: Vec<(usize, char)>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a SourceFile) -> Self {
        let chars: Vec<(usize, char)> = source.content.char_indices().collect();
        Self {
            source,
            chars,
            cursor: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).map(|(_, c)| *c)
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).map(|(_, c)| *c)
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        if self.cursor < self.chars.len() {
            let item = self.chars[self.cursor];
            self.cursor += 1;
            Some(item)
        } else {
            None
        }
    }

    fn current_offset(&self) -> usize {
        if self.cursor < self.chars.len() {
            self.chars[self.cursor].0
        } else {
            self.source.content.len()
        }
    }

    fn make_span(&self, start_offset: usize, end_offset: usize) -> Span {
        let (line, column) = self.source.get_location(start_offset);
        Span::new(start_offset, end_offset, line, column)
    }

    pub fn tokenize(&mut self, diagnostics: &mut DiagnosticBag) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            // Comments
            if c == '/' {
                if self.peek_next() == Some('/') {
                    // Line comment
                    self.advance();
                    self.advance();
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    continue;
                } else if self.peek_next() == Some('*') {
                    // Block comment (supports nesting)
                    let start = self.current_offset();
                    self.advance();
                    self.advance();
                    let mut depth = 1;
                    while depth > 0 && self.cursor < self.chars.len() {
                        if self.peek() == Some('/') && self.peek_next() == Some('*') {
                            self.advance();
                            self.advance();
                            depth += 1;
                        } else if self.peek() == Some('*') && self.peek_next() == Some('/') {
                            self.advance();
                            self.advance();
                            depth -= 1;
                        } else {
                            self.advance();
                        }
                    }
                    if depth > 0 {
                        let span = self.make_span(start, self.current_offset());
                        diagnostics.error("Unterminated block comment", span);
                    }
                    continue;
                }
            }

            let start = self.current_offset();

            // Identifiers / Keywords
            if c.is_alphabetic() || c == '_' {
                let token = self.lex_identifier_or_keyword(start);
                tokens.push(token);
                continue;
            }

            // Numeric Literals
            if c.is_ascii_digit() {
                let token = self.lex_number(start, diagnostics);
                tokens.push(token);
                continue;
            }

            // String Literals
            if c == '"' {
                let token = self.lex_string(start, diagnostics);
                tokens.push(token);
                continue;
            }

            // Character Literals
            if c == '\'' {
                let token = self.lex_char(start, diagnostics);
                tokens.push(token);
                continue;
            }

            // Operators & Punctuation
            self.advance();
            let end = self.current_offset();
            let span = self.make_span(start, end);

            let (kind, text) = match c {
                '+' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::PlusAssign, span, "+="));
                        continue;
                    }
                    (TokenKind::Plus, "+")
                }
                '-' => {
                    if self.peek() == Some('>') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::Arrow, span, "->"));
                        continue;
                    } else if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::MinusAssign, span, "-="));
                        continue;
                    }
                    (TokenKind::Minus, "-")
                }
                '*' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::StarAssign, span, "*="));
                        continue;
                    }
                    (TokenKind::Star, "*")
                }
                '/' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::SlashAssign, span, "/="));
                        continue;
                    }
                    (TokenKind::Slash, "/")
                }
                '%' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::PercentAssign, span, "%="));
                        continue;
                    }
                    (TokenKind::Percent, "%")
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::EqEq, span, "=="));
                        continue;
                    } else if self.peek() == Some('>') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::FatArrow, span, "=>"));
                        continue;
                    }
                    (TokenKind::Assign, "=")
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::NotEq, span, "!="));
                        continue;
                    }
                    (TokenKind::Bang, "!")
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::LtEq, span, "<="));
                        continue;
                    } else if self.peek() == Some('<') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::Shl, span, "<<"));
                        continue;
                    }
                    (TokenKind::Lt, "<")
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::GtEq, span, ">="));
                        continue;
                    } else if self.peek() == Some('>') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::Shr, span, ">>"));
                        continue;
                    }
                    (TokenKind::Gt, ">")
                }
                '&' => {
                    if self.peek() == Some('&') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::AndAnd, span, "&&"));
                        continue;
                    }
                    (TokenKind::Ampersand, "&")
                }
                '|' => {
                    if self.peek() == Some('|') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::OrOr, span, "||"));
                        continue;
                    }
                    (TokenKind::Pipe, "|")
                }
                '^' => (TokenKind::Caret, "^"),
                '~' => (TokenKind::Tilde, "~"),
                '?' => (TokenKind::Question, "?"),
                '.' => {
                    if self.peek() == Some('.') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::DotDot, span, ".."));
                        continue;
                    }
                    (TokenKind::Dot, ".")
                }
                '(' => (TokenKind::OpenParen, "("),
                ')' => (TokenKind::CloseParen, ")"),
                '{' => (TokenKind::OpenBrace, "{"),
                '}' => (TokenKind::CloseBrace, "}"),
                '[' => (TokenKind::OpenBracket, "["),
                ']' => (TokenKind::CloseBracket, "]"),
                ',' => (TokenKind::Comma, ","),
                ';' => (TokenKind::Semicolon, ";"),
                ':' => {
                    if self.peek() == Some(':') {
                        self.advance();
                        let span = self.make_span(start, self.current_offset());
                        tokens.push(Token::new(TokenKind::ColonColon, span, "::"));
                        continue;
                    }
                    (TokenKind::Colon, ":")
                }
                unknown => {
                    diagnostics.error(format!("Unexpected character '{}'", unknown), span);
                    continue;
                }
            };

            tokens.push(Token::new(kind, span, text));
        }

        let eof_offset = self.source.content.len();
        let eof_span = self.make_span(eof_offset, eof_offset);
        tokens.push(Token::new(TokenKind::Eof, eof_span, ""));

        tokens
    }

    fn lex_identifier_or_keyword(&mut self, start: usize) -> Token {
        let mut text = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let end = self.current_offset();
        let span = self.make_span(start, end);

        let kind = match text.as_str() {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "class" => TokenKind::Class,
            "struct" => TokenKind::Struct,
            "interface" => TokenKind::Interface,
            "trait" => TokenKind::Trait,
            "enum" => TokenKind::Enum,
            "abstract" => TokenKind::Abstract,
            "extends" => TokenKind::Extends,
            "implements" => TokenKind::Implements,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "module" => TokenKind::Module,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "unsafe" => TokenKind::Unsafe,
            "using" => TokenKind::Using,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "architecture" => TokenKind::Architecture,
            "layer" => TokenKind::Layer,
            "cannot" => TokenKind::Cannot,
            _ => TokenKind::Ident(text.clone()),
        };

        Token::new(kind, span, text)
    }

    fn lex_number(&mut self, start: usize, diagnostics: &mut DiagnosticBag) -> Token {
        let mut raw = String::new();

        // Check for 0x, 0b, 0o
        if self.peek() == Some('0') {
            raw.push('0');
            self.advance();
            if let Some(prefix) = self.peek() {
                if prefix == 'x' || prefix == 'X' {
                    raw.push(prefix);
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            if c != '_' {
                                raw.push(c);
                            }
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let end = self.current_offset();
                    let span = self.make_span(start, end);
                    let hex_digits = &raw[2..];
                    match i64::from_str_radix(hex_digits, 16) {
                        Ok(val) => return Token::new(TokenKind::IntLiteral(val), span, raw),
                        Err(_) => {
                            diagnostics.error("Invalid hex integer literal", span);
                            return Token::new(TokenKind::IntLiteral(0), span, raw);
                        }
                    }
                } else if prefix == 'b' || prefix == 'B' {
                    raw.push(prefix);
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            if c != '_' {
                                raw.push(c);
                            }
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let end = self.current_offset();
                    let span = self.make_span(start, end);
                    let bin_digits = &raw[2..];
                    match i64::from_str_radix(bin_digits, 2) {
                        Ok(val) => return Token::new(TokenKind::IntLiteral(val), span, raw),
                        Err(_) => {
                            diagnostics.error("Invalid binary integer literal", span);
                            return Token::new(TokenKind::IntLiteral(0), span, raw);
                        }
                    }
                } else if prefix == 'o' || prefix == 'O' {
                    raw.push(prefix);
                    self.advance();
                    while let Some(c) = self.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            if c != '_' {
                                raw.push(c);
                            }
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let end = self.current_offset();
                    let span = self.make_span(start, end);
                    let oct_digits = &raw[2..];
                    match i64::from_str_radix(oct_digits, 8) {
                        Ok(val) => return Token::new(TokenKind::IntLiteral(val), span, raw),
                        Err(_) => {
                            diagnostics.error("Invalid octal integer literal", span);
                            return Token::new(TokenKind::IntLiteral(0), span, raw);
                        }
                    }
                }
            }
        }

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                if c != '_' {
                    raw.push(c);
                }
                self.advance();
            } else {
                break;
            }
        }

        // Float check: '.' followed by digit
        let mut is_float = false;
        if self.peek() == Some('.')
            && self
                .peek_next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            is_float = true;
            raw.push('.');
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '_' {
                    if c != '_' {
                        raw.push(c);
                    }
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Exponent: e / E
        if self.peek() == Some('e') || self.peek() == Some('E') {
            is_float = true;
            raw.push('e');
            self.advance();
            if self.peek() == Some('+') || self.peek() == Some('-') {
                raw.push(self.peek().unwrap());
                self.advance();
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    raw.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let end = self.current_offset();
        let span = self.make_span(start, end);

        if is_float {
            match raw.parse::<f64>() {
                Ok(val) => Token::new(TokenKind::FloatLiteral(val), span, raw),
                Err(_) => {
                    diagnostics.error("Invalid float literal", span);
                    Token::new(TokenKind::FloatLiteral(0.0), span, raw)
                }
            }
        } else {
            match raw.parse::<i64>() {
                Ok(val) => Token::new(TokenKind::IntLiteral(val), span, raw),
                Err(_) => {
                    diagnostics.error("Integer literal out of range", span);
                    Token::new(TokenKind::IntLiteral(0), span, raw)
                }
            }
        }
    }

    fn lex_string(&mut self, start: usize, diagnostics: &mut DiagnosticBag) -> Token {
        self.advance(); // consume opening quote
        let mut content = String::new();
        let mut terminated = false;

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                terminated = true;
                break;
            } else if c == '\\' {
                self.advance();
                match self.advance().map(|(_, ch)| ch) {
                    Some('n') => content.push('\n'),
                    Some('t') => content.push('\t'),
                    Some('r') => content.push('\r'),
                    Some('\\') => content.push('\\'),
                    Some('"') => content.push('"'),
                    Some('0') => content.push('\0'),
                    Some(other) => {
                        let span = self.make_span(start, self.current_offset());
                        diagnostics.warning(format!("Unknown escape sequence '\\{}'", other), span);
                        content.push(other);
                    }
                    None => break,
                }
            } else {
                content.push(c);
                self.advance();
            }
        }

        let end = self.current_offset();
        let span = self.make_span(start, end);
        if !terminated {
            diagnostics.error("Unterminated string literal", span);
        }

        Token::new(TokenKind::StringLiteral(content), span, r#""...""#.to_string())
    }

    fn lex_char(&mut self, start: usize, diagnostics: &mut DiagnosticBag) -> Token {
        self.advance(); // consume opening quote
        let mut ch = ' ';
        if let Some(c) = self.peek() {
            if c == '\\' {
                self.advance();
                ch = match self.advance().map(|(_, ch)| ch) {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\\') => '\\',
                    Some('\'') => '\'',
                    Some('0') => '\0',
                    Some(other) => other,
                    None => ' ',
                };
            } else {
                ch = c;
                self.advance();
            }
        }

        if self.peek() == Some('\'') {
            self.advance();
        } else {
            let span = self.make_span(start, self.current_offset());
            diagnostics.error("Unclosed character literal", span);
        }

        let end = self.current_offset();
        let span = self.make_span(start, end);
        Token::new(TokenKind::CharLiteral(ch), span, format!("'{}'", ch))
    }
}
