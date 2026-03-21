# Building Your First Model

This guide walks you through building, training, and using a neural network from scratch with the UniLang ML Framework. By the end, you will have a working model that classifies data.

**Time:** 20 minutes.

---

## The Problem

You want to predict whether a customer will **buy**, **browse**, or **leave** based on 5 features: time on site, pages viewed, items in cart, previous purchases, and whether they are a returning customer.

---

## Step 1: Prepare Your Data

Every ML project starts with data. You need two tensors:
- **X** (inputs): features for each sample
- **y** (targets): the correct answer for each sample

```unilang
from core.tensor import Tensor

// Each row = one customer, each column = one feature
X = Tensor.from_list([
    [120, 5, 2, 3, 1],    // Customer 1: bought
    [30,  1, 0, 0, 0],    // Customer 2: left
    [300, 12, 5, 8, 1],   // Customer 3: bought
    [15,  1, 0, 1, 0],    // Customer 4: left
], shape=[4, 5])

// One-hot encoded targets: [buy, browse, leave]
y = Tensor.from_list([
    [1, 0, 0],   // Customer 1 -> bought
    [0, 0, 1],   // Customer 2 -> left
    [1, 0, 0],   // Customer 3 -> bought
    [0, 0, 1],   // Customer 4 -> left
], shape=[4, 3])
```

**Important:** Normalize your features so no single feature dominates because of scale differences. Subtract the mean and divide by the standard deviation for each feature column.

---

## Step 2: Choose Your Model

Think about your problem:

| Question | Answer | Parameter |
|----------|--------|-----------|
| How many input features? | 5 | `inputDim=5` |
| How many output classes? | 3 | `outputDim=3` |
| How complex is the problem? | Medium | `hiddenDim=32` |
| Classification or regression? | Classification | `task="classification"` |

```unilang
from models.uniNN import UniNN

model = UniNN(
    inputDim=5,
    hiddenDim=32,
    outputDim=3,
    numBlocks=2,
    dropoutRate=0.1,
    task="classification"
)

model.summary()    // Print architecture overview
```

**Rule of thumb for hidden size:** Start with 2-4x your input dimension. Too small means underfitting; too large means overfitting and slow training.

---

## Step 3: Choose Loss Function and Optimizer

```unilang
from core.loss import CrossEntropyLoss
from core.optimizers import Adam

loss_fn = CrossEntropyLoss()     // Multi-class classification
optimizer = Adam(
    model.parameters(),
    lr=0.001,                    // Learning rate
    weightDecay=1e-4             // Regularization
)
```

### How to choose

| Task | Loss Function | Optimizer |
|------|--------------|-----------|
| Predicting a number | `MSELoss` | `Adam` |
| Choosing one of N classes | `CrossEntropyLoss` | `Adam` |
| Yes/No decision | `BCELoss` | `Adam` |
| Number with outliers | `HuberLoss` | `Adam` |

---

## Step 4: Train the Model

The training loop repeats 5 steps: reset gradients, forward pass, compute loss, backward pass, update weights.

```unilang
numEpochs = 100

print("Training started...")

for epoch in range(1, numEpochs + 1):
    // Step 4a: Reset gradients (they accumulate by default)
    model.zero_grad()

    // Step 4b: Forward pass — predict outputs
    predictions = model.forward(X_train)

    // Step 4c: Compute loss — how wrong are we?
    loss = loss_fn.compute(predictions, y_train)

    // Step 4d: Backward pass — compute gradients
    loss.backward()

    // Step 4e: Update weights — learn from mistakes
    optimizer.step()

    if epoch % 10 == 0:
        print(f"  Epoch {epoch}/{numEpochs} | Loss: {loss.data[0]:.4f}")

print("Training complete!")
```

### What to expect

```
Training started...
  Epoch  10/100 | Loss: 1.0892    <- Guessing randomly
  Epoch  20/100 | Loss: 0.7234    <- Getting better
  Epoch  30/100 | Loss: 0.4518    <- Learning patterns
  Epoch  40/100 | Loss: 0.2891
  Epoch  50/100 | Loss: 0.1654
  Epoch  60/100 | Loss: 0.0987    <- Converging
  Epoch  70/100 | Loss: 0.0612
  Epoch  80/100 | Loss: 0.0398
  Epoch  90/100 | Loss: 0.0251
  Epoch 100/100 | Loss: 0.0178    <- Model has learned!
Training complete!
```

If the loss is not decreasing, try adjusting the learning rate: lower it (0.0001) if loss is unstable, raise it (0.01) if loss is stuck.

---

## Step 5: Evaluate

Never evaluate on training data. Use held-out test data:

```unilang
model.eval_mode()    // Disables dropout

test_predictions = model.forward(X_test)

correct = 0
total = X_test.shape[0]

for i in range(total):
    pred_class = 0
    max_prob = test_predictions.data[i * 3]
    for c in range(1, 3):
        if test_predictions.data[i * 3 + c] > max_prob:
            max_prob = test_predictions.data[i * 3 + c]
            pred_class = c

    true_class = 0
    for c in range(1, 3):
        if y_test.data[i * 3 + c] > y_test.data[i * 3 + true_class]:
            true_class = c

    if pred_class == true_class:
        correct += 1

accuracy = correct / total
print(f"Test Accuracy: {accuracy:.2%}")
```

### Accuracy guide

| Accuracy | Meaning | Action |
|----------|---------|--------|
| < 40% | Worse than random | Bug in code or data |
| 40-60% | Random guessing | Model too simple or bad features |
| 60-80% | Learning something | Increase model size or train longer |
| 80-95% | Good to very good | May be enough for production |
| 95-99% | Excellent | Check for data leakage |
| 99.9%+ | Suspicious | Almost certainly overfitting |

---

## Step 6: Save Your Model

```unilang
model.save("models/trained/customer_classifier_v1.json")
```

The JSON file contains the model configuration, all learned weights, and version info.

---

## Step 7: Use Your Model (Inference)

```unilang
from models.uniNN import UniNN
from core.tensor import Tensor

model = UniNN.load("models/trained/customer_classifier_v1.json")
model.eval_mode()

// New customer data (normalized)
new_customer = Tensor.from_list([[0.8, 1.2, 0.5, -0.3, 1.0]], shape=[1, 5])

output = model.forward(new_customer)
probs = [round(output.data[i], 3) for i in range(3)]

print(f"Buy: {probs[0]:.1%}, Browse: {probs[1]:.1%}, Leave: {probs[2]:.1%}")
// Output: Buy: 78.3%, Browse: 15.2%, Leave: 6.5%
```

---

## Complete Example

```unilang
from core.tensor import Tensor
from core.loss import CrossEntropyLoss
from core.optimizers import Adam
from models.uniNN import UniNN
import random

// 1. Generate sample data
random.seed(42)
numSamples = 500
X = Tensor([numSamples, 5])
y = Tensor([numSamples, 3])

for i in range(numSamples):
    time_on_site = random.uniform(10, 600)
    pages = random.randint(1, 20)
    cart_items = random.randint(0, 10)
    past_purchases = random.randint(0, 15)
    returning = random.choice([0, 1])

    if cart_items > 3 and past_purchases > 2:
        label = 0    // buy
    elif pages > 5:
        label = 1    // browse
    else:
        label = 2    // leave

    X.data[i*5+0] = time_on_site
    X.data[i*5+1] = pages
    X.data[i*5+2] = cart_items
    X.data[i*5+3] = past_purchases
    X.data[i*5+4] = returning
    y.data[i*3+label] = 1.0

// 2. Normalize features
for j in range(5):
    vals = [X.data[i*5+j] for i in range(numSamples)]
    mean = sum(vals) / len(vals)
    std = (sum((v - mean)**2 for v in vals) / len(vals)) ** 0.5 + 1e-8
    for i in range(numSamples):
        X.data[i*5+j] = (X.data[i*5+j] - mean) / std

// 3. Create model
model = UniNN(inputDim=5, hiddenDim=32, outputDim=3,
              numBlocks=2, task="classification")
model.summary()

// 4. Train
loss_fn = CrossEntropyLoss()
optimizer = Adam(model.parameters(), lr=0.005)

for epoch in range(1, 51):
    model.zero_grad()
    pred = model.forward(X)
    loss = loss_fn.compute(pred, y)
    loss.backward()
    optimizer.step()

    if epoch % 10 == 0:
        print(f"  Epoch {epoch}/50 | Loss: {loss.data[0]:.4f}")

// 5. Save
model.save("my_first_model.json")
print("Model saved!")
```

---

## Troubleshooting

| Problem | Likely Cause | Fix |
|---------|-------------|-----|
| Loss stays the same | Learning rate too small | Increase lr (try 0.01) |
| Loss goes UP | Learning rate too large | Decrease lr (try 0.0001) |
| Loss goes to NaN | Numerical overflow | Reduce lr, check data for infinity |
| 100% train accuracy, low test | Overfitting | Add dropout, reduce model size |
| Low accuracy everywhere | Underfitting | Increase model size, train longer |
| Training is slow | Too many parameters | Reduce hiddenDim or numBlocks |

---

**Previous**: [[ML Framework Overview]] | **Next**: [[How to Contribute]]
