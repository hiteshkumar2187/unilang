# Type System

UniLang uses a **gradual type system** that bridges Python's dynamic typing and Java's static typing. Code can range from fully untyped to fully typed, with the compiler providing maximum safety at each level.

---

## The Type Spectrum

UniLang supports five levels of typing:

| Level | Style | Example | Checking |
|-------|-------|---------|----------|
| 0 | No types | `x = 5` | Runtime only |
| 1 | Partial hints | `x: int = 5` | Annotated vars checked at compile time |
| 2 | Full Python hints | `def f(x: int) -> int:` | All visible types checked |
| 3 | Mixed Java + Python | `public int f(int x) {}` | Full static where types known |
| 4 | Full Java types | `int x = 5;` | Complete compile-time safety |

All five levels can coexist in a single file. You can add types gradually as code matures.

---

## Implicit Casting Rules

The compiler applies these conversions automatically:

| From | To | Behavior | Example |
|------|----|----------|---------|
| `int` (Java) | `Dynamic` (Python) | Always works, zero cost | `x = java_int` |
| `Dynamic` (Python) | `int` (Java) | Runtime type check inserted | `int x = python_var` |
| `Int` | `Float` | Widening, automatic | `float f = 10` |
| `Int` | `Double` | Widening, automatic | `double d = 10` |
| `Float` | `Double` | Widening, automatic | `double d = 3.14f` |
| `Bool` | `Int` | Widening: `True`->1, `False`->0 | `int x = True` |
| `Any` | `String` | Automatic with `+` | `"count: " + 42` |
| `null`/`None` | `Optional<T>` | Null safety, automatic | `Optional<String> s = None` |
| `python list` | `Java List<T>` | VM boundary crossing | `List<int> jl = [1, 2, 3]` |
| `python dict` | `Java Map<K,V>` | VM boundary crossing | `Map<String, int> m = {"a": 1}` |

### Explicit Conversions (Required)

These can lose information or fail, so the compiler requires explicit casts:

| From | To | Python Syntax | Java Syntax |
|------|----|--------------|-------------|
| `Float` | `Int` | `int(f)` | `(int) f` |
| `String` | `Int` | `int(s)` | `Integer.parseInt(s)` |
| `String` | `Float` | `float(s)` | `Float.parseFloat(s)` |
| `Object` | Specific type | N/A | `(Type) obj` |

The compiler emits a warning for narrowing conversions that could lose data:

```unilang
float pi = 3.14159
int truncated = int(pi)     // WARNING: narrowing Float -> Int (value will be 3)
int truncated = (int) pi    // No warning: explicit cast signals intent
```

---

## The `Dynamic` Type

`Dynamic` is UniLang's escape hatch for Python-style duck typing. Any variable without a type annotation has compile-time type `Dynamic`:

```unilang
x = get_something()         // x: Dynamic
x.any_method()              // Compiles — checked at runtime
x + 42                      // Compiles — checked at runtime
x.nonexistent_field         // Compiles — RuntimeError at execution
```

### Dynamic Propagation Rules

1. `Dynamic` op `T` produces `Dynamic` (operations with Dynamic propagate)
2. `Dynamic` assigned to `T` triggers a runtime type check
3. `Dynamic` passed to typed parameter triggers a runtime type check
4. Function with no return type returning `Dynamic` gives caller `Dynamic`

### Stopping Dynamic Propagation

Pin to a concrete type to stop propagation:

```unilang
x = unknown_func()          // x: Dynamic
y = x + 10                  // y: Dynamic (Dynamic + Int -> Dynamic)
int result = x + 10         // result: Int (runtime check on x)
```

### Strict Mode

Opt out of `Dynamic` entirely with file-level strict mode:

```unilang
# unilang: strict=true

x = 42                      // OK: inferred as Int
y = unknown_func()          // ERROR: cannot infer type — add annotation
y: Int = unknown_func()     // OK: runtime check that result is Int
```

---

## Type Annotations

### Python-Style (type after name)

```unilang
x: int = 42
name: str = "UniLang"
numbers: list[int] = [1, 2, 3]

def add(a: int, b: int) -> int:
    return a + b
```

### Java-Style (type before name)

```unilang
int x = 42;
String name = "UniLang";
List<Integer> numbers = new ArrayList<>();

public int add(int a, int b) {
    return a + b;
}
```

### Inferred (no explicit type)

```unilang
x = 42              // inferred as Int
name = "UniLang"    // inferred as String
numbers = [1, 2, 3] // inferred as List<Int>
```

---

## Primitive Types

| UniLang Type | Description | Python Equivalent | Java Equivalent |
|-------------|-------------|-------------------|-----------------|
| `Int` | Arbitrary-precision integer | `int` | `int`/`long`/`BigInteger` |
| `Float` | 64-bit floating point | `float` | `double` |
| `Bool` | Boolean | `bool` | `boolean` |
| `Char` | Single character | `str` (len 1) | `char` |
| `String` | Text | `str` | `String` |
| `Void` | No value | `None` | `void` |

## Collection Types

| Type | Description |
|------|-------------|
| `List<T>` | Ordered, mutable sequence |
| `Tuple<T1, T2>` | Ordered, immutable sequence |
| `Dict<K, V>` | Key-value mapping |
| `Set<T>` | Unordered unique elements |
| `Array<T>` | Fixed-size typed array (NumPy-compatible) |

## Special Types

| Type | Description |
|------|-------------|
| `Optional<T>` | `T` or `None`/`null` |
| `Union<A, B>` | Either `A` or `B` |
| `Dynamic` | Any type, runtime checked |
| `Callable<(A, B), R>` | Function type |

---

## When Types Are Checked vs. Deferred to Runtime

| Situation | Compile Time? | Runtime? |
|-----------|--------------|----------|
| Both sides statically typed | Yes | No |
| One side `Dynamic`, other typed | No | Yes (check inserted) |
| Both sides `Dynamic` | No | Yes (on operation) |
| Narrowing conversion (Float->Int) | Warning | Check inserted |
| `Dynamic` passed to typed parameter | No | Yes (at call site) |
| Type annotation present | Yes | No (unless `Dynamic` source) |

---

## Generics

### Java-Style

```unilang
public class Box<T> {
    private T value;
    public T get() { return value; }
    public void set(T value) { this.value = value; }
}

public class NumberBox<T extends Number> { ... }
```

### Python-Style (PEP 695)

```unilang
class Box[T]:
    def __init__(self, value: T):
        self.value = value
    def get(self) -> T:
        return self.value

class NumberBox[T: (int, float)]: pass
```

### Cross-Style Usage

```unilang
// Java generic class used from Python style
box = Box[str]("hello")

// Python generic class used from Java style
Box<String> box = new Box<>("hello");
```

---

## Null Safety

```unilang
// Non-nullable (default in Java-style)
String name = "Alice"       // Cannot be null
name = null                 // COMPILE ERROR

// Nullable (explicit)
String? name = null         // Java-style
name: Optional[str] = None  // Python-style

// Safe access operators
name?.length()              // Returns null if name is null
name ?? "default"           // Returns "default" if name is null

// Python None handling
x = some_func()
if x is not None:
    x.process()             // Compiler knows x is not None here
```

---

## Bidirectional Type Inference

UniLang infers types in both directions:

### Forward (bottom-up)

```unilang
x = 42              // x: Int (from literal)
y = x + 1           // y: Int (from operator + operands)
z = [x, y]          // z: List<Int> (from elements)
```

### Backward (top-down)

```unilang
List<String> names = get_names()    // get_names() must return List<String>-compatible
int result = compute(data)          // compute() must return Int-compatible
```

---

**Previous**: [[Python Java Interop]] | **Next**: [[Standard Library Reference]]
