# UniLang — Type System Design

**Version:** 1.0.0-draft
**Last Updated:** 2026-03-19

---

## Overview

UniLang uses a **gradual type system** that bridges Python's dynamic typing and Java's static typing. Code can range from fully untyped (Python-style) to fully typed (Java-style), with the compiler providing maximum safety at each level.

---

## Type Spectrum

```
Fully Dynamic                                              Fully Static
(Python, no hints)                                         (Java-style)
     │                                                          │
     ▼                                                          ▼
┌─────────┬──────────┬──────────────┬───────────────┬──────────┐
│  Level 0│  Level 1 │   Level 2    │    Level 3    │  Level 4 │
│  No     │  Partial │   Full Python│    Mixed      │  Full    │
│  types  │  hints   │   type hints │    Java+Py    │  Java    │
│         │          │              │    types      │  types   │
│  x = 5  │  x: int  │  def f(x:int)│  public int  │  int x=5;│
│         │  = 5     │  -> int:     │  f(int x) {}  │          │
└─────────┴──────────┴──────────────┴───────────────┴──────────┘
   ▲                                                       ▲
   │ Runtime checks only                 Full compile-time │
   │ (errors at execution)              checking (errors   │
   │                                     at compile time)  │
```

### Guarantees Per Level

| Level | Compile-time Checks | Runtime Checks | Use Case |
|-------|-------------------|----------------|----------|
| 0 | Syntax only | All type checks at runtime | Quick scripts, prototyping |
| 1 | Annotated variables checked | Unannotated variables at runtime | Gradual migration |
| 2 | All Python-visible types checked | Dynamic dispatch for duck typing | Python best practice |
| 3 | Full static checking where types known | Bridge calls validated | Interop code |
| 4 | Complete static type safety | None needed | Performance-critical Java code |

---

## Bidirectional Type Inference

UniLang's type inference flows in both directions:

### Forward Inference (Bottom-Up)
```unilang
x = 42              // x: Int (inferred from literal)
y = x + 1           // y: Int (inferred from operator + operand types)
z = [x, y]          // z: List<Int> (inferred from element types)
```

### Backward Inference (Top-Down)
```unilang
List<String> names = get_names()    // get_names() must return List<String>-compatible
int result = compute(data)          // compute() must return Int-compatible
```

### Cross-VM Inference
```unilang
import numpy as np
// np.array() is known to return numpy.ndarray → mapped to Array<Float>
data = np.random.randn(100, 10)    // data: Array<Float>

// When passed to Java context:
public void process(double[][] matrix) {
    // data is automatically marshaled from Array<Float> to double[][]
}
```

---

## Type Compatibility Rules

### Subtyping

```
Int <: Float <: Number
Bool <: Int
String <: Object
List<T> <: Iterable<T>
Dict<K,V> <: Mapping<K,V>
```

### Coercion (Implicit)

| From | To | Condition |
|------|----|-----------|
| `Int` | `Float` | Always (widening) |
| `Bool` | `Int` | Always (`True`→1, `False`→0) |
| `python list` | `Java List<T>` | When crossing VM boundary |
| `python dict` | `Java Map<K,V>` | When crossing VM boundary |
| `Java String` | `python str` | When crossing VM boundary |

### No Implicit Coercion (Explicit Required)

| From | To | Method |
|------|----|--------|
| `Float` | `Int` | `int(x)` or `(int) x` |
| `String` | `Int` | `int(x)` or `Integer.parseInt(x)` |
| `Object` | specific type | Cast: `(Type) x` or type check |

---

## The `Dynamic` Type

`Dynamic` is UniLang's escape hatch for Python-style duck typing:

```unilang
// Explicit dynamic
x: Dynamic = get_something()
x.any_method()           // Compiles — checked at runtime
x + 42                   // Compiles — checked at runtime
x.nonexistent_field      // Compiles — RuntimeError at execution

// Implicit dynamic (no type annotation in Python-style code)
def process(data):       // data: Dynamic (implicit)
    return data.transform()  // Checked at runtime
```

### Dynamic Propagation Rules

1. `Dynamic` op `T` → `Dynamic` (operations with Dynamic produce Dynamic)
2. `Dynamic` assignment to `T` → runtime type check
3. `Dynamic` passed to typed parameter → runtime type check
4. Function with no return type returning `Dynamic` → caller gets `Dynamic`

### Opting Out of Dynamic

```unilang
// File-level strict mode
# unilang: strict=true

// All variables must have types (inferred or explicit)
x = 42              // OK: inferred as Int
y = unknown_func()  // ERROR: cannot infer type — add annotation
y: Int = unknown_func()  // OK: runtime check that result is Int
```

---

## Generics

### Java-Style Generics
```unilang
public class Box<T> {
    private T value;
    public T get() { return value; }
    public void set(T value) { this.value = value; }
}

// Bounded generics
public class NumberBox<T extends Number> {
    public double sum(List<T> items) {
        return items.stream().mapToDouble(Number::doubleValue).sum();
    }
}
```

### Python-Style Generics (PEP 695)
```unilang
class Box[T]:
    def __init__(self, value: T):
        self.value = value
    def get(self) -> T:
        return self.value

// Bounded
class NumberBox[T: (int, float)]:
    def sum(self, items: list[T]) -> float:
        return sum(float(x) for x in items)
```

### Cross-Style Generic Usage
```unilang
// Java generic class used from Python style
box = Box[str]("hello")    // Python syntax for Java generic
value: str = box.get()

// Python generic class used from Java style
Box<String> box = new Box<>("hello");  // Java syntax for Python generic
String value = box.get();
```

---

## Null Safety

UniLang distinguishes nullable and non-nullable types:

```unilang
// Non-nullable (default in Java-style)
String name = "Alice"       // Cannot be null
name = null                 // COMPILE ERROR

// Nullable (explicit)
String? name = null         // OK (Java-style Optional)
name: Optional[str] = None  // OK (Python-style Optional)

// Safe access
name?.length()              // Returns null if name is null
name ?? "default"           // Returns "default" if name is null

// Python None handling
x = some_func()            // In Python style, None is always possible
if x is not None:
    x.process()            // Safe: compiler knows x is not None here
```

---

*This document is part of the UniLang specification. Changes require an RFC and community review.*
