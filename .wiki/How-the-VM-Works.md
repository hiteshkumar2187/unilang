# How the VM Works

UniLang uses a stack-based virtual machine to execute compiled bytecode. This page explains how the VM operates internally.

**Location:** `crates/unilang-runtime/`

---

## Stack-Based Execution

In a stack-based VM, there are no named registers. Instead, all operations push and pop values from a central **operand stack**.

### Example: How `5 + 3` executes

The compiler turns `5 + 3` into these bytecode instructions:

```
LoadConst 5    // Push 5 onto the stack
LoadConst 3    // Push 3 onto the stack
Add            // Pop two values, add them, push the result
```

The stack at each step:

```
Instruction     Stack (top on right)
-----------     --------------------
LoadConst 5     [5]
LoadConst 3     [5, 3]
Add             [8]
```

The result `8` is now on top of the stack, ready for the next operation.

---

## Opcode Categories

The VM supports 40+ opcodes organized into categories:

### Stack Operations

| Opcode | Description |
|--------|-------------|
| `LoadConst` | Push a constant value onto the stack |
| `LoadLocal` | Push a local variable's value |
| `StoreLocal` | Pop the top value and store it in a local variable |
| `LoadGlobal` | Push a global variable's value |
| `StoreGlobal` | Pop the top value and store it as a global |
| `Pop` | Discard the top value |
| `Dup` | Duplicate the top value |

### Arithmetic

| Opcode | Description |
|--------|-------------|
| `Add` | Pop two values, push their sum |
| `Sub` | Pop two values, push their difference |
| `Mul` | Pop two values, push their product |
| `Div` | Pop two values, push their quotient |
| `FloorDiv` | Integer division (rounds toward negative infinity) |
| `Mod` | Remainder |
| `Pow` | Exponentiation |
| `Neg` | Negate the top value |

**Arithmetic promotion:** When an `Int` and a `Float` are combined, the `Int` is promoted to `Float` and the result is `Float`.

### Comparison and Logical

| Opcode | Description |
|--------|-------------|
| `Eq`, `NotEq` | Equality comparison |
| `Lt`, `Gt`, `LtEq`, `GtEq` | Ordering comparison |
| `And`, `Or`, `Not` | Logical operations |

### Control Flow

| Opcode | Description |
|--------|-------------|
| `Jump` | Unconditional jump to an instruction address |
| `JumpIfFalse` | Pop the top value; jump if it is falsy |
| `JumpIfTrue` | Pop the top value; jump if it is truthy |

### Function Operations

| Opcode | Description |
|--------|-------------|
| `Call` | Call a function with N arguments |
| `Return` | Return from the current function |
| `MakeFunction` | Create a function object |

### Object Operations

| Opcode | Description |
|--------|-------------|
| `GetAttr` | Get an attribute from an object |
| `SetAttr` | Set an attribute on an object |
| `MakeClass` | Define a new class |
| `NewInstance` | Create a new instance of a class |

### Collection Operations

| Opcode | Description |
|--------|-------------|
| `MakeList` | Create a list from N stack values |
| `MakeDict` | Create a dict from N key-value pairs |
| `GetIndex` | Get an element by index |
| `SetIndex` | Set an element by index |

### I/O and Control

| Opcode | Description |
|--------|-------------|
| `Print` | Pop the top value and output it |
| `Concat` | Concatenate two strings |
| `Halt` | Stop execution |

---

## Call Frames

When a function is called, the VM pushes a new **call frame** onto a frame stack. Each frame tracks:

- **Instruction pointer (IP):** The current position in the function's bytecode
- **Base pointer:** The position in the operand stack where this frame's locals start
- **Function ID:** Which function this frame belongs to

### Example: Function call

```unilang
def add(a, b):
    return a + b

result = add(5, 3)
```

Execution trace:

```
1. Main frame: LoadConst 5, LoadConst 3, Call "add" (2 args)
   Stack: [5, 3]

2. VM pushes new call frame for "add"
   Frame stack: [main, add]
   Local variables: a=5, b=3

3. Inside "add": LoadLocal 0 (a), LoadLocal 1 (b), Add, Return
   Stack: [5, 3, 8]

4. Return pops the call frame, pushes result
   Frame stack: [main]
   Stack: [8]

5. Main frame: StoreGlobal "result"
   Globals: { result: 8 }
```

---

## How Built-In Functions Are Dispatched

Built-in functions (from `crates/unilang-stdlib/`) are registered in the VM at startup as `NativeFunction` values. When the VM encounters a `Call` instruction and the callee is a `NativeFunction`:

1. The VM pops the required number of arguments from the stack.
2. It looks up the native function by name in a dispatch table.
3. It calls the Rust implementation directly.
4. The return value is pushed onto the stack.

```
// For: print("hello")

LoadConst "hello"       // Push the string argument
LoadGlobal "print"      // Push the NativeFunction
Call 1                  // Call with 1 argument
                        // VM dispatches to Rust print implementation
                        // Output: "hello"
                        // Push Null (print returns void)
```

---

## Execution Trace: `x = 5 + 3; print(x)`

Here is a complete trace of a simple program:

### Source

```unilang
x = 5 + 3
print(x)
```

### Compiled Bytecode

```
0: LoadConst 5
1: LoadConst 3
2: Add
3: StoreGlobal "x"
4: LoadGlobal "print"
5: LoadGlobal "x"
6: Call 1
7: Pop
8: Halt
```

### Step-by-step Execution

| Step | Instruction | Stack | Globals | Output |
|------|-------------|-------|---------|--------|
| 0 | `LoadConst 5` | `[5]` | `{}` | |
| 1 | `LoadConst 3` | `[5, 3]` | `{}` | |
| 2 | `Add` | `[8]` | `{}` | |
| 3 | `StoreGlobal "x"` | `[]` | `{x: 8}` | |
| 4 | `LoadGlobal "print"` | `[NativeFunction("print")]` | `{x: 8}` | |
| 5 | `LoadGlobal "x"` | `[NativeFunction("print"), 8]` | `{x: 8}` | |
| 6 | `Call 1` | `[Null]` | `{x: 8}` | `8` |
| 7 | `Pop` | `[]` | `{x: 8}` | `8` |
| 8 | `Halt` | `[]` | `{x: 8}` | `8` |

**Step 6 detail:** The VM sees the callee is `NativeFunction("print")`. It pops 1 argument (`8`), calls the Rust `print` implementation which outputs `8`, and pushes `Null` as the return value.

---

## Runtime Value Types

All values in the VM are represented as a `RuntimeValue` enum:

| Variant | Description | Example |
|---------|-------------|---------|
| `Int(i64)` | 64-bit integer | `42` |
| `Float(f64)` | 64-bit float | `3.14` |
| `Bool(bool)` | Boolean | `true`, `false` |
| `String(String)` | UTF-8 string | `"hello"` |
| `Null` | Null/None | `None` / `null` |
| `List(Vec<RuntimeValue>)` | Ordered collection | `[1, 2, 3]` |
| `Dict(Vec<(RuntimeValue, RuntimeValue)>)` | Key-value pairs | `{"a": 1}` |
| `Function(usize)` | User-defined function (index into function table) | |
| `NativeFunction(String)` | Built-in function | `print`, `len` |
| `Instance { class_name, fields }` | Object instance | `Calculator()` |

---

## How Objects Work

When a class is defined, the VM stores its method table. When `NewInstance` executes:

1. A new `Instance` value is created with the class name and an empty field map.
2. If the class has an `__init__` (Python) or constructor (Java), it is called immediately.
3. Field assignments within the constructor populate the instance's field map.

Attribute access (`GetAttr`) first checks instance fields, then falls back to the class method table.

---

**Previous**: [[How the Parser Works]] | **Next**: [[ML Framework Overview]]
