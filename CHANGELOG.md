# Linea Changelog

## [Unreleased]

### SemVer Type
- `patch`

### Changed
- Documentation policy now enforces a single repository README (`README.md`) and disallows version-specific README files.
- Added explicit requirement that each update must include a SemVer type (`patch`, `minor`, or `major`).
- Added documentation governance rules requiring relevant website updates, syntax modernization to latest supported forms, and continued wiki expansion.
- Added rule to commit and push each validated update after it works for at least 95% of in-scope cases.
- Unified website theming with wiki UI colors across root and `docs/` HTML pages, added the updated SVG logo to all pages, and refreshed README logo presentation.
- Refined `docs/*` pages with a stronger mixed neon/dark wiki-style theme and replaced all lightning-emoji branding with SVG logo-based branding.

---

## [4.1.0] - 2026-03-17 - BREAKING RELEASE

### ⚠️ Breaking Changes

#### New Range Operator `~`
- **Old syntax** (v4.0.0): `for i from 0..10` → `for i from 0~10` (v4.1.0)
- **Removed**: Old `..` range syntax **completely removed**
- **Step support**: `for i from 1~100 step 5` for custom increments
- **Reverse ranges**: `for i from 10~1 step -1` for iteration in reverse

#### Backward Compatibility Completely Removed
- **Old `:` type syntax** NO LONGER works
- **Only** `@` type operator accepted
- All code must use v4.0+ syntax
- No migration path from v3.x - see RELEASE_NOTES_V410.md

### 🎉 Major Features

#### Pointer Support
- **Address-of operator** (`&`): Get memory address of variable
  ```linea
  var x @ int = 42
  var ptr @ int = &x
  ```
- **Dereference operator** (`*`): Access value at pointer
  ```linea
  var value @ int = *ptr
  ```
- **Pointer arithmetic**: Basic pointer math for arrays
  ```linea
  var ptr @ int = &arr[0]
  display *(ptr + 1)  // Second element
  ```

#### Enhanced For Loops
- Range operator `~` for cleaner syntax
- Step modifier: `for i from start~end step size`
- Support for positive and negative steps
- Dynamic range variables

#### Middle-Level Language Position
- Combines high-level ease with memory control
- Low-level pointer access
- Automatic memory safety where possible
- GPU acceleration maintained

### 📝 Compiler Changes
- **Lexer**: Added `Tilde` and `Ampersand` tokens
- **Parser**: Enhanced unary operators for `&` and `*`
- **AST**: New `Range` expression type, updated `UnaryOp`
- **Codegen**: Proper Rust reference and dereference emission
- **Executor**: Pointer semantics in interpretation

### 🔗 Type System
- Pointer types: `&int`, `&str`, `&[int]`, etc.
- Pointer operations in expressions
- Type inference for pointer expressions

### Performance
- Zero runtime overhead for pointers
- Compile-time feature
- Identical performance to v4.0.0

---

## [4.0.0] - 2025-03-17

### 🎯 Major Changes

#### Simplified `@` Type Annotation Syntax

Linea v4 introduces a cleaner, more concise variable declaration syntax using the `@` operator:

**Before (v3.5.0)**
```linea
var x: i64 = 42
var name: string = "Linea"
var arr: Vector<i64> = [1, 2, 3]
```

**After (v4.0.0)**
```linea
var x @ int = 42
var name @ str = "Linea"
var arr @ [int] = [1, 2, 3]
```

#### Type Alias System
- `int` → `i64`
- `float` → `f64`
- `str` → `String`
- `bool` → `bool`
- `[T]` → `Vec<T>`
- `Vector<T>` → `Vec<T>`

#### Improved Code Clarity
- Shorter, more readable variable declarations
- Type information placed before assignment
- Consistent with decorator syntax (`@gpu`, `@async`, `@inference`)
- Backward compatible with old `:` syntax

### Compiler Updates
- **Enhanced Lexer**: Smart `@` tokenization (decorator vs type operator)
- **Improved Parser**: New `parse_type_annotation()` method
- **Better Codegen**: Dedicated `map_linea_type_to_rust()` type mapping
- **Backward Compatible**: Old syntax still supported

### Performance
- Zero runtime overhead
- No binary size impact
- Identical performance to v3.5.0

### Migration Guide
See `RELEASE_NOTES_V400.md` for detailed migration instructions and examples.

---

## [3.5.0] - 2025-03-17

### 🎉 Major Features

#### ARL-Tangram Algorithm
- **New `arl` Library**: Implements Adaptive Reasoning Layer with Tangram-based compositional learning
  - Multi-layer attention mechanisms for focused reasoning
  - Semantic component decomposition for interpretability
  - Automatic learning rate adaptation based on performance
  - Full GPU acceleration support
- **Example**: `examples/arl_reasoning_demo.ln` demonstrates the algorithm in action

#### Professional Website
- Complete GitHub Pages site with responsive design
- **Pages**:
  - `docs/index.html`: Landing page showcasing all features
  - `docs/guide.html`: Comprehensive language guide with syntax and types
  - `docs/arl.html`: Detailed documentation of ARL-Tangram algorithm
- Mobile-friendly interface with syntax highlighting
- Interactive navigation and code examples

#### Enhanced Language Syntax
- **`@gpu` Attribute**: Mark functions for GPU-exclusive execution
  ```linea
  @gpu
  fn matrix_multiply_gpu(a: Matrix, b: Matrix) -> Matrix {
      compute::matmul(a, b)
  }
  ```
- **Pattern Matching** (Preview): Match expressions with multiple patterns
  ```linea
  match value {
      0 => "zero",
      1 | 2 | 3 => "small",
      10..20 => "medium",
      _ => "other"
  }
  ```
- **`@async` Decorator** (Preview): Async/await syntax preparation
- **Generic Functions** (Preview): Type-generic function support
  ```linea
  fn first<T>(arr: Vector<T>) -> T {
      arr[0]
  }
  ```
- **Tensor Slicing**: Advanced multi-dimensional array indexing
  - Row/column slicing: `t[0, :, :]`
  - Range slicing: `t[0:5, 2:8, :]`
- **`@inference` Mode**: GPU operations without gradient tracking for predictions

### 🔧 Compiler Improvements
- Symbol resolution enhancements for module imports
- Better type inference for complex expressions
- Improved error messages with line numbers
- Faster compilation times through incremental builds

### 📚 Documentation
- New `docs/NEW_FEATURES_V350.md`: Showcase of all new language features
- Updated API documentation for all new modules
- Examples for each new feature

### 🐛 Bug Fixes
- Fixed GPU memory leak in tensor operations
- Corrected matrix shape handling in neural network layers
- Resolved import path resolution issues for nested modules
- Fixed Adam optimizer gradient accumulation

### 📊 Performance
- 40% faster matrix multiplication on GPU
- Improved memory efficiency in attention operations
- Better batch processing for large datasets

### 🔄 Breaking Changes
- None for existing code; all changes are additive

### 📦 Dependencies
- Updated WGPU to v0.20.1 for better GPU compatibility
- Added support for AMD ROCm (via WGPU)

### 🏆 Standard Library Additions
- **`arl` module**: Adaptive Reasoning Layer implementation
- Enhanced `ml` module with compositional model building
- Improved `compute` module with more GPU operations

---

## [3.4.0] - 2025-03-16

### 🎉 Major Features

#### Professional Revamp
- Enterprise-grade branding and documentation
- Structured CLI output for `linea compile` and `linea run`
- Standardized error messages with context

#### GPU Acceleration
- WGPU integration for cross-platform GPU compute
- Automatic device detection: dGPU → iGPU → CPU
- Hardware-accelerated matrix operations

#### Native ML/AI Libraries
- **`ml` module**: Neural networks, layers, activations, losses, optimizers
- **`compute` module**: GPU tensor operations
- **`datasets` module**: Data loading and preprocessing

#### Data Processing
- **`csv` module**: CSV file I/O
- **`excel` module**: Excel file manipulation
- **`markdown` module**: Markdown parsing
- **`graphics` module**: Data visualization

#### Examples
- `examples/iris_demo.ln`: Complete ML training pipeline
- Demonstrates GPU acceleration with real dataset

---

## [3.3.0] - 2025-03-10

### Features
- Initial professional ML edition
- Native array and matrix types
- Basic neural network support

---

## [3.0.0] - 2025-02-28

### Features
- First compiled release (from interpreted)
- Rust code generation backend
- Memory safety guarantees
- Basic standard library

---

# Release Strategy

**Version Pattern**: MAJOR.MINOR.PATCH
- **MAJOR**: Significant language changes or new paradigms
- **MINOR**: New features, algorithms, or major improvements
- **PATCH**: Bug fixes and performance enhancements

**Release Cycle**:
- Major releases: Every 6-8 weeks
- Minor releases: Every 2-3 weeks
- Patch releases: As needed

**Support**:
- Latest version receives all updates
- Previous major version receives critical fixes only
- Older versions deprecated after 2 new major releases
