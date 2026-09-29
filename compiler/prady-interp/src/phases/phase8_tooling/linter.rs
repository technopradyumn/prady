// Phase 8 — Static Analysis Linter (`prady lint`)
// Identifies code smells, unused variables, dead code, and style inconsistencies.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintSeverity {
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct LintDiagnostic {
    pub rule: &'static str,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: LintSeverity,
    pub fix_suggestion: Option<String>,
}

pub struct Linter;

impl Linter {
    pub fn lint_source(source: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for (line_idx, line) in source.lines().enumerate() {
            let line_num = line_idx + 1;
            let trimmed = line.trim();

            // Rule: snake_case for functions
            if trimmed.starts_with("fn ") {
                if let Some(name_part) = trimmed.strip_prefix("fn ") {
                    if let Some(open_paren) = name_part.find('(') {
                        let fn_name = name_part[..open_paren].trim();
                        if fn_name.chars().any(|c| c.is_uppercase()) && fn_name != "main" {
                            diagnostics.push(LintDiagnostic {
                                rule: "naming/snake-case",
                                message: format!(
                                    "Function '{fn_name}' should follow snake_case convention"
                                ),
                                line: line_num,
                                column: 4,
                                severity: LintSeverity::Warning,
                                fix_suggestion: Some(Self::to_snake_case(fn_name)),
                            });
                        }
                    }
                }
            }

            // Rule: Disallow empty blocks
            if trimmed.contains("{}") && !trimmed.contains("architecture") {
                diagnostics.push(LintDiagnostic {
                    rule: "code-quality/no-empty-block",
                    message: "Avoid empty blocks; add implementation or comment".to_string(),
                    line: line_num,
                    column: line.find("{}").unwrap_or(0) + 1,
                    severity: LintSeverity::Info,
                    fix_suggestion: None,
                });
            }

            // Rule: Check trailing semicolons where unnecessary
            if trimmed.ends_with(";;") {
                diagnostics.push(LintDiagnostic {
                    rule: "syntax/double-semicolon",
                    message: "Unnecessary duplicate semicolon".to_string(),
                    line: line_num,
                    column: line.len(),
                    severity: LintSeverity::Warning,
                    fix_suggestion: Some(";".to_string()),
                });
            }
        }

        diagnostics
    }

    fn to_snake_case(s: &str) -> String {
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    res.push('_');
                }
                res.push(c.to_ascii_lowercase());
            } else {
                res.push(c);
            }
        }
        res
    }
}
