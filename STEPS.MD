# Prady — Master Guide

How to turn the current `Programming Language/` folder into: a language people can install like Node.js, a VS Code extension, a website with downloads, and (later) web, backend and mobile frameworks.

Written from an inspection of your uploaded zip. Everything in Part 1 was checked against your actual files.

---

## Part 1 — What you have today (audit)

### 1.1 What exists and works

| Piece | Location | State |
|---|---|---|
| Lexer | `compiler/prady-lexer` (780 lines) | Done, has tests |
| AST | `compiler/prady-ast` (500 lines) | Done |
| Parser | `compiler/prady-parser` (1251 lines) | Done, has tests. Pratt expressions + recursive descent |
| Diagnostics | `compiler/prady-diagnostics` | Done, colored errors with line/column |
| CLI `prady` | `compiler/prady-cli` | Commands: `check ast tokens run new init test doctor version` |
| VS Code extension | `editors/vscode-prady` | Syntax colors, snippets, bracket rules, problem matcher only |
| Website | `docs/` (index.html, app.js, style.css, prady-engine.js) | Static site: docs pages, install guide, live playground |
| Examples | `examples/*.pry` | 4 files |

### 1.2 What does NOT exist yet (important)

| You asked for | Reality in the zip |
|---|---|
| Run programs like Node.js | **`prady run` does not execute anything.** It lexes, parses, checks that `main` exists, then prints "Program verified cleanly". There is no interpreter, VM, or code generator in the Rust project. |
| Language runs on the website | The playground uses a **separate JavaScript re-implementation** (`docs/prady-engine.js`, 1146 lines) with its own lexer, parser and evaluator. It is a second copy of the language, not the real compiler. |
| Install with no setup | The install page previously had a placeholder repo. Now updated to `https://github.com/technopradyumn/prady`. |
| Type checker, modules, standard library | Not started (Roadmap Phase 2, 4, 5 unchecked) |
| VS Code auto-completion, hover, go-to-definition | Not present. Needs a language server (Roadmap Phase 9 unchecked) |
| Language icon | Not present (no icon files in the extension) |
| Mobile SDK | Not started, not in roadmap |
| Web frontend framework | Not started, not in roadmap |
| Backend framework | Not started. Roadmap Phase 7 mentions an HTTP server only |
| Extension `.pr` | Code, extension and website all use **`.pry`**, not `.pr` |

### 1.3 Verified behavior of the examples

I ran your four examples through the website's JS engine under Node:

| Example | Result |
|---|---|
| `hello.pry` | Runs. Prints `Hello Pradyumn`, `Age: 25`, `Result of add(10, 20): 30` |
| `architecture.pry` | Runs |
| `patterns.pry` | **Fails**: `match` and `=>` are not accepted by that engine |
| `oop.pry` | **Fails**: class fields (`gateway: PaymentGateway;`) and methods are not accepted |

So the Rust parser and the JS parser do not agree, which is the exact problem of keeping two implementations.

I could not build the Rust workspace here because the sandbox has no Rust toolchain, so I have not confirmed that `cargo test` passes. Run it yourself first (see 6.1).

**Bottom line:** you have a solid *front-end* (Phase 1 of 11 by your own roadmap). The language cannot yet run real programs outside the website playground. Everything below is ordered so each step produces something users can actually try.

---

## Part 2 — Decisions to make now

### 2.1 File extension: `.pr` or `.pry`
You told me `.pr`; the code says `.pry`. Pick one, then change it in: `prady-cli` messages and file checks, `package.json` (`extensions`), `README`, website (`index.html`, `app.js`), examples, and `prady new` templates. Do this first; it gets more expensive with every file.

### 2.2 How Prady programs will execute
This is the most important design choice. Options:

| Option | What it is | Pros | Cons |
|---|---|---|---|
| A. Tree-walking interpreter in Rust | Walk the AST and run it | Fastest way to a real `prady run` (days to weeks) | Slow programs |
| B. Bytecode VM in Rust | Compile to bytecode, run in a VM | Much faster than A, one runtime for CLI/server/embedding, ships as one binary | More work |
| C. Transpile to JavaScript | Prady → JS | Web frontend, and Node backend for free. Existing browser and Node ecosystems | Depends on Node for backend |
| D. LLVM native (your roadmap) | Real native binaries | Best performance | Very large effort. Not a first step |

**Recommendation:** A now, then B. Add C as a second backend specifically for the web framework. Keep LLVM (D) as a long-term goal; do not start there.

### 2.3 Where the frameworks get their power
A new language has no ecosystem, so each framework should sit on top of something that exists:

| Framework | Practical foundation |
|---|---|
| Web frontend UI | Prady → JS/DOM (option C), with a small reactive runtime |
| Backend | Rust runtime exposing HTTP/JSON/DB natives (e.g. built on `hyper`/`axum`) to Prady code |
| Mobile SDK | Hardest. Realistic route: Prady → JS + a React Native/Expo host app that ships your runtime. Native Swift/Kotlin bindings are a later step |

Be aware: web, backend and mobile frameworks are each a multi-month project. The order in Part 5 puts them last on purpose.

---

## Part 3 — Target architecture (split into separate products)

Right now everything is one folder. Split into a monorepo with clear products that release independently:

```
prady/                          (one Git repo, monorepo)
├── Cargo.toml                  workspace
├── compiler/
│   ├── prady-diagnostics/      (exists)
│   ├── prady-lexer/            (exists)
│   ├── prady-ast/              (exists)
│   ├── prady-parser/           (exists)
│   ├── prady-sema/             NEW: name resolution + type checker
│   ├── prady-interp/           NEW: interpreter (later prady-vm)
│   ├── prady-codegen-js/       NEW: Prady → JS backend
│   ├── prady-lsp/              NEW: language server
│   └── prady-cli/              (exists) the `prady` binary
├── std/                        NEW: standard library written in Prady + native hooks
├── frameworks/
│   ├── prady-web/              web UI framework
│   ├── prady-server/           backend framework
│   └── prady-mobile/           mobile SDK + host app
├── editors/
│   └── vscode-prady/           (exists) + icons + LSP client
├── website/                    (currently docs/) deployed to Vercel
├── examples/
├── docs/                       Markdown docs source (language spec, guides)
└── .github/workflows/          CI + release pipelines
```

Each product = its own version and installable artifact:

| Product | Artifact users get |
|---|---|
| Toolchain (`prady`, includes lsp) | Installer per OS (see Part 4) |
| VS Code extension | `.vsix` on VS Code Marketplace + Open VSX |
| Web/server/mobile frameworks | Packages installed with `prady add` (needs package manager, Roadmap Phase 8) |
| Website | Vercel deployment |

---

## Part 4 — Making it installable "like Node.js"

Goal: user downloads one file, double-clicks (or runs one line), then `prady run app.pr` works in any terminal with nothing else installed.

### 4.1 Build release binaries
Your compiler is Rust, which is perfect: it produces one self-contained executable per platform. Targets:

| OS | Target triple |
|---|---|
| Windows x64 | `x86_64-pc-windows-msvc` |
| macOS Intel / Apple Silicon | `x86_64-apple-darwin`, `aarch64-apple-darwin` |
| Linux x64 / ARM | `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` |

Use **GitHub Actions** with a build matrix, triggered by a Git tag like `v0.2.0`. The easiest path is **`cargo-dist`**, which generates the workflow, archives, checksums and installer scripts for you.

### 4.2 Installers users will see

| OS | Installer | Notes |
|---|---|---|
| Windows | `.msi` (via `cargo-wix`) or Inno Setup `.exe` | Must add `prady` to PATH. Sign it later to avoid SmartScreen warnings |
| macOS | `.pkg` | Notarization needs an Apple Developer account. Until then, offer the one-line script |
| Linux | `.tar.gz` + `install.sh`, later `.deb`/`.rpm` | |
| All | One-liners | `curl -fsSL https://YOURSITE/install.sh \| sh` and `irm https://YOURSITE/install.ps1 \| iex` (cargo-dist can generate these) |

### 4.3 What the installer bundles
- `prady` executable (includes `run`, `check`, `lsp`, `new`, `test`, `fmt` later)
- The standard library (embedded in the binary, so nothing to locate)
- Optional step: install the VS Code extension with `code --install-extension technopradyumn.prady-lang` if `code` is on PATH
- `prady --version`, `prady doctor` to verify

### 4.4 Hosting downloads
Put binaries on **GitHub Releases** (free, reliable). The website reads a small `latest.json` (version + asset URLs) and shows the right button for the visitor's OS. Vercel should host the site, not the big binaries.

### 4.5 Acceptance test
On a clean Windows VM, a clean macOS user and a clean Ubuntu container with no Rust installed: install, open a terminal, run `prady run hello.pr`. If it prints output, you have matched the Node.js experience.

---

## Part 5 — Step-by-step build plan

Each phase ends with something demonstrable. Do not start the next phase until the acceptance line passes.

### Phase 0 — Clean up (1–2 days)
1. Decide extension (2.1) and rename everywhere.
2. Set up the real GitHub repo under https://github.com/technopradyumn/prady.
3. Run `cargo test` and fix failures. Commit.
4. Add `.gitignore` with `target/` (your zip contained ~50 MB of build output).
5. Add a CI workflow that runs `cargo test` and `cargo clippy`.

**Accept:** CI is green on a fresh clone.

### Phase 1 — Make `prady run` real (1–3 weeks)
1. New crate `prady-interp`: environment, values (Int, Float, Bool, String, List, function, struct, enum), evaluator for `fn`, `let`, `let mut`, `if`, `while`, `for`, `return`, calls, operators.
2. Add `match`, `enum`, `struct`, `class` execution (the syntax already parses in Rust).
3. Built-ins: `print`, string/number conversion, basic list/map operations.
4. Wire it into `prady run`.
5. Port your `.pry` examples to be executed by tests (expected stdout files).

**Accept:** `hello`, `patterns` and `oop` examples all print the right output from the Rust CLI, not the JS engine.

### Phase 2 — Type checking and errors (2–4 weeks)
Roadmap Phase 2 and 4 items: symbol tables, primitive types, function signatures, `Option`/`Result`, `?` operator, modules (`import`/`export`).

**Accept:** a program with `let x: Int = "hi"` gives a clear compile error with line, column and a hint.

### Phase 3 — One implementation for website and CLI
Replace `docs/prady-engine.js` by compiling the Rust interpreter to **WebAssembly** (`wasm-pack`), and load that in the playground. Then the website, CLI and (later) LSP all use the same lexer/parser/interpreter, and the examples can never disagree again.

**Accept:** playground output is identical to `prady run` for every example.

### Phase 4 — Release and install (1 week)
Part 4 in full: GitHub Actions, cargo-dist, installers, `latest.json`, download page pointing at real files.

**Accept:** the clean-machine test in 4.5.

### Phase 5 — VS Code extension, complete (2–4 weeks)
See Part 6. Ends with Marketplace publishing.

### Phase 6 — Standard library and package manager (4+ weeks)
`std`: strings, math, collections, file IO, JSON, time, HTTP client. Then `prady add`, `prady.toml` dependencies, lockfile, registry (start with Git URLs or a GitHub-based index before building a registry service).

### Phase 7 — Web frontend framework (`prady-web`)
1. Add the JS code generator (option C).
2. Define a component model in Prady syntax (for example components as functions returning a view tree, with reactive state).
3. Build a small runtime: virtual DOM or fine-grained signals, event handling, routing.
4. `prady new web myapp`, `prady dev` (dev server with hot reload), `prady build` (static output for Vercel).

**Accept:** a to-do app written entirely in Prady, running in a browser.

### Phase 8 — Backend framework (`prady-server`)
Rust native HTTP server exposed to Prady: routes, middleware, JSON, environment config, database drivers (start with SQLite/Postgres). Then the async runtime from your roadmap. Your Clean Architecture `architecture { layer ... }` feature makes a good template (`prady new api --arch clean`, which the CLI already scaffolds).

**Accept:** REST API in Prady with 3 routes and persistence, started by `prady serve`.

### Phase 9 — Mobile SDK (`prady-mobile`)
Start only after Phases 7 and 8 work. Recommended route: Prady → JS, hosted in a React Native/Expo shell that you maintain, with a Prady component library mapped to native views. Provide `prady new mobile myapp`, `prady run --android|--ios`. Native (no JS) is a separate, much later project.

**Accept:** a "hello" app on an Android emulator and an iOS simulator, built from Prady source.

---

## Part 6 — VS Code extension: what to add

### 6.1 Already present
`syntaxes/prady.tmLanguage.json` (colors), `snippets/prady.json`, `language-configuration.json` (brackets, comments), problem matchers, `tasks.json`.

### 6.2 Missing, in priority order

1. **Icons.**
   - Extension icon: `"icon": "icon.png"` (128×128 PNG) in `package.json`.
   - File icon for `.pr` files: contribute `"iconThemes"` (a file icon theme) or `"languages[].icon": {"light": ..., "dark": ...}`. Note: language-level icons show in some places (tabs, Command Palette language picker); full Explorer file icons require an icon theme, and users must select it. That's a VS Code limitation, not something you can force.
2. **Language server (LSP)** for auto-completion, hover, go-to-definition, live errors, rename, formatting.
   - Write `prady-lsp` in Rust with the `tower-lsp` crate; it reuses your lexer, parser and (later) type checker, so it needs Phase 2 for good completions.
   - Expose it as `prady lsp` so one binary does everything.
   - In the extension add a small TypeScript client (`vscode-languageclient`) that starts `prady lsp` over stdio.
   - Start with: diagnostics (you already have them), keyword and snippet completion, document symbols. Add type-aware features after the type checker exists.
3. **Commands:** Run File, Check File, Format File, with a play button (`editor/title/run` menu).
4. **Bundled binary or PATH check:** the extension should detect `prady` on PATH and offer to open the download page if missing.
5. **Packaging:** `npm i -g @vscode/vsce`, then `vsce package` creates the `.vsix`; publish with `vsce publish` (Marketplace publisher account) and `ovsx publish` (Open VSX, which Antigravity-style forks use).
6. Add a `LICENSE`, changelog, and screenshots to the extension README.

**Accept:** open a `.pr` file in a fresh VS Code with only the extension and installer applied: colors, icon, completion, red squiggles, and Run button all work.

---

## Part 7 — Website and hosting on Vercel

The current `docs/` folder is a plain static site, so Vercel is straightforward.

### 7.1 Deploy
1. Move `docs/` to `website/` (or keep the name).
2. Push the repo to GitHub, import it in Vercel, set **Root Directory** to `website` and framework preset **Other** (no build command, output directory `.`).
3. Add your domain later; localhost testing keeps working with any static server (`npx serve website`).

### 7.2 Fix before going public
- Replace every placeholder GitHub URL.
- Make the Download section read `latest.json` and link to real release assets per OS. Right now it only shows `cargo install`, which requires Rust.
- Replace `prady-engine.js` with the WASM build (Phase 3).
- Add a proper docs section from Markdown (language tour, standard library reference, CLI reference, each framework). A static generator such as Docusaurus, VitePress or Astro Starlight gives search and versioned docs; all deploy to Vercel.
- Add a `/playground`, `/download`, `/docs`, `/blog` structure and a changelog page.

---

## Part 8 — What "the language" needs for real developers

Beyond the phases above, budget for these; they are what makes a language usable:
- **Formatter** (`prady fmt`) and **linter** (`prady lint`), which your roadmap already lists
- **Debugger** (DAP), later
- **Tests** as a first-class command (`prady test` exists as a stub)
- **Language specification** (grammar, semantics) so behavior is defined by a document, not by whatever the code does
- **Versioning and stability policy** before 1.0
- **Security:** signed releases, checksums, pinned dependencies

---

## Part 9 — Prompts to give Antigravity (one per phase)

Antigravity produced this project, so keep it on a short leash. Give it one phase at a time, with tests as the acceptance criteria, and require it to run them. Template:

> Context: this is the Prady language monorepo (Rust workspace). Current state: lexer, parser, AST, diagnostics and CLI exist; `prady run` only parses. Task: implement `<PHASE TASK>`. Rules: no mocked output, no fake implementations; add unit tests and an end-to-end test using `examples/*.pr` with expected-output files; run `cargo test` and `cargo clippy` and show me the results; do not touch other crates unless required; update `docs/` and `ROADMAP.md` checkboxes only for what actually passes.

Phase 1 example: *"Create crate `prady-interp` that evaluates the AST produced by `prady-parser`. Support functions, let/let mut, if/while/for, return, all operators, structs, enums, match, and classes with methods. Wire it into `prady run`. Tests: hello, patterns and oop examples must print the expected output."*

Your own review checklist for anything it generates: does it run from the real CLI, do tests exist and pass, and does the website use the same code?

---

## Part 10 — Suggested order for the next two weeks

1. Decide `.pr` vs `.pry`, create the Git repo, add `.gitignore`, run `cargo test`.
2. Split the repo into the layout in Part 3 (only create folders you will use now).
3. Build `prady-interp` so `prady run hello.pr` prints real output.
4. Fix the `patterns` and `oop` examples end-to-end.
5. Set up GitHub Actions to build a Windows `.zip` of `prady.exe`, and attach it to a release. Test it on a machine without Rust.
6. Deploy `docs/` to Vercel with a working download button.

After that: type checker, WASM playground, VS Code LSP, then the frameworks.

---

## Appendix — Language syntax that parses today (Rust parser)

```prady
// functions and variables
fn add(a: Int, b: Int) -> Int { return a + b; }
fn main() {
    let name = "Pradyumn";
    let mut counter = 0;
    counter += 1;
    print("Hello " + name);
}

// enums, match, ? operator
enum Status { Pending, Active(Int), Completed(String), Failed(String) }
fn describe(s: Status) -> String {
    let m = match s { Pending => "waiting", Active(id) => "running", Completed(x) => x, Failed(e) => e };
    return m;
}
fn loadProfile(id: Int) -> Result<String, Error> { let d = fetchRecord(id)?; return Ok(d); }

// interfaces and classes
interface PaymentGateway { fn charge(amount: Money) -> Result<Receipt, PaymentError>; }
class CheckoutService {
    gateway: PaymentGateway;
    fn new(gateway: PaymentGateway) -> CheckoutService { return CheckoutService { gateway }; }
}

// architecture as code
architecture backend {
    layer presentation; layer application; layer domain; layer infrastructure;
    presentation -> application; application -> domain; infrastructure -> domain;
    domain cannot import presentation;
}
```

Keywords recognized by the JS engine: `fn let mut return if else while for in architecture layer spec contract cannot import class interface struct enum match true false null`. Types named in docs: `Int8/16/32/64`, `UInt8/16/32/64`, `Int`, `UInt`, `Float32/64`, `Bool`, `Char`, `String`, `Unit`, `Never` (none are type-checked yet).