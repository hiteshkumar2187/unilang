# UniLang — Threading Model Design

**Version:** 1.0.0-draft
**Last Updated:** 2026-03-19

---

## Problem

Python's GIL prevents true multi-threaded parallelism for CPU-bound Python code. Java has mature multi-threading. UniLang must provide real parallelism for both styles.

---

## Solution: Three-Tier Threading Strategy

### Tier 1: JVM Bytecode Compilation (Best Performance)

For Python-style code that does **not** use CPython C-extensions:

```unilang
def compute(data):
    result = 0
    for item in data:
        result += item * item
    return result

# This Python-style code compiles to JVM bytecode
# → True parallelism on Java threads, no GIL
executor = Executors.newFixedThreadPool(8)
futures = [executor.submit(lambda: compute(chunk)) for chunk in chunks]
```

**When used:** Pure computation, string processing, collection operations, business logic.
**Limitation:** Cannot use NumPy, TensorFlow, or any CPython C-extension.

### Tier 2: Python Subinterpreters (Good Performance)

For Python code that uses C-extensions, leveraging PEP 684 (Python 3.12+):

```unilang
import numpy as np

def train_model(data):
    X = np.array(data)           # Requires CPython (NumPy C-extension)
    model = LinearRegression()
    model.fit(X, y)
    return model

# Each thread gets its own Python subinterpreter with its own GIL
# → True parallelism between subinterpreters
executor = Executors.newFixedThreadPool(4)
futures = [executor.submit(lambda: train_model(chunk)) for chunk in chunks]
```

**When used:** ML training, NumPy operations, any CPython C-extension code.
**Limitation:** Some C-extensions may not support subinterpreters; data must be copied between subinterpreters.

### Tier 3: GIL-Aware Scheduling (Fallback)

For C-extensions that don't support subinterpreters:

```unilang
# Runtime detects that this library doesn't support subinterpreters
import legacy_c_library

def process(data):
    return legacy_c_library.compute(data)

# Runtime schedules tasks with GIL awareness:
# - Releases GIL during Java bridge calls
# - Batches Python tasks to minimize GIL contention
# - Overlaps Python GIL-holding with Java parallel work
```

**When used:** Automatically when Tier 2 is not available.

---

## Java Concurrency Primitives

All Java concurrency primitives work natively in UniLang:

### Thread Pools
```unilang
import java.util.concurrent.*

executor = Executors.newFixedThreadPool(4)
executor = Executors.newCachedThreadPool()
executor = Executors.newVirtualThreadPerTaskExecutor()  // Project Loom
scheduled = Executors.newScheduledThreadPool(2)
```

### Futures and CompletableFuture
```unilang
future = CompletableFuture.supplyAsync(() -> load_data())
    .thenApply(data -> transform(data))       # Java lambda
    .thenApply(lambda data: ml_predict(data))  # Python lambda
    .thenAccept(result -> save(result))
```

### Locks and Synchronization
```unilang
// synchronized keyword (works with Python objects too)
synchronized (shared_state) {
    shared_state.update(value)
}

// ReentrantLock
lock = ReentrantLock()
lock.lock()
try:
    shared_state.update(value)
finally:
    lock.unlock()

// Python-style context manager (maps to try-finally)
with lock:
    shared_state.update(value)

// Read-Write locks
rwLock = ReentrantReadWriteLock()
with rwLock.readLock():
    data = shared_state.read()
with rwLock.writeLock():
    shared_state.write(new_data)
```

### Concurrent Collections
```unilang
// Thread-safe collections available directly
map = ConcurrentHashMap()
queue = ConcurrentLinkedQueue()
list = CopyOnWriteArrayList()
```

### Atomic Operations
```unilang
counter = AtomicInteger(0)
counter.incrementAndGet()
counter.compareAndSet(expected, new_value)
```

---

## Cross-VM Thread Safety

When a Python object is accessed from multiple Java threads:

```
Thread-1 (JVM) ──→ Bridge ──→ Python Proxy ──→ Subinterpreter-1
Thread-2 (JVM) ──→ Bridge ──→ Python Proxy ──→ Subinterpreter-2
                                                (separate GILs)
```

### Shared State Protocol

1. **Immutable data:** Shared freely between threads and VMs (zero-copy where possible)
2. **Mutable data with Java lock:** Java monitor protects access; bridge acquires lock before Python execution
3. **Mutable data with Python lock:** Python Lock/RLock; bridge acquires before Java access
4. **NumPy arrays:** Shared memory buffer with explicit synchronization barriers

---

## Async/Await Integration

```unilang
// Python async functions run on virtual threads (Project Loom)
async def fetch_data(url):
    response = await http_get(url)
    return response.json()

// Equivalent to:
Thread.startVirtualThread(() -> {
    response = http_get(url)     // Blocking call on virtual thread
    return response.json()
})

// Java CompletableFuture can be awaited from Python
data = await CompletableFuture.supplyAsync(() -> loadFromDB())

// Python coroutines can be converted to CompletableFuture
CompletableFuture<Data> future = toFuture(fetch_data("http://..."))
```

---

*This document defines the threading model for UniLang. Changes require an RFC and benchmarking.*
