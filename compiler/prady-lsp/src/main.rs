/// prady-lsp: A Language Server Protocol implementation for the Prady language.
///
/// Speaks JSON-RPC 2.0 / LSP over stdin/stdout.
/// Provides:
///   - Diagnostics (errors/warnings) on open/change/save
///   - Go-to-definition (Ctrl+Click) for functions, classes, structs
///   - Hover information (type signatures, doc comments)
///   - Document symbols (outline view / breadcrumbs)
///   - Completion (keywords + declared symbols)

use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_lexer::Lexer;
use prady_parser::Parser;
use prady_ast::{Item, Stmt, Expr, Ident};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

// ─── LSP wire types ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RpcMessage {
    #[serde(default)]
    id: Value,
    method: Option<String>,
    params: Option<Value>,
}

fn send(msg: &Value) {
    let text = serde_json::to_string(msg).unwrap();
    let out = io::stdout();
    let mut lock = out.lock();
    write!(lock, "Content-Length: {}\r\n\r\n{}", text.len(), text).unwrap();
    lock.flush().unwrap();
}

fn send_response(id: &Value, result: Value) {
    send(&json!({ "jsonrpc": "2.0", "id": id, "result": result }));
}

fn send_notification(method: &str, params: Value) {
    send(&json!({ "jsonrpc": "2.0", "method": method, "params": params }));
}

// ─── Symbol table built from AST ────────────────────────────────────────────

#[derive(Debug, Clone)]
struct SymbolDef {
    name: String,
    kind: u32,          // LSP SymbolKind
    detail: String,
    line: u32,
    character: u32,
    end_line: u32,
    end_char: u32,
    uri: String,
}

fn lsp_pos(src: &SourceFile, offset: usize) -> (u32, u32) {
    let (line, col) = src.get_location(offset);
    (line.saturating_sub(1) as u32, col.saturating_sub(1) as u32)
}

/// Parse a source file and return (diagnostics_json, symbols)
fn analyse(uri: &str, text: &str) -> (Value, Vec<SymbolDef>) {
    let source = SourceFile::new(uri.to_string(), text.to_string());
    let mut diag_bag = DiagnosticBag::new();

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut diag_bag);
    let mut parser = Parser::new(&tokens, &mut diag_bag);
    let program = parser.parse_program();

    // Build diagnostics array
    let mut diags = Vec::new();
    for d in diag_bag.diagnostics() {
        let sev = if d.is_error() { 1u32 } else { 2u32 };
        let (sl, sc) = lsp_pos(&source, d.span.start);
        let (mut el, mut ec) = lsp_pos(&source, d.span.end.max(d.span.start));
        if sl == el && ec <= sc {
            ec = sc + 1;
        }
        let mut msg = d.message.clone();
        for sug in &d.suggestions {
            msg.push_str(&format!("\nhelp: {}", sug));
        }
        for note in &d.notes {
            msg.push_str(&format!("\nnote: {}", note));
        }
        let mut obj = json!({
            "range": {
                "start": { "line": sl, "character": sc },
                "end":   { "line": el, "character": ec }
            },
            "severity": sev,
            "source": "prady",
            "message": msg
        });
        if let Some(code) = &d.code {
            obj["code"] = json!(code);
        }
        diags.push(obj);
    }

    // Build symbol list from top-level items
    let mut symbols: Vec<SymbolDef> = Vec::new();
    for item in &program.items {
        match item {
            Item::Function(f) => {
                let (sl, sc) = lsp_pos(&source, f.span.start);
                let (el, ec) = lsp_pos(&source, f.span.end);
                let mut sig = format!("fn {}(", f.name.name);
                for (i, p) in f.params.iter().enumerate() {
                    if i > 0 { sig.push_str(", "); }
                    sig.push_str(&format!("{}: {:?}", p.name.name, p.ty));
                }
                sig.push(')');
                if let Some(ref ret) = f.return_type {
                    sig.push_str(&format!(" -> {:?}", ret));
                }
                symbols.push(SymbolDef {
                    name: f.name.name.clone(),
                    kind: 12, // Function
                    detail: sig,
                    line: sl, character: sc,
                    end_line: el, end_char: ec,
                    uri: uri.to_string(),
                });

                // Collect local let-bindings inside function body
                collect_block_symbols(&f.body.stmts, uri, &source, &mut symbols);
            }
            Item::Class(c) => {
                let (sl, sc) = lsp_pos(&source, c.span.start);
                let (el, ec) = lsp_pos(&source, c.span.end);
                symbols.push(SymbolDef {
                    name: c.name.name.clone(),
                    kind: 5, // Class
                    detail: format!("class {}", c.name.name),
                    line: sl, character: sc,
                    end_line: el, end_char: ec,
                    uri: uri.to_string(),
                });
                for field in &c.fields {
                    let (fl, fc) = lsp_pos(&source, field.span.start);
                    symbols.push(SymbolDef {
                        name: field.name.name.clone(),
                        kind: 8, // Field
                        detail: format!("{}: {:?}", field.name.name, field.ty),
                        line: fl, character: fc,
                        end_line: fl, end_char: fc + field.name.name.len() as u32,
                        uri: uri.to_string(),
                    });
                }
                for method in &c.methods {
                    let (ml, mc) = lsp_pos(&source, method.span.start);
                    let (mel, mec) = lsp_pos(&source, method.span.end);
                    symbols.push(SymbolDef {
                        name: method.name.name.clone(),
                        kind: 6, // Method
                        detail: format!("fn {} (in class {})", method.name.name, c.name.name),
                        line: ml, character: mc,
                        end_line: mel, end_char: mec,
                        uri: uri.to_string(),
                    });
                }
            }
            Item::Struct(s) => {
                let (sl, sc) = lsp_pos(&source, s.span.start);
                let (el, ec) = lsp_pos(&source, s.span.end);
                symbols.push(SymbolDef {
                    name: s.name.name.clone(),
                    kind: 23, // Struct
                    detail: format!("struct {}", s.name.name),
                    line: sl, character: sc,
                    end_line: el, end_char: ec,
                    uri: uri.to_string(),
                });
                for field in &s.fields {
                    let (fl, fc) = lsp_pos(&source, field.span.start);
                    symbols.push(SymbolDef {
                        name: field.name.name.clone(),
                        kind: 8,
                        detail: format!("{}: {:?}", field.name.name, field.ty),
                        line: fl, character: fc,
                        end_line: fl, end_char: fc + field.name.name.len() as u32,
                        uri: uri.to_string(),
                    });
                }
            }
            Item::Import(imp) => {
                let (sl, sc) = lsp_pos(&source, imp.span.start);
                let parts: Vec<_> = imp.path.iter().map(|p| p.name.as_str()).collect();
                symbols.push(SymbolDef {
                    name: parts.join("."),
                    kind: 9, // Module
                    detail: format!("import {}", parts.join(".")),
                    line: sl, character: sc,
                    end_line: sl, end_char: sc,
                    uri: uri.to_string(),
                });
            }
            _ => {}
        }
    }

    (json!(diags), symbols)
}

fn collect_block_symbols(
    stmts: &[prady_ast::Stmt],
    uri: &str,
    source: &SourceFile,
    symbols: &mut Vec<SymbolDef>,
) {
    for stmt in stmts {
        if let Stmt::Let(l) = stmt {
            let (sl, sc) = lsp_pos(source, l.span.start);
            let kw = if l.is_const { "const" } else { "let" };
            let ty_str = l.ty.as_ref().map(|t| format!(" : {:?}", t)).unwrap_or_default();
            symbols.push(SymbolDef {
                name: l.name.name.clone(),
                kind: 13, // Variable
                detail: format!("{} {}{}", kw, l.name.name, ty_str),
                line: sl, character: sc,
                end_line: sl, end_char: sc + l.name.name.len() as u32,
                uri: uri.to_string(),
            });
        }
    }
}

// ─── Server state ────────────────────────────────────────────────────────────

struct Server {
    /// uri → (text, symbols)
    docs: HashMap<String, (String, Vec<SymbolDef>)>,
}

impl Server {
    fn new() -> Self { Server { docs: HashMap::new() } }

    fn update_doc(&mut self, uri: &str, text: &str) {
        let (diags_json, symbols) = analyse(uri, text);
        self.docs.insert(uri.to_string(), (text.to_string(), symbols));
        send_notification("textDocument/publishDiagnostics", json!({
            "uri": uri,
            "diagnostics": diags_json
        }));
    }

    fn handle(&mut self, msg: RpcMessage) {
        let method = match msg.method.as_deref() {
            Some(m) => m,
            None => return,
        };
        let params = msg.params.clone().unwrap_or(Value::Null);

        match method {
            "initialize" => {
                send_response(&msg.id, json!({
                    "capabilities": {
                        "textDocumentSync": 1,
                        "hoverProvider": true,
                        "definitionProvider": true,
                        "documentSymbolProvider": true,
                        "completionProvider": {
                            "triggerCharacters": [".", ":"]
                        },
                        "diagnosticProvider": {
                            "interFileDependencies": false,
                            "workspaceDiagnostics": false
                        }
                    },
                    "serverInfo": { "name": "prady-lsp", "version": "1.0.0" }
                }));
            }

            "initialized" => {} // no-op

            "shutdown" => {
                send_response(&msg.id, Value::Null);
            }

            "exit" => {
                std::process::exit(0);
            }

            "textDocument/didOpen" => {
                if let Some(doc) = params.get("textDocument") {
                    let uri  = doc["uri"].as_str().unwrap_or("").to_string();
                    let text = doc["text"].as_str().unwrap_or("").to_string();
                    self.update_doc(&uri, &text);
                }
            }

            "textDocument/didChange" => {
                if let Some(doc) = params.get("textDocument") {
                    let uri = doc["uri"].as_str().unwrap_or("").to_string();
                    if let Some(changes) = params["contentChanges"].as_array() {
                        if let Some(last) = changes.last() {
                            let text = last["text"].as_str().unwrap_or("").to_string();
                            self.update_doc(&uri, &text);
                        }
                    }
                }
            }

            "textDocument/didSave" => {
                // Re-analyse on save
                if let Some(doc) = params.get("textDocument") {
                    let uri = doc["uri"].as_str().unwrap_or("").to_string();
                    if let Some((text, _)) = self.docs.get(&uri).cloned() {
                        self.update_doc(&uri, &text);
                    }
                }
            }

            "textDocument/didClose" => {
                if let Some(doc) = params.get("textDocument") {
                    let uri = doc["uri"].as_str().unwrap_or("");
                    self.docs.remove(uri);
                    // Clear diagnostics
                    send_notification("textDocument/publishDiagnostics", json!({
                        "uri": uri,
                        "diagnostics": []
                    }));
                }
            }

            // ── Go-to-definition / Ctrl+Click ─────────────────────────────
            "textDocument/definition" => {
                let uri  = params["textDocument"]["uri"].as_str().unwrap_or("");
                let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
                let col  = params["position"]["character"].as_u64().unwrap_or(0) as u32;

                let word = self.word_at(uri, line, col);

                // Search for definition across ALL open documents (cross-file)
                let location = self.find_definition(&word);
                match location {
                    Some(loc) => send_response(&msg.id, loc),
                    None      => send_response(&msg.id, Value::Null),
                }
            }

            // ── Hover ─────────────────────────────────────────────────────
            "textDocument/hover" => {
                let uri  = params["textDocument"]["uri"].as_str().unwrap_or("");
                let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
                let col  = params["position"]["character"].as_u64().unwrap_or(0) as u32;

                let word = self.word_at(uri, line, col);
                let info = self.find_hover(&word, uri);
                match info {
                    Some(text) => send_response(&msg.id, json!({
                        "contents": { "kind": "markdown", "value": text }
                    })),
                    None => send_response(&msg.id, Value::Null),
                }
            }

            // ── Document symbols (Outline) ─────────────────────────────────
            "textDocument/documentSymbol" => {
                let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                let syms = self.doc_symbols(uri);
                send_response(&msg.id, syms);
            }

            // ── Completion ────────────────────────────────────────────────
            "textDocument/completion" => {
                let uri  = params["textDocument"]["uri"].as_str().unwrap_or("");
                let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
                let col  = params["position"]["character"].as_u64().unwrap_or(0) as u32;
                let prefix = self.prefix_at(uri, line, col);
                let items  = self.completions(&prefix, uri);
                send_response(&msg.id, json!({ "isIncomplete": false, "items": items }));
            }

            _ => {
                // Unknown request — send null result so client doesn't hang
                if msg.id != Value::Null {
                    send_response(&msg.id, Value::Null);
                }
            }
        }
    }

    // ── Helper: extract word under cursor ─────────────────────────────────
    fn word_at(&self, uri: &str, line: u32, col: u32) -> String {
        let text = match self.docs.get(uri) {
            Some((t, _)) => t.clone(),
            None => return String::new(),
        };
        let lines: Vec<&str> = text.lines().collect();
        let l = lines.get(line as usize).copied().unwrap_or("");
        let bytes = l.as_bytes();
        let c = col as usize;
        let start = (0..c).rev().find(|&i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_')
            .map(|i| i + 1).unwrap_or(0);
        let end = (c..bytes.len()).find(|&i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_')
            .unwrap_or(bytes.len());
        l[start..end].to_string()
    }

    fn prefix_at(&self, uri: &str, line: u32, col: u32) -> String {
        let text = match self.docs.get(uri) {
            Some((t, _)) => t.clone(),
            None => return String::new(),
        };
        let lines: Vec<&str> = text.lines().collect();
        let l = lines.get(line as usize).copied().unwrap_or("");
        let bytes = l.as_bytes();
        let c = col as usize;
        let start = (0..c).rev()
            .find(|&i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_')
            .map(|i| i + 1).unwrap_or(0);
        l[start..c].to_string()
    }

    // ── Helper: find definition location ──────────────────────────────────
    fn find_definition(&self, name: &str) -> Option<Value> {
        if name.is_empty() { return None; }
        for (_, (_, symbols)) in &self.docs {
            // Prefer exact top-level definitions (functions, classes, structs)
            for sym in symbols {
                if sym.name == name && matches!(sym.kind, 12 | 5 | 23 | 6) {
                    return Some(json!({
                        "uri": sym.uri,
                        "range": {
                            "start": { "line": sym.line, "character": sym.character },
                            "end":   { "line": sym.end_line, "character": sym.end_char }
                        }
                    }));
                }
            }
        }
        // Fallback: any symbol with that name
        for (_, (_, symbols)) in &self.docs {
            for sym in symbols {
                if sym.name == name {
                    return Some(json!({
                        "uri": sym.uri,
                        "range": {
                            "start": { "line": sym.line, "character": sym.character },
                            "end":   { "line": sym.end_line, "character": sym.end_char }
                        }
                    }));
                }
            }
        }
        None
    }

    // ── Helper: hover info ────────────────────────────────────────────────
    fn find_hover(&self, name: &str, uri: &str) -> Option<String> {
        if name.is_empty() { return None; }

        // Check built-ins first
        let builtin = match name {
            "print"   => Some("```prady\nfn print(value: Any)\n```\n\nPrints a value to stdout."),
            "println" => Some("```prady\nfn println(value: Any)\n```\n\nPrints a value followed by a newline."),
            "len"     => Some("```prady\nfn len(collection: Any) -> Int\n```\n\nReturns the length of an array or string."),
            "assert"  => Some("```prady\nfn assert(cond: Bool, msg: String)\n```\n\nAsserts a condition is true."),
            "type_of" => Some("```prady\nfn type_of(val: Any) -> String\n```\n\nReturns the runtime type name of a value."),
            _ => None,
        };
        if let Some(bi) = builtin {
            return Some(bi.to_string());
        }

        // Look in current doc first, then all docs
        let search_order = std::iter::once(uri)
            .chain(self.docs.keys().map(|s| s.as_str()));
        for search_uri in search_order {
            if let Some((_, symbols)) = self.docs.get(search_uri) {
                for sym in symbols {
                    if sym.name == name {
                        return Some(format!("```prady\n{}\n```", sym.detail));
                    }
                }
            }
        }
        None
    }

    // ── Helper: document symbols for outline view ─────────────────────────
    fn doc_symbols(&self, uri: &str) -> Value {
        let symbols = match self.docs.get(uri) {
            Some((_, s)) => s,
            None => return json!([]),
        };
        let items: Vec<Value> = symbols.iter().map(|s| json!({
            "name": s.name,
            "kind": s.kind,
            "detail": s.detail,
            "location": {
                "uri": s.uri,
                "range": {
                    "start": { "line": s.line, "character": s.character },
                    "end":   { "line": s.end_line, "character": s.end_char }
                }
            }
        })).collect();
        json!(items)
    }

    // ── Helper: completion items ──────────────────────────────────────────
    fn completions(&self, prefix: &str, uri: &str) -> Vec<Value> {
        let keywords = [
            "fn","let","const","mut","class","struct","interface","trait","enum",
            "return","if","else","while","for","loop","break","continue",
            "import","export","true","false","null","match","async","await",
            "architecture","layer","using","abstract","extends","implements",
        ];

        let mut items: Vec<Value> = keywords.iter()
            .filter(|k| k.starts_with(prefix))
            .map(|k| json!({ "label": k, "kind": 14 }))
            .collect();

        // Add symbols from all open docs
        for (_, (_, symbols)) in &self.docs {
            for sym in symbols {
                if sym.name.to_lowercase().starts_with(&prefix.to_lowercase()) {
                    let kind = match sym.kind {
                        12 => 3,  // Function
                        5  => 7,  // Class
                        23 => 22, // Struct
                        6  => 2,  // Method
                        8  => 5,  // Field
                        13 => 6,  // Variable
                        _  => 1,
                    };
                    items.push(json!({
                        "label": sym.name,
                        "kind": kind,
                        "detail": sym.detail,
                        "documentation": {
                            "kind": "markdown",
                            "value": format!("```prady\n{}\n```", sym.detail)
                        }
                    }));
                }
            }
        }

        items.dedup_by(|a, b| a["label"] == b["label"]);
        items
    }
}

// ─── Main loop ───────────────────────────────────────────────────────────────

fn main() {
    let stdin = io::stdin();
    let mut server = Server::new();
    let mut buf = String::new();

    loop {
        // Read headers
        let mut content_length: usize = 0;
        buf.clear();
        loop {
            buf.clear();
            if stdin.lock().read_line(&mut buf).unwrap_or(0) == 0 {
                return; // EOF
            }
            let trimmed = buf.trim_end_matches(|c| c == '\r' || c == '\n');
            if trimmed.is_empty() { break; }
            if let Some(rest) = trimmed.strip_prefix("Content-Length: ") {
                content_length = rest.trim().parse().unwrap_or(0);
            }
        }

        if content_length == 0 { continue; }

        // Read body
        let mut body = vec![0u8; content_length];
        use std::io::Read;
        if stdin.lock().read_exact(&mut body).is_err() { return; }

        let body_str = match std::str::from_utf8(&body) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let msg: RpcMessage = match serde_json::from_str(body_str) {
            Ok(m) => m,
            Err(_) => continue,
        };

        server.handle(msg);
    }
}
