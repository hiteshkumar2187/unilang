# Standard Library Reference

UniLang includes 35+ built-in functions available without any imports. These are registered in the VM and callable from both Python-style and Java-style code.

---

## I/O

### `print(value)`

Outputs a value to standard output, followed by a newline.

```unilang
print("Hello, world!")
print(42)
print(f"x = {x}")
```

### `input(prompt)`

Reads a line of text from standard input. Returns a `String`.

```unilang
name = input("Enter your name: ")
print(f"Hello, {name}!")
```

---

## Type Conversion

### `int(value)`

Converts a value to an integer. Truncates floats toward zero. Parses strings.

```unilang
int(3.7)       // 3
int("42")      // 42
int(True)      // 1
```

### `float(value)`

Converts a value to a floating-point number.

```unilang
float(42)       // 42.0
float("3.14")   // 3.14
float(True)     // 1.0
```

### `str(value)`

Converts any value to its string representation.

```unilang
str(42)         // "42"
str(3.14)       // "3.14"
str(True)       // "True"
str([1, 2, 3])  // "[1, 2, 3]"
```

### `bool(value)`

Converts a value to a boolean. Falsy values: `0`, `0.0`, `""`, `[]`, `None`/`null`, `False`/`false`.

```unilang
bool(1)         // True
bool(0)         // False
bool("")        // False
bool("hello")   // True
bool([])        // False
```

---

## Math

### `abs(x)`

Returns the absolute value of a number.

```unilang
abs(-5)       // 5
abs(3.14)     // 3.14
abs(-2.7)     // 2.7
```

### `min(a, b)` / `min(collection)`

Returns the smallest value.

```unilang
min(3, 7)           // 3
min([5, 2, 8, 1])   // 1
```

### `max(a, b)` / `max(collection)`

Returns the largest value.

```unilang
max(3, 7)           // 7
max([5, 2, 8, 1])   // 8
```

### `pow(base, exponent)`

Returns `base` raised to the power of `exponent`.

```unilang
pow(2, 10)    // 1024
pow(3, 0.5)   // 1.7320508075688772
```

### `sqrt(x)`

Returns the square root of `x`.

```unilang
sqrt(16)      // 4.0
sqrt(2)       // 1.4142135623730951
```

### `floor(x)`

Returns the largest integer less than or equal to `x`.

```unilang
floor(3.7)    // 3
floor(-2.3)   // -3
```

### `ceil(x)`

Returns the smallest integer greater than or equal to `x`.

```unilang
ceil(3.2)     // 4
ceil(-2.7)    // -2
```

### `round(x)`

Rounds `x` to the nearest integer. Rounds half to even (banker's rounding).

```unilang
round(3.5)    // 4
round(4.5)    // 4
round(3.7)    // 4
round(3.2)    // 3
```

---

## Collections

### `len(collection)`

Returns the number of elements in a collection or the length of a string.

```unilang
len([1, 2, 3])         // 3
len({"a": 1, "b": 2})  // 2
len("hello")           // 5
```

### `range(stop)` / `range(start, stop)` / `range(start, stop, step)`

Generates a sequence of integers.

```unilang
range(5)          // [0, 1, 2, 3, 4]
range(2, 7)       // [2, 3, 4, 5, 6]
range(0, 10, 2)   // [0, 2, 4, 6, 8]
range(5, 0, -1)   // [5, 4, 3, 2, 1]
```

### `sorted(collection)`

Returns a new sorted list from the items in the collection.

```unilang
sorted([3, 1, 4, 1, 5])           // [1, 1, 3, 4, 5]
sorted(["banana", "apple", "cherry"])  // ["apple", "banana", "cherry"]
```

### `reversed(collection)`

Returns a new list with elements in reverse order.

```unilang
reversed([1, 2, 3])    // [3, 2, 1]
reversed("hello")      // "olleh"
```

### `enumerate(collection)`

Returns a list of (index, value) pairs.

```unilang
enumerate(["a", "b", "c"])    // [(0, "a"), (1, "b"), (2, "c")]

for i, item in enumerate(items):
    print(f"{i}: {item}")
```

### `zip(collection1, collection2)`

Pairs elements from two collections. Stops at the shorter one.

```unilang
zip([1, 2, 3], ["a", "b", "c"])    // [(1, "a"), (2, "b"), (3, "c")]

for name, score in zip(names, scores):
    print(f"{name}: {score}")
```

---

## Strings

### `upper(s)`

Returns a copy of the string with all characters converted to uppercase.

```unilang
upper("hello")      // "HELLO"
upper("Hello World") // "HELLO WORLD"
```

### `lower(s)`

Returns a copy of the string with all characters converted to lowercase.

```unilang
lower("HELLO")      // "hello"
lower("Hello World") // "hello world"
```

### `split(s, delimiter)`

Splits a string into a list by the given delimiter.

```unilang
split("a,b,c", ",")         // ["a", "b", "c"]
split("hello world", " ")   // ["hello", "world"]
```

### `join(delimiter, collection)`

Joins a collection of strings with the given delimiter.

```unilang
join(", ", ["a", "b", "c"])   // "a, b, c"
join("-", ["2026", "03", "21"]) // "2026-03-21"
```

### `strip(s)`

Removes leading and trailing whitespace from a string.

```unilang
strip("  hello  ")   // "hello"
strip("\n text \t")   // "text"
```

### `replace(s, old, new)`

Replaces all occurrences of `old` with `new` in the string.

```unilang
replace("hello world", "world", "UniLang")   // "hello UniLang"
replace("aaa", "a", "b")                     // "bbb"
```

### `contains(s, substring)`

Returns `True` if the string contains the given substring.

```unilang
contains("hello world", "world")   // True
contains("hello world", "xyz")     // False
```

### `startswith(s, prefix)`

Returns `True` if the string starts with the given prefix.

```unilang
startswith("hello world", "hello")  // True
startswith("hello world", "world")  // False
```

### `endswith(s, suffix)`

Returns `True` if the string ends with the given suffix.

```unilang
endswith("hello.uniL", ".uniL")    // True
endswith("hello.py", ".uniL")      // False
```

---

## Type Checking

### `type(value)`

Returns the type of a value as a string.

```unilang
type(42)         // "Int"
type(3.14)       // "Float"
type("hello")    // "String"
type(True)       // "Bool"
type([1, 2])     // "List"
type(None)       // "Null"
```

### `isinstance(value, type_name)`

Returns `True` if the value is of the given type.

```unilang
isinstance(42, "Int")         // True
isinstance("hi", "String")   // True
isinstance(3.14, "Int")      // False
isinstance(True, "Bool")     // True
```

---

## Quick Reference Table

| Category | Function | Signature | Returns |
|----------|----------|-----------|---------|
| **I/O** | `print` | `print(value)` | `Void` |
| | `input` | `input(prompt) -> String` | `String` |
| **Conversion** | `int` | `int(value) -> Int` | `Int` |
| | `float` | `float(value) -> Float` | `Float` |
| | `str` | `str(value) -> String` | `String` |
| | `bool` | `bool(value) -> Bool` | `Bool` |
| **Math** | `abs` | `abs(x) -> Number` | `Number` |
| | `min` | `min(a, b) -> Number` | `Number` |
| | `max` | `max(a, b) -> Number` | `Number` |
| | `pow` | `pow(base, exp) -> Number` | `Number` |
| | `sqrt` | `sqrt(x) -> Float` | `Float` |
| | `floor` | `floor(x) -> Int` | `Int` |
| | `ceil` | `ceil(x) -> Int` | `Int` |
| | `round` | `round(x) -> Int` | `Int` |
| **Collections** | `len` | `len(collection) -> Int` | `Int` |
| | `range` | `range(start, stop, step) -> List<Int>` | `List<Int>` |
| | `sorted` | `sorted(collection) -> List` | `List` |
| | `reversed` | `reversed(collection) -> List` | `List` |
| | `enumerate` | `enumerate(collection) -> List<Tuple>` | `List<Tuple>` |
| | `zip` | `zip(a, b) -> List<Tuple>` | `List<Tuple>` |
| **Strings** | `upper` | `upper(s) -> String` | `String` |
| | `lower` | `lower(s) -> String` | `String` |
| | `split` | `split(s, delim) -> List<String>` | `List<String>` |
| | `join` | `join(delim, list) -> String` | `String` |
| | `strip` | `strip(s) -> String` | `String` |
| | `replace` | `replace(s, old, new) -> String` | `String` |
| | `contains` | `contains(s, sub) -> Bool` | `Bool` |
| | `startswith` | `startswith(s, prefix) -> Bool` | `Bool` |
| | `endswith` | `endswith(s, suffix) -> Bool` | `Bool` |
| **Type checking** | `type` | `type(value) -> String` | `String` |
| | `isinstance` | `isinstance(value, type) -> Bool` | `Bool` |

---

**Previous**: [[Type System]] | **Next**: [[Compiler Pipeline]]
