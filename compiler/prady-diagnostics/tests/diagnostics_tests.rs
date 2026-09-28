use prady_diagnostics::{Diagnostic, DiagnosticBag, SourceFile, Span};

#[test]
fn test_source_file_line_column() {
    let content = "fn main() {\n    let x = 10;\n    print(x);\n}";
    let file = SourceFile::new("test.pr".to_string(), content.to_string());
    assert_eq!(file.line_count(), 4);

    let line2 = file.get_line(2).unwrap();
    assert_eq!(line2, "    let x = 10;");

    let (line, col) = file.get_location(16); // 'let' on line 2
    assert_eq!(line, 2);
    assert_eq!(col, 5);
}

#[test]
fn test_diagnostic_emission() {
    let content = "fn main() {\n    let x = 10\n}";
    let file = SourceFile::new("main.pr".to_string(), content.to_string());
    let mut bag = DiagnosticBag::new();

    let span = Span::new(24, 25, 2, 14);
    let diag = Diagnostic::error("Expected ';' after let statement", span)
        .with_code("E0005")
        .with_label("missing semicolon")
        .with_suggestion("add ';' here");

    bag.add(diag);
    assert!(bag.has_errors());

    let rendered = bag.emit_to_string(&file);
    assert!(rendered.contains("error"));
    assert!(rendered.contains("[E0005]"));
    assert!(rendered.contains("Expected ';' after let statement"));
    assert!(rendered.contains("main.pr:2:14"));
    assert!(rendered.contains("missing semicolon"));
    assert!(rendered.contains("add ';' here"));
}
