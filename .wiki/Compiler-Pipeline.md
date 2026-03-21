# Compiler Pipeline

UniLang compiles `.uniL` source files through a 6-stage pipeline, from raw text to execution.

---

## Architecture Diagram

```
.uniL Source Code
       |
       v
+------------------+
|   Stage 1:       |  crates/unilang-lexer/
|   LEXER          |  Input:  Raw source text
|   (Tokenizer)    |  Output: Vec<Token>
+--------+---------+
         |
         v
+------------------+
|   Stage 2:       |  crates/unilang-parser/
|   PARSER         |  Input:  Token stream
|   (AST Gen)      |  Output: Module (AST)
+--------+---------+
         |
         v
+------------------+
|   Stage 3:       |  crates/unilang-semantic/
|   SEMANTIC       |  Input:  Untyped AST
|   ANALYZER       |  Output: Validated AST + Diagnostics
+--------+---------+
         |
         v
+------------------+
|   Stage 4:       |  crates/unilang-codegen/
|   CODE           |  Input:  AST
|   GENERATOR      |  Output: Bytecode
+--------+---------+
         |
         v
+------------------+
|   Stage 5:       |  crates/unilang-runtime/
|   RUNTIME VM     |  Input:  Bytecode
|   (Executor)     |  Output: Program results
+------------------+
         |
         v
+------------------+
|   Stage 6:       |  crates/unilang-stdlib/
|   STANDARD       |  35+ built-in functions
|   LIBRARY        |  registered in VM
+------------------+
```

---

## Stage 1: Lexer

**Location:** `crates/unilang-lexer/`

The lexer converts raw source text into a stream of tokens.

**Key features:**
- Handles the union of Python and Java token sets (75+ keywords)
- Indentation tracking: emits `Indent`/`Dedent` tokens for Python-style blocks
- Automatic Semicolon Insertion (ASI): emits `Newline` tokens at statement boundaries
- All string types: regular, triple-quoted, f-strings, raw strings
- All numeric formats: decimal, hex (`0x`), octal (`0o`), binary (`0b`), float, scientific
- Brace depth tracking: suppresses indentation inside `{}`
- Paren depth tracking: suppresses newlines inside `()` and `[]`

**Input:** Source text (`&str`)
**Output:** `Vec<Token>` where each `Token` has a `TokenKind` and `Span`

---

## Stage 2: Parser

**Location:** `crates/unilang-parser/`

The parser converts the token stream into an Abstract Syntax Tree (AST).

**Key features:**
- **Pratt expression parser** with 15 precedence levels
- **Recursive descent** for statements
- Handles both indentation-based (Python) and brace-based (Java) blocks
- Error recovery: skips to next statement boundary on parse error
- Tags nodes with `SyntaxOrigin` (Python, Java, Ambiguous)

For more details, see [[How the Parser Works]].

**Input:** `Vec<Token>` + source text
**Output:** `Module` (AST root containing `Vec<Spanned<Stmt>>`)

---

## Stage 3: Semantic Analyzer

**Location:** `crates/unilang-semantic/`

Validates the AST for correctness.

**Checks performed:**
- **Name resolution:** Every identifier resolves to a declaration
- **Scope management:** Lexical scoping with nested function/class/block scopes
- **Type checking:** Gradual type system where `Dynamic` is compatible with everything
- **Declaration validation:** Duplicate names, missing initializers
- **Context validation:** `return` inside functions, `break`/`continue` inside loops
- **Mutability:** Prevents reassignment to `val`, `const`, `final` variables
- **Call arity:** Function call argument count matches parameter count

**Type system internals:**
- Primitive types: `Int`, `Float`, `Double`, `Bool`, `String`, `Char`, `Void`
- Compound types: `Array(T)`, `Generic(name, params)`, `Optional(T)`, `Union(types)`
- Special: `Dynamic` (Python-style untyped), `Unknown`, `Error`

**Input:** `&Module` (AST)
**Output:** `(AnalysisResult, DiagnosticBag)`

---

## Stage 4: Code Generator

**Location:** `crates/unilang-codegen/`

Compiles the validated AST into stack-based bytecode.

**Instruction set (40+ opcodes):**

| Category | Opcodes |
|----------|---------|
| Stack | `LoadConst`, `LoadLocal`, `StoreLocal`, `LoadGlobal`, `StoreGlobal`, `Pop`, `Dup` |
| Arithmetic | `Add`, `Sub`, `Mul`, `Div`, `FloorDiv`, `Mod`, `Pow`, `Neg` |
| Comparison | `Eq`, `NotEq`, `Lt`, `Gt`, `LtEq`, `GtEq` |
| Logical | `And`, `Or`, `Not` |
| Bitwise | `BitAnd`, `BitOr`, `BitXor`, `BitNot`, `LShift`, `RShift` |
| Control flow | `Jump`, `JumpIfFalse`, `JumpIfTrue` |
| Functions | `Call`, `Return`, `MakeFunction` |
| Objects | `GetAttr`, `SetAttr`, `MakeClass`, `NewInstance` |
| Collections | `MakeList`, `MakeDict`, `GetIndex`, `SetIndex` |
| I/O | `Print` |
| Control | `Halt`, `Concat` |

Forward jumps (if/else, loops) use placeholder targets that are patched when the destination is known.

**Input:** `&Module` (AST)
**Output:** `Bytecode` (instructions + function table + class definitions)

---

## Stage 5: Runtime VM

**Location:** `crates/unilang-runtime/`

A stack-based virtual machine that interprets the bytecode.

For more details, see [[How the VM Works]].

**Architecture:**
- **Operand stack:** Values pushed/popped during computation
- **Call frames:** Each function call pushes a frame with its own locals and instruction pointer
- **Globals:** Module-level variable store
- **Function table:** Compiled function bodies indexed by ID
- **Output buffer:** Captured print output (for testing)

**Runtime value types:**
- `Int(i64)`, `Float(f64)`, `String`, `Bool`, `Null`
- `List(Vec<RuntimeValue>)`, `Dict(Vec<(key, value)>)`
- `Function(usize)` -- index into function table
- `NativeFunction(String)` -- built-in function by name
- `Instance { class_name, fields }` -- object instance

---

## Stage 6: Standard Library

**Location:** `crates/unilang-stdlib/`

35+ built-in functions registered in the VM at startup. See [[Standard Library Reference]] for the complete list.

| Category | Functions |
|----------|-----------|
| I/O | `print`, `input` |
| Type conversion | `int`, `float`, `str`, `bool` |
| Type checking | `type`, `isinstance` |
| Math | `abs`, `min`, `max`, `pow`, `sqrt`, `floor`, `ceil`, `round` |
| Collections | `len`, `range`, `sorted`, `reversed`, `enumerate`, `zip` |
| Strings | `upper`, `lower`, `split`, `join`, `strip`, `replace`, `contains`, `startswith`, `endswith` |

---

## CLI Commands

```bash
# Full pipeline: lex -> parse -> semantic -> codegen -> run
unilang run hello.uniL

# Check for errors without running (stages 1-3)
unilang check hello.uniL

# Compile and show bytecode disassembly (stages 1-4)
unilang compile hello.uniL

# Tokenize and show token stream (stage 1 only)
unilang lex hello.uniL
```

---

## Test Coverage

| Crate | Tests | What's Tested |
|-------|-------|---------------|
| unilang-common | 10 | Span, SourceFile, DiagnosticBag |
| unilang-lexer | 35 | All token types, indentation, ASI, strings, numbers |
| unilang-parser | 15 | Expressions, statements, blocks, classes, imports |
| unilang-semantic | 18 | Scoping, types, name resolution, context validation |
| unilang-codegen | 16 | All compilation targets, jump patching |
| unilang-runtime | 24 | All opcodes, function calls, collections, full pipeline |
| unilang-stdlib | 24 | All built-in functions |
| **Total** | **142** | |

---

**Previous**: [[Standard Library Reference]] | **Next**: [[How the Parser Works]]
