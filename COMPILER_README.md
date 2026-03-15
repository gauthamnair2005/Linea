# Linea Programming Language - Compiled Edition

## Overview

Linea is now a **compiled, statically-typed programming language** built in Rust with native performance and memory safety guarantees. The new compiler transforms Linea source code into optimized native executables that run standalone without any runtime dependencies.

## Architecture

The Linea compiler is built with a modular architecture:

- **linea-core**: Type system, error handling, and value representation
- **linea-ast**: Lexer, parser, and AST definitions  
- **linea-executor**: Interpreter for direct execution of Linea programs
- **linea-codegen**: Code generator that produces optimized Rust code
- **linea-compiler**: CLI tool for compilation, interpretation, and parsing

## Features

### Compilation Pipeline

1. **Lexer**: Tokenizes Linea source code
2. **Parser**: Builds an Abstract Syntax Tree (AST)
3. **Type Checker**: Validates types and catches type errors at compile time
4. **Code Generator**: Converts AST to Rust code with native performance
5. **Rust Compiler**: Compiles to optimized machine code using `rustc`

### Language Features

- **Variable Declaration**: `var x = 10`
- **Type Inference**: Automatic type detection from expressions
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^` (power)
- **Comparisons**: `<`, `>`, `<=`, `>=`, `==`, `!=`
- **Logic**: `&&`, `||`, `!`
- **Control Flow**: `if`, `else`, `while`, `for i from start~end`
- **Type Casting**: `typeCast x = int`
- **Display Output**: `display x + " Hello"`
- **String Concatenation**: `var msg = "Hello" + " World"`

## Installation

### From Source

```bash
cd compiler
cargo build --release
ln -s target/release/linea-compiler /usr/local/bin/linea
```

## Usage

### Compile a Linea Program

```bash
linea compile program.ln -o program
./program  # Run the standalone executable
```

### Run Linea Directly (Interpreted)

```bash
linea run program.ln
```

### Generate Rust Code

```bash
linea gen-rust program.ln -o program.rs
```

### Parse and Display AST

```bash
linea parse program.ln
```

## Example Programs

### Hello World with Variables

```linea
var message = "Hello, Linea!"
display message
```

### Arithmetic

```linea
var x = 10
var y = 20
var sum = x + y
display "Sum: " + sum
```

### Loops

```linea
for i from 0~5
  display i

var count = 0
while count < 3 {
  display "Count: " + count
  varUpd count = count + 1
}
```

### Type System

```linea
var num = 42
var text = "Number: "
display text + num

var float_val = 3.14
display float_val

var is_true = True
display is_true
```

### Conditional Execution

```linea
var age = 25
if age >= 18 {
  display "Adult"
} else {
  display "Minor"
}
```

## Memory Safety

The Linea compiler leverages Rust's ownership system to provide:

- **No null pointer errors**: All variables are initialized
- **No buffer overflows**: Bounds checking on arrays
- **No data races**: Thread-safe compiled code
- **Automatic cleanup**: Memory is freed when variables go out of scope

## Performance

Compiled Linea programs are optimized by `rustc` with the `-O` flag, producing highly efficient native machine code. Performance is comparable to hand-written Rust code.

## Type System

Linea supports the following types:

- `int`: 64-bit signed integer (default numeric type)
- `float`: 64-bit floating point  
- `string`: UTF-8 encoded text
- `bool`: Boolean (True/False, Yes/No)
- `array`: Homogeneous collections

Type inference automatically determines types from context, but explicit type casting is available:

```linea
var str_num = "42"
typeCast str_num = int
display str_num + 8  # Output: 50
```

## Error Handling

The compiler provides detailed error messages:

```
✗ Compilation failed: Syntax error at line 1, column 5: Unexpected token: ...
✗ Runtime error: Variable 'undefined_var' not found
✗ Type error: Cannot assign string to variable of type int
```

## Compilation Targets

Linea compiles to:

- Linux (x86-64, ARM, ARM64)
- macOS (x86-64, ARM64)
- Windows (via WSL or cross-compilation)

## Roadmap

Future enhancements:

- [ ] Functions with parameters and return types
- [ ] Modules and imports (`use liblinea_math`)
- [ ] Standard library integration (math, networking, data processing)
- [ ] Web compilation (WebAssembly)
- [ ] Pattern matching
- [ ] Generics
- [ ] Advanced type system features

## Development

Build the compiler:

```bash
cd compiler
cargo build --release
cargo test
```

The compiler source code is organized as a Cargo workspace with clear separation of concerns.

## Author

**Gautham Nair** - Creator and maintainer of Linea 3.0.0 "Avocado"  
Email: gautham.nair.2005@gmail.com  
GitHub: [@gauthamnair2005](https://github.com/gauthamnair2005)

## License

See LICENSE in the root directory.
