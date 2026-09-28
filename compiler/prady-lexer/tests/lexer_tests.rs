use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_lexer::{Lexer, TokenKind};

#[test]
fn test_tokenize_keywords_and_vars() {
    let src = "let mut count: Int = 42; fn add(x: Int, y: Int) -> Int { return x + y; }";
    let file = SourceFile::new("test.pr".to_string(), src.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);

    assert!(!diag.has_errors());
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Mut);
    assert_eq!(tokens[2].kind, TokenKind::Ident("count".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::Colon);
    assert_eq!(tokens[4].kind, TokenKind::Ident("Int".to_string()));
    assert_eq!(tokens[5].kind, TokenKind::Assign);
    assert_eq!(tokens[6].kind, TokenKind::IntLiteral(42));
    assert_eq!(tokens[7].kind, TokenKind::Semicolon);
    assert_eq!(tokens[8].kind, TokenKind::Fn);
}

#[test]
fn test_tokenize_hex_bin_floats() {
    let src = "0xFF 0b1010 42.5 1.5e3";
    let file = SourceFile::new("nums.pr".to_string(), src.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);

    assert!(!diag.has_errors());
    assert_eq!(tokens[0].kind, TokenKind::IntLiteral(255));
    assert_eq!(tokens[1].kind, TokenKind::IntLiteral(10));
    assert_eq!(tokens[2].kind, TokenKind::FloatLiteral(42.5));
    assert_eq!(tokens[3].kind, TokenKind::FloatLiteral(1500.0));
}

#[test]
fn test_tokenize_strings_and_chars() {
    let src = r#""Hello \"Prady\"!\n" 'z'"#;
    let file = SourceFile::new("str.pr".to_string(), src.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);

    assert!(!diag.has_errors());
    assert_eq!(
        tokens[0].kind,
        TokenKind::StringLiteral("Hello \"Prady\"!\n".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::CharLiteral('z'));
}

#[test]
fn test_tokenize_architecture_keywords() {
    let src = "architecture backend { layer domain; presentation -> application; domain cannot import presentation; }";
    let file = SourceFile::new("arch.pr".to_string(), src.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);

    assert!(!diag.has_errors());
    assert_eq!(tokens[0].kind, TokenKind::Architecture);
    assert_eq!(tokens[1].kind, TokenKind::Ident("backend".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::OpenBrace);
    assert_eq!(tokens[3].kind, TokenKind::Layer);
    assert_eq!(tokens[4].kind, TokenKind::Ident("domain".to_string()));
    assert_eq!(tokens[5].kind, TokenKind::Semicolon);
}

#[test]
fn test_nested_block_comments() {
    let src = "let x = /* outer /* inner */ still comment */ 100;";
    let file = SourceFile::new("comment.pr".to_string(), src.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);

    assert!(!diag.has_errors());
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Ident("x".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::Assign);
    assert_eq!(tokens[3].kind, TokenKind::IntLiteral(100));
}
