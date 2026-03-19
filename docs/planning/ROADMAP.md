# UniLang — Development Roadmap

**Version:** 1.0.0-draft
**Last Updated:** 2026-03-19

---

## Phase Overview

```
Phase 0: Foundation        ███████████░░░░░░░░░  [Current]
Phase 1: Core Compiler     ░░░░░░░░░░░░░░░░░░░░
Phase 2: Runtime & Interop ░░░░░░░░░░░░░░░░░░░░
Phase 3: Toolchain         ░░░░░░░░░░░░░░░░░░░░
Phase 4: IDE & Ecosystem   ░░░░░░░░░░░░░░░░░░░░
Phase 5: Production Ready  ░░░░░░░░░░░░░░░░░░░░
```

---

## Phase 0: Foundation (Months 1–2)

**Goal:** Establish project infrastructure, governance, and core design documents.

| Task | Status | Owner | Notes |
|------|--------|-------|-------|
| Project repository setup | ✅ Done | Core Team | Directory structure, CI, license |
| PRD and vision document | ✅ Done | Core Team | |
| Architecture design document | ✅ Done | Core Team | |
| Language specification (draft) | ✅ Done | Core Team | |
| Design decisions document | ✅ Done | Core Team | |
| Contributing guidelines | ✅ Done | Core Team | |
| Apache License 2.0 setup | ✅ Done | Core Team | LICENSE, NOTICE, headers |
| Formal grammar (EBNF) | 🔲 TODO | | Full grammar file |
| Build system setup (Cargo) | 🔲 TODO | | Rust workspace config |
| CI/CD pipeline | 🔲 TODO | | GitHub Actions |
| Community infrastructure | 🔲 TODO | | Discussions, issue templates |
| Apache incubation proposal draft | 🔲 TODO | | |

**Exit Criteria:** All design docs reviewed, build system compiles empty project, CI green.

---

## Phase 1: Core Compiler (Months 3–6)

**Goal:** Build the compiler frontend — lexer, parser, and semantic analyzer that handle both Python and Java syntax.

### Milestone 1.1: Lexer (Month 3)

| Task | Status | Notes |
|------|--------|-------|
| Unified token type definitions | 🔲 TODO | Union of Python + Java tokens |
| Python tokenization rules | 🔲 TODO | Including indentation tracking |
| Java tokenization rules | 🔲 TODO | Including brace tracking |
| Automatic semicolon insertion | 🔲 TODO | ASI rules |
| String literal handling (all types) | 🔲 TODO | f-strings, raw, triple-quoted |
| Comment handling (all styles) | 🔲 TODO | `//`, `#`, `/* */`, `"""` |
| Lexer error recovery | 🔲 TODO | |
| Lexer test suite (>95% coverage) | 🔲 TODO | |

### Milestone 1.2: Parser (Months 3–5)

| Task | Status | Notes |
|------|--------|-------|
| Unified AST node definitions | 🔲 TODO | |
| Context-stack implementation | 🔲 TODO | Syntax context tracking |
| Expression parser (Pratt) | 🔲 TODO | Handles both precedence systems |
| Statement parser | 🔲 TODO | |
| Block parser (indent + brace) | 🔲 TODO | Four block styles |
| Class declaration parser | 🔲 TODO | Both Python and Java styles |
| Function declaration parser | 🔲 TODO | Both styles |
| Import statement parser | 🔲 TODO | Both styles |
| Control flow parser | 🔲 TODO | All loop/conditional types |
| Exception handling parser | 🔲 TODO | try/except + try/catch |
| Ambiguity resolution rules | 🔲 TODO | Priority-based disambiguation |
| Parser error recovery | 🔲 TODO | Continue parsing after errors |
| Parser test suite (>95% coverage) | 🔲 TODO | |

### Milestone 1.3: Semantic Analyzer (Months 5–6)

| Task | Status | Notes |
|------|--------|-------|
| Symbol table implementation | 🔲 TODO | Cross-syntax scoping |
| Type inference engine | 🔲 TODO | Bidirectional inference |
| Unified type hierarchy | 🔲 TODO | Python ↔ Java type mapping |
| Import resolution | 🔲 TODO | PyPI vs Maven detection |
| Overload resolution | 🔲 TODO | |
| Generic type checking | 🔲 TODO | |
| Interop type validation | 🔲 TODO | Cross-VM type compatibility |
| Semantic error reporting | 🔲 TODO | Context-aware messages |
| Semantic analyzer tests | 🔲 TODO | |

**Exit Criteria:** Compiler parses and type-checks a 500-line mixed-syntax program without errors.

---

## Phase 2: Runtime & Interop (Months 6–10)

**Goal:** Build the dual-VM runtime, bridge layer, and code generation backends.

### Milestone 2.1: UIR (Month 6–7)

| Task | Status | Notes |
|------|--------|-------|
| UIR instruction set design | 🔲 TODO | SSA-based |
| AST → UIR lowering | 🔲 TODO | |
| UIR printer/dumper | 🔲 TODO | For debugging |
| UIR verification pass | 🔲 TODO | Validates well-formedness |
| Basic optimization passes | 🔲 TODO | DCE, constant folding |

### Milestone 2.2: JVM Backend (Months 7–8)

| Task | Status | Notes |
|------|--------|-------|
| UIR → JVM bytecode generation | 🔲 TODO | |
| Class file writer | 🔲 TODO | |
| Method compilation | 🔲 TODO | |
| Exception table generation | 🔲 TODO | |
| JVM target tests | 🔲 TODO | Verify with `javap` |

### Milestone 2.3: Python Backend (Month 8–9)

| Task | Status | Notes |
|------|--------|-------|
| UIR → Python AST generation | 🔲 TODO | |
| .pyc file writer | 🔲 TODO | |
| C-extension interop layer | 🔲 TODO | NumPy, TF, PyTorch |
| Python target tests | 🔲 TODO | |

### Milestone 2.4: Bridge Layer (Months 9–10)

| Task | Status | Notes |
|------|--------|-------|
| JNI bridge implementation | 🔲 TODO | |
| CPython C API bridge | 🔲 TODO | |
| Type marshaling (all types) | 🔲 TODO | |
| Zero-copy array bridge | 🔲 TODO | NumPy ↔ Java arrays |
| Proxy object implementation | 🔲 TODO | |
| Cross-VM exception handling | 🔲 TODO | |
| Bridge performance benchmarks | 🔲 TODO | Target: <1μs per call |

### Milestone 2.5: Threading (Month 10)

| Task | Status | Notes |
|------|--------|-------|
| Thread manager implementation | 🔲 TODO | |
| Subinterpreter pool | 🔲 TODO | PEP 684 |
| Synchronized block support | 🔲 TODO | Cross-VM locks |
| Thread safety tests | 🔲 TODO | Race condition detection |
| Threading benchmarks | 🔲 TODO | vs native Java threading |

**Exit Criteria:** End-to-end execution of a program that trains a scikit-learn model from a Java thread pool.

---

## Phase 3: Toolchain (Months 10–13)

**Goal:** Build the developer-facing CLI tools.

| Task | Status | Notes |
|------|--------|-------|
| `unilang build` command | 🔲 TODO | Full compilation pipeline |
| `unilang run` command | 🔲 TODO | Compile + execute |
| `unilang test` command | 🔲 TODO | Pytest + JUnit style |
| `unilang fmt` command | 🔲 TODO | Source formatter |
| `unilang lint` command | 🔲 TODO | Static analysis |
| `unilang deps` command | 🔲 TODO | Unified dependency management |
| `unilang init` command | 🔲 TODO | Project scaffolding |
| `unilang.toml` parser | 🔲 TODO | Configuration |
| `unilang.lock` generator | 🔲 TODO | Reproducible builds |
| Artifact packaging (.uniL) | 🔲 TODO | ZIP-based archive |
| Incremental compilation | 🔲 TODO | File hash tracking |
| Parallel compilation | 🔲 TODO | Multi-module |

**Exit Criteria:** A developer can `unilang init`, write code, `unilang build`, and `unilang run` a project from scratch.

---

## Phase 4: IDE & Ecosystem (Months 13–16)

**Goal:** IDE support, documentation, and community resources.

### VS Code Extension

| Task | Status | Notes |
|------|--------|-------|
| Syntax highlighting (TextMate grammar) | 🔲 TODO | |
| Basic autocomplete | 🔲 TODO | |
| Error diagnostics | 🔲 TODO | |
| Go to definition | 🔲 TODO | |
| Language Server Protocol (LSP) | 🔲 TODO | |
| Hover documentation | 🔲 TODO | |
| Code formatting integration | 🔲 TODO | |
| Debugging support (DAP) | 🔲 TODO | |
| Snippet library | 🔲 TODO | |

### Documentation & Examples

| Task | Status | Notes |
|------|--------|-------|
| Language tutorial | 🔲 TODO | Getting started guide |
| API reference | 🔲 TODO | Standard library docs |
| Example: ML pipeline | 🔲 TODO | Showcase project |
| Example: Web service | 🔲 TODO | Spring Boot + Python |
| Example: Data processing | 🔲 TODO | Multi-threaded ETL |
| Cookbook / recipes | 🔲 TODO | Common patterns |
| Migration guide (Java → UniLang) | 🔲 TODO | |
| Migration guide (Python → UniLang) | 🔲 TODO | |

### Custom IDE (Research Phase)

| Task | Status | Notes |
|------|--------|-------|
| Evaluate VS Code OSS fork viability | 🔲 TODO | License compatibility |
| Evaluate building on Eclipse Theia | 🔲 TODO | |
| Evaluate JetBrains plugin | 🔲 TODO | IntelliJ Platform |
| Decision: custom IDE strategy | 🔲 TODO | Fork vs plugin vs new |

**Exit Criteria:** VS Code extension published on marketplace; 3 complete example projects.

---

## Phase 5: Production Ready (Months 16–20)

**Goal:** Stability, performance, and Apache incubation.

| Task | Status | Notes |
|------|--------|-------|
| Performance benchmarks suite | 🔲 TODO | |
| Stress testing (large projects) | 🔲 TODO | 100K+ LOC |
| Security audit | 🔲 TODO | |
| Memory leak testing | 🔲 TODO | Long-running processes |
| Cross-platform testing | 🔲 TODO | Linux, macOS, Windows |
| Release candidate process | 🔲 TODO | Semantic versioning |
| Apache incubation submission | 🔲 TODO | |
| Public announcement / launch | 🔲 TODO | Blog post, HN, Reddit |
| Conference talk preparation | 🔲 TODO | |

**Exit Criteria:** v1.0.0 release published; Apache incubation proposal submitted.

---

## Long-term Vision (Post v1.0)

| Feature | Target Version | Notes |
|---------|---------------|-------|
| REPL / interactive mode | v1.1 | |
| Jupyter kernel | v1.2 | |
| GraalVM AOT compilation | v1.3 | Single native binary |
| Custom IDE | v2.0 | Based on Phase 4 research |
| Debug Adapter Protocol | v1.1 | |
| Package registry | v2.0 | `unilang.dev` |
| Cloud deployment tools | v2.0 | |
| WebAssembly target | v2.x | |
| Additional language support | v3.0 | JavaScript, TypeScript |

---

*Dates are estimates and will be adjusted based on community capacity and priorities.*
