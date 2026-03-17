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
  <a href="https://github.com/gauthamnair2005/Linea/releases">
    <img src="https://img.shields.io/badge/Version-4.0.0-green.svg" alt="Version" />
  </a>
</p>

---

**Linea** is an enterprise-grade, statically-typed compiled programming language designed for high-performance systems programming and native artificial intelligence development. With **ARL-Tangram** adaptive reasoning algorithm, GPU acceleration, and comprehensive ML/AI capabilities, Linea combines the memory safety of Rust with the expressiveness of Python, delivering a robust platform for modern software engineering and advanced AI reasoning.

## 🚀 Key Capabilities

### Advanced AI & Reasoning
*   **ARL-Tangram Algorithm**: Adaptive Reasoning Layer with Tangram-based compositional learning for interpretable AI systems with multi-layer attention mechanisms.
*   **Hardware Acceleration**: Built-in support for GPU-accelerated tensor operations using WGPU (Vulkan/Metal/DX12), enabling high-performance machine learning inference and training.
*   **Machine Learning Primitives**: Native implementation of neural network layers, activation functions, and optimizers (SGD, Adam) directly in the core library.

### Systems Performance
*   **Compiled Efficiency**: Compiles directly to optimized machine code via LLVM/Rust backend.
*   **Zero-Cost Abstractions**: High-level syntax with low-level performance characteristics.
*   **Memory Safety**: Ownership-based memory management ensures zero use-after-free bugs without a garbage collector.
*   **Standalone Binaries**: Produces single, dependency-free executables (~1.3MB) for easy deployment.

### Enterprise Ecosystem
*   **Data Processing**: Native support for CSV, Excel, and JSON processing.
*   **Visualization**: Built-in graphics library for data plotting and charting.
*   **Documentation**: Integrated Markdown rendering and documentation generation.
*   **Networking**: Robust HTTP client for API integration.

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
Linea includes a comprehensive native Machine Learning library. See `examples/iris_demo.ln` for a full example.

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

Linea provides a rich standard library for professional development:

| Module | Description |
| :--- | :--- |
| **`arl`** | Adaptive Reasoning Layer with Tangram decomposition for advanced AI |
| **`ml`** | Neural network layers, activations, losses, and optimizers |
| **`compute`** | Hardware-accelerated GPU tensor operations |
| **`datasets`** | Data loading and preprocessing utilities |
| **`csv`** | High-performance CSV reading and writing |
| **`excel`** | Excel file manipulation (read/write) |
| **`markdown`** | Markdown parsing and HTML generation |
| **`graphics`** | Data visualization and plotting |
| **`http`** | HTTP client for REST API integration |
| **`math`** | Mathematical functions and constants |
| **`strings`** | String manipulation utilities |
| **`utils`** | General utility functions |

## 📂 Project Structure

*   `compiler/`: Rust source code for the Linea compiler.
*   `libs/`: Standard library source files (`.ln`).
*   `examples/`: Example programs including ML demos.
*   `docs/`: Documentation files.
*   `linea`: Pre-compiled binary.

## 🤝 Contributing

We welcome contributions from the community. Please read `CONTRIBUTING.md` for guidelines on how to contribute to the Linea project.

## 📄 License

Copyright © 2025 Gautham Nair.
Licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
