<div align="center">

<img src="prady-logo.svg" width="96" alt="Prady Logo" />

# Prady Programming Language

**Native · Statically Typed · Safe · Architecture-Aware**

[![Version](https://img.shields.io/badge/version-1.0.0%20GA-blue?style=flat-square)](https://github.com/technopradyumn/prady/releases/tag/v1.0.0)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green?style=flat-square)](LICENSE)
[![Website](https://img.shields.io/badge/website-pradylang.vercel.app-indigo?style=flat-square)](https://pradylang.vercel.app)

[🌐 Website](https://pradylang.vercel.app) &nbsp;|&nbsp;
[📖 Docs](https://pradylang.vercel.app/docs) &nbsp;|&nbsp;
[▶ Playground](https://pradylang.vercel.app/play) &nbsp;|&nbsp;
[⬇ Download](https://pradylang.vercel.app/download) &nbsp;|&nbsp;
[🛠️ Error Guide](https://pradylang.vercel.app/errors) &nbsp;|&nbsp;
[📦 Releases](https://github.com/technopradyumn/prady/releases)

</div>

---

## 🌟 Vision

**Prady** (`.pr`) is designed for engineers who demand:

- **Clean, readable syntax** — Intuitive blocks, explicit expressions, and modern ergonomics.
- **Native performance** — Compiles to native machine code via LLVM without GC overhead.
- **Safety by default** — Immutability by default, explicit `mut`, `Option<T>` / `Result<T, E>`, and no unhandled null pointers.
- **Architecture as Code** — Native `architecture` declarations, layer dependency validation, and SOLID quality engine heuristics built into the toolchain.
- **Complete DSA library** — 28 production-grade data structures with documented time & space complexities.
- **Server and Systems grade** — Native async/await, first-class HTTP, JSON, WebAssembly (WASI), and controlled FFI.

---

## 🚀 Quick Install

### One-Line Terminal Installation (Recommended)

**macOS & Linux** (Bash / Zsh):
```bash
curl -fsSL https://raw.githubusercontent.com/technopradyumn/prady/main/install.sh | sh
```

**Windows** (PowerShell):
```powershell
irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex
```

**Windows** (Command Prompt / CMD):
```cmd
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex"
```

The installer downloads the latest Windows x64 release, installs both `prady` and `prady-lsp` to `%USERPROFILE%\.prady\bin`, and adds that folder to your **user** `PATH` (no administrator rights or Rust installation needed). Open a new terminal after installation, then verify and run a program:

```powershell
prady version
prady run .\hello.pr
```

In Command Prompt, use `prady run hello.pr`. If you installed from a release archive manually, extract it and run `prady.exe` from that folder, or add the folder containing it to your user `PATH`.

> Full installation guide: **[pradylang.vercel.app/docs/getting-started/installation](https://pradylang.vercel.app/docs/getting-started/installation)**

### Download Standalone Binaries

Get pre-built native binaries from the official download page:

| Platform | Link |
|---|---|
| 🪟 Windows (x64) | [pradylang.vercel.app/download](https://pradylang.vercel.app/download) |
| 🍎 macOS Apple Silicon | [pradylang.vercel.app/download](https://pradylang.vercel.app/download) |
| 🍎 macOS Intel | [pradylang.vercel.app/download](https://pradylang.vercel.app/download) |
| 🐧 Linux (x64) | [pradylang.vercel.app/download](https://pradylang.vercel.app/download) |
| 🐧 Linux (ARM64) | [pradylang.vercel.app/download](https://pradylang.vercel.app/download) |

> **Windows users:** If SmartScreen blocks the installer, click **"More info" → "Run anyway"**. The binary is safe — it's unsigned only because it's new.

### Build from Source

```bash
# 1. Clone the official repository
git clone https://github.com/technopradyumn/prady.git
cd prady

# 2. Build optimized release binaries
cargo build --release

# 3. Install globally via Cargo (cargo build alone does not add prady to PATH)
cargo install --path compiler/prady-cli
cargo install --path compiler/prady-lsp
```

After `cargo build --release`, you can also run the compiler directly from the repository with `.\target\release\prady.exe run .\examples\hello.pr` in PowerShell. `cargo install` or the one-line installer is what makes `prady` available as a command from any folder.

---

## ▶ Try It Online

No installation needed — run Prady code directly in your browser:

**[pradylang.vercel.app/play](https://pradylang.vercel.app/play)**

Features: Live execution · Token inspector · AST viewer · Syntax highlighting

---

## 📝 Syntax Example

```prady
// examples/hello.pr
fn add(a: Int, b: Int) -> Int {
    return a + b;
}

fn main() {
    let name = "Pradyumn";
    let age: Int = 25;
    let mut counter = 0;

    counter = counter + 1;
    print("Hello " + name);
    print("Sum: " + add(10, 20));
}
```

### Reading Program Input

`input()` reads one line from the terminal. An optional prompt is printed before it waits:

```prady
fn main() {
    let name = input("Name: ");
    print("Hello " + name);
}
```

Run it with `prady run path/to/main.pr`, then type a line and press Enter. The VS Code **Prady: Run File** command uses an interactive terminal too.

### Architecture as Code

```prady
// Enforce Clean Architecture at compile time
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
```

> Learn more: **[pradylang.vercel.app/docs/handbook/architecture-contracts](https://pradylang.vercel.app/docs/handbook/architecture-contracts)**

---

## 📚 Documentation

Full documentation is available at **[pradylang.vercel.app/docs](https://pradylang.vercel.app/docs)**

### Getting Started
| Page | URL |
|---|---|
| 🚀 Overview & Philosophy | [/docs/getting-started/overview](https://pradylang.vercel.app/docs/getting-started/overview) |
| ⚡ Installation & Binaries | [/docs/getting-started/installation](https://pradylang.vercel.app/docs/getting-started/installation) |
| 💻 Hello World Tutorial | [/docs/getting-started/hello-world](https://pradylang.vercel.app/docs/getting-started/hello-world) |

### The Handbook
| Page | URL |
|---|---|
| 📖 The Basics | [/docs/handbook/the-basics](https://pradylang.vercel.app/docs/handbook/the-basics) |
| 🧩 Everyday Types | [/docs/handbook/everyday-types](https://pradylang.vercel.app/docs/handbook/everyday-types) |
| 🔄 Control Flow & Matching | [/docs/handbook/control-flow](https://pradylang.vercel.app/docs/handbook/control-flow) |
| λ Functions & Lambdas | [/docs/handbook/functions-and-lambdas](https://pradylang.vercel.app/docs/handbook/functions-and-lambdas) |
| 🏛️ Architecture Contracts | [/docs/handbook/architecture-contracts](https://pradylang.vercel.app/docs/handbook/architecture-contracts) |

### Language Reference
| Page | URL |
|---|---|
| 🔤 Keywords & Grammar | [/docs/reference/keywords-and-syntax](https://pradylang.vercel.app/docs/reference/keywords-and-syntax) |
| 🏷️ Built-In Primitive Types | [/docs/reference/built-in-types](https://pradylang.vercel.app/docs/reference/built-in-types) |
| 🌲 All 28 Data Structures | [/docs/reference/data-structures](https://pradylang.vercel.app/docs/reference/data-structures) |
| 📚 Standard Library | [/docs/reference/standard-library](https://pradylang.vercel.app/docs/reference/standard-library) |

### Tooling & IDE
| Page | URL |
|---|---|
| ⌨️ CLI Reference | [/docs/tooling/cli-reference](https://pradylang.vercel.app/docs/tooling/cli-reference) |
| 📦 Package Manager | [/docs/tooling/package-manager](https://pradylang.vercel.app/docs/tooling/package-manager) |
| 💡 VS Code & LSP | [/docs/tooling/editor-extensions](https://pradylang.vercel.app/docs/tooling/editor-extensions) |

---

## 🛠️ CLI Reference

```bash
prady version              # Show compiler version
prady run examples/hello.pr   # Run a .pr file
prady check examples/hello.pr # Type-check and report diagnostics
prady ast examples/hello.pr   # Print the Abstract Syntax Tree
prady tokens examples/hello.pr # Print tokenizer output
prady new my-project          # Scaffold a new Prady project
prady build                   # Compile to native binary
```

> Full CLI reference: **[pradylang.vercel.app/docs/tooling/cli-reference](https://pradylang.vercel.app/docs/tooling/cli-reference)**

---

## 💡 VS Code Extension

Install full IDE support for `.pr` files:

**[pradylang.vercel.app/docs/tooling/editor-extensions](https://pradylang.vercel.app/docs/tooling/editor-extensions)**

**Repository:** [github.com/technopradyumn/vscode-prady](https://github.com/technopradyumn/vscode-prady)

Features:
- 🎨 Syntax highlighting for all keywords, types, architecture blocks
- ⚡ `Ctrl+Alt+N` to run `.pr` files instantly
- 🔍 Live error diagnostics with precise source spans
- 💡 Snippets for `fn`, `main`, `let`, `arch`, `struct`, `@test`
- 🔵 LSP: hover types, go-to-definition, auto-complete

---

## 📦 Workspace Structure

```
├── Cargo.toml                    # Root Rust workspace definition
├── prady.toml                    # Language configuration & quality thresholds
├── compiler/
│   ├── prady-diagnostics/        # Spans, error codes, colorized source reporting
│   ├── prady-ast/                # Abstract Syntax Tree nodes & visitor
│   ├── prady-lexer/              # Tokenizer with full keyword & literal support
│   ├── prady-parser/             # Recursive descent & Pratt expression parser
│   ├── prady-interp/             # Tree-walk interpreter & runtime
│   ├── prady-cli/                # 'prady' developer tool and command runner
│   └── prady-lsp/                # Language Server Protocol implementation
├── examples/                     # Verified .pr source examples
└── install.sh / install.ps1      # One-line system installers
```

---

## 🗺️ Roadmap

- [x] **Phase 1** — Workspace, Lexer, AST, Parser, Diagnostics Engine, CLI, Test harness
- [x] **Phase 2** — Name Resolution, Primitive Types, Functions, Structs, and Interpreter
- [x] **Phase 9** — Language Server Protocol (LSP) and VS Code Extension
- [ ] **Phase 3** — LLVM Native Code Generation & Executable Linking
- [ ] **Phase 4** — Enums, Pattern Matching, Generics, Option/Result, Module system
- [ ] **Phase 5** — Complete DSA & Collections standard library with complexity guarantees
- [ ] **Phase 6** — OOP, Interfaces, Traits, and Architecture/Quality Policy Engine
- [ ] **Phase 7** — Async Runtime, Networking, HTTP, JSON, and Server Templates
- [ ] **Phase 8** — Package Manager, Registry, Formatter, and Linter
- [ ] **Phase 10** — WebAssembly (WASI), FFI, Security Hardening, and Performance Suite
- [ ] **Phase 11** — Stabilization and stable release

---

## 🔗 Links

| Resource | URL |
|---|---|
| 🌐 Official Website | https://pradylang.vercel.app |
| 📖 Documentation | https://pradylang.vercel.app/docs |
| ▶ Playground | https://pradylang.vercel.app/play |
| ⬇ Downloads | https://pradylang.vercel.app/download |
| 🐙 GitHub (Compiler) | https://github.com/technopradyumn/prady |
| 🔌 GitHub (VS Code Ext.) | https://github.com/technopradyumn/vscode-prady |
| 🐛 Issues | https://github.com/technopradyumn/prady/issues |
| 📦 Releases | https://github.com/technopradyumn/prady/releases |

---

## 📄 License

Prady is dual-licensed under the **MIT** or **Apache-2.0** license. See [LICENSE](LICENSE) for details.
