use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use prady_ast::{Item, Program};
use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_lexer::Lexer;
use prady_parser::Parser;

pub struct LoadedProject {
    pub main_source: SourceFile,
    pub program: Program,
}

pub fn load_program_and_modules(entry_file: &Path) -> Result<LoadedProject, String> {
    if !entry_file.exists() {
        return Err(format!("File '{}' not found.", entry_file.display()));
    }

    let canonical_entry = fs::canonicalize(entry_file).unwrap_or_else(|_| entry_file.to_path_buf());
    let entry_dir = canonical_entry
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    // 1. Read and parse entry file
    let entry_content = fs::read_to_string(&canonical_entry)
        .map_err(|e| format!("Failed to read '{}': {}", entry_file.display(), e))?;
    let main_source = SourceFile::new(entry_file.display().to_string(), entry_content);

    let mut diagnostics = DiagnosticBag::new();
    let mut lexer = Lexer::new(&main_source);
    let tokens = lexer.tokenize(&mut diagnostics);
    let mut parser = Parser::new(&tokens, &mut diagnostics);
    let mut combined_prog = parser.parse_program();

    if diagnostics.has_errors() {
        diagnostics.emit(&main_source);
        return Err("Diagnostics error in entry file".to_string());
    }

    // 2. Discover related files to auto-import:
    // Sibling .pr files in the same directory, plus explicit imports
    let mut visited: HashSet<PathBuf> = HashSet::new();
    visited.insert(canonical_entry.clone());

    let mut files_to_load: Vec<PathBuf> = Vec::new();

    // Check sibling files in the same directory
    collect_pr_files(&entry_dir, &mut files_to_load, &visited, 2);

    // Also look up if we are inside a project (e.g. prady.toml)
    if let Some(project_root) = find_project_root(&entry_dir) {
        let src_dir = project_root.join("src");
        if src_dir.exists() && src_dir != entry_dir {
            collect_pr_files(&src_dir, &mut files_to_load, &visited, 3);
        }
    }

    // Check explicit import statements in the entry program
    for item in &combined_prog.items {
        if let Item::Import(imp) = item {
            resolve_import(&entry_dir, &imp.path, &mut files_to_load, &visited);
        }
    }

    // 3. Load and merge all discovered files
    let mut i = 0;
    while i < files_to_load.len() {
        let file_path = files_to_load[i].clone();
        i += 1;

        if !visited.insert(file_path.clone()) {
            continue;
        }

        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let file_source = SourceFile::new(file_path.display().to_string(), content);
        let mut file_diag = DiagnosticBag::new();
        let mut file_lexer = Lexer::new(&file_source);
        let file_tokens = file_lexer.tokenize(&mut file_diag);
        let mut file_parser = Parser::new(&file_tokens, &mut file_diag);
        let file_prog = file_parser.parse_program();

        if file_diag.has_errors() {
            file_diag.emit(&file_source);
            continue;
        }

        // Merge declarations, ignoring sibling `main()`
        for item in file_prog.items {
            match &item {
                Item::Function(f) if f.name.name == "main" => {
                    // Do not overwrite entry file's main()
                    continue;
                }
                Item::Import(imp) => {
                    // Resolve nested imports
                    let cur_dir = file_path.parent().unwrap_or(Path::new("."));
                    resolve_import(cur_dir, &imp.path, &mut files_to_load, &visited);
                }
                _ => {
                    combined_prog.items.push(item);
                }
            }
        }
    }

    Ok(LoadedProject {
        main_source,
        program: combined_prog,
    })
}

fn collect_pr_files(
    dir: &Path,
    list: &mut Vec<PathBuf>,
    visited: &HashSet<PathBuf>,
    max_depth: usize,
) {
    if max_depth == 0 || !dir.is_dir() {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !name.starts_with('.') && name != "target" && name != "node_modules" {
                collect_pr_files(&path, list, visited, max_depth - 1);
            }
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("pr") {
            let canonical = fs::canonicalize(&path).unwrap_or(path);
            if !visited.contains(&canonical) {
                list.push(canonical);
            }
        }
    }
}

fn find_project_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = Some(start_dir);
    while let Some(dir) = current {
        if dir.join("prady.toml").exists() || dir.join(".git").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

fn resolve_import(
    base_dir: &Path,
    path_idents: &[prady_ast::Ident],
    list: &mut Vec<PathBuf>,
    visited: &HashSet<PathBuf>,
) {
    if path_idents.is_empty() {
        return;
    }

    let segments: Vec<&str> = path_idents.iter().map(|id| id.name.as_str()).collect();

    // 1. Base dir + segments.join("/") + ".pr"
    let mut file_path = base_dir.to_path_buf();
    for seg in &segments {
        file_path.push(seg);
    }
    file_path.set_extension("pr");

    if file_path.exists() {
        let canonical = fs::canonicalize(&file_path).unwrap_or(file_path);
        if !visited.contains(&canonical) {
            list.push(canonical);
        }
        return;
    }

    // 2. Base dir + "src" + segments.join("/") + ".pr"
    let mut src_path = base_dir.join("src");
    for seg in &segments {
        src_path.push(seg);
    }
    src_path.set_extension("pr");

    if src_path.exists() {
        let canonical = fs::canonicalize(&src_path).unwrap_or(src_path);
        if !visited.contains(&canonical) {
            list.push(canonical);
        }
    }
}
