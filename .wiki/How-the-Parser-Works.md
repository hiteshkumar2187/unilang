# How the Parser Works

The UniLang parser converts a token stream into an Abstract Syntax Tree (AST). It must handle two complete grammars -- Python and Java -- within a single file. This page explains how it works.

**Location:** `crates/unilang-parser/`

---

## Pratt Parsing Explained

UniLang uses a **Pratt parser** (also called a "top-down operator precedence" parser) for expressions. The key idea is simple: every token has a **binding power** (precedence) that determines how tightly it holds onto its operands.

### How it works

1. Parse the left-hand side (a primary expression like a number or variable).
2. Look at the next operator token and its binding power.
3. If the operator's binding power is higher than the current minimum, consume it and parse the right-hand side recursively.
4. Repeat until the next operator's binding power is too low.

### Example: Parsing `2 + 3 * 4`

```
Step 1: Parse primary "2"
Step 2: See "+", binding power = 11
Step 3: Consume "+", parse right side with min power = 12
  Step 3a: Parse primary "3"
  Step 3b: See "*", binding power = 12, which >= 12
  Step 3c: Consume "*", parse right side with min power = 13
    Step 3c-i: Parse primary "4"
    Step 3c-ii: No more operators with power >= 13
  Step 3d: Result: Mul(3, 4)
Step 4: Result: Add(2, Mul(3, 4))
```

The multiplication binds tighter than addition because `*` has a higher binding power (12) than `+` (11).

---

## Precedence Levels

The parser uses 16 precedence levels, from lowest to highest:

| Level | Operators | Example |
|-------|-----------|---------|
| 1 | Assignment: `=`, `+=`, `-=`, etc. | `x = 5` |
| 2 | Ternary: `? :`, `x if cond else y` | `a ? b : c` |
| 3 | Logical Or: `or`, `\|\|` | `a or b` |
| 4 | Logical And: `and`, `&&` | `a and b` |
| 5 | Logical Not: `not`, `!` | `not x` |
| 6 | Comparison: `==`, `!=`, `<`, `>`, `in`, `is`, `instanceof` | `a == b` |
| 7 | Bitwise Or: `\|` | `a \| b` |
| 8 | Bitwise Xor: `^` | `a ^ b` |
| 9 | Bitwise And: `&` | `a & b` |
| 10 | Shift: `<<`, `>>` | `a << 2` |
| 11 | Addition/Subtraction: `+`, `-` | `a + b` |
| 12 | Multiplication/Division: `*`, `/`, `//`, `%` | `a * b` |
| 13 | Power: `**` | `a ** 2` |
| 14 | Unary: `-`, `+`, `~` | `-x` |
| 15 | Postfix: call, index, attribute, `new` | `f(x)`, `a.b`, `a[0]` |
| 16 | Primary: literals, identifiers, parenthesized | `42`, `x`, `(expr)` |

---

## Handling Both Block Styles

The parser supports two block styles and must determine which one is in use:

### Indentation-based blocks (Python style)

```unilang
def greet(name):
    return f"Hello, {name}!"
```

The lexer emits `Indent` and `Dedent` tokens. The parser sees:
```
DEF IDENT("greet") LPAREN IDENT("name") RPAREN COLON NEWLINE
INDENT
    RETURN FSTRING("Hello, {name}!")
DEDENT
```

### Brace-delimited blocks (Java style)

```unilang
public String greet(String name) {
    return "Hello, " + name + "!";
}
```

The parser sees:
```
PUBLIC STRING IDENT("greet") LPAREN STRING IDENT("name") RPAREN LBRACE
    RETURN STRING("Hello, ") PLUS IDENT("name") PLUS STRING("!") SEMICOLON
RBRACE
```

### Detection rules

The parser determines block style at each block boundary:

1. If the next token after the declaration is `{` (left brace), it is a brace-delimited block.
2. If the next token is `:` followed by `Newline` and `Indent`, it is an indentation block.
3. Inside a brace-delimited block, indentation tracking is suspended.

### Disambiguation

When the parser encounters ambiguous constructs, it uses a priority-ordered set of rules:

1. **Explicit markers:** `public`, `private`, `protected`, `static`, `final` indicate Java context.
2. **Explicit markers:** `def`, `class` (without braces on same line), `import X as Y` indicate Python context.
3. **Block delimiters:** `{` for Java-style; `:` + newline + indent for Python-style.
4. **Type annotations:** `Type name` (e.g., `int x`) for Java; `name: Type` for Python.
5. **Parent context:** Inherit from the enclosing block.
6. **Module default:** Configurable per file with `# unilang: default=python` or `// unilang: default=java`.

---

## Statements: Recursive Descent

While expressions use Pratt parsing, statements are parsed using **recursive descent**. The parser looks at the current token to decide which statement rule to apply:

| Current Token | Statement Type |
|---------------|---------------|
| `def` | Python-style function declaration |
| `class` (+ `:`) | Python-style class declaration |
| `class` (+ `{`) | Java-style class declaration |
| `public`/`private`/`protected`/`static` | Java-style declaration |
| `if` | If/elif/else statement |
| `for` | For loop (Python or Java style) |
| `while` | While loop |
| `try` | Try/except or try/catch block |
| `return` | Return statement |
| `import`/`from` | Import statement |
| `match` | Pattern matching |
| `switch` | Switch expression |
| Otherwise | Expression statement |

---

## Error Recovery

When the parser encounters an unexpected token, it does not stop. Instead:

1. The parser records a diagnostic error with the token's location.
2. It creates an error node (`Stmt::Error` or `Expr::Error`) in the AST.
3. It enters **recovery mode** and skips tokens until it finds a safe synchronization point:
   - A newline at the top level
   - A `}` that closes the current brace block
   - A `Dedent` that closes the current indentation block
   - A keyword that starts a new statement (`def`, `class`, `if`, `for`, `while`, `return`, etc.)
4. The parser resumes normal parsing from the synchronization point.

This means a single syntax error does not prevent the rest of the file from being parsed. The semantic analyzer and code generator skip error nodes.

### Example

```unilang
def add(a, b):
    return a +          // ERROR: unexpected end of expression

def multiply(a, b):     // Parser recovers here and parses this function
    return a * b
```

The parser produces an AST with two function nodes: `add` (with an error in its body) and `multiply` (fully parsed).

---

## AST Node Structure

Every AST node carries metadata:

```
AstNode
  +-- kind: NodeKind (FunctionDecl, ClassDecl, IfStmt, BinaryExpr, etc.)
  +-- span: Span (start offset, end offset in source)
  +-- source_syntax: SyntaxOrigin (Python | Java | Ambiguous | UniLang)
```

The `SyntaxOrigin` tag is used by the parser for disambiguation but is transparent to later stages. By the time code reaches the code generator, both Python-style and Java-style constructs have been normalized into the same AST representation.

---

**Previous**: [[Compiler Pipeline]] | **Next**: [[How the VM Works]]
