// Phase 7 — Built-in Lightweight JSON Serializer & Deserializer
// Zero-dependency JSON parser for networking, configuration, and API payloads.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn stringify(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|item| item.stringify()).collect();
                format!("[{}]", items.join(", "))
            }
            JsonValue::Object(map) => {
                let pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.stringify()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    /// Very simple JSON string scanner.
    pub fn parse_string_field(json_str: &str, field: &str) -> Option<String> {
        let pattern = format!("\"{field}\":");
        if let Some(pos) = json_str.find(&pattern) {
            let after = &json_str[pos + pattern.len()..].trim_start();
            if after.starts_with('"') {
                let inner = &after[1..];
                if let Some(end) = inner.find('"') {
                    return Some(inner[..end].to_string());
                }
            }
        }
        None
    }
}
