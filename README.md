# The Linea Programming Language

<p align="center">
  <img src="linea-logo.svg" alt="Linea Logo" width="120" />
</p>

<p align="center">
  <strong>The Professional Systems Language for the AI Era</strong>
</p>

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="License" />
  </a>
  <a href="https://github.com/gauthamnair2005/Linea">
    <img src="https://img.shields.io/badge/Platform-Linux-lightgrey.svg" alt="Platform" />
  </a>
</p>

---

**Linea** is a modern, statically-typed, compiled programming language designed for high-performance systems programming and artificial intelligence. It combines the safety and speed of Rust with the simplicity of Python.

## 🚀 Key Features

*   **Native Performance**: Compiles directly to machine code via LLVM/Rust backend.
*   **GPU Accelerated AI**: Built-in support for hardware-accelerated tensor operations using WGPU (Vulkan/Metal/DX12).
*   **Memory Safety**: Ownership-based memory management ensures zero use-after-free bugs.
*   **Zero Dependencies**: Produces single, standalone binaries (~1.3MB) that run on Linux.
*   **Professional Tooling**: Built-in package manager and build system.

## 📦 Installation

### Linux

Clone the repository and run the install script:

```bash
git clone https://github.com/gauthamnair2005/Linea.git
cd Linea
./install.sh
```

Or build from source:

```bash
cargo build --release
./target/release/linea --version
```

## ⚡ Quick Start

### 1. Hello World
Create `hello.ln`:
```linea
display "Hello, Linea!"
```

Compile and run:
```bash
linea compile hello.ln -o hello
./hello
```

### 2. Train a Neural Network
Linea includes a native Machine Learning library. See `examples/iris_demo.ln` for a full example.

```linea
import ml
import datasets

# Load dataset
var data = datasets::load_csv("examples/iris_dummy.csv")

# Create a model
var model = ml::Linear(4, 3) 

# Training loop...
```

## 📚 Standard Library

The following libraries are included in `libs/`:

*   **`ml`**: Neural network layers (Linear), activations (ReLU, Sigmoid, Tanh, Softmax), losses (MSE, CrossEntropy), and optimizers (SGD, Adam).
*   **`compute`**: Low-level GPU tensor operations with automatic CPU fallback.
*   **`datasets`**: Data loading and preprocessing utilities.
*   **`http`**: Native HTTP client (`get`, `post`, `download`).
*   **`math`**: Mathematical functions.
*   **`strings`**: String manipulation utilities.
*   **`utils`**: General utility functions.

## 📂 Project Structure

*   `compiler/`: Rust source code for the Linea compiler.
*   `libs/`: Standard library source files (`.ln`).
*   `examples/`: Example programs including ML demos.
*   `docs/`: Documentation files.
*   `linea`: Pre-compiled binary.

## 🤝 Contributing

Linea is open source. Contributions are welcome!

## 📄 License

Copyright © 2025 Gautham Nair.
Licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
