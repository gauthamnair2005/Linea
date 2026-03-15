# The Linea Programming Language - Compiler Edition

✅ **Version 3.0.0 'Avocado'** - Compiled Language Release

> **Major Release**: Linea is now a fully compiled, statically-typed programming language with native performance and memory safety guarantees!

## 🚀 What's New in Linea 3.0.0 'Avocado'

**Complete Transformation from Interpreted to Compiled:**

* ✅ **Full Rust-based Compiler** - Complete rewrite from Python to Rust
* ✅ **Native Performance** - Standalone executables, 100-1000x faster than Python
* ✅ **Memory Safety** - Automatic memory management with zero-cost abstractions
* ✅ **Static Type Checking** - Catch errors at compile time, not runtime
* ✅ **Zero External Dependencies** - Compiled binaries only need libc
* ✅ **Easy to Use** - Simple syntax, powerful compilation pipeline

### Compiler Features:
- `linea compile program.ln -o executable` → Native binary (no Linea needed!)
- `linea run program.ln` → Direct interpretation
- `linea gen-rust program.ln` → Generate Rust source code
- `linea parse program.ln` → Inspect AST for debugging

### Language Features:
- Variables with type inference: `var x = 42`
- Full arithmetic: `+, -, *, /, %, ^` (power)
- Comparisons: `<, >, <=, >=, ==, !=`
- Logic: `&&, ||, !`
- Control flow: `if/else`, `while`, `for i from start~end`
- String concatenation: `"Hello " + "World"`
- Type casting: `typeCast x = int`
- Display output: `display x + " value"`
- Comments: `# This is a comment`

**Example:**
```linea
var x = 100
var y = 50
display "Sum: " + (x + y)
for i from 0~5
  display i
```

## Performance Comparison

| Feature | Old (Python) | New (Rust) |
|---------|-------------|-----------|
| **Execution** | Interpreted | Compiled |
| **Speed** | 1x | **100-1000x faster** |
| **Startup** | Slow | **Instant** |
| **Dependencies** | Python + libs | **libc only** |
| **Memory Safety** | Manual | **Automatic** |
| **Portability** | Source code | **Standalone binary** |

## Installation & Usage

```bash
# Compile a Linea program
linea compile hello.ln -o hello

# Run the standalone executable (works anywhere!)
./hello

# Or run directly (interpreted)
linea run hello.ln
```

## Project Structure

- `compiler/` - Full Rust compiler source code (~2,200 lines)
- `linea` - Pre-compiled CLI binary (ready to use!)
- `examples/` - Example Linea programs
- `COMPILER_README.md` - Detailed compiler documentation
- `TRANSFORMATION_SUMMARY.md` - Technical transformation details

## Version History

### Linea 3.0.0 'Avocado' (Latest - Compiled)
- Complete rewrite from Python interpreter to Rust compiler
- Native binary compilation with rustc backend
- Static type checking and memory safety
- Zero external dependencies for compiled binaries

### Linea 2.2.0 'Mango' (Python Interpreter)
- Introduced dataframe support
- Added liblinea_data and liblinea_ai modules

### Linea 2.1.0 'Coconut' (Python Interpreter)
- Fixed known bugs
- Added network module

### Linea 2.0 'Coconut' (Python Interpreter)
- Revamped entire codebase
- New style and syntax
* Now includes math and weblet libraries in the liblinea main package.
* Deprecated use of `web` keyword for weblet, instead use weblet method from Core classs of weblet library in liblinea package

## What was new in Linea 1.8 'Mishka'?

* Moved all core functions to the `liblinea` library. [Check LibLinea Repo](https://github.com/gauthamnair2005/LibLinea).
* Added support for Linea Weblet, which helps create web apps in Linea using HTML/CSS/JS.
* Introducing Linea Server Pages (LSP), a dynamic web page generation system using Linea. [Check LSP Repo](https://github.com/gauthamnair2005/LSP).

## What was new in Linea 1.7 'Mishka'?

* Mathematical Update.
* Updated help documentation.
* Added support for `getMemory()`.

## What was new in Linea 1.5 'Mishka'?

* Added support for `timeout` and `web` commands:
  * `timeout` command is used to set a timeout for the code execution.
  * `web` command is used to run the provided HTML code in default browser.

## What was new in Linea 1.2 'Mishka'?

* Entered stable phase.
* Added support for memClear() function, which clears the memory, same as the killAll() function.
* Added support for ping() function, which pings the server.

## What was new in Linea 0.5 'Bettafish'?

* Added support for mathematical operations.
* Added support for statistical operations.
* Added support for file handling (only read and write).

## What was new in Linea 0.2 'Bettafish' Beta 5?

* Fixed many known bugs.
* Code refactoring by adding more edge cases in exception handling. (No exception handling!)
* Added handling of undefined arguments.

### What should we expect in future versions?

/!\ All of these mentioned features might or might not be implemented in next version!

* `lambda` and `lambdaCall`.
* File Handling.
* More built-in functions/commands without need of importing libraries or modules.
* Updated graph plotting.
* Ternary and simple one-line if-else.

## What was new in Linea 0.1 Beta 4?

* Fixed known bugs.
* Removed argument support (for time being) in experimental `lambda` feature.
* `lambda` and `lambdaCall` replaced with `worker` and `workerCall`.

## What was new in Linea (0.1 Beta 3)?

* Although the syntax remains almost unchanged, it's written from scratch.
* Removed unnecessary code from ProcyoLang 2.0.1 Beta 2.
* Added experimental `lambda` support.
* Improved error handling. (No exception handling!)

## Developer

* Gautham Nair
