# Quick Start

Go from zero to running code in 5 minutes. This guide assumes you have already installed UniLang. If not, see [[Installation]] first.

---

## Hello World

Create a file called `hello.uniL`:

```unilang
// Hello World in UniLang — mixing Python and Java styles
print("Hello from UniLang!")

// Python-style function
def greet(name):
    return f"Welcome, {name}!"

// Java-style class
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
}

// Use both together
message = greet("Developer")
print(message)

calc = Calculator()
result = calc.add(5, 3)
print(f"5 + 3 = {result}")
```

## Run It

```bash
unilang run hello.uniL
```

Expected output:

```
Hello from UniLang!
Welcome, Developer!
5 + 3 = 8
```

## Check for Errors

UniLang includes a static checker that finds problems without running your code:

```bash
unilang check hello.uniL
```

If there are no errors:

```
No errors found in hello.uniL
```

## See the Bytecode

Curious about what happens under the hood?

```bash
unilang compile hello.uniL
```

This shows how UniLang translates your mixed-paradigm code into unified bytecode.

---

## Example 2: A Multi-Paradigm Calculator

Create `calculator.uniL`:

```unilang
// Python-style: quick utility functions
def square(x):
    return x * x

def cube(x):
    return x * x * x

// Java-style: structured class with methods
public class ScientificCalculator {

    private double memory = 0.0;

    public double add(double a, double b) {
        return a + b;
    }

    public double subtract(double a, double b) {
        return a - b;
    }

    public double multiply(double a, double b) {
        return a * b;
    }

    public double divide(double a, double b) {
        if (b == 0) {
            print("Error: division by zero")
            return 0.0;
        }
        return a / b;
    }

    public void storeInMemory(double value) {
        this.memory = value;
    }

    public double recallMemory() {
        return this.memory;
    }
}

// Use everything together
calc = ScientificCalculator()

print(f"10 + 5 = {calc.add(10, 5)}")
print(f"10 - 5 = {calc.subtract(10, 5)}")
print(f"10 * 5 = {calc.multiply(10, 5)}")
print(f"10 / 5 = {calc.divide(10, 5)}")

print(f"4 squared = {square(4)}")
print(f"3 cubed = {cube(3)}")

calc.storeInMemory(42.0)
print(f"Recalled from memory: {calc.recallMemory()}")
```

Run it:

```bash
unilang run calculator.uniL
```

Expected output:

```
10 + 5 = 15.0
10 - 5 = 5.0
10 * 5 = 50.0
10 / 5 = 2.0
4 squared = 16
3 cubed = 27
Recalled from memory: 42.0
```

---

## Example 3: ML Model in UniLang

Create `ml_demo.uniL`:

```unilang
// ML model definition — Python-style data handling with Java-style structure
import unilang.ml

// Python-style: prepare data
def load_data():
    features = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]]
    labels = [0, 1, 1, 0]
    return features, labels

def split_data(features, labels, ratio=0.75):
    split_index = int(len(features) * ratio)
    train_x = features[:split_index]
    train_y = labels[:split_index]
    test_x = features[split_index:]
    test_y = labels[split_index:]
    return train_x, train_y, test_x, test_y

// Java-style: define the model as a class
public class SimpleClassifier {

    private double learningRate;
    private int epochs;

    public SimpleClassifier(double learningRate, int epochs) {
        this.learningRate = learningRate;
        this.epochs = epochs;
    }

    public void train(List<double[]> features, List<int> labels) {
        print(f"Training with learning rate {this.learningRate} for {this.epochs} epochs...")
        for (int i = 0; i < this.epochs; i++) {
            if (i % 10 == 0) {
                print(f"  Epoch {i}: loss = {1.0 / (i + 1):.4f}")
            }
        }
        print("Training complete!")
    }

    public int predict(double[] input) {
        return 1;
    }
}

// Put it all together
features, labels = load_data()
train_x, train_y, test_x, test_y = split_data(features, labels)

print("=== UniLang ML Demo ===")
print(f"Training samples: {len(train_x)}")
print(f"Test samples: {len(test_x)}")

model = SimpleClassifier(0.01, 50)
model.train(train_x, train_y)

for sample in test_x:
    prediction = model.predict(sample)
    print(f"Input: {sample} -> Predicted: {prediction}")

print("Done!")
```

Run it:

```bash
unilang run ml_demo.uniL
```

This demonstrates how UniLang lets you write data-processing code in Python style and model architecture in Java style, all in one file.

---

## CLI Commands Reference

| Command | Description |
|---------|-------------|
| `unilang run <file>` | Run a `.uniL` program (full pipeline) |
| `unilang check <file>` | Check for errors without running |
| `unilang compile <file>` | Compile and show bytecode disassembly |
| `unilang lex <file>` | Tokenize and show token stream |
| `unilang --version` | Show version |
| `unilang --help` | Show help |

---

## Next Steps

1. **Set up your editor** for syntax highlighting: [[IDE Setup]]
2. **Learn the language** in depth: [[Language Overview]]
3. **Explore the ML framework**: [[ML Framework Overview]]
4. **Read the interop guide**: [[Python Java Interop]]

---

**Previous**: [[Installation]] | **Next**: [[IDE Setup]]
