# Language Overview

UniLang lets you write Python-style and Java-style code in the same `.uniL` file. Both syntaxes compile to the same internal representation and interoperate seamlessly.

---

## Variable Declarations

### Python Style

```unilang
x = 10                    // type inferred
name = "Alice"
scores = [90, 85, 95]
x: int = 10               // explicit type annotation
```

### Java Style

```unilang
int x = 10;
String name = "Alice";
List<Integer> scores = new ArrayList<>();
final int MAX = 100;      // immutable
```

### UniLang Shorthand

```unilang
var x = 10                // explicit mutable
val y = 20                // explicit immutable
const MAX = 100           // constant
```

---

## Functions

### Python Style

```unilang
def add(a, b):
    return a + b

def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

// Lambda
square = lambda x: x ** 2
```

### Java Style

```unilang
public static int add(int a, int b) {
    return a + b;
}

public String greet(String name) {
    return "Hello, " + name + "!";
}

// Lambda
Function<Integer, Integer> square = (x) -> x * x;
```

### Async Functions

```unilang
// Python-style
async def fetch_data(url: str):
    response = await http.get(url)
    return response.json()

// Java-style
public CompletableFuture<Response> fetchData(String url) {
    return httpClient.sendAsync(request, BodyHandlers.ofString());
}
```

---

## Classes

### Python Style

```unilang
class Animal:
    def __init__(self, name: str):
        self.name = name

    def speak(self) -> str:
        return f"{self.name} speaks"
```

### Java Style

```unilang
public class Animal {
    private String name;

    public Animal(String name) {
        this.name = name;
    }

    public String speak() {
        return this.name + " speaks";
    }
}
```

### Hybrid (Java class with Python methods)

```unilang
public class Animal {
    private String name;

    public Animal(String name) {
        this.name = name
    }

    def speak(self) -> str:
        return f"{self.name} speaks"
}
```

### Inheritance

```unilang
// Python style
class Dog(Animal):
    def speak(self) -> str:
        return f"{self.name} barks"

// Java style
public class Dog extends Animal {
    @Override
    public String speak() {
        return this.name + " barks";
    }
}
```

### Interfaces

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
```

### Enums

```unilang
// Java style
public enum Status { ACTIVE, INACTIVE, PENDING; }

// Python style
from enum import Enum
class Status(Enum):
    ACTIVE = 1
    INACTIVE = 2
    PENDING = 3
```

---

## Control Flow

### If/Else

```unilang
// Python style
if x > 0:
    print("positive")
elif x == 0:
    print("zero")
else:
    print("negative")

// Java style
if (x > 0) {
    System.out.println("positive");
} else if (x == 0) {
    System.out.println("zero");
} else {
    System.out.println("negative");
}
```

### For Loops

```unilang
// Python for loop
for item in collection:
    process(item)

// Java for loop
for (int i = 0; i < 10; i++) {
    process(i);
}

// Java-style for-each
for (String item : collection) {
    process(item);
}
```

### While Loops

```unilang
// Python style
while condition:
    do_something()

// Java style
while (condition) {
    doSomething();
}

// Do-while (Java only)
do {
    process();
} while (condition);
```

### Pattern Matching

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

### Ternary Expressions

```unilang
// Python style
result = "yes" if condition else "no"

// Java style
String result = condition ? "yes" : "no";
```

---

## Imports

```unilang
// Python-style imports
import numpy as np
from collections import defaultdict
from sklearn.linear_model import LinearRegression

// Java-style imports
import java.util.List
import java.util.concurrent.*
import static java.lang.Math.PI

// Mixing in the same file
import numpy as np
import java.util.concurrent.Executors
```

---

## String Types

UniLang supports all Python and Java string formats:

```unilang
// Regular strings
s1 = "hello"
s2 = 'hello'

// Triple-quoted strings (multi-line)
s3 = """
This is a
multi-line string
"""

// F-strings (format strings)
name = "Alice"
s4 = f"Hello, {name}!"
s5 = f"2 + 2 = {2 + 2}"

// Raw strings (no escape processing)
s6 = r"C:\Users\name\file"
```

---

## Comments

UniLang supports three comment styles:

```unilang
// Single-line comment (Java-style)

# Single-line comment (Python-style)

/* Multi-line comment
   spanning multiple lines
   (Java-style) */

"""
Docstring (Python-style)
Used for documentation.
"""

/**
 * Javadoc-style documentation comment
 */
```

---

## Operators

UniLang supports the union of Python and Java operators:

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `//` (floor div), `%`, `**` (power) |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical | `and`/`&&`, `or`/`||`, `not`/`!` |
| Bitwise | `&`, `\|`, `^`, `~`, `<<`, `>>` |
| Assignment | `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=` |
| Identity | `is`, `is not` (Python), `instanceof` (Java) |
| Membership | `in`, `not in` |
| Null-safe | `?.` (optional chaining), `??` (null coalescing) |

### Operator Equivalences

| Python | Java | Meaning |
|--------|------|---------|
| `and` | `&&` | Logical AND |
| `or` | `\|\|` | Logical OR |
| `not` | `!` | Logical NOT |
| `True` | `true` | Boolean true |
| `False` | `false` | Boolean false |
| `None` | `null` | Null/no value |

---

## Comprehensions and Streams

```unilang
// Python list comprehension
squares = [x ** 2 for x in range(10)]

// Python dict comprehension
mapping = {k: v for k, v in items}

// Java stream
List<Integer> evens = numbers.stream()
    .filter(x -> x % 2 == 0)
    .collect(Collectors.toList())
```

---

## Decorators and Annotations

Decorators (Python) and annotations (Java) use the same `@` syntax:

```unilang
// Python decorator on Java method
@cache
public List<Integer> computeExpensiveResult(int n) { ... }

// Java annotation on Python function
@Override
def speak(self) -> str:
    return "woof"
```

---

**Previous**: [[IDE Setup]] | **Next**: [[Python Java Interop]]
