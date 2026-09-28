# Prady Programming Language

> **Native, statically typed, safe, and architecture-aware general-purpose programming language.**

```
CLI: prady   |   Extension: .pr   |   Compiler: Rust + LLVM
```

---

## 🌟 Vision

**Prady** (`.pr`) is designed for engineers who demand:
- **Clean, readable syntax**: Intuitive blocks, explicit expressions, and modern ergonomics.
- **Native performance**: Compiling to native machine code via LLVM without GC overhead.
- **Safety by default**: Immutability by default, explicit `mut`, explicit `Option<T>` / `Result<T, E>`, and no unhandled null pointers.
- **Architecture as Code**: Native `architecture` declarations, layer dependency validation, and SOLID quality engine heuristics built directly into the toolchain.
- **Complete DSA library**: Production-grade collection and algorithm implementations with documented time & space complexities.
- **Server and Systems grade**: Native async/await, first-class HTTP, JSON, WebAssembly (WASI), and controlled FFI.

---

## 🚀 Quick Start

### Building the Compiler
Prady compiler is built with Rust (workspace edition 2021+):

```bash
cargo build --release
```

The resulting `prady` CLI binary is available at `target/release/prady`.

### Checking a Prady File
```bash
# Verify syntax, AST, and report rich compiler diagnostics
cargo run -p prady-cli -- check examples/hello.pr

# Inspect Abstract Syntax Tree (AST)
cargo run -p prady-cli -- ast examples/hello.pr

# Inspect Lexer Tokens
cargo run -p prady-cli -- tokens examples/hello.pr
```

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

### Architecture as Code
```prady
// Declare and enforce Clean Architecture at compile time
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
│   └── prady-cli/                # 'prady' developer tool and command runner
├── examples/                     # Verified .pr source examples
├── tests/                        # Comprehensive integration & snapshot test suites
└── docs/                         # Specification & architecture documentation
```

---

## 🗺️ Roadmap
- [x] **Phase 1**: Workspace, Lexer, AST, Parser, Diagnostics Engine, CLI (`prady`), Test harness.
- [ ] **Phase 2**: Name Resolution, Primitive Types, Functions, Structs, and Basic Evaluation.
- [ ] **Phase 3**: LLVM Native Code Generation & Executable Linking.
- [ ] **Phase 4**: Enums, Pattern Matching, Generics, Option/Result, Module system.
- [ ] **Phase 5**: Complete DSA & Collections standard library with complexity guarantees.
- [ ] **Phase 6**: OOP, Interfaces, Traits, and Architecture/Quality Policy Engine.
- [ ] **Phase 7**: Async Runtime, Networking, HTTP, JSON, and Server Templates.
- [ ] **Phase 8**: Package Manager, Registry, Formatter, and Linter.
- [ ] **Phase 9**: Language Server Protocol (LSP) and VS Code Extension.
- [ ] **Phase 10**: WebAssembly (WASI), FFI, Security Hardening, and Performance Suite.
- [ ] **Phase 11**: Stabilization and 1.0 Release.

---

## 📄 License
Prady is dual-licensed under the **MIT** or **Apache-2.0** license.
