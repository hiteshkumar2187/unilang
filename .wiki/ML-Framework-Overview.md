# ML Framework Overview

The UniLang ML Framework is a neural network framework built **entirely from scratch** -- no PyTorch, no TensorFlow. Every piece, from the math to the training loop, is written by hand in UniLang.

**Location:** `examples/ml-framework/`

---

## What It Is

A complete machine learning toolkit that lets you build, train, and deploy neural networks. It is designed for developers who want to understand how ML works under the hood, not just call `model.fit()`.

---

## Components

### Tensor (`core/tensor`)

The fundamental data container. A multi-dimensional array with:
- Element-wise and matrix operations (`matmul`, `add`, `mul`, `relu`, `sigmoid`, `softmax`)
- Automatic gradient tracking (`backward()`)
- Shape management

```unilang
from core.tensor import Tensor

X = Tensor.from_list([[1.0, 2.0], [3.0, 4.0]], shape=[2, 2])
result = X.matmul(W).add(b).relu()
result.backward()    // Compute gradients through the entire computation graph
```

### Layers (`core/layers`)

Pre-built neural network building blocks:

| Layer | Description |
|-------|-------------|
| `Linear` | Fully connected layer (y = Wx + b) |
| `BatchNorm` | Batch normalization for training stability |
| `Dropout` | Randomly zeros elements during training to prevent overfitting |
| `Embedding` | Maps discrete indices to dense vectors |
| `LSTM` | Long Short-Term Memory for sequential data |
| `Conv1D` | 1D convolution for time series and signal data |
| `MaxPool1D` | 1D max pooling to reduce dimensionality |

### Loss Functions (`core/loss`)

| Loss | Use Case |
|------|----------|
| `MSELoss` | Regression (predicting numbers) |
| `CrossEntropyLoss` | Multi-class classification |
| `BCELoss` | Binary classification (yes/no) |
| `HuberLoss` | Regression with outliers |

### Optimizers (`core/optimizers`)

| Optimizer | Description |
|-----------|-------------|
| `SGD` | Stochastic Gradient Descent (simplest) |
| `Adam` | Adaptive learning rates (best default) |
| `RMSProp` | Good for recurrent networks |

Plus learning rate schedulers for adjusting the learning rate during training.

### UniNN Model (`models/uniNN`)

UniLang's original neural network architecture featuring:
- **Gated residual blocks** that learn which information to keep and which to discard
- **Multi-scale feature mixing** that captures patterns at different scales
- **Task-aware output heads** for classification or regression
- Configurable depth, width, and dropout

---

## LSTM and Conv1D for Time Series

The framework includes specialized layers for sequential and time-series data:

```unilang
// LSTM for sequence prediction
lstm = LSTM(inputDim=10, hiddenDim=64)
output = lstm.forward(sequence_data)

// Conv1D for pattern detection in time series
conv = Conv1D(inChannels=1, outChannels=16, kernelSize=3)
features = conv.forward(signal_data)
```

---

## Quick Example

```unilang
from models.uniNN import UniNN
from core.loss import CrossEntropyLoss
from core.optimizers import Adam

// 1. Create a model
model = UniNN(inputDim=10, hiddenDim=64, outputDim=3, task="classification")

// 2. Pick loss function and optimizer
loss_fn = CrossEntropyLoss()
optimizer = Adam(model.parameters(), lr=0.001)

// 3. Training loop
for epoch in range(100):
    model.zero_grad()
    predictions = model.forward(X_train)
    loss = loss_fn.compute(predictions, y_train)
    loss.backward()
    optimizer.step()

    if epoch % 10 == 0:
        print(f"Epoch {epoch} | Loss: {loss.data[0]:.4f}")

// 4. Save the trained model
model.save("my_model.json")
```

No GPU required. No external dependencies. Just UniLang.

---

## Documentation

| Document | What You'll Learn | Time |
|----------|------------------|------|
| [[Building Your First Model]] | Step-by-step tutorial: create, train, use a model | 20 min |
| [Core Concepts](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/01_CORE_CONCEPTS.md) | Tensors, neurons, and networks explained with code | 15 min |
| [Architecture](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/02_ARCHITECTURE.md) | How the framework is built and why | 10 min |
| [UniNN Model](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/03_UNINN_MODEL.md) | Custom architecture with gated residual blocks | 10 min |
| [Training Deep Dive](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/05_TRAINING_DEEP_DIVE.md) | Backpropagation, optimizers, and loss functions | 15 min |
| [Best Practices](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/06_BEST_PRACTICES.md) | Tips for building reliable models | 10 min |
| [API Reference](https://github.com/hiteshkumar2187/unilang/blob/main/examples/ml-framework/docs/07_API_REFERENCE.md) | Complete class and method reference | Reference |

---

**Previous**: [[How the VM Works]] | **Next**: [[Building Your First Model]]
