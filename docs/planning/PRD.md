# UniLang — Product Requirements Document (PRD)

**Version:** 1.0.0-draft
**Status:** Draft
**Last Updated:** 2026-03-19
**Authors:** UniLang Core Team

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Problem Statement](#2-problem-statement)
3. [Vision and Goals](#3-vision-and-goals)
4. [Target Users](#4-target-users)
5. [User Stories](#5-user-stories)
6. [Functional Requirements](#6-functional-requirements)
7. [Non-Functional Requirements](#7-non-functional-requirements)
8. [Scope and Boundaries](#8-scope-and-boundaries)
9. [Success Metrics](#9-success-metrics)
10. [Risks and Mitigations](#10-risks-and-mitigations)
11. [Dependencies](#11-dependencies)
12. [Open Questions](#12-open-questions)

---

## 1. Executive Summary

UniLang is a new programming language that unifies Python and Java syntax into a single coherent language. Developers can write code using either Python or Java syntax — or both simultaneously — within a single `.uniL` source file. The compiler understands both grammars, resolves ambiguities through context analysis, and compiles to an intermediate representation that bridges the JVM and CPython runtimes.

The primary motivation is to allow **Java developers to leverage Python's ML/AI ecosystem** and **Python developers to access Java's enterprise concurrency and type system** — all without leaving their preferred syntax style.

---

## 2. Problem Statement

### 2.1 Current Pain Points

1. **Language Boundary Friction:** Teams building ML-powered enterprise applications must maintain separate Python (ML models) and Java (backend services) codebases, with brittle inter-process communication (REST, gRPC, message queues) bridging them.

2. **Skill Gap Overhead:** Java developers forced to learn Python for ML work (and vice versa) face productivity loss and cognitive overhead from context-switching between paradigms.

3. **Interoperability Complexity:** Existing solutions (Jython, GraalVM polyglot, Py4J, JPype) each have significant limitations:
   - **Jython:** Limited to Python 2.7, no CPython C-extension support (no NumPy/TensorFlow).
   - **GraalVM:** Requires GraalPython (incomplete CPython compatibility), complex setup.
   - **Py4J / JPype:** Bridge libraries with serialization overhead, no unified type system, separate process management.

4. **Deployment Fragmentation:** ML models trained in Python must be exported (ONNX, PMML, pickle) and loaded in Java services — a lossy, error-prone pipeline.

### 2.2 Opportunity

A language that natively understands both syntaxes and bridges both runtimes eliminates these boundaries. Developers write in their preferred style, the compiler handles interoperability, and the runtime manages dual-VM coordination transparently.

---

## 3. Vision and Goals

### 3.1 Vision Statement

> **UniLang makes the boundary between Python and Java disappear.** Write ML models in Python syntax, enterprise services in Java syntax, and let the compiler handle everything in between.

### 3.2 Primary Goals

| # | Goal | Measurable Outcome |
|---|------|--------------------|
| G1 | **Seamless syntax mixing** | Any valid Python 3.12 or Java 21 syntax is valid UniLang (within defined scope) |
| G2 | **Full Python library access** | NumPy, pandas, TensorFlow, PyTorch, scikit-learn work out of the box |
| G3 | **Full Java library access** | Any Maven/Gradle dependency can be imported and used |
| G4 | **Native multi-threading** | Java's `java.util.concurrent` works natively, including thread pools, futures, locks |
| G5 | **Zero-copy interop where possible** | Data passed between Python and Java contexts avoids serialization when feasible |
| G6 | **Flexible block syntax** | Braces `{}` are optional in Python-style code; indentation is optional in Java-style code |
| G7 | **Apache-grade open source** | Code quality, documentation, and governance meet Apache Software Foundation incubation requirements |

### 3.3 Non-Goals (Explicitly Out of Scope for v1)

- Replacing Python or Java as standalone languages
- Supporting languages beyond Python and Java (e.g., JavaScript, Rust)
- Native mobile development (Android/iOS)
- Backward compatibility with Python 2.x
- Full REPL/notebook support (planned for v2)

---

## 4. Target Users

### 4.1 Primary Personas

#### Persona 1: "Java Enterprise Developer — ML Curious"
- **Role:** Senior Java developer at a fintech company
- **Pain:** Needs to integrate ML models (fraud detection) but doesn't want to maintain a separate Python microservice
- **Goal:** Write training code in Python syntax directly within their Java service class
- **Technical Profile:** Expert Java, beginner Python, familiar with Maven/Gradle

#### Persona 2: "Python Data Scientist — Production Needs"
- **Role:** ML engineer at a healthtech startup
- **Pain:** Python prototypes work but need Java's concurrency and type safety for production deployment
- **Goal:** Wrap their Python ML pipeline in Java-style concurrent services without rewriting
- **Technical Profile:** Expert Python/ML, intermediate Java, familiar with pip/conda

#### Persona 3: "Full-Stack Polyglot — Efficiency Seeker"
- **Role:** Tech lead at a mid-size SaaS company
- **Pain:** Team has both Java and Python specialists; code reviews and onboarding are slow due to dual codebases
- **Goal:** Unify the codebase so everyone can contribute in their preferred style
- **Technical Profile:** Proficient in both Java and Python

### 4.2 Secondary Personas

- **Academic Researchers:** Need Python ML + Java simulation frameworks
- **Open Source Contributors:** Interested in compiler/language design
- **DevOps Engineers:** Need unified build/deploy pipelines

---

## 5. User Stories

### 5.1 Core User Stories

| ID | Story | Priority | Acceptance Criteria |
|----|-------|----------|-------------------|
| US-01 | As a Java developer, I want to call `sklearn.linear_model.LinearRegression` directly from my Java class so that I don't need a separate Python service. | P0 | Import resolves, model trains, predictions return as Java-compatible types |
| US-02 | As a Python developer, I want to use `ExecutorService` to parallelize my data processing so that I get real multi-threading (not GIL-limited). | P0 | Thread pool executes Python-defined tasks concurrently with true parallelism |
| US-03 | As a developer, I want to write a Python function inside a Java class without any special syntax markers so that the code reads naturally. | P0 | Parser correctly identifies syntax context and compiles without errors |
| US-04 | As a developer, I want to optionally use braces `{}` in Python-style code blocks so that I can use my preferred style. | P0 | Both `if x > 0 { print("yes") }` and `if x > 0:\n    print("yes")` compile identically |
| US-05 | As a developer, I want `.uniL` files to compile to a single deployable artifact so that deployment is simple. | P1 | `unilang build` produces a runnable artifact |
| US-06 | As a developer, I want IDE support (syntax highlighting, autocomplete, error detection) so that I can write UniLang productively. | P1 | VS Code extension provides basic language support |
| US-07 | As a team lead, I want a formatter and linter so that mixed-syntax code remains readable. | P2 | `unilang fmt` and `unilang lint` produce consistent output |
| US-08 | As a developer, I want to use Java annotations on Python-style functions so that frameworks like Spring can discover them. | P1 | `@RequestMapping` on a `def` function works correctly |
| US-09 | As a developer, I want to use Python decorators on Java-style methods so that I can apply cross-cutting concerns. | P1 | `@cache` on a `public void method()` works correctly |
| US-10 | As a developer, I want clear error messages that understand both syntaxes so that debugging is not harder than in either language alone. | P0 | Error messages reference the correct syntax context and suggest fixes |

### 5.2 Advanced User Stories

| ID | Story | Priority |
|----|-------|----------|
| US-11 | As a developer, I want to use Java generics with Python-defined classes | P1 |
| US-12 | As a developer, I want Python list comprehensions to work inside Java methods | P1 |
| US-13 | As a developer, I want to use Java's `synchronized` keyword with Python objects | P1 |
| US-14 | As a developer, I want Python generators to be usable as Java `Iterator<T>` | P2 |
| US-15 | As a developer, I want to use Python's `with` statement for Java `AutoCloseable` resources | P1 |

---

## 6. Functional Requirements

### 6.1 Language Features

#### 6.1.1 Syntax Support

| Feature | Python Syntax | Java Syntax | UniLang Behavior |
|---------|--------------|-------------|-----------------|
| Variable declaration | `x = 10` | `int x = 10;` | Both valid; type inferred for Python-style |
| Function definition | `def foo(x):` | `public int foo(int x) {}` | Both valid; return type inferred for Python-style |
| Class definition | `class Foo:` | `public class Foo {}` | Both valid; merged semantics |
| Conditionals | `if x > 0:` | `if (x > 0) {}` | Both valid; braces optional everywhere |
| Loops | `for x in range(10):` | `for (int i = 0; i < 10; i++) {}` | Both valid |
| Exception handling | `try: ... except:` | `try {} catch() {}` | Both valid; unified exception hierarchy |
| Lambda / closure | `lambda x: x + 1` | `(x) -> x + 1` | Both valid; interchangeable |
| String formatting | `f"Hello {name}"` | `"Hello " + name` or `String.format()` | All valid |
| Imports | `import numpy as np` | `import java.util.List;` | Resolved to correct ecosystem based on package |
| Multi-threading | N/A (GIL-limited) | `Thread`, `ExecutorService`, `synchronized` | Java threading model with Python code execution |
| Decorators / Annotations | `@decorator` | `@Annotation` | Unified; context determines behavior |
| Type hints | `def foo(x: int) -> str:` | `public String foo(int x)` | Unified type system with bidirectional mapping |

#### 6.1.2 Block Syntax Flexibility

The compiler supports **four block styles** — all semantically equivalent:

```unilang
// Style 1: Pure Python (indentation-based)
if x > 0:
    print("positive")

// Style 2: Python with optional braces
if x > 0: {
    print("positive")
}

// Style 3: Java style (braces required, no colon)
if (x > 0) {
    System.out.println("positive");
}

// Style 4: Hybrid (parens optional, braces optional)
if x > 0
    print("positive")
```

**Rule:** If an opening brace `{` is present, a closing brace `}` is required. If no brace is present, indentation determines the block scope (Python rules apply).

#### 6.1.3 Import Resolution

```unilang
// Python ecosystem imports — resolved via pip/conda
import numpy as np
from tensorflow.keras import layers

// Java ecosystem imports — resolved via Maven/Gradle
import java.util.concurrent.ExecutorService
import org.springframework.web.bind.annotation.RestController

// UniLang standard library
import unilang.bridge.convert
import unilang.runtime.ThreadPool
```

The compiler determines the target ecosystem by:
1. Checking the package name against known registries (PyPI vs Maven Central)
2. Checking local project dependencies (`unilang.toml` configuration)
3. Raising an error if ambiguous (user must qualify)

#### 6.1.4 Multi-Threading

UniLang provides **true multi-threading** by leveraging the JVM's threading model:

```unilang
import java.util.concurrent.Executors

// Python-style function running on Java threads
def process_data(chunk):
    result = heavy_computation(chunk)  # Runs without GIL limitation
    return result

executor = Executors.newFixedThreadPool(8)
futures = [executor.submit(lambda: process_data(chunk)) for chunk in data_chunks]
results = [f.get() for f in futures]
```

**Threading model:**
- Python code blocks that run on Java threads are compiled to JVM bytecode or executed via a GIL-free bridge
- Shared state between Python and Java contexts is protected by Java's memory model
- `synchronized`, `Lock`, `ReentrantLock`, `Semaphore` all work with Python objects

### 6.2 Compiler

| Requirement | Description |
|-------------|-------------|
| CR-01 | Lexer tokenizes both Python and Java token sets |
| CR-02 | Parser produces a unified AST from mixed-syntax input |
| CR-03 | Semantic analyzer performs type inference across both syntax styles |
| CR-04 | Code generator emits JVM bytecode and/or CPython-compatible code |
| CR-05 | Optimizer performs cross-language optimizations |
| CR-06 | Error reporter provides context-aware messages for both syntaxes |
| CR-07 | Incremental compilation for large projects |
| CR-08 | Source maps for debugging back to `.uniL` source |

### 6.3 Runtime

| Requirement | Description |
|-------------|-------------|
| RR-01 | Dual-VM runtime: JVM + CPython, managed by UniLang runtime |
| RR-02 | Zero-copy data bridge for primitive types and arrays |
| RR-03 | Automatic type marshaling for complex objects |
| RR-04 | Unified garbage collection coordination |
| RR-05 | Thread-safe Python ↔ Java object sharing |
| RR-06 | Unified exception propagation across VM boundaries |

### 6.4 Toolchain

| Requirement | Description |
|-------------|-------------|
| TR-01 | `unilang build` — compile `.uniL` files |
| TR-02 | `unilang run` — compile and execute |
| TR-03 | `unilang test` — run tests (supports both pytest and JUnit style) |
| TR-04 | `unilang fmt` — format source code |
| TR-05 | `unilang lint` — static analysis |
| TR-06 | `unilang deps` — dependency management (pip + Maven unified) |
| TR-07 | `unilang repl` — interactive shell (v2) |

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric | Target |
|--------|--------|
| Compilation speed | < 2s for 10K LOC project (incremental) |
| Runtime overhead vs native Java | < 15% for Java-style code |
| Runtime overhead vs native Python | < 20% for Python-style code |
| Interop call overhead | < 1ms for cross-VM function call |
| Memory overhead | < 30% above max(JVM, CPython) for equivalent code |

### 7.2 Reliability

- Compiler must never produce incorrect code silently
- Runtime must handle VM crashes gracefully (one VM failing does not kill the other)
- All error states must produce actionable error messages

### 7.3 Compatibility

| Dependency | Minimum Version |
|------------|----------------|
| Java | 21 (LTS) |
| Python | 3.11 |
| OS | Linux (x86_64, aarch64), macOS (aarch64), Windows (x86_64) |

### 7.4 Security

- No `eval()` or dynamic code execution by default
- Sandboxed execution mode available
- Dependency scanning integrated into `unilang deps`
- CVE reporting process established per Apache guidelines

### 7.5 Governance (Apache Foundation)

- Apache License 2.0
- NOTICE file with all attributions
- Contributor License Agreement (CLA) required
- Follows Apache release process
- Three-vote consensus for major decisions
- Public mailing list for all technical discussions

---

## 8. Scope and Boundaries

### 8.1 In Scope (v1.0)

- Core language syntax (Python 3.12 + Java 21 feature set)
- Compiler (lexer, parser, semantic analysis, code generation)
- Runtime (JVM + CPython bridge)
- CLI toolchain (`build`, `run`, `test`, `fmt`, `lint`, `deps`)
- VS Code extension (syntax highlighting, basic autocomplete)
- Standard library (type conversion, threading utilities, I/O bridge)
- Documentation (language spec, tutorials, API reference)
- Example projects (ML pipeline, web service, hybrid application)

### 8.2 Out of Scope (v1.0)

- Custom IDE (deferred to v2+)
- REPL / Jupyter kernel (deferred to v2)
- Native compilation (AOT via GraalVM — deferred to v2)
- Language Server Protocol (LSP) full implementation (basic in v1, full in v2)
- Package registry (use pip + Maven initially)
- Cloud deployment tooling

---

## 9. Success Metrics

| Metric | Target (12 months post-launch) |
|--------|-------------------------------|
| GitHub stars | 5,000+ |
| Monthly active developers | 500+ |
| Apache incubation acceptance | Yes |
| Published packages using UniLang | 50+ |
| Community contributors | 100+ |
| Performance benchmarks published | All core benchmarks passing |
| Documentation completeness | 100% API coverage |

---

## 10. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Syntax ambiguity between Python and Java | High | High | Context-aware parser with explicit disambiguation rules; comprehensive test suite |
| Performance overhead from dual-VM runtime | Medium | High | Zero-copy bridge for hot paths; AOT compilation path; benchmark-driven optimization |
| GIL limitations for Python code in threads | High | Medium | Compile Python-style code to JVM bytecode where possible; use subinterpreters (PEP 684) |
| Apache Foundation governance overhead | Low | Medium | Early engagement with Apache mentors; follow incubation playbook |
| Community adoption challenges | Medium | Medium | Strong documentation; compelling examples; conference talks |
| Dependency conflicts (pip vs Maven) | Medium | Medium | Isolated virtual environments; unified lock file |

---

## 11. Dependencies

### 11.1 Technical Dependencies

| Dependency | Purpose | Version |
|------------|---------|---------|
| Rust | Compiler implementation language | 1.75+ |
| LLVM | Optional native code generation | 17+ |
| JVM (OpenJDK) | Java runtime | 21+ |
| CPython | Python runtime | 3.11+ |
| Tree-sitter | Incremental parsing foundation | Latest |
| GraalVM (optional) | Polyglot runtime for advanced interop | 23+ |

### 11.2 Organizational Dependencies

- Apache Software Foundation incubation sponsor
- CLA infrastructure (Apache CLA system)
- CI/CD infrastructure (GitHub Actions or Apache Jenkins)

---

## 12. Open Questions

| # | Question | Status | Notes |
|---|----------|--------|-------|
| OQ-01 | Should the compiler be written in Rust, Java, or UniLang itself (bootstrapping)? | **Decision: Rust** | Performance, memory safety, no runtime dependency |
| OQ-02 | How to handle Python's significant whitespace when mixed with Java's brace-delimited blocks? | Open | See Design Decisions document |
| OQ-03 | Should semicolons be optional everywhere or only in Python-style code? | Open | Leaning: optional everywhere |
| OQ-04 | How to handle naming conflicts (e.g., `list` in Python vs `List` in Java)? | Open | Leaning: case-sensitive, both valid |
| OQ-05 | Should the runtime use JNI, JEP (Java-Python bridge), or a custom bridge? | Open | Needs prototyping |
| OQ-06 | How to handle Python's duck typing alongside Java's static typing? | Open | See Type System design |
| OQ-07 | Custom IDE vs VS Code extension priority? | **Decision: VS Code first** | Lower effort, broader reach |

---

## Appendix A: Glossary

| Term | Definition |
|------|-----------|
| **UIR** | UniLang Intermediate Representation — the compiler's internal code representation |
| **Bridge Layer** | Runtime component managing JVM ↔ CPython communication |
| **Syntax Context** | The parser's determination of whether a code block follows Python or Java grammar |
| **Zero-Copy Interop** | Passing data between VMs without serialization/deserialization |
| **GIL** | Global Interpreter Lock — CPython's thread safety mechanism that limits true parallelism |

---

*This document is a living artifact. Updates require review by at least two core maintainers.*
