use std::fmt;

/// Terminal styling helpers using standard ANSI escape sequences.
pub mod style {
    pub fn red(text: &str) -> String {
        format!("\x1b[31m{}\x1b[0m", text)
    }
    pub fn red_bold(text: &str) -> String {
        format!("\x1b[31;1m{}\x1b[0m", text)
    }
    pub fn yellow_bold(text: &str) -> String {
        format!("\x1b[33;1m{}\x1b[0m", text)
    }
    pub fn blue_bold(text: &str) -> String {
        format!("\x1b[34;1m{}\x1b[0m", text)
    }
    pub fn cyan_bold(text: &str) -> String {
        format!("\x1b[36;1m{}\x1b[0m", text)
    }
    pub fn bold(text: &str) -> String {
        format!("\x1b[1m{}\x1b[0m", text)
    }
    pub fn cyan(text: &str) -> String {
        format!("\x1b[36m{}\x1b[0m", text)
    }
    pub fn green_bold(text: &str) -> String {
        format!("\x1b[32;1m{}\x1b[0m", text)
    }
}

/// Represents a source span within a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn merge(self, other: Span) -> Span {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        let (line, column) = if self.start <= other.start {
            (self.line, self.column)
        } else {
            (other.line, other.column)
        };
        Span {
            start,
            end,
            line,
            column,
        }
    }
}

/// Represents a loaded source file for diagnostics reporting.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub content: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(name: String, content: String) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in content.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            name,
            content,
            line_starts,
        }
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num == 0 || line_num > self.line_starts.len() {
            return None;
        }
        let start = self.line_starts[line_num - 1];
        let end = if line_num < self.line_starts.len() {
            let next_start = self.line_starts[line_num];
            if next_start > 0 && self.content.as_bytes().get(next_start - 1) == Some(&b'\n') {
                if next_start > 1 && self.content.as_bytes().get(next_start - 2) == Some(&b'\r') {
                    next_start - 2
                } else {
                    next_start - 1
                }
            } else {
                next_start
            }
        } else {
            self.content.len()
        };
        self.content.get(start..end)
    }

    pub fn get_location(&self, offset: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        };
        let line_start = if line > 0 && line <= self.line_starts.len() {
            self.line_starts[line - 1]
        } else {
            0
        };
        let col = offset.saturating_sub(line_start) + 1;
        (line, col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
    Suggestion,
}

impl DiagnosticLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Suggestion => "suggestion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: Option<String>,
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span,
    pub label: Option<String>,
    pub notes: Vec<String>,
    pub suggestions: Vec<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            code: None,
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            label: None,
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            code: None,
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
            label: None,
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn is_error(&self) -> bool {
        self.level == DiagnosticLevel::Error
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn error(&mut self, message: impl Into<String>, span: Span) {
        self.add(Diagnostic::error(message, span));
    }

    pub fn warning(&mut self, message: impl Into<String>, span: Span) {
        self.add(Diagnostic::warning(message, span));
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn emit_to_string(&self, file: &SourceFile) -> String {
        let mut out = String::new();
        for diag in &self.diagnostics {
            let header_color = match diag.level {
                DiagnosticLevel::Error => style::red_bold(diag.level.as_str()),
                DiagnosticLevel::Warning => style::yellow_bold(diag.level.as_str()),
                DiagnosticLevel::Info => style::blue_bold(diag.level.as_str()),
                DiagnosticLevel::Suggestion => style::cyan_bold(diag.level.as_str()),
            };

            let code_str = match &diag.code {
                Some(code) => style::bold(&format!("[{}]", code)),
                None => String::new(),
            };

            out.push_str(&format!(
                "{}{}: {}\n  --> {}:{}:{}\n",
                header_color,
                code_str,
                style::bold(&diag.message),
                file.name,
                diag.span.line,
                diag.span.column
            ));

            if let Some(line_content) = file.get_line(diag.span.line) {
                let line_str = format!("{}", diag.span.line);
                let padding = " ".repeat(line_str.len());
                out.push_str(&format!(" {} |\n", padding));
                out.push_str(&format!(
                    " {} | {}\n",
                    style::blue_bold(&line_str),
                    line_content
                ));

                let col_offset = diag.span.column.saturating_sub(1);
                let width = (diag.span.end.saturating_sub(diag.span.start)).max(1);
                let caret_line = format!(
                    " {} | {}{}",
                    padding,
                    " ".repeat(col_offset),
                    style::red_bold(&"^".repeat(width))
                );
                if let Some(ref label) = diag.label {
                    out.push_str(&format!("{} {}\n", caret_line, style::red(label)));
                } else {
                    out.push_str(&format!("{}\n", caret_line));
                }
            }

            for note in &diag.notes {
                out.push_str(&format!("   = {}: {}\n", style::bold("note"), note));
            }
            for suggestion in &diag.suggestions {
                out.push_str(&format!(
                    "   = {}: {}\n",
                    style::cyan_bold("help"),
                    suggestion
                ));
            }
            out.push('\n');
        }
        out
    }

    pub fn emit(&self, file: &SourceFile) {
        print!("{}", self.emit_to_string(file));
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} at line {}:{}",
            self.level.as_str(),
            self.message,
            self.span.line,
            self.span.column
        )
    }
}
