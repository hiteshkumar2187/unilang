# UniLang — Design Decisions Document

**Version:** 1.0.0-draft
**Status:** Draft
**Last Updated:** 2026-03-19

---

## Table of Contents

1. [Decision Log](#1-decision-log)
2. [DD-001: Compiler Implementation Language](#dd-001-compiler-implementation-language)
3. [DD-002: Syntax Ambiguity Resolution](#dd-002-syntax-ambiguity-resolution)
4. [DD-003: Block Syntax Flexibility](#dd-003-block-syntax-flexibility)
5. [DD-004: Runtime Strategy (Dual-VM vs Single VM)](#dd-004-runtime-strategy)
6. [DD-005: Type System Design](#dd-005-type-system-design)
7. [DD-006: Threading and GIL Management](#dd-006-threading-and-gil-management)
8. [DD-007: Semicolons](#dd-007-semicolons)
9. [DD-008: Import Resolution Strategy](#dd-008-import-resolution-strategy)
10. [DD-009: File Extension](#dd-009-file-extension)
11. [DD-010: Intermediate Representation](#dd-010-intermediate-representation)

---

## 1. Decision Log

| ID | Decision | Status | Date |
|----|----------|--------|------|
| DD-001 | Compiler written in Rust | **Accepted** | 2026-03-19 |
| DD-002 | Context-stack parser for ambiguity resolution | **Accepted** | 2026-03-19 |
| DD-003 | Braces optional everywhere, indentation optional everywhere | **Accepted** | 2026-03-19 |
| DD-004 | Dual-VM runtime (JVM + CPython) with bridge | **Accepted** | 2026-03-19 |
| DD-005 | Gradual type system with unified type hierarchy | **Accepted** | 2026-03-19 |
| DD-006 | Subinterpreters (PEP 684) + JVM bytecode compilation | **Accepted** | 2026-03-19 |
| DD-007 | Semicolons optional everywhere | **Accepted** | 2026-03-19 |
| DD-008 | Package-name-based import resolution with registry lookup | **Accepted** | 2026-03-19 |
| DD-009 | `.uniL` file extension | **Accepted** | 2026-03-19 |
| DD-010 | SSA-based custom IR (UIR) | **Accepted** | 2026-03-19 |

---

## DD-001: Compiler Implementation Language

### Context
We need to choose the language for implementing the UniLang compiler itself. The compiler must be fast, memory-safe, and produce reliable output.

### Options Considered

| Option | Pros | Cons |
|--------|------|------|
| **Rust** | Fast, memory-safe, no GC pauses, excellent error handling, strong ecosystem (cranelift, tree-sitter) | Steeper learning curve, longer initial development time |
| **Java** | Team familiarity, JVM ecosystem, GraalVM integration | GC pauses during compilation, large runtime dependency |
| **C++** | Maximum performance, LLVM is C++ | Memory safety issues, slower development |
| **UniLang (bootstrap)** | Dogfooding, proves the language | Chicken-and-egg problem, delays v1 |

### Decision
**Rust** — The compiler is a performance-critical, long-running tool where memory safety and predictable performance are paramount. Rust's type system catches entire classes of bugs at compile time. The Rust ecosystem also provides excellent libraries for compiler construction.

### Consequences
- Contributors need Rust proficiency (or willingness to learn)
- No runtime dependency for the compiler itself
- Excellent cross-compilation for distributing the compiler

---

## DD-002: Syntax Ambiguity Resolution

### Context
Python and Java have overlapping but conflicting syntax. For example:
- `for` loops have entirely different syntax
- `class` declarations differ in inheritance syntax
- Type annotations go on different sides of the identifier
- `import` syntax differs

The parser must unambiguously determine the intended syntax.

### Options Considered

| Option | Pros | Cons |
|--------|------|------|
| **Explicit mode markers** (`#!python`, `#!java`) | Simple, unambiguous | Verbose, breaks the "seamless" promise |
| **Context-stack parser** | Seamless mixing, natural reading | Complex parser, potential edge cases |
| **Two-pass parsing** (try Python, fallback to Java) | Simpler implementation | Inconsistent behavior, poor error messages |
| **Unified grammar** (superset grammar) | Single parse path | Overly complex grammar, hard to maintain |

### Decision
**Context-stack parser** with disambiguation rules. The parser maintains a stack of syntax contexts and uses the following priority-ordered rules:

1. Java-specific keywords (`public`, `private`, `static`, `final`, `void`, `throws`) → Java context
2. Python-specific keywords (`def`, `elif`, `except`, `yield`, `async def`) → Python context
3. Block delimiters: `{` → Java-style; `:` + newline + indent → Python-style
4. Type position: `Type name` → Java; `name: Type` → Python
5. Parent context inheritance
6. File-level default (configurable)

### Consequences
- The parser is more complex but produces a natural developer experience
- Some edge cases will need explicit documentation (e.g., `for` syntax)
- Excellent error messages can suggest "did you mean Java-style or Python-style?"

### Edge Cases and Resolutions

| Ambiguous Code | Resolution |
|----------------|-----------|
| `list = [1, 2, 3]` | Python (lowercase `list`; Java would be `List<Integer> list`) |
| `print("hello")` | Python (Java: `System.out.println`) — but UniLang provides `print` as builtin |
| `x = (int) y` | Java cast (Python would use `int(y)`) |
| `class Foo(Bar):` | Python (inheritance syntax with colon) |
| `class Foo extends Bar {` | Java (explicit `extends` keyword) |

---

## DD-003: Block Syntax Flexibility

### Context
UniLang must support both Python's indentation-based blocks and Java's brace-delimited blocks. The user requirement specifies: "if someone uses an open bracket, they should close that bracket, but if they don't use one, the system should still work."

### Decision
**Four valid block styles, one rule: if you open a brace, you must close it.**

```
Style 1 (Python):        if x > 0:
                              do_something()

Style 2 (Python+braces): if x > 0: {
                              do_something()
                          }

Style 3 (Java):           if (x > 0) {
                              doSomething();
                          }

Style 4 (Minimal):        if x > 0
                              do_something()
```

### Rules
1. If `{` appears after a control structure, `}` is **required** to close it
2. If no `{` appears, indentation determines the block scope
3. Parentheses around conditions are optional (Java-style is accepted but not required)
4. `:` after conditions is optional (Python-style is accepted but not required)
5. Within a single block, mixing styles is NOT allowed (the block must be consistent)

### Consequences
- The lexer must track both indentation levels and brace nesting
- The formatter (`unilang fmt`) must normalize to a chosen style per-project
- Error messages must understand which style the developer intended

---

## DD-004: Runtime Strategy

### Context
UniLang must execute both Java and Python code, including Python C-extensions (NumPy, TensorFlow). We need to decide whether to use a single VM or multiple VMs.

### Options Considered

| Option | Pros | Cons |
|--------|------|------|
| **JVM only** (compile Python to bytecode) | Single VM, simpler deployment, true threading | Cannot run CPython C-extensions (NumPy, TF) |
| **CPython only** (compile Java to Python) | Single VM, full Python ecosystem | No real multi-threading (GIL), poor Java perf |
| **GraalVM polyglot** | Single VM, polyglot support | GraalPython has limited C-extension support |
| **Dual-VM with bridge** | Full feature support for both ecosystems | Complexity, inter-VM overhead |

### Decision
**Dual-VM (JVM + CPython) with a native bridge layer.** This is the only option that fully satisfies the requirements of supporting all Python C-extensions AND Java's threading model.

### Hybrid Strategy
The compiler uses a decision tree to determine where code runs:
- Code that uses CPython C-extensions → CPython
- Code that doesn't need C-extensions → JVM (better threading)
- The bridge layer handles cross-VM calls with minimal overhead

### Consequences
- More complex deployment (requires both JVM and Python)
- Bridge layer is a critical performance component
- Future: GraalVM AOT can provide single-binary deployment (v2)

---

## DD-005: Type System Design

### Context
Python is dynamically typed (with optional type hints). Java is statically typed. UniLang must bridge both approaches.

### Decision
**Gradual type system**: Code can be fully typed (Java-style), partially typed (Python with type hints), or untyped (Python without hints). The compiler provides maximum safety where types are known and falls back to runtime checks where they are not.

### Type Inference Rules
1. Explicit types are always honored: `int x = 5` and `x: int = 5`
2. Literals have known types: `5` is `Int`, `"hi"` is `String`, `[1,2]` is `List<Int>`
3. Return types inferred from all return paths
4. Cross-VM calls use the type mapping table
5. `Dynamic` type is the escape hatch — any operation is allowed, checked at runtime

### Consequences
- Developers can start untyped and gradually add types
- The compiler catches more errors as more types are annotated
- Java-style code gets full static type checking
- Python-style code gets Python-level checking (plus extra inference)

---

## DD-006: Threading and GIL Management

### Context
Java has true multi-threading. Python has the GIL, which prevents true parallel execution of Python bytecode. UniLang must provide real multi-threading even for Python-style code.

### Decision
**Three-strategy approach:**

1. **Primary: Compile Python to JVM bytecode** — For Python-style code that doesn't use C-extensions, compile directly to JVM bytecode. No GIL, true parallelism.

2. **Secondary: Python subinterpreters (PEP 684)** — For code that must use C-extensions, use Python 3.12+ subinterpreters, each with its own GIL.

3. **Tertiary: GIL-aware scheduling** — For C-extensions that don't support subinterpreters, schedule Python tasks to minimize GIL contention and release GIL during bridge calls.

### Consequences
- Most Python code in UniLang gets true parallelism (strategy 1)
- NumPy/TensorFlow code works but may have GIL considerations (strategy 2/3)
- The runtime must detect which strategy applies per code block

---

## DD-007: Semicolons

### Context
Java requires semicolons; Python doesn't use them. UniLang must decide on a policy.

### Decision
**Semicolons are optional everywhere.** The lexer performs automatic semicolon insertion (ASI) similar to Go/JavaScript, using these rules:

1. A newline acts as a statement terminator if the current token could end a statement
2. An explicit `;` always terminates a statement
3. Multiple statements on one line require `;` separation: `x = 1; y = 2`
4. Semicolons inside `for` loop headers are required (Java-style for loops): `for (int i = 0; i < 10; i++)`

### Consequences
- Java developers can keep writing semicolons (they're ignored/optional)
- Python developers don't need to add them
- The formatter can normalize based on project preference

---

## DD-008: Import Resolution Strategy

### Context
`import numpy` should resolve to Python's NumPy. `import java.util.List` should resolve to Java's standard library. The compiler must determine which ecosystem an import belongs to.

### Decision
**Package-name-based resolution with registry lookup:**

1. Check `unilang.toml` dependency declarations (definitive source)
2. Check against known package prefixes:
   - `java.*`, `javax.*`, `org.springframework.*`, etc. → Java
   - `numpy`, `pandas`, `tensorflow`, `sklearn`, etc. → Python
3. Check installed packages in both environments
4. If still ambiguous → compile error with suggestion to qualify in `unilang.toml`

### Import Syntax Normalization

```unilang
// All of these are valid:
import java.util.List                    // Java style
import java.util.List;                   // Java style with semicolon
from java.util import List               // Python style
from java.util import List as JavaList   // Python style with alias

// Python packages:
import numpy as np                       // Python style
import numpy                             // Simplified
from numpy import array                  // Selective import
```

### Consequences
- Most imports resolve unambiguously from their package name
- Edge cases are caught at compile time with clear error messages
- `unilang.toml` is the definitive resolver for any ambiguity

---

## DD-009: File Extension

### Decision
**`.uniL`** — Short for "UniLang", distinctive, and unlikely to conflict with existing extensions.

### Conventions
- Source files: `.uniL`
- Compiled artifacts: `.uniL` (archive, distinguished by content)
- Configuration: `unilang.toml`
- Lock file: `unilang.lock`

---

## DD-010: Intermediate Representation

### Context
The compiler needs an intermediate representation between the AST and final code generation targets (JVM bytecode + CPython).

### Decision
**Custom SSA-based IR called UIR (UniLang Intermediate Representation).** UIR is a language-agnostic, typed, SSA-form IR that:

- Represents both Python and Java semantics uniformly
- Supports optimization passes independent of source syntax
- Can be lowered to either JVM bytecode or CPython AST
- Preserves source location information for debugging

### UIR Design Principles
1. **Typed:** Every value has a known type (or `Dynamic`)
2. **SSA:** Each variable is assigned exactly once
3. **VM-annotated:** Each instruction carries a "preferred VM" hint
4. **Debuggable:** Source spans map every IR instruction to `.uniL` source

### Consequences
- Enables cross-language optimizations (e.g., inlining a Python function into a Java method)
- Provides a stable optimization target independent of either source language
- Adds one more compiler phase but significantly improves code quality

---

*New design decisions should be proposed via RFC (Request for Comments) on the project mailing list and added to this document upon acceptance.*
