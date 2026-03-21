# Python Java Interop

UniLang's core feature is seamless cross-syntax interoperability. Python-style and Java-style code share the same scope, the same type hierarchy, and the same runtime. There is no foreign-function interface or marshalling layer you invoke by hand.

---

## Philosophy

**"Write what feels natural, it just works."**

A Python developer and a Java developer can open the same `.uniL` file, each write code in the style they know best, and have it compile and run correctly without explicit bridges or adapters.

The compiler's context-stack parser determines the syntax origin of every construct, and the gradual type system resolves types across both styles automatically. By the time code reaches the Unified IR, both styles have been lowered to the same representation.

---

## Cross-Syntax Variable Operations

Both groups of variables live in the same scope with full visibility to each other:

```unilang
// Python-style declarations
name = "Alice"
age = 30
scores = [90, 85, 95]

// Java-style declarations
String greeting = "Hello"
int count = 10
List<Integer> numbers = new ArrayList<>()

// Cross-syntax operations — all valid
message = greeting + " " + name     // Java String + Python str -> String
total = count + age                  // Java int + Python int -> int
combined = scores + numbers          // Python list + Java List -> merged list
is_adult = age >= count              // Python int >= Java int -> bool
```

This works because Python's `int` and Java's `int` resolve to the same `Type::Int` internally. There is no conversion because there is nothing to convert.

---

## Function Calling Patterns

UniLang supports every combination of cross-syntax function calls. Here is a summary of the 9 key patterns:

### 1. Python function, Python class

```unilang
class Calculator:
    def add(self, x, y):
        return x + y

calc = Calculator()
result = calc.add(5, 3)    // Dynamic return type
```

### 2. Java method, Java class

```unilang
public class Calculator {
    public int add(int a, int b) { return a + b; }
}
Calculator calc = new Calculator();
int result = calc.add(5, 3);    // Statically typed
```

### 3. Mixed class (Java class with Python methods)

```unilang
public class DataProcessor {
    private List<Integer> data;

    public DataProcessor(List<Integer> data) { this.data = data; }

    def process(self):
        return sum(self.data) / len(self.data)

    public double getAverage() {
        return self.process();    // Java method calls Python method
    }
}
```

### 4. Python function accepting Java-typed arguments

```unilang
def calculate_tax(amount, rate):
    return amount * rate

double price = 99.99;
tax = calculate_tax(price, 0.08)    // Java vars passed to Python func
```

### 5. Java method accepting Python variables

```unilang
public static String format(String template, Object... args) {
    return String.format(template, args);
}

name = "Alice"    // Python variable (Dynamic, runtime type: String)
result = format("Hello %s", name)    // Runtime type check inserted
```

### 6. Static methods and module-level functions

```unilang
// Java-style static call
double root = Math.sqrt(16)

// Python-style module call
import math
root = math.sqrt(16)
```

### 7. Constructor patterns

```unilang
// All four produce the same object:
Calculator calc1 = new Calculator()    // Java style
calc2 = Calculator()                   // Python style
calc3 = new Calculator()               // Mixed: new + Python binding
Calculator calc4 = Calculator()        // Mixed: Java type + Python call
```

### 8. Lambda and closure interop

```unilang
// Python lambda in Java stream
list.stream()
    .map(lambda x: x * 2)
    .filter(lambda x: x > 10)
    .collect(Collectors.toList())

// Java lambda in Python built-in
sorted_data = sorted(data, key=(x) -> -x)
```

### 9. Keyword arguments and overloading

```unilang
public class HttpClient {
    public Response get(String url) { ... }
    public Response get(String url, Map<String, String> headers) { ... }

    def post(self, url, data=None, headers=None, timeout=30):
        ...
}

client = HttpClient()
client.get("https://api.example.com")           // Java overload
client.post("https://api.example.com", data=payload, timeout=60)  // Python kwargs
```

---

## Collection Interop

Python lists and Java Lists, Python dicts and Java Maps are unified. Operations from either language work on collections from either style.

```unilang
py_list = [1, 2, 3]
ArrayList<Integer> java_list = new ArrayList<>()
java_list.add(4)

// Cross-style operations
combined = py_list + java_list          // [1, 2, 3, 4]
java_list.append(5)                     // Compiler maps to .add(5)
py_list.add(6)                          // Compiler maps to .append(6)
length = len(java_list)                 // Python built-in on Java collection

// Iteration works across styles
for item in java_list:                  // Python for-in on Java List
    print(item)

for (int item : py_list) {             // Java for-each on Python list
    System.out.println(item)
}
```

### Dictionary / Map interop

```unilang
py_dict = {"name": "Alice", "age": 30}
HashMap<String, Object> java_map = new HashMap<>()
java_map.put("name", "Bob")

print(java_map["name"])                // Bracket access mapped to .get()
String name = py_dict.get("name")      // .get() mapped to bracket access

if "name" in java_map:                 // Python 'in' works on Java maps
    print("found")

merged = {**py_dict, **java_map}       // Python unpacking works with Java maps
```

---

## Exception Handling Across Styles

UniLang maps Python and Java exceptions into a single hierarchy:

| Python Exception | Java Exception | Shared Name |
|-----------------|----------------|-------------|
| `RuntimeError` | `RuntimeException` | Either works |
| `TypeError` | `ClassCastException` | Either works |
| `ValueError` | `IllegalArgumentException` | Either works |
| `IOError` | `IOException` | Either works |
| `KeyError` | `NoSuchElementException` | Either works |
| `IndexError` | `ArrayIndexOutOfBoundsException` | Either works |

### Python except catches Java exceptions

```unilang
try:
    content = FileReader.read("/etc/config")
except IOError as e:
    print(f"Failed to read file: {e}")
```

### Java catch catches Python exceptions

```unilang
try {
    result = parse_json(input_text)
} catch (ValueError e) {
    System.out.println("Parse error: " + e.getMessage())
}
```

### Mixed exception handling

```unilang
try:
    data = FileReader.read("data.json")      // May throw Java IOException
    parsed = parse_json(data)                 // May raise Python ValueError
except IOError as e:
    print("File error")
except ValueError as e:
    print("Parse error")
```

---

## Quick Reference Table

| Task | Python Style | Java Style |
|------|-------------|-----------|
| Declare variable | `x = 10` | `int x = 10` |
| Declare string | `s = "hello"` | `String s = "hello"` |
| Declare list | `lst = [1, 2, 3]` | `List<Integer> lst = new ArrayList<>()` |
| Define function | `def foo(x):` | `void foo(int x) {}` |
| Define class | `class Foo:` | `public class Foo {}` |
| Inherit | `class Foo(Bar):` | `class Foo extends Bar {}` |
| Create object | `obj = MyClass()` | `MyClass obj = new MyClass()` |
| Lambda | `lambda x: x * 2` | `(x) -> x * 2` |
| If statement | `if x > 0:` | `if (x > 0) {}` |
| For loop | `for i in range(10):` | `for (int i = 0; i < 10; i++) {}` |
| For-each | `for item in items:` | `for (var item : items) {}` |
| Try/except | `try: ... except E as e:` | `try {} catch (E e) {}` |
| Raise/throw | `raise ValueError("msg")` | `throw new ValueError("msg")` |
| Add to list | `lst.append(x)` | `lst.add(x)` |
| Dict/Map access | `d["key"]` | `m.get("key")` |
| Check contains | `x in collection` | `collection.contains(x)` |

---

## Best Practices

| Scenario | Recommended Style | Reason |
|----------|------------------|--------|
| Quick scripting / prototyping | Python | Less boilerplate |
| Performance-critical hot paths | Java | Static types enable optimizations |
| Public API boundaries | Java | Explicit types as documentation |
| Data exploration | Python | Dynamic typing for flexibility |
| ML / data science | Python | Ecosystem conventions |
| Concurrent / threaded code | Java | Explicit types prevent subtle bugs |
| Mixed teams | Both | Each developer uses their native style |

---

**Previous**: [[Language Overview]] | **Next**: [[Type System]]
