# UniLang — Language Specification

**Version:** 1.0.0-draft
**Status:** Draft
**Last Updated:** 2026-03-19

---

## Table of Contents

1. [Notation](#1-notation)
2. [Lexical Structure](#2-lexical-structure)
3. [Types](#3-types)
4. [Declarations](#4-declarations)
5. [Expressions](#5-expressions)
6. [Statements](#6-statements)
7. [Classes and Objects](#7-classes-and-objects)
8. [Functions and Methods](#8-functions-and-methods)
9. [Control Flow](#9-control-flow)
10. [Exception Handling](#10-exception-handling)
11. [Modules and Imports](#11-modules-and-imports)
12. [Concurrency](#12-concurrency)
13. [Interoperability Semantics](#13-interoperability-semantics)
14. [Grammar Summary](#14-grammar-summary)

---

## 1. Notation

This specification uses Extended Backus-Naur Form (EBNF) with the following conventions:

```
rule       = definition ;
'terminal' = literal text
[optional] = zero or one
{repeated} = zero or more
(grouped)  = grouping
|          = alternative
```

---

## 2. Lexical Structure

### 2.1 Source Encoding

UniLang source files (`.uniL`) are encoded in UTF-8.

### 2.2 Line Terminators

```
LineTerminator = '\n' | '\r\n' | '\r' ;
```

### 2.3 Whitespace and Indentation

Whitespace is significant for indentation-based blocks (Python-style). The lexer tracks indentation levels using a stack:

- INDENT token: emitted when indentation increases
- DEDENT token: emitted when indentation decreases
- Tabs and spaces must not be mixed within a file (configurable: tabs OR spaces)

When a brace-delimited block `{ }` is active, indentation tracking is suspended for that block.

### 2.4 Comments

```
// Single-line comment (Java-style)
# Single-line comment (Python-style)

/* Multi-line comment
   (Java-style) */

"""
Docstring (Python-style)
Triple-quoted strings used as documentation.
"""

/**
 * Javadoc-style documentation comment
 */
```

### 2.5 Keywords

UniLang recognizes the **union** of Python and Java keywords:

**Shared keywords:** `class`, `if`, `else`, `for`, `while`, `return`, `import`, `try`, `finally`, `continue`, `break`, `assert`

**Python-origin keywords:** `def`, `elif`, `except`, `raise`, `pass`, `yield`, `async`, `await`, `with`, `as`, `from`, `in`, `is`, `not`, `and`, `or`, `lambda`, `nonlocal`, `global`, `True`, `False`, `None`, `del`

**Java-origin keywords:** `public`, `private`, `protected`, `static`, `final`, `abstract`, `interface`, `enum`, `extends`, `implements`, `new`, `this`, `super`, `void`, `throws`, `throw`, `catch`, `synchronized`, `volatile`, `transient`, `native`, `instanceof`, `switch`, `case`, `default`, `do`, `true`, `false`, `null`

**UniLang-specific keywords:** `bridge`, `vm`, `interop`

### 2.6 Identifiers

```
Identifier     = IdentStart {IdentContinue} ;
IdentStart     = Letter | '_' ;
IdentContinue  = Letter | Digit | '_' ;
Letter         = 'a'..'z' | 'A'..'Z' | UnicodeAlpha ;
Digit          = '0'..'9' ;
```

UniLang is **case-sensitive**: `list` and `List` are different identifiers.

### 2.7 Literals

#### Numeric Literals
```
IntLiteral     = DecimalInt | HexInt | OctalInt | BinaryInt ;
DecimalInt     = Digit {Digit | '_'} ['l' | 'L'] ;
HexInt         = '0' ('x'|'X') HexDigit {HexDigit | '_'} ;
OctalInt       = '0' ('o'|'O') OctalDigit {OctalDigit | '_'} ;
BinaryInt      = '0' ('b'|'B') BinDigit {BinDigit | '_'} ;
FloatLiteral   = Digits '.' [Digits] [Exponent] | Digits Exponent ;
```

#### String Literals
```
StringLiteral  = SingleString | DoubleString | TripleString | FString ;
SingleString   = "'" {Character} "'" ;
DoubleString   = '"' {Character} '"' ;
TripleString   = '"""' {Character} '"""' | "'''" {Character} "'''" ;
FString        = 'f"' {Character | '{' Expression '}'} '"' ;
RawString      = 'r"' {Character} '"' ;
```

UniLang supports **all Python string prefixes** (`f`, `r`, `b`, `rb`) and **Java text blocks** (`"""..."""`).

### 2.8 Operators

UniLang supports the **union** of Python and Java operators:

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `//` (floor div), `%`, `**` (power) |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical | `and`/`&&`, `or`/`||`, `not`/`!` |
| Bitwise | `&`, `|`, `^`, `~`, `<<`, `>>`, `>>>` (unsigned right shift) |
| Assignment | `=`, `+=`, `-=`, `*=`, `/=`, `//=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=` |
| Identity | `is`, `is not` (Python), `instanceof` (Java) |
| Membership | `in`, `not in` |
| Ternary | `x if cond else y` (Python), `cond ? x : y` (Java) |
| Null-safe | `?.` (optional chaining), `??` (null coalescing) |
| Arrow | `->` (lambda / return type annotation) |
| Spread | `*args`, `**kwargs` (Python), `...` (Java varargs) |

**Operator Equivalences:**
- `and` ≡ `&&`
- `or` ≡ `||`
- `not` ≡ `!`
- `True` ≡ `true`
- `False` ≡ `false`
- `None` ≡ `null`

### 2.9 Automatic Semicolon Insertion (ASI)

A newline is treated as a statement terminator when the preceding token is one of:
- Identifier, literal, `return`, `break`, `continue`, `pass`
- `)`, `]`, `}`
- `++`, `--`

A newline is **not** a terminator when the preceding token is one of:
- `,`, `(`, `[`, `{`, operator (binary), `\` (line continuation)

---

## 3. Types

### 3.1 Primitive Types

| UniLang Type | Description | Python Equivalent | Java Equivalent |
|-------------|-------------|-------------------|-----------------|
| `Int` | Arbitrary-precision integer | `int` | `int`/`long`/`BigInteger` |
| `Float` | 64-bit floating point | `float` | `double` |
| `Bool` | Boolean | `bool` | `boolean` |
| `Char` | Single character | `str` (len 1) | `char` |
| `String` | Text | `str` | `String` |
| `Void` | No value | `None` | `void` |

### 3.2 Collection Types

```
List<T>         # Ordered, mutable sequence
Tuple<T1, T2>   # Ordered, immutable sequence
Dict<K, V>      # Key-value mapping
Set<T>          # Unordered unique elements
Array<T>        # Fixed-size, typed array (NumPy-compatible)
```

### 3.3 Special Types

```
Optional<T>     # T | None/null
Union<A, B>     # A | B (Python Union / Java sealed interface)
Dynamic         # Any type — runtime checked (Python duck typing)
Callable<(A, B), R>  # Function type
```

### 3.4 Type Declaration Syntax

```unilang
// Java-style (type before name)
int x = 42
String name = "UniLang"
List<Integer> numbers = new ArrayList<>()

// Python-style (type after name with colon)
x: int = 42
name: str = "UniLang"
numbers: list[int] = [1, 2, 3]

// Inferred (no explicit type)
x = 42              // inferred as Int
name = "UniLang"    // inferred as String
numbers = [1, 2, 3] // inferred as List<Int>
```

### 3.5 Generics

```unilang
// Java-style generics
public class Box<T> {
    private T value;
    public T get() { return value; }
}

// Python-style generics (PEP 695)
class Box[T]:
    def __init__(self, value: T):
        self.value = value
    def get(self) -> T:
        return self.value

// Bounded generics
class NumberBox<T extends Number> { }
class NumberBox[T: (int, float)]: pass
```

---

## 4. Declarations

### 4.1 Variable Declarations

```unilang
// Mutable
x = 10                    // inferred type
int x = 10                // Java-style explicit
x: int = 10               // Python-style explicit
var x = 10                // explicit mutable marker

// Immutable
final int x = 10          // Java-style
x: Final[int] = 10        // Python-style
val x = 10                // UniLang shorthand
```

### 4.2 Constants

```unilang
static final int MAX = 100     // Java-style
MAX: Final = 100               // Python-style
const MAX = 100                // UniLang shorthand
```

---

## 5. Expressions

### 5.1 Python-style Expressions

```unilang
// List comprehension
squares = [x ** 2 for x in range(10)]

// Dict comprehension
mapping = {k: v for k, v in items}

// Set comprehension
unique = {x.lower() for x in words}

// Generator expression
total = sum(x ** 2 for x in range(1000))

// Conditional expression
result = "yes" if condition else "no"

// Walrus operator
if (n := len(data)) > 10:
    print(f"Too many: {n}")

// Unpacking
a, b, *rest = [1, 2, 3, 4, 5]
```

### 5.2 Java-style Expressions

```unilang
// Ternary operator
String result = condition ? "yes" : "no";

// Method reference
Function<String, Integer> parser = Integer::parseInt;

// Lambda
Runnable task = () -> System.out.println("running");
Comparator<String> cmp = (a, b) -> a.compareTo(b);

// instanceof pattern matching (Java 21)
if (obj instanceof String s) {
    System.out.println(s.length());
}

// Switch expression (Java 21)
String result = switch (day) {
    case MONDAY, FRIDAY -> "work";
    case SATURDAY, SUNDAY -> "rest";
    default -> "other";
};

// Array creation
int[] arr = new int[]{1, 2, 3};
```

### 5.3 Unified Expressions

Both styles can coexist and interoperate:

```unilang
// Python list comprehension assigned to Java-typed variable
List<Integer> squares = [x ** 2 for x in range(10)]

// Java lambda used in Python-style code
sorter = (a, b) -> a.compareTo(b)
data.sort(key=sorter)

// Python f-string in Java method
public String greet(String name) {
    return f"Hello, {name}!"
}
```

---

## 6. Statements

### 6.1 Statement Terminators

- Newline (automatic semicolon insertion)
- Explicit semicolon `;`
- Multiple statements on one line: `a = 1; b = 2`

### 6.2 Assignment Statements

```unilang
x = 10                     // simple
x += 1                     // augmented
a, b = b, a                // swap (Python)
int x = 10, y = 20;        // multi-declaration (Java)
```

### 6.3 Print / Output

```unilang
print("hello")                          // Python built-in (always available)
System.out.println("hello")             // Java standard (always available)
print(f"x = {x}, type = {type(x)}")     // f-string support
```

---

## 7. Classes and Objects

### 7.1 Class Declaration

```unilang
// Python-style
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return f"{self.name} speaks"

// Java-style
public class Animal {
    private String name;

    public Animal(String name) {
        this.name = name;
    }

    public String speak() {
        return this.name + " speaks";
    }
}

// Hybrid — Java class with Python methods
public class Animal {
    private String name;

    public Animal(String name) {
        this.name = name
    }

    def speak(self) -> str:
        return f"{self.name} speaks"

    public void train(self):
        # Python-style code inside Java class
        import sklearn
        model = sklearn.linear_model.LinearRegression()
        model.fit(self.training_data, self.labels)
}
```

### 7.2 Inheritance

```unilang
// Python-style
class Dog(Animal):
    def speak(self) -> str:
        return f"{self.name} barks"

// Java-style
public class Dog extends Animal {
    @Override
    public String speak() {
        return this.name + " barks";
    }
}

// Cross-style: Python class extending Java class
class SmartDog(Animal):
    def analyze(self):
        import numpy as np
        return np.mean(self.scores)
```

### 7.3 Interfaces

```unilang
// Java-style interface
public interface Trainable {
    void train(Object data);
    default void evaluate() {
        System.out.println("Evaluating...");
    }
}

// Python-style abstract class (equivalent)
from abc import ABC, abstractmethod
class Trainable(ABC):
    @abstractmethod
    def train(self, data): pass

    def evaluate(self):
        print("Evaluating...")

// Implementation
class MLModel(Trainable):
    def train(self, data):
        # Python ML code here
        pass
```

### 7.4 Enums

```unilang
// Java-style
public enum Status {
    ACTIVE, INACTIVE, PENDING;
}

// Python-style
from enum import Enum
class Status(Enum):
    ACTIVE = 1
    INACTIVE = 2
    PENDING = 3
```

### 7.5 Decorators and Annotations

Decorators (Python) and annotations (Java) use the same `@` syntax and are unified:

```unilang
// Python decorator on Java method
@cache
public List<Integer> computeExpensiveResult(int n) {
    // ...
}

// Java annotation on Python function
@Override
def speak(self) -> str:
    return "woof"

// Spring annotation on Python-style handler
@RestController
class UserController:
    @GetMapping("/users")
    def get_users(self):
        return User.find_all()
```

---

## 8. Functions and Methods

### 8.1 Function Declaration

```unilang
// Python-style
def add(a: int, b: int) -> int:
    return a + b

// Java-style
public static int add(int a, int b) {
    return a + b;
}

// Python-style with default arguments
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

// Java-style with overloading (achieves similar to defaults)
public String greet(String name) {
    return greet(name, "Hello");
}
public String greet(String name, String greeting) {
    return greeting + ", " + name + "!";
}

// Varargs
def func(*args, **kwargs): pass                  // Python
public void func(String... args) { }              // Java
```

### 8.2 Lambda Expressions

```unilang
// Python-style
square = lambda x: x ** 2
filtered = filter(lambda x: x > 0, numbers)

// Java-style
Function<Integer, Integer> square = (x) -> x * x;
Predicate<Integer> positive = x -> x > 0;
```

### 8.3 Async Functions

```unilang
// Python-style
async def fetch_data(url: str):
    response = await http.get(url)
    return response.json()

// Java-style (CompletableFuture)
public CompletableFuture<Response> fetchData(String url) {
    return httpClient.sendAsync(request, BodyHandlers.ofString());
}
```

---

## 9. Control Flow

### 9.1 Conditionals

```unilang
// Python-style
if x > 0:
    print("positive")
elif x == 0:
    print("zero")
else:
    print("negative")

// Java-style
if (x > 0) {
    System.out.println("positive");
} else if (x == 0) {
    System.out.println("zero");
} else {
    System.out.println("negative");
}

// Hybrid (braces, no parens, no colon)
if x > 0 {
    print("positive")
} else if x == 0 {
    print("zero")
} else {
    print("negative")
}
```

### 9.2 Loops

```unilang
// Python for loop
for item in collection:
    process(item)

// Java for loop
for (int i = 0; i < 10; i++) {
    process(i);
}

// Python-style for with braces
for item in collection {
    process(item)
}

// While loop (both styles)
while condition:
    do_something()

while (condition) {
    doSomething();
}

// Do-while (Java only — no Python equivalent)
do {
    process();
} while (condition);

// For-each (Java-style)
for (String item : collection) {
    process(item);
}
```

### 9.3 Pattern Matching

```unilang
// Python match (3.10+)
match command:
    case "quit":
        exit()
    case "hello":
        print("Hi!")
    case _:
        print("Unknown")

// Java switch expression (21+)
String result = switch (command) {
    case "quit" -> { System.exit(0); yield ""; }
    case "hello" -> "Hi!";
    default -> "Unknown";
};
```

---

## 10. Exception Handling

### 10.1 Try-Catch/Except

```unilang
// Python-style
try:
    result = risky_operation()
except ValueError as e:
    print(f"Value error: {e}")
except Exception as e:
    print(f"Error: {e}")
finally:
    cleanup()

// Java-style
try {
    result = riskyOperation();
} catch (ValueError e) {
    System.out.println("Value error: " + e.getMessage());
} catch (Exception e) {
    System.out.println("Error: " + e.getMessage());
} finally {
    cleanup();
}
```

### 10.2 Throwing/Raising Exceptions

```unilang
// Python-style
raise ValueError("invalid input")

// Java-style
throw new IllegalArgumentException("invalid input");
```

### 10.3 Cross-VM Exception Propagation

When an exception crosses a VM boundary, it is wrapped:

```unilang
// Python code calling Java method that throws
try:
    java_object.riskyMethod()
except JavaException as e:
    # e wraps the original java.lang.Exception
    print(f"Java error: {e.java_class}: {e.message}")

// Java code calling Python function that raises
try {
    pythonFunc.call();
} catch (PythonException e) {
    // e wraps the original Python exception
    System.out.println("Python error: " + e.getPythonType() + ": " + e.getMessage());
}
```

---

## 11. Modules and Imports

### 11.1 Import Syntax

```unilang
// Python-style imports
import numpy as np
from collections import defaultdict
from sklearn.linear_model import LinearRegression

// Java-style imports
import java.util.List
import java.util.concurrent.*          // Wildcard
import static java.lang.Math.PI       // Static import

// Python-style import of Java packages
from java.util import List, Map
from java.util.concurrent import ExecutorService as Executor

// Mixed in same file
import numpy as np
import java.util.concurrent.Executors
from sklearn.ensemble import RandomForestClassifier
```

### 11.2 Module Definition

Each `.uniL` file is a module. Modules can be organized into packages using directory structure:

```
src/
├── main.uniL              # main module
├── models/
│   ├── __init__.uniL      # Python-style package init
│   ├── user.uniL
│   └── product.uniL
└── services/
    ├── package-info.uniL   # Java-style package info
    ├── UserService.uniL
    └── MLService.uniL
```

### 11.3 Visibility

```unilang
// Python convention (underscore prefix = private)
def _internal_helper():
    pass

// Java explicit visibility
public class PublicAPI { }
private void internalHelper() { }
protected void subclassHelper() { }

// UniLang: both conventions honored
// _underscore is private to module
// public/private/protected keywords are explicit
```

---

## 12. Concurrency

### 12.1 Java Threading

```unilang
import java.util.concurrent.Executors
import java.util.concurrent.CompletableFuture

// Thread pool
executor = Executors.newFixedThreadPool(4)

// Submit tasks (Python functions on Java threads)
def compute(data):
    return heavy_ml_computation(data)

future = executor.submit(lambda: compute(dataset))
result = future.get()

// CompletableFuture chaining
CompletableFuture.supplyAsync(() -> loadData())
    .thenApply(data -> process(data))
    .thenAccept(result -> print(result))
```

### 12.2 Synchronized Blocks

```unilang
// Java-style synchronized
synchronized (lock) {
    shared_state.update(value)
}

// Python-style with Lock
from threading import Lock
lock = Lock()
with lock:
    shared_state.update(value)

// Both work — the runtime ensures thread safety across VMs
```

### 12.3 Async/Await

```unilang
// Python-style async
async def fetch_and_process(url):
    data = await http_get(url)
    return process(data)

// Java-style virtual threads (Project Loom)
Thread.startVirtualThread(() -> {
    data = httpGet(url)
    process(data)
})
```

---

## 13. Interoperability Semantics

### 13.1 Automatic Type Coercion

When a value crosses a VM boundary, automatic coercion applies:

```unilang
// Python list → Java List
def get_numbers():
    return [1, 2, 3]            # Python list

List<Integer> nums = get_numbers()  # Automatically coerced

// Java ArrayList → Python list
public ArrayList<String> getNames() {
    return new ArrayList<>(List.of("Alice", "Bob"));
}

names = getNames()              # Usable as Python list
names.append("Charlie")        # Python list operations work
```

### 13.2 NumPy Interop

```unilang
import numpy as np

// NumPy array created in Python context
data = np.random.randn(1000, 10)

// Accessible from Java context via zero-copy bridge
public void processData(Array<Double> data) {
    // data points to same memory as NumPy array
    for (int i = 0; i < data.length; i++) {
        data[i] = Math.abs(data[i]);
    }
}
// Changes visible in Python context immediately
```

### 13.3 Exception Interop

See Section 10.3.

### 13.4 Callback Interop

```unilang
// Pass Python function to Java API
def my_handler(event):
    print(f"Event received: {event}")

button.addActionListener(my_handler)  # Python func as Java interface

// Pass Java lambda to Python API
Comparator<Integer> cmp = (a, b) -> b - a;
sorted_list = sorted(numbers, key=cmp)  # Java lambda as Python key function
```

---

## 14. Grammar Summary

This is a simplified grammar overview. The full formal grammar is maintained in `grammar.ebnf`.

```ebnf
Program        = {Statement} ;

Statement      = Declaration
               | Expression [';']
               | ControlFlow
               | ImportStatement
               | ClassDeclaration
               | FunctionDeclaration
               ;

Declaration    = [Modifiers] TypedDecl | UntypedDecl ;
TypedDecl      = Type Identifier '=' Expression           (* Java-style *)
               | Identifier ':' Type '=' Expression        (* Python-style *)
               ;
UntypedDecl    = Identifier '=' Expression ;

FunctionDeclaration
               = 'def' Identifier '(' Params ')' ['->' Type] ':' Block   (* Python *)
               | [Modifiers] Type Identifier '(' Params ')' BraceBlock   (* Java *)
               ;

ClassDeclaration
               = 'class' Identifier ['(' BaseList ')'] ':' Block          (* Python *)
               | [Modifiers] 'class' Identifier ['extends' Type]
                 ['implements' TypeList] BraceBlock                        (* Java *)
               ;

Block          = IndentedBlock | BraceBlock ;
IndentedBlock  = NEWLINE INDENT {Statement} DEDENT ;
BraceBlock     = '{' {Statement} '}' ;

ControlFlow    = IfStatement | ForStatement | WhileStatement
               | TryStatement | SwitchStatement | MatchStatement ;

ImportStatement = PythonImport | JavaImport ;
PythonImport   = 'import' DottedName ['as' Identifier]
               | 'from' DottedName 'import' ImportList ;
JavaImport     = 'import' ['static'] QualifiedName [';'] ;

(* ... full grammar continues in grammar.ebnf *)
```

---

## Appendix A: Reserved Words (Complete List)

```
abstract   and        as         assert     async      await
bool       break      bridge     byte       case       catch
char       class      const      continue   def        default
del        do         double     elif       else       enum
except     extends    false      False      final      finally
float      for        from       global     if         implements
import     in         instanceof int        interface  interop
is         lambda     long       match      native     new
None       nonlocal   not        null       or         pass
private    protected  public     raise      return     short
static     strictfp   super      switch     synchronized
this       throw      throws     transient  true       True
try        val        var        vm         void       volatile
while      with       yield
```

---

*This specification is normative for the UniLang compiler. Deviations from this specification are compiler bugs.*
