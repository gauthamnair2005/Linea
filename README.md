# The Linea Programming Language

<p align="center">
  <img src="linea-logo.svg" alt="Linea Logo" width="140" />
</p>

<p align="center">
  <strong>The Professional Systems Language for the AI Era</strong>
</p>

<p align="center">
  Created by <strong>Gautham Nair</strong>
</p>

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="License" />
  </a>
  <a href="https://github.com/gauthamnair2005/Linea">
    <img src="https://img.shields.io/badge/Platform-Linux-lightgrey.svg" alt="Platform" />
  </a>
  <a href="https://github.com/gauthamnair2005/Linea/releases">
    <img src="https://img.shields.io/badge/Version-4.5.1-green.svg" alt="Version" />
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
*   **Auto-Parallel Compilation**: The compiler detects available system threads and builds generated Rust code with matching parallel jobs for faster compilation.
*   **Zero-Cost Abstractions**: High-level syntax with low-level performance characteristics.
*   **Memory Safety**: Ownership-based memory management ensures zero use-after-free bugs without a garbage collector.
*   **Standalone Binaries**: Produces single, dependency-free executables (~1.3MB) for easy deployment.

### Enterprise Ecosystem
*   **Data Processing**: Native support for CSV, Excel, and JSON processing.
*   **SQL/SQLite Module**: Native SQLite support with parameterized queries and secure database locking.
*   **Password/Security Module**: CLI/GUI password prompts with mask/bullet modes plus hashing/verification helpers.
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

### 3. New in v4.1: Range Loops & Pointers

**Range Operator `~` for cleaner loops:**
```linea
// Simple range (1 to 10)
for i from 1~10 {
    display i
}

// With custom step
for i from 0~20 step 2 {
    display i
}

// Reverse iteration
for i from 10~1 step -1 {
    display i
}
```

**Pointer Support with `ptr` Datatype:**
```linea
// Simplified pointer syntax with ptr type
var x @ int = 42
var ptr_to_x @ ptr = x    // Automatically captures address

var y @ int = 100
var ptr_to_y @ ptr = y    // Type-safe pointer storage

// Traditional address-of and dereference
var ptr @ &int = &x       // Address-of operator
var value @ int = *ptr    // Dereference operator

// Array pointers
var arr @ [int] = [1, 2, 3, 4, 5]
var ptr_arr @ ptr = arr[0]
display *(ptr_arr + 1)    // Access second element
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
| **`sql`** | SQLite access with parameterized queries and secure lock/unlock |
| **`password`** | Masked CLI/GUI password entry plus hashing/verification helpers |
| **`system`** | System operations: threads, files, env vars, process exec, time |
| **`math`** | Mathematical functions and constants |
| **`strings`** | String manipulation utilities |
| **`utils`** | General utility functions |

### System Programming Primitives (v4.5.1)

`system` now includes low-level/mid-level operations:

* Thread/core introspection: `system::threads()`, `system::compileJobs()`
* File/dir operations: `cwd`, `exists`, `isFile`, `isDir`, `readText`, `writeText`, `appendText`, `mkdir`, `rename`, `removeFile`, `removeDir`
* Environment variables: `envGet`, `envSet`
* Time/process: `nowMillis`, `sleepMs`, `exec`

See runnable example: `examples/system_ops_demo.ln`.

## 📦 Third-Party Package Manager

Linea now includes a built-in third-party package installer scaffold in `package-manager/`.

Install one or more custom libraries from an XML-driven registry:

```bash
python3 package-manager/linea_pkg.py greeter --registry ~/projects/linea-third-party-registry --libs-dir ./libs
```

The installer:

* Reads package metadata from `metadata/<libname>.xml`
* Resolves and installs transitive dependencies first
* Copies `.ln` files into your local `libs/` directory
* Writes `libs/.linea-packages.lock.json` for reproducibility

Use `package-manager/package-template.xml` as the metadata format template for library submissions.

## 📂 Project Structure
 
*   `compiler/`: Rust source code for the Linea compiler.
*   `libs/`: Standard library source files (`.ln`).
*   `examples/`: Example programs including ML demos.
*   `docs/`: Documentation files.
*   `package-manager/`: XML-driven third-party package installer.
*   `linea`: Pre-compiled binary.

## 🤝 Contributing

We welcome contributions from the community. Please read `CONTRIBUTING.md` for guidelines on how to contribute to the Linea project.

## 📄 License

Copyright © 2025 Gautham Nair.
Licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
