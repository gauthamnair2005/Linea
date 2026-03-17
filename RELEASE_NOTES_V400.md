# Linea v4.0.0 Release Notes

**Release Date**: March 17, 2025  
**Version**: 4.0.0  
**Theme**: "Clean Syntax, Powerful AI"

---

## 🎉 Major Features

### 1. Simplified `@` Type Annotation Syntax (Breaking Change)

Linea v4 introduces a cleaner, more concise syntax for variable declarations using the `@` operator for type annotation:

#### Old Syntax (v3.5.0 and earlier)
```linea
var x: i64 = 42
var name: string = "Linea"
var pi: f64 = 3.14
var active: bool = True
var numbers: Vector<i64> = [1, 2, 3]
```

#### New Syntax (v4.0.0+)
```linea
var x @ int = 42
var name @ str = "Linea"
var pi @ float = 3.14
var active @ bool = True
var numbers @ [int] = [1, 2, 3]
```

### 2. Type Alias Mapping

Linea v4 introduces human-friendly type aliases that map to native Rust types:

| Linea Type | Rust Type | Usage |
|-----------|-----------|-------|
| `int` | `i64` | `var x @ int = 42` |
| `float` | `f64` | `var pi @ float = 3.14` |
| `str` | `String` | `var name @ str = "Hello"` |
| `bool` | `bool` | `var flag @ bool = True` |
| `[T]` | `Vec<T>` | `var arr @ [int] = [1,2,3]` |
| `Vector<T>` | `Vec<T>` | `var vec @ Vector<int> = [1,2,3]` |

### 3. Improved Code Clarity

The new `@` syntax makes code more readable:

**Comparison:**
```
Old:  var user_id: i64 = 12345
New:  var user_id @ int = 12345

Old:  var items: Vector<f64> = [1.5, 2.3, 3.1]
New:  var items @ [float] = [1.5, 2.3, 3.1]
```

---

## 🔄 Migration Guide

### For Existing Code

**Option 1: Gradual Migration**  
Linea v4 maintains **backward compatibility** with old syntax:
```linea
// Both work in v4
var x: int = 10     // Old syntax (still supported)
var y @ int = 20    // New syntax
```

**Option 2: Full Migration**  
Use the recommended `@` syntax going forward:
```linea
var x @ int = 10
var y @ str = "Hello"
var z @ float = 3.14
```

### Common Patterns

| Use Case | Old Syntax | New Syntax |
|----------|-----------|-----------|
| Integer | `var n: i64 = 42` | `var n @ int = 42` |
| String | `var s: string = "hi"` | `var s @ str = "hi"` |
| Float | `var f: f64 = 2.5` | `var f @ float = 2.5` |
| Boolean | `var b: bool = True` | `var b @ bool = True` |
| Array | `var arr: Vector<i64>` | `var arr @ [int]` |
| Matrix | `var mat: Matrix` | `var mat @ Matrix` |
| Tensor | `var t: Tensor` | `var t @ Tensor` |

---

## 💡 Examples

### Basic Variables
```linea
var age @ int = 25
var height @ float = 5.9
var name @ str = "Alice"
var is_student @ bool = True

display age
display height
display name
display is_student
```

### Collections
```linea
var scores @ [int] = [85, 90, 95]
var temperatures @ [float] = [72.5, 75.3, 68.1]
var usernames @ [str] = ["alice", "bob", "charlie"]
```

### Type Inference Still Works
```linea
// Type annotation optional when type is obvious
var implicit_int = 42           // Inferred as int
var explicit_int @ int = 42     // Explicit annotation
```

---

## 🎯 Benefits

1. **Conciseness**: Shorter, cleaner code
2. **Readability**: Type annotation comes before value
3. **Consistency**: Aligns with `@gpu`, `@async`, `@inference` decorators
4. **Pythonic**: Familiar syntax for users from Python/Rust backgrounds
5. **Future-proof**: Extensible for complex type annotations

---

## 🚀 Performance

- No performance impact compared to v3.5.0
- Same compilation target (Rust with WGPU GPU acceleration)
- Identical runtime characteristics

---

## 📊 Statistics

- **Type Annotation Overhead**: 0% (compile-time only)
- **Binary Size**: Same as v3.5.0
- **Compilation Time**: Unchanged
- **Runtime Performance**: Identical to v3.5.0

---

## 🔗 Decorator Compatibility

The `@` operator is also used for decorators:

```linea
@gpu
func fast_compute(x @ [float]) {
    // GPU-exclusive computation
}

@async
func async_task() {
    // Asynchronous execution
}

@inference
func predict(x @ [float]) -> float {
    // Inference mode (no gradients)
}
```

**Context Rule**: Parser distinguishes based on position:
- Before `var`/`func`: `@` is a decorator
- After identifier in declaration: `@` is a type operator

---

## 📝 What's New in Compiler

### Changes
- **linea-ast**: Added `type_annotation: Option<String>` to VarDeclaration
- **linea-lexer**: Enhanced `@` tokenization to support both decorators and type operator
- **linea-parser**: New `parse_type_annotation()` method for parsing type specs
- **linea-codegen**: Added `map_linea_type_to_rust()` function for type mapping

### Backward Compatibility
- Old syntax (`var x: int = 42`) still supported
- Existing code compiles without modification
- No breaking changes to function signatures or module system

---

## 🌟 Highlights

### For Beginners
- Easier to learn with shorter, clearer syntax
- Consistent with visual appearance of values
- Less typing, more focus on logic

### For Power Users
- Faster prototyping with concise declarations
- Better code clarity in large files
- Seamless interop with GPU decorators

---

## 🐛 Known Limitations

1. Type annotation still optional - type inference works
2. Complex generic types still use `<>` syntax
3. Tuple types not yet optimized for `@` syntax

---

## 🔮 Future Direction

v4.0 establishes the `@` operator as Linea's primary type annotation mechanism. Future versions may extend this to:
- Constraint specifications: `var x @ int {1..100}`
- Ownership annotations: `var x @ owned<int>`
- Lifetime markers: `var x @ &'a int`

---

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/gauthamnair2005/Linea.git
cd Linea

# Build the compiler
cd compiler
cargo build --release

# Use the compiler
./target/release/linea-compiler --help
```

---

## 🙋 Support

- **Documentation**: https://github.com/gauthamnair2005/Linea
- **Issues**: GitHub Issues
- **Examples**: `examples/` directory

---

## ✨ Release History

- **v4.0.0** - Clean Syntax Release (March 17, 2025)
- **v3.5.0** - ARL-Tangram & Professional Website (Feb 2025)
- **v3.4.0** - ML/AI Core Release (Jan 2025)
- **v3.3.0** - GPU Acceleration (Dec 2024)

---

*Linea: AI-First Compiled Language for the Modern Developer*
