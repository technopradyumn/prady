use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_interp::Interpreter;
use prady_lexer::Lexer;
use prady_parser::Parser;

fn run_code(src: &str) -> Result<Vec<String>, String> {
    let source = SourceFile::new("test".to_string(), src.to_string());
    let mut bag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut bag);
    let mut parser = Parser::new(&tokens, &mut bag);
    let prog = parser.parse_program();

    if bag.has_errors() {
        return Err("Parse error".to_string());
    }

    let buffer = Rc::new(RefCell::new(Vec::new()));
    let mut interp = Interpreter::new(prog).with_output_buffer(Rc::clone(&buffer));
    match interp.run_main() {
        Ok(_) => {
            let out = buffer.borrow().clone();
            Ok(out)
        }
        Err(e) => Err(e.message),
    }
}

fn run_code_with_input(src: &str, inputs: &[&str]) -> Result<Vec<String>, String> {
    let source = SourceFile::new("test".to_string(), src.to_string());
    let mut bag = DiagnosticBag::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut bag);
    let mut parser = Parser::new(&tokens, &mut bag);
    let prog = parser.parse_program();

    if bag.has_errors() {
        return Err("Parse error".to_string());
    }

    let output = Rc::new(RefCell::new(Vec::new()));
    let input = Rc::new(RefCell::new(
        inputs
            .iter()
            .map(|line| (*line).to_string())
            .collect::<VecDeque<_>>(),
    ));
    let mut interp = Interpreter::new(prog)
        .with_output_buffer(Rc::clone(&output))
        .with_input_buffer(input);
    interp.run_main().map_err(|error| error.message)?;
    let result = output.borrow().clone();
    Ok(result)
}

#[test]
fn test_print_hello() {
    let src = r#"
        fn main() {
            print("Hello");
        }
    "#;
    let out = run_code(src).expect("Execution failed");
    assert_eq!(out, vec!["Hello"]);
}

#[test]
fn test_input_reads_lines_and_writes_optional_prompt() {
    let source = r#"
        fn main() {
            let name = input("Name: ");
            let city = input();
            print("Hello " + name + " from " + city);
        }
    "#;

    let output = run_code_with_input(source, &["Ada", "London"]).expect("Execution failed");
    assert_eq!(output, vec!["Name: ", "Hello Ada from London"]);
}

#[test]
fn test_input_reports_when_no_line_is_available() {
    let source = r#"
        fn main() {
            input();
        }
    "#;

    assert_eq!(
        run_code_with_input(source, &[]).unwrap_err(),
        "No program input is available."
    );
}

#[test]
fn test_variables_and_arithmetic() {
    let src = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b;
        }

        fn main() {
            let name = "Pradyumn";
            let age: Int = 25;
            let mut counter = 0;
            counter += 1;

            print("Hello " + name);
            print("Age: " + age);
            print("Result of add(10, 20): " + add(10, 20));
            print("Counter: " + counter);
        }
    "#;
    let out = run_code(src).expect("Execution failed");
    assert_eq!(
        out,
        vec![
            "Hello Pradyumn",
            "Age: 25",
            "Result of add(10, 20): 30",
            "Counter: 1"
        ]
    );
}

#[test]
fn test_fibonacci_while_loop() {
    let src = r#"
        fn fib(n: Int) -> Int {
            if (n <= 1) {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }

        fn main() {
            let mut i = 0;
            while (i <= 5) {
                print("fib(" + i + ") = " + fib(i));
                i += 1;
            }
        }
    "#;
    let out = run_code(src).expect("Execution failed");
    assert_eq!(
        out,
        vec![
            "fib(0) = 0",
            "fib(1) = 1",
            "fib(2) = 1",
            "fib(3) = 2",
            "fib(4) = 3",
            "fib(5) = 5"
        ]
    );
}

#[test]
fn test_immutable_variable_error() {
    let src = r#"
        fn main() {
            let x = 10;
            x = 20;
        }
    "#;
    let res = run_code(src);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("Cannot reassign immutable variable 'x'"));
}

#[test]
fn test_pattern_matching_enums() {
    let src = r#"
        enum Status {
            Pending,
            Active(Int),
            Done(String),
        }

        fn describe(s: Status) -> String {
            let msg = match s {
                Pending => "waiting",
                Active(id) => "active " + id,
                Done(summary) => summary
            };
            return msg;
        }

        fn main() {
            print(describe(Pending));
            print(describe(Active(101)));
            print(describe(Done("finished")));
        }
    "#;
    let out = run_code(src).expect("Execution failed");
    assert_eq!(out, vec!["waiting", "active 101", "finished"]);
}
