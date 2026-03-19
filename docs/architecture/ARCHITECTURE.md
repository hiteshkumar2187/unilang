# UniLang — System Architecture Document

**Version:** 1.0.0-draft
**Status:** Draft
**Last Updated:** 2026-03-19

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Design Principles](#2-design-principles)
3. [Component Architecture](#3-component-architecture)
4. [Compiler Pipeline](#4-compiler-pipeline)
5. [Runtime Architecture](#5-runtime-architecture)
6. [Type System](#6-type-system)
7. [Interoperability Layer](#7-interoperability-layer)
8. [Threading Model](#8-threading-model)
9. [Dependency Management](#9-dependency-management)
10. [Build System](#10-build-system)
11. [Error Handling](#11-error-handling)
12. [Deployment Architecture](#12-deployment-architecture)

---

## 1. Architecture Overview

UniLang's architecture is composed of three major subsystems:

```
┌────────────────────────────────────────────────────────────┐
│                     Developer Interface                     │
│  ┌─────────┐  ┌──────────┐  ┌────────┐  ┌──────────────┐  │
│  │   CLI   │  │ VS Code  │  │ LSP    │  │ Build Plugin │  │
│  │ (unilang)│  │Extension │  │ Server │  │ (Maven/Gradle)│ │
│  └────┬────┘  └────┬─────┘  └───┬────┘  └──────┬───────┘  │
│       └─────────────┴────────────┴──────────────┘          │
└────────────────────────┬───────────────────────────────────┘
                         │
┌────────────────────────▼───────────────────────────────────┐
│                    Compiler Subsystem                       │
│  ┌─────────┐  ┌────────┐  ┌──────────┐  ┌──────────────┐  │
│  │  Lexer  │→ │ Parser │→ │ Semantic │→ │  Code Gen    │  │
│  │         │  │  (AST) │  │ Analyzer │  │  (UIR→Target)│  │
│  └─────────┘  └────────┘  └──────────┘  └──────────────┘  │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │  Optimizer  │  │ Error Engine │  │  Source Maps     │  │
│  └─────────────┘  └──────────────┘  └──────────────────┘  │
└────────────────────────┬───────────────────────────────────┘
                         │
┌────────────────────────▼───────────────────────────────────┐
│                    Runtime Subsystem                        │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              UniLang Runtime Manager                  │  │
│  │  ┌──────────┐  ┌────────────┐  ┌──────────────────┐ │  │
│  │  │JVM Engine│  │  Bridge    │  │ CPython Engine   │ │  │
│  │  │          │←→│  Layer     │←→│                  │ │  │
│  │  │ Bytecode │  │ (Zero-Copy)│  │ C-Extensions    │ │  │
│  │  │ Execution│  │ (Type Map) │  │ NumPy/TF/PyTorch│ │  │
│  │  └──────────┘  └────────────┘  └──────────────────┘ │  │
│  │  ┌──────────────────┐  ┌────────────────────────┐   │  │
│  │  │ Thread Scheduler │  │   Memory Manager       │   │  │
│  │  │ (Java Threads +  │  │   (Shared Heap +       │   │  │
│  │  │  Python Tasks)   │  │    Reference Bridge)   │   │  │
│  │  └──────────────────┘  └────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

---

## 2. Design Principles

| Principle | Description | Rationale |
|-----------|-------------|-----------|
| **P1: Syntax Agnosticism** | The compiler treats Python and Java syntax as two projections of the same semantic model | Enables genuine mixing without "foreign function" ceremony |
| **P2: Zero Surprise Interop** | Calling Python from Java (and vice versa) should feel like calling native code | Reduces cognitive load and adoption barriers |
| **P3: Correctness Over Performance** | The compiler must never silently miscompile mixed-syntax code | Trust in the compiler is essential for adoption |
| **P4: Gradual Complexity** | Simple programs should be simple to write; advanced features are opt-in | Lowers the entry barrier for developers from either ecosystem |
| **P5: Runtime Transparency** | Developers should understand which VM executes their code (via tooling, not syntax) | Aids debugging and performance tuning |
| **P6: Modular Architecture** | Each compiler phase and runtime component is independently testable and replaceable | Enables parallel development and community contributions |

---

## 3. Component Architecture

### 3.1 Component Dependency Graph

```
                    ┌───────────┐
                    │  unilang   │
                    │   (CLI)    │
                    └─────┬─────┘
                          │
            ┌─────────────┼─────────────┐
            │             │             │
      ┌─────▼─────┐ ┌────▼────┐ ┌─────▼─────┐
      │  Compiler  │ │ Runtime │ │  Tooling  │
      │   Core     │ │  Core   │ │  (fmt,    │
      │            │ │         │ │  lint,    │
      │            │ │         │ │  test)    │
      └─────┬─────┘ └────┬────┘ └───────────┘
            │             │
   ┌────────┼────────┐    │
   │        │        │    │
┌──▼──┐ ┌──▼──┐ ┌───▼┐ ┌─▼──────┐
│Lexer│ │Parse│ │Sema│ │Bridge  │
│     │ │     │ │ntic│ │Layer   │
└─────┘ └─────┘ └────┘ └────────┘
```

### 3.2 Component Descriptions

| Component | Responsibility | Language | Key Interfaces |
|-----------|---------------|----------|----------------|
| **CLI** | Command-line interface, argument parsing, orchestration | Rust | `CompilerConfig`, `RuntimeConfig` |
| **Lexer** | Tokenization of `.uniL` source into unified token stream | Rust | `TokenStream`, `Token`, `Span` |
| **Parser** | Context-aware parsing into unified AST | Rust | `AstNode`, `ParseResult`, `SyntaxContext` |
| **Semantic Analyzer** | Type inference, scope resolution, interop validation | Rust | `TypedAst`, `SymbolTable`, `TypeEnv` |
| **Code Generator** | UIR → JVM bytecode + CPython AST | Rust | `JvmBytecode`, `PythonAst`, `UIR` |
| **Optimizer** | Cross-language optimizations on UIR | Rust | `OptimizationPass`, `UIR` |
| **Runtime Manager** | Lifecycle management of JVM + CPython VMs | Rust + C | `VmHandle`, `RuntimeState` |
| **Bridge Layer** | Type marshaling, function dispatch across VMs | Rust + C + JNI | `BridgeCall`, `TypeMarshaler` |
| **Thread Scheduler** | Unified thread management across VMs | Rust + Java | `UnifiedThread`, `TaskQueue` |
| **Memory Manager** | Shared memory regions, reference counting bridge | Rust + C | `SharedRegion`, `RefBridge` |

---

## 4. Compiler Pipeline

### 4.1 Pipeline Stages

```
.uniL Source
    │
    ▼
┌─────────────────────────────────────────┐
│ Stage 1: LEXICAL ANALYSIS               │
│                                         │
│ Input:  Raw source text                 │
│ Output: Token stream                    │
│                                         │
│ - Unified tokenizer handles both        │
│   Python and Java tokens                │
│ - Indentation tracker for Python blocks │
│ - Semicolon insertion (optional semis)  │
│ - String literal unification            │
│   (f-strings, template strings)         │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Stage 2: PARSING                        │
│                                         │
│ Input:  Token stream                    │
│ Output: Unified AST                     │
│                                         │
│ - Context-aware grammar switching       │
│ - Ambiguity resolution via lookahead    │
│ - Block style detection (braces vs      │
│   indentation vs hybrid)               │
│ - Error recovery (continue parsing      │
│   after errors)                        │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Stage 3: SEMANTIC ANALYSIS              │
│                                         │
│ Input:  Untyped AST                     │
│ Output: Typed AST + Symbol Table        │
│                                         │
│ - Bidirectional type inference          │
│ - Cross-syntax scope resolution         │
│ - Import resolution (PyPI vs Maven)     │
│ - Overload resolution                   │
│ - Interop validation (type compat)     │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Stage 4: UIR GENERATION                 │
│                                         │
│ Input:  Typed AST                       │
│ Output: UniLang IR (UIR)                │
│                                         │
│ - Language-agnostic intermediate form   │
│ - SSA (Static Single Assignment) based  │
│ - Preserves source mapping              │
│ - Annotated with VM target hints        │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Stage 5: OPTIMIZATION                   │
│                                         │
│ Input:  UIR                             │
│ Output: Optimized UIR                   │
│                                         │
│ - Dead code elimination                 │
│ - Inlining (cross-VM boundary aware)    │
│ - Interop call fusion                   │
│ - Type specialization                   │
│ - Escape analysis for bridge objects    │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Stage 6: CODE GENERATION                │
│                                         │
│ Input:  Optimized UIR                   │
│ Output: JVM .class files +              │
│         Python .pyc / AST               │
│                                         │
│ - Split UIR into JVM and Python targets │
│ - Generate bridge stubs                 │
│ - Emit source maps for debugging        │
│ - Package into .uniL artifact           │
└─────────────────────────────────────────┘
```

### 4.2 Context-Aware Parsing Strategy

The parser uses a **context stack** to determine which grammar rules to apply:

```
Context Stack:
┌──────────────────┐
│ JAVA_CLASS_BODY  │  ← Currently parsing inside a Java-style class
├──────────────────┤
│ PYTHON_FUNCTION  │  ← Inside a def function
├──────────────────┤
│ MODULE_LEVEL     │  ← Top-level (either syntax valid)
└──────────────────┘
```

**Disambiguation rules (ordered by priority):**

1. **Explicit markers:** `public`, `private`, `protected`, `static`, `final` → Java context
2. **Explicit markers:** `def`, `class` (without braces on same line), `import X as Y` → Python context
3. **Block delimiters:** `{` → Java-style block; `:` + newline + indent → Python-style block
4. **Type annotations:** `Type name` (e.g., `int x`) → Java context; `name: Type` → Python context
5. **Parent context:** Inherit from enclosing block
6. **Module default:** Configurable per-file (`# unilang: default=python` or `// unilang: default=java`)

### 4.3 AST Node Design

The unified AST uses a common node structure that can represent both syntaxes:

```
AstNode
├── FunctionDecl
│   ├── name: String
│   ├── params: Vec<Param>
│   ├── return_type: Option<Type>       // None if inferred
│   ├── body: Block
│   ├── visibility: Visibility          // Public/Private/Protected/Default
│   ├── modifiers: Vec<Modifier>        // static, final, abstract, async
│   ├── decorators: Vec<Decorator>      // Python decorators + Java annotations
│   └── source_syntax: SyntaxOrigin     // Python | Java | Ambiguous
│
├── ClassDecl
│   ├── name: String
│   ├── type_params: Vec<TypeParam>     // Java generics
│   ├── bases: Vec<Type>                // Python inheritance
│   ├── implements: Vec<Type>           // Java interfaces
│   ├── body: Vec<AstNode>
│   └── source_syntax: SyntaxOrigin
│
├── Block
│   ├── style: BlockStyle               // Braces | Indentation | Hybrid
│   ├── statements: Vec<AstNode>
│   └── scope: ScopeId
│
└── ... (expressions, statements, etc.)
```

---

## 5. Runtime Architecture

### 5.1 Dual-VM Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  UniLang Runtime Manager                     │
│                                                             │
│  ┌─────────────────────┐       ┌─────────────────────────┐ │
│  │    JVM Instance     │       │   CPython Instance      │ │
│  │                     │       │                         │ │
│  │  ┌───────────────┐  │       │  ┌───────────────────┐  │ │
│  │  │ UniLang       │  │       │  │ UniLang Python    │  │ │
│  │  │ ClassLoader   │  │       │  │ Module Loader     │  │ │
│  │  └───────────────┘  │       │  └───────────────────┘  │ │
│  │  ┌───────────────┐  │       │  ┌───────────────────┐  │ │
│  │  │ Bridge        │  │  JNI  │  │ Bridge            │  │ │
│  │  │ Native Lib    │◄─┼──────►┼──│ C Extension       │  │ │
│  │  └───────────────┘  │       │  └───────────────────┘  │ │
│  │  ┌───────────────┐  │       │  ┌───────────────────┐  │ │
│  │  │ Thread Pool   │  │       │  │ Subinterpreters   │  │ │
│  │  │ Manager       │  │       │  │ (PEP 684)         │  │ │
│  │  └───────────────┘  │       │  └───────────────────┘  │ │
│  └─────────────────────┘       └─────────────────────────┘ │
│                                                             │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Shared Memory Region                      │ │
│  │  ┌──────────┐  ┌──────────┐  ┌─────────────────────┐  │ │
│  │  │ Primitive │  │ Array    │  │ Object Reference    │  │ │
│  │  │ Buffer    │  │ Buffer   │  │ Table               │  │ │
│  │  └──────────┘  └──────────┘  └─────────────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 VM Lifecycle

```
1. unilang run app.uniL
   │
   ├─ 2. Compile .uniL → UIR → split into JVM + Python targets
   │
   ├─ 3. Start JVM (embedded via JNI)
   │     ├─ Load UniLang ClassLoader
   │     ├─ Load compiled .class files
   │     └─ Initialize bridge native library
   │
   ├─ 4. Start CPython (embedded via C API)
   │     ├─ Load UniLang module loader
   │     ├─ Load compiled .pyc files
   │     └─ Initialize bridge C extension
   │
   ├─ 5. Initialize shared memory region
   │
   ├─ 6. Execute main entry point
   │     └─ (May be Java-style or Python-style)
   │
   ├─ 7. Runtime manages cross-VM calls as needed
   │
   └─ 8. Shutdown (coordinated VM teardown)
```

### 5.3 Execution Strategy Decision Tree

For each function/block in the UIR, the compiler decides the execution target:

```
Is this code block...
├── Pure Java syntax with Java imports?
│   └── → Execute on JVM
│
├── Pure Python syntax with Python imports?
│   └── → Execute on CPython
│
├── Python syntax but no Python-specific imports?
│   └── → Compile to JVM bytecode (better threading)
│
├── Mixed: Java class with Python method?
│   └── → Class on JVM, Python method via bridge call
│
├── Calls a Python C-extension (NumPy, etc.)?
│   └── → Must execute on CPython
│
└── Ambiguous?
    └── → Default to JVM (configurable)
```

---

## 6. Type System

### 6.1 Unified Type Hierarchy

```
UniType (root)
├── Primitive
│   ├── Int       (Python int ↔ Java int/long/BigInteger)
│   ├── Float     (Python float ↔ Java double)
│   ├── Bool      (Python bool ↔ Java boolean)
│   ├── Char      (Java char, Python str[0])
│   └── Void      (Java void, Python None)
│
├── String        (Python str ↔ Java String)
│
├── Collection
│   ├── List<T>        (Python list ↔ Java List<T>)
│   ├── Dict<K,V>      (Python dict ↔ Java Map<K,V>)
│   ├── Set<T>         (Python set ↔ Java Set<T>)
│   ├── Tuple<T...>    (Python tuple ↔ Java Record/custom)
│   └── Array<T>       (numpy.ndarray ↔ Java array)
│
├── Function
│   ├── Callable<Args, Return>
│   └── Lambda<Args, Return>
│
├── Class
│   ├── UniClass       (unified class representation)
│   ├── JavaClass      (wrapper for pure Java classes)
│   └── PythonClass    (wrapper for pure Python classes)
│
├── Generic<T>
│   └── Bounded<T extends/super U>
│
├── Optional<T>        (Python Optional ↔ Java Optional<T>)
│
├── Union<A, B>        (Python Union[A, B] — Java sealed interface)
│
└── Dynamic            (Python's duck typing — opt-in escape hatch)
```

### 6.2 Type Inference Strategy

```
1. Local inference:     x = 42          → x: Int
2. Return inference:    def foo(): return "hi"  → foo() -> String
3. Cross-VM inference:  np.array([1,2]) → Array<Float>  (known library types)
4. Generic inference:   List<> list = new ArrayList<>()  → List<Object> (Java diamond)
5. Gradual typing:      def bar(x):     → x: Dynamic (no annotation)
6. Bidirectional:       int y = foo()   → foo() must return Int-compatible
```

### 6.3 Type Mapping Table

| Python Type | Java Type | UIR Type | Bridge Strategy |
|-------------|-----------|----------|-----------------|
| `int` | `int` / `long` | `Int` | Direct (if fits) or BigInteger |
| `float` | `double` | `Float` | Direct |
| `str` | `String` | `String` | Copy (UTF-8 ↔ UTF-16) |
| `bool` | `boolean` | `Bool` | Direct |
| `None` | `void` / `null` | `Void` | Sentinel |
| `list` | `List<T>` | `List<T>` | Proxy (lazy copy) |
| `dict` | `Map<K,V>` | `Dict<K,V>` | Proxy (lazy copy) |
| `tuple` | `Record` | `Tuple<T...>` | Copy |
| `set` | `Set<T>` | `Set<T>` | Proxy (lazy copy) |
| `numpy.ndarray` | `double[]` / Buffer | `Array<T>` | Zero-copy (shared memory) |
| `Callable` | `Function<T,R>` | `Callable<A,R>` | Bridge stub |
| `object` | `Object` | `Dynamic` | Full bridge proxy |

---

## 7. Interoperability Layer

### 7.1 Bridge Call Protocol

```
Java Code                    Bridge Layer                 Python Code
    │                            │                            │
    │  call python_func(args)    │                            │
    ├──────────────────────────►│                            │
    │                            │  marshal args to Python   │
    │                            ├──────────────────────────►│
    │                            │                            │ execute
    │                            │  marshal return to Java   │ python_func
    │                            │◄────────────────────────┤
    │  receive Java result       │                            │
    │◄──────────────────────────┤                            │
    │                            │                            │
```

### 7.2 Bridge Call Overhead Budget

| Operation | Target Latency | Strategy |
|-----------|---------------|----------|
| Primitive pass | < 10ns | Direct memory copy |
| String pass | < 100ns | UTF-8/16 conversion |
| Array pass (NumPy) | < 50ns | Shared memory pointer |
| Object pass | < 1μs | Proxy object creation |
| Function call overhead | < 500ns | Pre-compiled stub |
| Exception propagation | < 10μs | Unified exception wrapper |

### 7.3 Proxy Objects

When a Java object is accessed from Python (or vice versa), the bridge creates a **proxy object**:

```
┌──────────────────────────────────┐
│       Python Proxy Object        │
│                                  │
│  __getattr__ → JNI call         │
│  __setattr__ → JNI call         │
│  __call__    → JNI call         │
│  __del__     → Release JVM ref  │
│                                  │
│  _java_ref: JNI GlobalRef       │
│  _type_cache: HashMap           │
└──────────────────────────────────┘
```

---

## 8. Threading Model

### 8.1 Architecture

UniLang's threading model is built on Java's `java.util.concurrent` framework, extended to support Python code execution:

```
┌──────────────────────────────────────────────────────┐
│              UniLang Thread Manager                    │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │           Java Thread Pool                       │ │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐   │ │
│  │  │Thread-1│ │Thread-2│ │Thread-3│ │Thread-N│   │ │
│  │  │        │ │        │ │        │ │        │   │ │
│  │  │ JVM    │ │ Python │ │ JVM    │ │ Mixed  │   │ │
│  │  │ task   │ │ task   │ │ task   │ │ task   │   │ │
│  │  └────────┘ └────────┘ └────────┘ └────────┘   │ │
│  └─────────────────────────────────────────────────┘ │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │         Python Subinterpreter Pool               │ │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐           │ │
│  │  │Sub-1 │ │Sub-2 │ │Sub-3 │ │Sub-N │           │ │
│  │  │(own  │ │(own  │ │(own  │ │(own  │           │ │
│  │  │ GIL) │ │ GIL) │ │ GIL) │ │ GIL) │           │ │
│  │  └──────┘ └──────┘ └──────┘ └──────┘           │ │
│  └─────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

### 8.2 Threading Strategies

| Scenario | Strategy |
|----------|----------|
| Pure Java code | Standard Java threads — no GIL concern |
| Pure Python code (no C-extensions) | Compile to JVM bytecode → true parallelism |
| Python code with C-extensions (NumPy) | Python subinterpreters (PEP 684) — per-interpreter GIL |
| Mixed Java + Python in same thread | JVM thread with bridge calls to Python subinterpreter |
| `synchronized` on Python object | Java monitor on proxy object + Python lock on real object |

### 8.3 GIL Management

```
Option A: Subinterpreters (Primary Strategy)
- Each Java thread gets its own Python subinterpreter
- Each subinterpreter has its own GIL
- True parallelism for Python code
- Limitation: Not all C-extensions support subinterpreters yet

Option B: JVM Compilation (Fallback)
- Compile Python-style code to JVM bytecode
- No GIL needed — runs as native JVM code
- Limitation: Cannot use C-extensions (NumPy, etc.)

Option C: GIL-Aware Scheduling (Legacy Support)
- For C-extensions that don't support subinterpreters
- Runtime schedules Python tasks to minimize GIL contention
- Releases GIL during Java bridge calls
```

---

## 9. Dependency Management

### 9.1 Unified Configuration (`unilang.toml`)

```toml
[project]
name = "my-app"
version = "1.0.0"
unilang-version = "1.0"
default-syntax = "python"   # or "java"

[dependencies.python]
numpy = ">=1.24"
scikit-learn = ">=1.3"
tensorflow = ">=2.15"

[dependencies.java]
"org.springframework:spring-boot-starter-web" = "3.2.0"
"com.google.guava:guava" = "33.0"

[dependencies.unilang]
unilang-ml = "1.0"          # UniLang-native libraries

[build]
target = "jvm"              # Primary target: jvm | python | native
optimization = "release"
threads = "auto"

[runtime]
jvm-args = ["-Xmx4g", "-Xms1g"]
python-path = "/usr/bin/python3"
```

### 9.2 Dependency Resolution Flow

```
unilang deps install
    │
    ├─ Parse unilang.toml
    │
    ├─ Python dependencies → pip install into .unilang/python_env/
    │
    ├─ Java dependencies → Maven resolve into .unilang/java_libs/
    │
    ├─ UniLang dependencies → Download into .unilang/unilang_libs/
    │
    ├─ Cross-check for conflicts
    │   (e.g., Python lib that embeds a Java JAR)
    │
    └─ Generate unilang.lock (reproducible builds)
```

---

## 10. Build System

### 10.1 Build Pipeline

```
unilang build
    │
    ├─ 1. Read unilang.toml configuration
    │
    ├─ 2. Discover .uniL source files
    │
    ├─ 3. Incremental compilation check (file hashes)
    │
    ├─ 4. Compile changed files through pipeline
    │     ├─ Lexer → Parser → Semantic → UIR → Optimize → CodeGen
    │     └─ Parallel compilation of independent modules
    │
    ├─ 5. Generate bridge stubs
    │
    ├─ 6. Package outputs
    │     ├─ .class files → JAR
    │     ├─ .pyc files → Python package
    │     ├─ Bridge native lib → platform-specific
    │     └─ Runtime config → manifest
    │
    └─ 7. Produce .uniL artifact
          (single deployable archive containing all components)
```

### 10.2 Artifact Format (`.uniL` archive)

```
myapp.uniL (ZIP-based archive)
├── META-INF/
│   ├── MANIFEST.MF          # Entry point, runtime requirements
│   └── unilang.toml          # Build configuration snapshot
├── jvm/
│   ├── classes/              # Compiled .class files
│   └── libs/                 # Dependency JARs
├── python/
│   ├── modules/              # Compiled .pyc files
│   └── site-packages/        # Dependency packages
├── bridge/
│   ├── stubs/                # Generated bridge code
│   └── native/               # Platform-specific native libs
│       ├── linux-x86_64/
│       ├── darwin-aarch64/
│       └── windows-x86_64/
└── source-maps/              # .uniL → compiled code mappings
```

---

## 11. Error Handling

### 11.1 Unified Exception Hierarchy

```
UniLangException (root)
├── CompileError
│   ├── SyntaxError
│   │   ├── PythonSyntaxError    (with Python-style error message)
│   │   ├── JavaSyntaxError      (with Java-style error message)
│   │   └── AmbiguousSyntaxError (suggestions for both interpretations)
│   ├── TypeError
│   ├── ImportError
│   └── SemanticError
│
├── RuntimeException
│   ├── BridgeException          (cross-VM call failure)
│   ├── TypeMarshalException     (type conversion failure)
│   ├── ThreadingException       (cross-VM thread issue)
│   └── VMException              (JVM or CPython crash)
│
└── WrappedException
    ├── JavaException → UniLangException  (wraps java.lang.Exception)
    └── PythonException → UniLangException (wraps Python BaseException)
```

### 11.2 Error Message Format

```
error[E0042]: type mismatch in cross-syntax call
  --> src/pipeline.uniL:23:15
   |
22 |     def process(self, data):
23 |         List<String> results = self.transform(data)
   |                       ^^^^^^^ expected Java List<String>,
   |                               found Python list[str]
   |
   = help: Add explicit conversion: `List.of(self.transform(data))`
   = note: Python list and Java List are interoperable but require
           explicit bridging when used in Java type declarations
```

---

## 12. Deployment Architecture

### 12.1 Deployment Options

```
Option A: Fat Archive (.uniL)
├── Contains all compiled code + dependencies
├── Requires JVM + Python installed on target
└── unilang run app.uniL

Option B: Docker Container
├── UniLang base image (JVM + Python + Runtime)
├── Application layer
└── docker run unilang/myapp

Option C: Native Binary (v2 — via GraalVM AOT)
├── Self-contained native executable
├── No JVM/Python required at runtime
└── ./myapp

Option D: Cloud Function
├── Packaged as serverless function
├── UniLang runtime layer
└── Deployed to AWS Lambda / GCP Functions / Azure Functions
```

---

## Appendix A: Technology Stack Summary

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| Compiler | Rust | Performance, memory safety, no runtime dependency |
| Parser | Custom (Pratt parser + context stack) | Need custom ambiguity resolution |
| JVM Bridge | JNI (Java Native Interface) | Standard, mature, well-documented |
| Python Bridge | CPython C API | Direct access to interpreter internals |
| Build Config | TOML | Rust ecosystem standard, human-readable |
| Package Format | ZIP-based archive | Portable, tooling-friendly |
| Testing | Rust (compiler), JUnit + pytest (runtime) | Match respective ecosystems |
| CI/CD | GitHub Actions | Apache-compatible, free for open source |

---

*This document is a living artifact. Major architecture changes require an RFC process and community review.*
