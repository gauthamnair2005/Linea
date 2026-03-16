# The Linea Programming Language

<p align="center">
  <img src="linea-logo.svg" alt="Linea Logo" width="120" />
</p>

<p align="center">
  <strong>The Professional Systems Language for the AI Era</strong>
</p>

<p align="center">
  <a href="https://github.com/gauthamnair2005/Linea/releases">
    <img src="https://img.shields.io/github/v/release/gauthamnair2005/Linea?style=flat-square" alt="Version" />
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/gauthamnair2005/Linea?style=flat-square" alt="License" />
  </a>
  <a href="https://github.com/gauthamnair2005/Linea/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/gauthamnair2005/Linea/ci.yml?style=flat-square" alt="Build Status" />
  </a>
</p>

---

**Linea** is a modern, statically-typed, compiled programming language designed for high-performance systems programming and artificial intelligence. It combines the safety and speed of Rust with the simplicity of Python, making it the ideal choice for building next-generation AI applications, system tools, and web services.

## 🚀 Key Features

*   **Native Performance**: Compiles directly to machine code via LLVM/Rust backend. 100x-1000x faster than interpreted languages.
*   **GPU Accelerated AI**: First-class support for hardware-accelerated tensor operations. Automatically detects and utilizes NVIDIA (CUDA), AMD (ROCm), and Intel (OneAPI) GPUs via Vulkan/WGPU.
*   **Memory Safety**: Ownership-based memory management ensures zero use-after-free or buffer overflow bugs without the overhead of a garbage collector.
*   **Zero Dependencies**: Produces single, standalone binaries (~1.3MB) that run anywhere. No "dependency hell".
*   **Professional Tooling**: Built-in package manager, build system, formatter, and language server protocol (LSP) support.

## 📦 Installation

### Linux / macOS
```bash
curl -fsSL https://get.linea-lang.org/install.sh | sh
```

### Windows
```powershell
iwr https://get.linea-lang.org/install.ps1 -useb | iex
```

## ⚡ Quick Start

### 1. Hello World
```linea
func main() {
    display "Hello, Linea!"
}
```

Compile and run:
```bash
linea compile hello.ln -o hello
./hello
```

### 2. Train a Neural Network
Linea makes AI development simple and fast. Here is a complete example of training a classifier:

```linea
import ml
import datasets

# Load dataset
var data = datasets::load_csv("iris.csv")
var model = ml::Linear(4, 3) # 4 inputs, 3 classes

# Training Loop
display "Starting training..."
for epoch from 0~100 {
    # Forward Pass
    var pred = ml::forward(model, data.features)
    
    # Calculate Loss
    var loss = ml::cross_entropy(pred, data.labels)
    
    # Backward Pass & Optimization
    var grad = ml::backward(pred, data.labels)
    ml::step(model, grad, 0.01)
    
    if epoch % 10 == 0 {
        display "Epoch " + epoch + " | Loss: " + loss
    }
}
```

## 📚 Documentation

*   [Language Tour](docs/tour.md)
*   [Standard Library Reference](docs/stdlib/README.md)
*   [AI/ML Guide](docs/ai/README.md)
*   [Compiler Internals](docs/internals.md)

## 🛠️ Building from Source

Requirements: Rust 1.75+

```bash
git clone https://github.com/gauthamnair2005/Linea.git
cd Linea
cargo build --release
./target/release/linea --version
```

## 🤝 Contributing

Linea is 100% open source and community-driven. We welcome contributions of all kinds!

1.  Fork the repository
2.  Create a feature branch (`git checkout -b feature/amazing-feature`)
3.  Commit your changes (`git commit -m 'Add amazing feature'`)
4.  Push to the branch (`git push origin feature/amazing-feature`)
5.  Open a Pull Request

## 📄 License

Copyright © 2025 Gautham Nair.
Licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
