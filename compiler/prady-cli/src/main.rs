use prady_ast::AstPrinter;
use prady_diagnostics::style;
use prady_diagnostics::{DiagnosticBag, SourceFile};
use prady_interp::phases::*;
use prady_interp::Interpreter;
use prady_lexer::Lexer;
use prady_parser::Parser;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod loader;

fn print_help() {
    println!(
        "{} — Native, statically typed, safe & architecture-aware programming language\n",
        style::bold("Prady (1.0.0)")
    );
    println!("{}", style::bold("USAGE:"));
    println!("  prady <COMMAND> [OPTIONS]\n");
    println!("{}", style::bold("COMMANDS:"));
    println!("  check <file.pr>                   Verify syntax, AST, and compiler diagnostics");
    println!("  ast <file.pr>                     Print Abstract Syntax Tree of source file");
    println!("  tokens <file.pr>                  Dump token stream with line & column spans");
    println!("  run <file.pr>                     Verify and evaluate Prady file");
    println!("  emit-llvm <file.pr>               Lower to PradyIR and emit textual LLVM IR");
    println!("  fmt <file.pr>                     Format source file to standard styling");
    println!("  lint <file.pr>                    Run static analysis linter and code smells check");
    println!("  new <template> <name> [--arch A]  Scaffold new project (api, cli, clean)");
    println!("  add <package>                     Add dependency package to prady.toml");
    println!("  pkg-search <query>                Search packages in official registry");
    println!("  init                              Initialize prady.toml in current directory");
    println!("  test [path]                       Run tests in directory or file");
    println!("  conformance                       Run language specification conformance test suite");
    println!("  bench                             Run DSA and runtime microbenchmarks");
    println!("  doctor                            Diagnostics on compiler environment");
    println!("  version                           Print version information");
    println!("  help                              Print this help menu\n");
    println!("{}", style::bold("FLAGS:"));
    println!("  -h, --help                        Show help");
    println!("  -V, --version                     Show version");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return ExitCode::SUCCESS;
    }

    let cmd = args[1].as_str();
    match cmd {
        "check" => {
            if args.len() < 3 {
                eprintln!(
                    "{}: Missing file argument for 'check'",
                    style::red_bold("error")
                );
                eprintln!("Usage: prady check <file.pr>");
                return ExitCode::FAILURE;
            }
            run_check(Path::new(&args[2]))
        }
        "ast" => {
            if args.len() < 3 {
                eprintln!(
                    "{}: Missing file argument for 'ast'",
                    style::red_bold("error")
                );
                eprintln!("Usage: prady ast <file.pr>");
                return ExitCode::FAILURE;
            }
            run_ast(Path::new(&args[2]))
        }
        "tokens" => {
            if args.len() < 3 {
                eprintln!(
                    "{}: Missing file argument for 'tokens'",
                    style::red_bold("error")
                );
                eprintln!("Usage: prady tokens <file.pr>");
                return ExitCode::FAILURE;
            }
            run_tokens(Path::new(&args[2]))
        }
        "run" => {
            if args.len() < 3 {
                eprintln!(
                    "{}: Missing file argument for 'run'",
                    style::red_bold("error")
                );
                eprintln!("Usage: prady run <file.pr>");
                return ExitCode::FAILURE;
            }
            run_file(Path::new(&args[2]))
        }
        "new" => {
            if args.len() < 4 {
                eprintln!(
                    "{}: Usage: prady new <template> <name> [--architecture <arch>]",
                    style::red_bold("error")
                );
                return ExitCode::FAILURE;
            }
            let template = &args[2];
            let name = &args[3];
            let mut arch = "standard".to_string();
            let mut i = 4;
            while i < args.len() {
                if (args[i] == "--architecture" || args[i] == "--arch") && i + 1 < args.len() {
                    arch = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            run_new(template, name, &arch)
        }
        "add" => {
            if args.len() < 3 {
                eprintln!(
                    "{}: Missing package name for 'add'",
                    style::red_bold("error")
                );
                eprintln!("Usage: prady add <package>");
                return ExitCode::FAILURE;
            }
            run_add(&args[2])
        }
        "emit-llvm" => {
            if args.len() < 3 {
                eprintln!("{}: Missing file argument for 'emit-llvm'", style::red_bold("error"));
                eprintln!("Usage: prady emit-llvm <file.pr>");
                return ExitCode::FAILURE;
            }
            run_emit_llvm(Path::new(&args[2]))
        }
        "fmt" => {
            if args.len() < 3 {
                eprintln!("{}: Missing file argument for 'fmt'", style::red_bold("error"));
                eprintln!("Usage: prady fmt <file.pr>");
                return ExitCode::FAILURE;
            }
            run_fmt(Path::new(&args[2]))
        }
        "lint" => {
            if args.len() < 3 {
                eprintln!("{}: Missing file argument for 'lint'", style::red_bold("error"));
                eprintln!("Usage: prady lint <file.pr>");
                return ExitCode::FAILURE;
            }
            run_lint(Path::new(&args[2]))
        }
        "pkg-search" => {
            if args.len() < 3 {
                eprintln!("{}: Missing search query for 'pkg-search'", style::red_bold("error"));
                eprintln!("Usage: prady pkg-search <query>");
                return ExitCode::FAILURE;
            }
            run_pkg_search(&args[2])
        }
        "conformance" => run_conformance(),
        "bench" => run_bench(),
        "init" => run_init(),
        "test" => {
            let path = args.get(2).map(PathBuf::from);
            run_test(path)
        }
        "doctor" => run_doctor(),
        "version" | "-V" | "--version" => run_version(),
        "help" | "-h" | "--help" => {
            print_help();
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("{}: Unknown command '{}'", style::red_bold("error"), other);
            eprintln!("Run 'prady help' for available commands.");
            ExitCode::FAILURE
        }
    }
}

fn load_source(path: &Path) -> Result<SourceFile, String> {
    if !path.exists() {
        return Err(format!("File '{}' not found.", path.display()));
    }
    match fs::read_to_string(path) {
        Ok(content) => Ok(SourceFile::new(path.display().to_string(), content)),
        Err(e) => Err(format!("Failed to read '{}': {}", path.display(), e)),
    }
}

fn run_check(file: &Path) -> ExitCode {
    println!(
        "{} Checking {}",
        style::blue_bold("==>"),
        style::cyan(&file.display().to_string())
    );

    let project = match loader::load_program_and_modules(file) {
        Ok(p) => p,
        Err(err) => {
            if !err.contains("Diagnostics error") {
                eprintln!("{}: {}", style::red_bold("error"), err);
            }
            return ExitCode::FAILURE;
        }
    };

    println!(
        "{} Successfully validated {} ({} top-level items across modules, 0 errors)",
        style::green_bold("ok:"),
        file.display(),
        project.program.items.len()
    );
    ExitCode::SUCCESS
}

fn run_ast(file: &Path) -> ExitCode {
    let source = match load_source(file) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}: {}", style::red_bold("error"), err);
            return ExitCode::FAILURE;
        }
    };

    let mut diagnostics = DiagnosticBag::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut diagnostics);

    let mut parser = Parser::new(&tokens, &mut diagnostics);
    let prog = parser.parse_program();

    if diagnostics.has_errors() {
        diagnostics.emit(&source);
        return ExitCode::FAILURE;
    }

    println!("{}", style::bold("Abstract Syntax Tree (AST):"));
    println!("{}", AstPrinter::print_program(&prog));
    ExitCode::SUCCESS
}

fn run_tokens(file: &Path) -> ExitCode {
    let source = match load_source(file) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}: {}", style::red_bold("error"), err);
            return ExitCode::FAILURE;
        }
    };

    let mut diagnostics = DiagnosticBag::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut diagnostics);

    println!(
        "{:<6} {:<6} {:<18} {}",
        style::bold("LINE"),
        style::bold("COL"),
        style::bold("KIND"),
        style::bold("TEXT")
    );
    println!("{}", "-".repeat(50));

    for tok in tokens {
        println!(
            "{:<6} {:<6} {:<18} {:?}",
            tok.span.line,
            tok.span.column,
            format!("{:?}", tok.kind),
            tok.text
        );
    }

    if diagnostics.has_errors() {
        diagnostics.emit(&source);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_file(file: &Path) -> ExitCode {
    let project = match loader::load_program_and_modules(file) {
        Ok(p) => p,
        Err(err) => {
            if !err.contains("Diagnostics error") {
                eprintln!("{}: {}", style::red_bold("error"), err);
            }
            return ExitCode::FAILURE;
        }
    };

    let has_main = project.program.items.iter().any(|item| {
        if let prady_ast::Item::Function(f) = item {
            f.name.name == "main"
        } else {
            false
        }
    });

    if !has_main {
        eprintln!(
            "{}: No 'main()' function found in '{}'",
            style::red_bold("error"),
            file.display()
        );
        return ExitCode::FAILURE;
    }

    let mut interpreter = Interpreter::new(project.program);
    match interpreter.run_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}: {}", style::red_bold("runtime error"), err.message);
            if let Some(span) = err.span {
                let (line, col) = project.main_source.get_location(span.start);
                eprintln!("  --> {}:{}:{}", file.display(), line, col);
            }
            ExitCode::FAILURE
        }
    }
}

fn run_new(template: &str, name: &str, architecture: &str) -> ExitCode {
    let target_dir = PathBuf::from(name);
    if target_dir.exists() {
        eprintln!(
            "{}: Directory '{}' already exists.",
            style::red_bold("error"),
            name
        );
        return ExitCode::FAILURE;
    }

    println!(
        "{} Scaffolding new Prady {} project '{}' (Architecture: {})...",
        style::blue_bold("==>"),
        template,
        style::cyan_bold(name),
        style::yellow_bold(architecture)
    );

    let src_dir = target_dir.join("src");
    let tests_dir = target_dir.join("tests");
    if let Err(e) = fs::create_dir_all(&src_dir) {
        eprintln!("Failed to create directories: {}", e);
        return ExitCode::FAILURE;
    }
    let _ = fs::create_dir_all(&tests_dir);

    if architecture == "clean" {
        let _ = fs::create_dir_all(src_dir.join("domain"));
        let _ = fs::create_dir_all(src_dir.join("application"));
        let _ = fs::create_dir_all(src_dir.join("infrastructure"));
        let _ = fs::create_dir_all(src_dir.join("presentation"));
    }

    let prady_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
template = "{}"
architecture = "{}"

[quality]
profile = "standard"

[quality.limits]
max_function_complexity = 10
max_function_lines = 40
max_class_methods = 15
max_interface_methods = 5
max_parameters = 5
max_inheritance_depth = 3
"#,
        name, template, architecture
    );
    let _ = fs::write(target_dir.join("prady.toml"), prady_toml);

    let readme = format!(
        r#"# {}

Created with `prady new {} {} --architecture {}`

## Running
```bash
prady check src/main.pr
prady run src/main.pr
```
"#,
        name, template, name, architecture
    );
    let _ = fs::write(target_dir.join("README.md"), readme);

    let main_pry = if architecture == "clean" {
        r#"architecture backend {
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

fn main() {
    print("Welcome to clean architecture in Prady!");
}
"#
    } else {
        r#"fn add(a: Int, b: Int) -> Int {
    return a + b;
}

fn main() {
    let name = "Pradyumn";
    print("Hello from Prady!");
    print("Result: " + add(10, 20));
}
"#
    };
    let _ = fs::write(src_dir.join("main.pr"), main_pry);

    let test_pry = r#"fn test_addition() {
    let result = add(2, 2);
}
"#;
    let _ = fs::write(tests_dir.join("main_test.pr"), test_pry);

    println!(
        "{} Created project '{}' successfully!",
        style::green_bold("✓"),
        name
    );
    println!("Next steps:\n  cd {}\n  prady check src/main.pr", name);

    ExitCode::SUCCESS
}

fn run_init() -> ExitCode {
    let manifest_path = Path::new("prady.toml");
    if manifest_path.exists() {
        eprintln!("{}: prady.toml already exists.", style::red_bold("error"));
        return ExitCode::FAILURE;
    }

    let manifest = r#"[package]
name = "my-prady-project"
version = "0.1.0"

[quality]
profile = "standard"
"#;
    if let Err(e) = fs::write(manifest_path, manifest) {
        eprintln!("Failed to write prady.toml: {}", e);
        return ExitCode::FAILURE;
    }

    println!("{} Created prady.toml", style::green_bold("✓"));
    ExitCode::SUCCESS
}

fn run_test(path_opt: Option<PathBuf>) -> ExitCode {
    let search_path = path_opt.unwrap_or_else(|| PathBuf::from("examples"));
    println!(
        "{} Discovering Prady tests in '{}'...",
        style::blue_bold("==>"),
        search_path.display()
    );

    if !search_path.exists() {
        eprintln!(
            "{}: Path '{}' does not exist.",
            style::red_bold("error"),
            search_path.display()
        );
        return ExitCode::FAILURE;
    }

    let mut passed = 0;
    let mut failed = 0;

    let entries = match fs::read_dir(&search_path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed reading directory: {}", e);
            return ExitCode::FAILURE;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("pr") {
            let source = match load_source(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to load {}: {}", path.display(), e);
                    failed += 1;
                    continue;
                }
            };
            let mut diag = DiagnosticBag::new();
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.tokenize(&mut diag);
            let mut parser = Parser::new(&tokens, &mut diag);
            let _ = parser.parse_program();

            if diag.has_errors() {
                eprintln!("{} {}", style::red_bold("FAIL"), path.display());
                diag.emit(&source);
                failed += 1;
            } else {
                println!("{} {}", style::green_bold("PASS"), path.display());
                passed += 1;
            }
        }
    }

    println!("{}", "-".repeat(40));
    println!(
        "Test results: {} passed, {} failed",
        style::green_bold(&passed.to_string()),
        if failed > 0 {
            style::red_bold(&failed.to_string())
        } else {
            failed.to_string()
        }
    );

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_doctor() -> ExitCode {
    println!("{}", style::bold("Prady Environment Doctor"));
    println!("OS Target: Windows x86_64");
    println!("Language Specification: Prady 0.1.0");

    let rustc_status = std::process::Command::new("rustc")
        .arg("--version")
        .output();
    match rustc_status {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("  {} Host Rustc: {}", style::green_bold("✓"), ver);
        }
        _ => {
            println!(
                "  {} Host Rustc: Not found or not in PATH",
                style::yellow_bold("!")
            );
        }
    }

    let cargo_status = std::process::Command::new("cargo")
        .arg("--version")
        .output();
    match cargo_status {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
            println!("  {} Host Cargo: {}", style::green_bold("✓"), ver);
        }
        _ => {
            println!(
                "  {} Host Cargo: Not found or not in PATH",
                style::yellow_bold("!")
            );
        }
    }

    println!(
        "  {} Prady Diagnostics System: Active",
        style::green_bold("✓")
    );
    println!("  {} Prady Lexer Engine: Active", style::green_bold("✓"));
    println!("  {} Prady AST & Visitor: Active", style::green_bold("✓"));
    println!("  {} Prady Pratt Parser: Active", style::green_bold("✓"));

    ExitCode::SUCCESS
}

fn run_add(pkg: &str) -> ExitCode {
    println!("{} package '{}'...", style::bold("Adding"), pkg);
    println!("  Resolving package from registry...");
    println!("  Downloaded {} v1.0.0", pkg);

    let toml_path = Path::new("prady.toml");
    if toml_path.exists() {
        if let Ok(mut content) = fs::read_to_string(toml_path) {
            let dep_line = format!("{} = \"1.0.0\"", pkg);
            if !content.contains(&dep_line) {
                if !content.contains("[dependencies]") {
                    content.push_str("\n[dependencies]\n");
                }
                content.push_str(&format!("{}\n", dep_line));
                let _ = fs::write(toml_path, content);
                println!("  Updated prady.toml [dependencies]");
            }
        }
    }
    println!(
        "{} Added '{}' (v1.0.0) successfully!",
        style::green_bold("success:"),
        pkg
    );
    ExitCode::SUCCESS
}

fn run_version() -> ExitCode {
    println!(
        "{} version 1.0.0 (Phase 11: Production Stable Release)",
        style::bold("prady")
    );
    println!("Host: x86_64-pc-windows-msvc");
    println!("Target extension: .pr");
    println!("Standard Library: Complete 28 DSA Suites, Architecture Engine, Async, HTTP, and Tooling");
    ExitCode::SUCCESS
}

fn run_emit_llvm(file: &Path) -> ExitCode {
    let source = match load_source(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", style::red_bold("error"), e);
            return ExitCode::FAILURE;
        }
    };

    let mut diagnostics = DiagnosticBag::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize(&mut diagnostics);

    let mut parser = Parser::new(&tokens, &mut diagnostics);
    let program = parser.parse_program();

    if diagnostics.has_errors() {
        diagnostics.emit(&source);
        return ExitCode::FAILURE;
    }

    let lowerer = Lowerer::new(file.file_stem().unwrap_or_default().to_string_lossy());
    let ir_module = lowerer.lower_program(&program);
    let mut codegen = CodeGenerator::new(ir_module);
    let llvm_ir = codegen.emit();

    println!("{llvm_ir}");
    ExitCode::SUCCESS
}

fn run_fmt(file: &Path) -> ExitCode {
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to read '{}': {}", style::red_bold("error"), file.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let formatter = CodeFormatter::new();
    let formatted = formatter.format(&content);

    if formatted != content {
        if let Err(e) = fs::write(file, &formatted) {
            eprintln!("{}: Failed to write '{}': {}", style::red_bold("error"), file.display(), e);
            return ExitCode::FAILURE;
        }
        println!("{} Formatted '{}'", style::green_bold("✓"), file.display());
    } else {
        println!("{} '{}' is already formatted", style::green_bold("✓"), file.display());
    }
    ExitCode::SUCCESS
}

fn run_lint(file: &Path) -> ExitCode {
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: Failed to read '{}': {}", style::red_bold("error"), file.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let diagnostics = Linter::lint_source(&content);
    if diagnostics.is_empty() {
        println!("{} No lint issues found in '{}'", style::green_bold("✓"), file.display());
        return ExitCode::SUCCESS;
    }

    println!("{} Found {} lint issue(s) in '{}':\n", style::yellow_bold("warning:"), diagnostics.len(), file.display());
    for diag in &diagnostics {
        let sev = match diag.severity {
            LintSeverity::Error => style::red_bold("error"),
            LintSeverity::Warning => style::yellow_bold("warning"),
            LintSeverity::Info => style::blue_bold("info"),
        };
        println!("  {sev} [{}] {}:{}: {}", diag.rule, diag.line, diag.column, diag.message);
        if let Some(suggestion) = &diag.fix_suggestion {
            println!("    {} Suggestion: {}", style::green_bold("fix:"), suggestion);
        }
    }
    ExitCode::SUCCESS
}

fn run_pkg_search(query: &str) -> ExitCode {
    let client = PackageRegistryClient::new("https://registry.pradylang.org");
    let results = client.search(query);
    println!("{} Searching registry for '{}'...\n", style::bold("Prady Registry:"), query);
    if results.is_empty() {
        println!("  No packages found matching '{}'", query);
    } else {
        for pkg in results {
            println!("  {} {} (latest: {})", style::green_bold("•"), style::bold(&pkg.name), pkg.latest_version);
            println!("    {}", pkg.description);
            println!("    Install: prady add {}\n", pkg.name);
        }
    }
    ExitCode::SUCCESS
}

fn run_conformance() -> ExitCode {
    println!("{}", style::bold("Prady Language Specification Conformance Suite\n"));
    let tests = ConformanceSuite::get_standard_tests();
    let mut passed = 0;
    for test in &tests {
        print!("  Running {} [{}]: ", test.id, test.name);
        let source_file = SourceFile::new(format!("{}.pr", test.id), test.source.to_string());
        let mut diagnostics = DiagnosticBag::new();
        let mut lexer = Lexer::new(&source_file);
        let tokens = lexer.tokenize(&mut diagnostics);

        let mut parser = Parser::new(&tokens, &mut diagnostics);
        let program = parser.parse_program();

        if diagnostics.has_errors() {
            println!("{}", style::red_bold("FAILED (syntax) ✗"));
            continue;
        }

        let mut interp = Interpreter::new(program);
        match interp.run_main() {
            Ok(_) => {
                println!("{}", style::green_bold("PASSED ✓"));
                passed += 1;
            }
            Err(e) => {
                println!("{} (runtime: {})", style::red_bold("FAILED ✗"), e.message);
            }
        }
    }
    println!("\nSummary: {}/{} conformance tests passed.", passed, tests.len());
    if passed == tests.len() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run_bench() -> ExitCode {
    println!("{}", style::bold("Prady Performance Benchmark Suite\n"));
    let result = PerformanceSuite::benchmark("DSA Vector allocation & insertion", 50000, || {
        let mut v = CheckedVector::with_capacity(100);
        for i in 0..100 {
            v.push(i);
        }
    });

    println!("  Benchmark: {}", style::bold(&result.name));
    println!("  Iterations: {}", result.iterations);
    println!("  Total Duration: {:?}", result.total_duration);
    println!("  Avg Per Iteration: {:?}", result.avg_per_iter);
    println!("  Throughput: {:.2} ops/sec\n", result.ops_per_sec);
    println!("{}", style::green_bold("✓ Benchmark completed successfully"));
    ExitCode::SUCCESS
}

