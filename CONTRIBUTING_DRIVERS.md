# Contributing a Community Driver

UniLang ships with a built-in driver ecosystem for the most common databases,
caches, queues, and search engines.  If you need a driver that is not yet
included, you can contribute one — and the build system will **automatically
discover, register, and activate it** the moment you drop a single `.rs` file
into the right directory.

---

## How it works

The `unilang-drivers` crate has a `build.rs` script that runs every time the
crate is compiled.  It scans `crates/unilang-drivers/src/community/` for
`*.rs` files (excluding `mod.rs`), then generates Rust code that:

1. Declares a `pub mod <name>;` for each discovered file.
2. Calls `registry.add(Box::new(<Name>Driver::new()))` for each one.

The result: a new driver is fully wired in at compile time without any manual
changes to `lib.rs`, `Cargo.toml`, or the semantic analyzer.

---

## Step 1 — Generate a template

```bash
unilang driver new mydb
```

This writes `mydb.rs` to the current directory.  The file contains a complete,
compilable driver skeleton with three stub functions
(`mydb_connect`, `mydb_query`, `mydb_close`).

You can also pass `--out <dir>` to write the file somewhere specific.

---

## Step 2 — Move it to the community directory

```bash
mv mydb.rs crates/unilang-drivers/src/community/mydb.rs
```

Naming rules:

- File name must be a valid Rust module name (`snake_case`, no hyphens).
- The struct inside must be named `<Capitalized>Driver`
  (e.g. file `mydb.rs` → struct `MydbDriver`).
- Implement the `UniLangDriver` trait fully (all six methods).

---

## Step 3 — Build

```bash
cargo build
```

The build script detects `mydb.rs`, generates the registration glue, and
compiles your driver in.  No other files need to be edited.

To verify the driver appeared:

```bash
unilang driver list
```

---

## Step 4 — Test

Write a `.uniL` script that exercises your driver:

```python
# test_mydb.uniL
ok = mydb_connect("mydb://localhost/testdb")
rows = mydb_query("SELECT 1")
print(rows)
mydb_close()
```

Run it:

```bash
unilang run test_mydb.uniL
```

The semantic analyzer automatically picks up the exported function names from
your driver, so `unilang check test_mydb.uniL` will not report false
"undefined variable" errors.

---

## Step 5 — Submit a PR

1. Fork the repository and create a branch: `feat/driver-mydb`.
2. Add your driver file to `crates/unilang-drivers/src/community/mydb.rs`.
3. Add any required dependencies to `crates/unilang-drivers/Cargo.toml`
   under a new optional feature flag (e.g. `mydb-driver = ["dep:mydb_crate"]`).
4. Update `Cargo.toml` `[features]` defaults if the driver has no heavy deps.
5. Open a PR with a description covering:
   - What system the driver connects to.
   - Which functions it exports and what they do.
   - How to run a local integration test.
6. All CI checks must pass: `cargo build --workspace`, `cargo clippy -- -D warnings`,
   and `cargo fmt --all -- --check`.

---

## Template reference

The generated template (from `unilang driver new <name>`) looks like this:

```rust
use std::sync::{Arc, Mutex};
use unilang_runtime::error::RuntimeError;
use unilang_runtime::value::RuntimeValue;
use unilang_runtime::vm::VM;
use crate::{DriverCategory, UniLangDriver};

pub struct MydbDriver {
    _state: Arc<Mutex<Option<()>>>,
}

impl MydbDriver {
    pub fn new() -> Self {
        Self { _state: Arc::new(Mutex::new(None)) }
    }
}

impl Default for MydbDriver {
    fn default() -> Self { Self::new() }
}

impl UniLangDriver for MydbDriver {
    fn name(&self)        -> &str { "mydb" }
    fn version(&self)     -> &str { "0.1.0" }
    fn description(&self) -> &str { "MyDB driver for UniLang" }
    fn category(&self)    -> DriverCategory { DriverCategory::Other }

    fn exported_functions(&self) -> &'static [&'static str] {
        &["mydb_connect", "mydb_query", "mydb_close"]
    }

    fn register(&self, vm: &mut VM) {
        // Register each function with Arc-cloned state ...
    }
}
```

---

## Design guidelines

- **Wrap connection state in `Arc<Mutex<Option<Connection>>>`** so closures can
  share it safely without touching the VM.
- **Return `RuntimeError::type_error(...)` for bad arguments** — never panic.
- **Use a consistent naming prefix** (`mydb_*`) so users can identify which
  driver a function belongs to at a glance.
- **List every exported function in `exported_functions()`** so the semantic
  analyzer can pre-populate the global scope.
- **Keep dependencies optional** — add a new feature flag in `Cargo.toml`
  rather than making a heavy crate unconditionally required.
- **Write a smoke test** — a short `.uniL` script in `examples/` that confirms
  end-to-end operation.
