# Linea v4.1.0 Release Notes - BREAKING RELEASE

**Release Date**: March 17, 2026  
**Version**: 4.1.0 (Breaking Changes)
**Theme**: "Middle-Level Language with Pointers and Simplified Ranges"

---

## ⚠️ BREAKING CHANGES - Read Before Upgrading

This is a **major breaking release**. Code written for v4.0.0 requires updates:

### 1. For Loop Syntax Changed
**Old (v4.0.0):**
```linea
for i from 0..10 {
    display i
}
```

**New (v4.1.0):**
```linea
for i from 0~10 {
    display i
}
```

### 2. Backward Compatibility Removed
- Old colon syntax (`:`) NO LONGER supported
- **Only** `@` type operator syntax works
- **Only** `~` range operator works in for loops

### 3. New Pointer Support
- `&variable` - Get address (address-of operator)
- `*pointer` - Dereference pointer
- New middle-level control over memory

---

## 🎉 Major Features

### 1. Range Operator `~`

The tilde (`~`) simplifies for loop ranges:

```linea
// Basic range (1 to 10 inclusive)
for i from 1~10 {
    display i
}

// With custom step
for i from 1~100 step 5 {
    display i
}

// Reverse iteration
for i from 10~1 step -1 {
    display i
}

// Dynamic ranges
var start @ int = 1
var end @ int = 50
for i from start~end {
    display i
}

// With different steps
for i from 0~20 step 2 {  // 0, 2, 4, 6, ..., 20
    display i
}
```

### 2. Pointer Support

Linea v4.1 adds low-level pointer support while maintaining high-level safety:

#### Ptr Datatype (Simplified Syntax)

The `ptr` datatype provides a clean, type-safe way to work with pointers:

```linea
// Simple pointer syntax
var x @ int = 42
var ptr_to_x @ ptr = x    // Automatically captures address

// Multiple pointers
var y @ int = 100
var ptr_to_y @ ptr = y

// ptr stores memory addresses safely
display "Pointers created"
```

#### Address-Of Operator (`&`)
```linea
var x @ int = 42
var ptr @ &int = &x    // Traditional pointer syntax - ptr holds address of x
```

#### Dereference Operator (`*`)
```linea
var value @ int = *ptr    // Get value at address
```

#### Pointer Arithmetic
```linea
var arr @ [int] = [1, 2, 3, 4, 5]
var ptr @ &int = &arr[0]      // Point to first element
var second @ int = *(ptr + 1) // Access second element
```

#### Pointer Examples
```linea
// Example 1: Using ptr datatype
var x @ int = 99
var ptr @ ptr = x
display "Address stored in ptr"

// Example 2: Array pointers with traditional syntax
var numbers @ [int] = [10, 20, 30]
var ptr @ &int = &numbers[0]
display *ptr        // 10
display *(ptr + 1)  // 20
display *(ptr + 2)  // 30
```

### 3. Dynamic Type Sizing (Preview)

Upcoming: Automatic `i32` → `i64` promotion based on value magnitude:

```linea
var small @ int = 100          // i32 (fits in 32 bits)
var large @ int = 1000000000   // i64 (needs 64 bits)
var explicit32 @ i32 = 42      // Force 32-bit
var explicit64 @ i64 = 999999  // Force 64-bit
```

---

## 🔄 Migration Guide (v4.0.0 → v4.1.0)

### Step 1: Update For Loops

**Before:**
```linea
for i from 1..10 {
    display i
}
```

**After:**
```linea
for i from 1~10 {
    display i
}
```

### Step 2: Verify Type Syntax

Type syntax hasn't changed in v4.1, but **old `:` syntax no longer works**:

**Before (v3.5.0):**
```linea
var x: i64 = 42  // ❌ NO LONGER WORKS
```

**Current (v4.0.0+):**
```linea
var x @ int = 42  // ✓ WORKS
```

### Step 3: Replace All `.._` with `~`

Find all instances of `..` and replace with `~`:
- `1..10` → `1~10`
- `0..100` → `0~100`
- `10..1` → `10~1`

### Step 4: Add Pointers (Optional)

If using memory-level operations, add pointer syntax:
```linea
var x @ int = 42
var ptr @ int = &x  // Get address
display *ptr        // Use address
```

---

## 📊 Syntax Summary

| Feature | v4.0.0 | v4.1.0 |
|---------|--------|--------|
| Type Annotation | `@` | `@` ✓ |
| For Loop | `1..10` | `1~10` ✓ |
| Pointers | ❌ | `&`, `*` ✓ |
| Step Support | ❌ | `step N` ✓ |
| Colon Syntax | `:` | ❌ Removed |

---

## 🚀 Compiler Updates

### New Tokens
- `Tilde` (`~`) - Range operator
- `Ampersand` (`&`) - Address-of operator
- (Dereference uses existing `Star` token)

### Parser Enhancements
- Range expressions: `for i from 1~10`
- Unary operators: `&expr`, `*expr`
- Step modifier: `for i from 1~10 step 2`

### Type System
- Pointer types: `&int`, `&str`, etc.
- Pointer arithmetic in expressions

### Code Generation
- Rust reference emission (`&`)
- Rust dereference emission (`*`)
- Smart range compilation

---

## 📈 Performance

- **No runtime overhead** for pointers (compile-time feature)
- **Identical performance** to v4.0.0
- Range loops compile to optimized Rust code

---

## 🔗 Compatibility Notes

### With Decorators
Pointers don't conflict with decorators:
```linea
@gpu
fn compute(arr @ &[int]) {
    // GPU compute with pointers
}

@async
fn fetch(ptr @ &str) {
    // Async operations with pointers
}
```

### With Collections
Arrays, matrices, tensors fully support pointers:
```linea
var mat @ Matrix = ...
var ptr @ &Matrix = &mat
```

---

## ✨ Language Positioning

Linea v4.1 positions itself as a **middle-level language**:

- **High-Level**: Automatic memory management, type inference
- **Middle-Level**: Pointers, addresses, manual memory access
- **Safety**: Compile-time checking, bounds verification
- **AI-First**: Native GPU support, tensor operations

---

## 🐛 Known Limitations

1. Pointer arithmetic limited (no `ptr[i]` syntax yet)
2. Function pointers not supported in v4.1
3. Unsafe blocks not implemented
4. Dynamic type sizing preview only

---

## 📚 Examples

### Example 1: Basic Pointers
```linea
var x @ int = 42
var ptr @ int = &x
display *ptr  // Output: 42
```

### Example 2: Array Iteration
```linea
var arr @ [int] = [1, 2, 3, 4, 5]

// Traditional loop
for i from 0~4 {
    display arr[i]
}

// With step
for i from 0~10 step 2 {
    display i
}
```

### Example 3: Reverse Loop
```linea
var items @ [str] = ["first", "second", "third"]
var count @ int = 2

for i from count~0 step -1 {
    display items[i]
}
```

---

## 🔮 Future Direction

**v4.2 Planned Features:**
- Function pointers
- Unsafe blocks
- Pointer to pointer (`&&T`)
- Smart memory management with pointers

**v5.0 Vision:**
- Multi-platform compilation
- Advanced pointer patterns
- Macro system

---

## 📦 Installation

```bash
cd /path/to/Linea/compiler
cargo build --release
cp target/release/linea-compiler ../linea
```

---

## ✅ Verification Checklist

- [ ] Update all `.ln` files to use `~` instead of `..`
- [ ] Remove any old `:` syntax
- [ ] Test pointers with `&` and `*`
- [ ] Verify `step` works in for loops
- [ ] Update documentation

---

## 📝 Support

- **Documentation**: See `guide.html`
- **Examples**: Check `examples/v41_*.ln`
- **Issues**: GitHub Issues

---

*Linea v4.1: Middle-Level Language with Memory Control*
