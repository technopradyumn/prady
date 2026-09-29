// Phase 8 — Code Formatter (`prady fmt`)
// Deterministic source code formatter implementing official Prady styling standards.

pub struct CodeFormatter {
    pub indent_size: usize,
}

impl CodeFormatter {
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    /// Format Prady source code lines with normalized indentation and spacing.
    pub fn format(&self, source: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level: usize = 0;

        for raw_line in source.lines() {
            let trimmed = raw_line.trim();

            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }

            // Adjust indent down if closing brace is at start of line
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }

            let spaces = " ".repeat(indent_level * self.indent_size);
            formatted.push_str(&spaces);
            formatted.push_str(trimmed);
            formatted.push('\n');

            // Adjust indent up if opening brace is at end of line
            let opens = trimmed.chars().filter(|&c| c == '{').count();
            let closes = trimmed.chars().filter(|&c| c == '}').count();

            if opens > closes {
                indent_level += opens - closes;
            } else if closes > opens && !trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(closes - opens);
            }
        }

        formatted
    }
}

impl Default for CodeFormatter {
    fn default() -> Self {
        Self::new()
    }
}
