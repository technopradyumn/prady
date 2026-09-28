use prady_ast::*;
use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_lexer::Lexer;
use prady_parser::Parser;

fn parse(code: &str) -> (Program, DiagnosticBag) {
    let file = SourceFile::new("test.pr".to_string(), code.to_string());
    let mut diag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&file);
    let tokens = lexer.tokenize(&mut diag);
    let mut parser = Parser::new(&tokens, &mut diag);
    let prog = parser.parse_program();
    (prog, diag)
}

#[test]
fn test_parse_simple_function() {
    let code = r#"
    fn add(a: Int, b: Int) -> Int {
        return a + b;
    }
    "#;
    let (prog, diag) = parse(code);
    assert!(!diag.has_errors());
    assert_eq!(prog.items.len(), 1);

    if let Item::Function(f) = &prog.items[0] {
        assert_eq!(f.name.name, "add");
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.body.stmts.len(), 1);
        if let Stmt::Return(Some(Expr::Binary(left, op, right, _)), _) = &f.body.stmts[0] {
            assert_eq!(*op, BinaryOp::Add);
            assert!(matches!(**left, Expr::Ident(_)));
            assert!(matches!(**right, Expr::Ident(_)));
        } else {
            panic!("Expected return binary expr");
        }
    } else {
        panic!("Expected function");
    }
}

#[test]
fn test_parse_variables_and_mut() {
    let code = r#"
    fn main() {
        let x = 10;
        let mut y: Int = 20;
        y += 5;
    }
    "#;
    let (prog, diag) = parse(code);
    assert!(!diag.has_errors());
    if let Item::Function(f) = &prog.items[0] {
        assert_eq!(f.body.stmts.len(), 3);
        assert!(matches!(f.body.stmts[0], Stmt::Let(ref l) if !l.is_mut && l.name.name == "x"));
        assert!(matches!(f.body.stmts[1], Stmt::Let(ref l) if l.is_mut && l.name.name == "y"));
        assert!(matches!(f.body.stmts[2], Stmt::Assign(ref a) if a.op == AssignOp::AddAssign));
    }
}

#[test]
fn test_parse_architecture_block() {
    let code = r#"
    architecture backend {
        layer presentation;
        layer application;
        layer domain;
        layer infrastructure;

        presentation -> application;
        application -> domain;
        infrastructure -> domain;
        domain cannot import presentation;
        domain cannot import infrastructure;
    }
    "#;
    let (prog, diag) = parse(code);
    assert!(!diag.has_errors());
    assert_eq!(prog.items.len(), 1);
    if let Item::Architecture(a) = &prog.items[0] {
        assert_eq!(a.name.name, "backend");
        assert_eq!(a.layers.len(), 4);
        assert_eq!(a.rules.len(), 5);
    } else {
        panic!("Expected architecture block");
    }
}

#[test]
fn test_parse_oop_class_and_interface() {
    let code = r#"
    interface PaymentGateway {
        fn charge(amount: Money) -> Result<Receipt, PaymentError>;
    }

    class CheckoutService {
        gateway: PaymentGateway;

        fn new(gateway: PaymentGateway) -> CheckoutService {
            return CheckoutService { gateway };
        }

        fn checkout(amount: Money) -> Result<Receipt, PaymentError> {
            return gateway.charge(amount);
        }
    }
    "#;
    let (prog, diag) = parse(code);
    assert!(!diag.has_errors());
    assert_eq!(prog.items.len(), 2);
    assert!(matches!(prog.items[0], Item::Interface(_)));
    assert!(matches!(prog.items[1], Item::Class(_)));
}

#[test]
fn test_parse_match_and_try() {
    let code = r#"
    fn handle(opt: Option<Int>) -> Result<Int, Error> {
        let val = loadData()?;
        let res = match opt {
            Some(v) => v * 2,
            None => 0
        };
        return Ok(res);
    }
    "#;
    let (prog, diag) = parse(code);
    assert!(!diag.has_errors());
    assert_eq!(prog.items.len(), 1);
}
