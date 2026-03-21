# Development Setup

This guide walks you through setting up a development environment for contributing to the UniLang compiler, runtime, and tooling.

---

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.75+ | Compiler development |
| **Cargo** | Latest (comes with Rust) | Rust build system |
| **JDK** | 21+ | JVM runtime and testing |
| **Python** | 3.11+ | Python runtime and testing |
| **Node.js** | 20+ | VS Code extension and UniLang IDE development |
| **Git** | 2.40+ | Version control |
| **Make** | Any | Build orchestration |

### Installing Prerequisites

**Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version    # Should show 1.75+
```

**JDK 21+ (macOS):**
```bash
brew install openjdk@21
```

**JDK 21+ (Ubuntu/Debian):**
```bash
sudo apt install openjdk-21-jdk
```

**Node.js 20+:**
```bash
# Using nvm (recommended)
nvm install 20
nvm use 20
```

---

## Clone and Build

```bash
# 1. Fork the repository on GitHub, then clone your fork
git clone https://github.com/<your-username>/unilang.git
cd unilang

# 2. Add the upstream remote
git remote add upstream https://github.com/hiteshkumar2187/unilang.git

# 3. Install development dependencies
make dev-setup

# 4. Build the project
cargo build --release

# 5. Verify the build
./target/release/unilang-cli --version
```

---

## Run Tests

```bash
# Run the entire test suite
make test
# Or directly:
cargo test --workspace

# Run tests for a specific crate
cargo test -p unilang-lexer
cargo test -p unilang-parser
cargo test -p unilang-semantic
cargo test -p unilang-codegen
cargo test -p unilang-runtime
cargo test -p unilang-stdlib

# Run tests with output visible
cargo test -p unilang-runtime -- --nocapture

# Run a specific test by name
cargo test -p unilang-lexer test_f_string_tokenization
```

---

## Project Structure

```
unilang/
|-- Cargo.toml              # Workspace manifest
|-- Makefile                 # Build orchestration
|-- unilang.toml             # Project configuration
|
|-- crates/
|   |-- unilang-common/      # Shared types: Span, SourceFile, DiagnosticBag
|   |-- unilang-lexer/       # Tokenizer (Python + Java tokens, indentation)
|   |-- unilang-parser/      # Parser (Pratt expressions + recursive descent)
|   |-- unilang-semantic/    # Semantic analysis (types, scopes, names)
|   |-- unilang-codegen/     # Code generation (AST -> bytecode)
|   |-- unilang-runtime/     # Stack-based VM interpreter
|   |-- unilang-stdlib/      # 35+ built-in functions
|   |-- unilang-cli/         # CLI tool (run, check, compile, lex)
|   |-- unilang-lsp/         # Language Server Protocol server
|
|-- src/                     # Main entry point
|-- tests/                   # Integration and end-to-end tests
|-- examples/                # Example programs
|   |-- ml-framework/        # Neural network framework
|   |-- library-mgmt/        # Full-stack library management app
|
|-- tools/
|   |-- vscode-extension/    # VS Code / Cursor extension
|   |-- jetbrains-plugin/    # IntelliJ / PyCharm plugin
|   |-- eclipse-plugin/      # Eclipse plugin
|   |-- unilang-ide/         # Standalone IDE (Electron)
|
|-- docs/
|   |-- architecture/        # System architecture, compiler pipeline
|   |-- design/              # Design decisions, type system, interop guide
|   |-- guides/              # Installation, quickstart, IDE setup
|   |-- specifications/      # Language specification
|   |-- planning/            # PRD, roadmap, governance
|   |-- contributing/        # Code of conduct
|
|-- config/                  # Configuration files
```

---

## How to Run Individual Crates

Each crate in the `crates/` directory is an independent Rust library. You can work on them individually:

### Lexer

```bash
# Run lexer tests
cargo test -p unilang-lexer

# Test tokenizing a specific file
cargo run -p unilang-cli -- lex examples/basic/hello.uniL
```

### Parser

```bash
# Run parser tests
cargo test -p unilang-parser

# The CLI does not expose a parse-only command, but tests cover parsing
```

### Semantic Analyzer

```bash
# Run semantic analysis tests
cargo test -p unilang-semantic

# Check a file for errors (runs lexer + parser + semantic)
cargo run -p unilang-cli -- check examples/basic/hello.uniL
```

### Code Generator

```bash
# Run codegen tests
cargo test -p unilang-codegen

# Compile and show bytecode
cargo run -p unilang-cli -- compile examples/basic/hello.uniL
```

### Runtime

```bash
# Run runtime tests
cargo test -p unilang-runtime

# Run a full program (all stages)
cargo run -p unilang-cli -- run examples/basic/hello.uniL
```

### Standard Library

```bash
# Run stdlib tests
cargo test -p unilang-stdlib
```

---

## Development Commands

```bash
# Build
make build              # Full release build
make build-debug        # Debug build (faster, with debug symbols)
cargo build             # Debug build via Cargo directly

# Test
make test               # All tests
make test-lexer         # Just lexer tests
make test-parser        # Just parser tests
make test-integration   # Integration tests

# Code Quality
make fmt                # Format code with rustfmt
make lint               # Run clippy
make ci                 # Full CI check: fmt + lint + test + build

# Benchmarks
make bench              # Run performance benchmarks
```

---

## Common Development Tasks

### Adding a New Token to the Lexer

1. Add the token variant to `TokenKind` in `crates/unilang-lexer/src/token.rs`
2. Add the recognition logic in `crates/unilang-lexer/src/lexer.rs`
3. Add tests in `crates/unilang-lexer/src/tests/`

### Adding a New AST Node

1. Add the node type in `crates/unilang-parser/src/ast.rs`
2. Add parsing logic in the appropriate parser method
3. Handle the node in `crates/unilang-semantic/` and `crates/unilang-codegen/`
4. Add tests at each stage

### Adding a New Built-in Function

1. Implement the function in `crates/unilang-stdlib/src/`
2. Register it in the function table
3. Add tests in `crates/unilang-stdlib/src/tests/`
4. Update the [[Standard Library Reference]] wiki page

### Adding a New Opcode

1. Add the opcode to the enum in `crates/unilang-codegen/`
2. Emit the opcode in the code generator
3. Handle execution in `crates/unilang-runtime/`
4. Add tests for both compilation and execution

---

## Tips

- Run `cargo clippy -- -D warnings` before committing to catch issues early.
- The compiler must never panic on user code. Use `Result<T, E>` for error handling.
- Each compiler phase should be independently testable.
- When in doubt, write a `.uniL` test file and run it through the full pipeline.

---

**Previous**: [[How to Contribute]] | **Next**: [[Home]]
