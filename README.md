# The Linea Programming Language - Compiler Edition

✅ **Version 3.2.0-alpha-1 'Avocado - Data Science Edition'** - Native Array Support (Minor: Core Datatypes)

> **Latest Update**: Added native array, matrix, and tensor types as core language features! Zero-copy indexing, element-wise operations, slicing, and built-in functions. Seamless interpreter ↔ compiler parity.

## 🚀 What's New in Linea 3.2.0-alpha-1 'Avocado - Data Science Edition'

**Alpha Release - New Features: Native Arrays, Matrices & Tensors:**

* ✅ **Native Arrays (1D)** - `var arr = [1, 2, 3, 4, 5]` - no imports needed!
* ✅ **Native Matrices (2D)** - `var matrix = [[1, 2], [3, 4]]` - direct support
* ✅ **Native Tensors (3D)** - `var tensor = [[[1, 2]], [[3, 4]]]` - multi-dimensional
* ✅ **Array Indexing** - `arr[0]`, `matrix[1][2]`, `tensor[0][1][1]`
* ✅ **Array Slicing** - `arr[1:4]` gets elements 1-3, `arr[::2]` gets every 2nd element
* ✅ **Element-wise Arithmetic** - `[1, 2] + [3, 4] = [4, 6]`, `arr * 2 = [2, 4, 6]`
* ✅ **Built-in Array Functions** - `len()`, `sum()`, `mean()`, `max()`, `min()`, `shape()`
* ✅ **Type Conversions** - `asFloat()`, `asInt()`, `asString()` work on arrays
* ✅ **Array Iteration** - `for x in arr` iterates over elements
* ✅ **Optimized Rust Code** - Uses Vec<T> with zero-copy operations
* ✅ **Interpreter ↔ Compiler Parity** - Identical output in both modes

### Array Examples:

**1D Arrays (Vectors):**
```linea
var numbers = [1, 2, 3, 4, 5]
display len(numbers)           # 5
display sum(numbers)           # 15
display numbers[0]             # 1
display numbers[1:4]           # [2, 3, 4] (slice from index 1 to 4)
```

**2D Arrays (Matrices):**
```linea
var matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
display matrix[0]              # [1, 2, 3] (first row)
display matrix[1][2]           # 6 (row 1, column 2)
display shape(matrix)          # [3, 3] (3 rows, 3 columns)
```

**3D Arrays (Tensors):**
```linea
var tensor = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
display tensor[0]              # [[1, 2], [3, 4]] (first plane)
display tensor[0][1][0]        # 3 (plane 0, row 1, column 0)
display shape(tensor)          # [2, 2, 2] (2×2×2 tensor)
```

**Element-wise Operations:**
```linea
var a = [1, 2, 3]
var b = [2, 3, 4]
display a + b                  # [3, 5, 7] (element-wise addition)
display a * 2                  # [2, 4, 6] (scalar multiplication)
```

**Built-in Functions:**
```linea
var arr = [1, 5, 3, 2, 4]
display len(arr)               # 5
display sum(arr)               # 15
display mean(arr)              # 3.0
display max(arr)               # 5
display min(arr)               # 1
display shape(arr)             # [5]
```

**Type Conversions:**
```linea
var ints = [1, 2, 3]
var floats = asFloat(ints)     # [1.0, 2.0, 3.0]
var back = asInt(floats)       # [1, 2, 3]
```

### Compiler Commands:
- `linea -V` → Show version information
- `linea --version` → Show version information
- `linea compile program.ln -o executable` → Native binary (no Linea needed!)
- `linea run program.ln` → Direct interpretation
- `linea gen-rust program.ln` → Generate Rust source code
- `linea parse program.ln` → Inspect AST for debugging

### Module System (v3.1.0+):

Linea supports importable libraries! Create `.ln` files in the `libs/` directory and import them in your programs:

```linea
# Import all items from a module
import math

# Import specific items only
import math { abs, max, isEven }

# Import from multiple modules
import strings { concat, repeat }
import utils { isPrime, sumTo }
```

**Available Standard Libraries:**

1. **math.ln** - Mathematical functions
   - `abs(x)` - Absolute value
   - `max(a, b)` - Maximum of two numbers
   - `min(a, b)` - Minimum of two numbers
   - `factorial(n)` - Calculate factorial
   - `isEven(x)` - Check if number is even
   - `isOdd(x)` - Check if number is odd

2. **strings.ln** - String manipulation
   - `intToString(num)` - Convert integer to string
   - `repeat(str, times)` - Repeat string n times
   - `length(str)` - Get string length
   - `concat(str1, str2)` - Concatenate strings
   - `separator(char, length)` - Create separator line

3. **utils.ln** - Utility functions
   - `printHeader(title)` - Print formatted header
   - `printSeparator()` - Print separator line
   - `printKeyValue(key, value)` - Print key-value pair
   - `isPrime(num)` - Check if number is prime
   - `sumTo(n)` - Sum of 1 to n
   - `multiplicationTable(num)` - Print multiplication table

**Example Usage:**

```linea
# Import libraries
import math { abs, isEven }
import utils { printHeader }

# Use library functions
printHeader("Calculator Demo")
var number = -15
display "Absolute value: " + abs(number)
display "Is even: " + isEven(number)
```

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
- Module imports: `import math { abs, max }`
- Function declarations: `func name { params } ... end`

**Example:**
```linea
var x = 100
var y = 50
display "Sum: " + (x + y)
for i from 0~5
  display i
```

## Version Information

Check the compiler version with:

```bash
$ linea -V
linea 3.1.0

$ linea --version
linea 3.1.0

# Get full help
$ linea --help
The Linea Programming Language Compiler

Usage: linea <COMMAND>

Commands:
  compile   Compile a Linea source file to an executable
  run       Run a Linea source file directly (interpreted)
  parse     Parse a Linea file and display the AST
  gen-rust  Generate Rust code from Linea source
  help      Print this message or the help of the given subcommand(s)
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

# With libraries
linea compile myapp.ln -o myapp
./myapp
```

## Project Structure

- `compiler/` - Full Rust compiler source code (~2,300 lines)
- `linea` - Pre-compiled CLI binary (ready to use!)
- `libs/` - Standard library files (math.ln, strings.ln, utils.ln)
- `examples/` - Example Linea programs
- `demo.ln` - Feature showcase (demonstrates libraries and imports)

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

## Version History

### Linea 3.2.0-alpha-1 (Current)
- **Type**: Alpha (features: native arrays, matrices, tensors)
- **Release Date**: March 2025
- **Key Additions**: 
  - Native array, matrix, tensor types (no imports needed)
  - Array indexing and slicing
  - Element-wise arithmetic operations
  - 6 built-in array functions: len, sum, mean, max, min, shape
  - Type conversion for arrays: asFloat, asInt, asString
  - Full interpreter ↔ compiler parity

### Linea 3.1.0 (Previous Minor Release)
- **Type**: Minor (feature: module system)
- **Release Date**: March 2025
- **Key Additions**:
  - Complete module system with import statements
  - Three standard libraries: math.ln, strings.ln, utils.ln
  - Version flags: -V, --version
  - 24 library functions

### Linea 3.0.1 (Previous Patch Release)
- **Type**: Patch (bug fixes)
- **Release Date**: March 2025
- **Key Fixes**:
  - Fixed string concatenation (String + int now works)
  - Fixed type casting (proper .parse() for String→Int)
  - Removed compiler warnings from generated code

### Linea 3.0.0 'Avocado' (Initial Major Release)
- **Type**: Major (complete rewrite)
- **Release Date**: March 2025
- **Key Features**:
  - Complete Rust compiler from scratch
  - Standalone binary generation
  - Full memory safety
  - 100-1000x performance improvement
  - Modern website design
  - Professional branding

## Roadmap

### v3.2.0 (Next Minor - Data Science)
- [ ] Optimize array operation codegen
- [ ] Add array slicing with step support
- [ ] Add negative indexing support
- [ ] Create 8 standard data science libraries
  - arrays.ln, dataframe.ln, io.ln, stats.ln
  - linalg.ln, preprocessing.ln, datasets.ln, transforms.ln

### v4.0.0 (Future Major - GPU & AI)
- [ ] GPU detection (NVIDIA, AMD, Intel dGPU/iGPU)
- [ ] LLM inference with GGML (GGUF format)
- [ ] Model training with LoRA support
- [ ] AI/ML standard libraries

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

## Author & License

**Author:** Gautham Nair  
**Email:** gautham.nair.2005@gmail.com  
**GitHub:** [@gauthamnair2005](https://github.com/gauthamnair2005)  
**License:** GPLv3 (See LICENSE file)

Linea 3.2.0-alpha-1 "Avocado" - Data Science Edition with native array, matrix, and tensor types added to the core language. Still a complete compilation from Python interpreter to Rust-based compiled language with full memory safety and native data science support.

Copyright © 2025 Gautham Nair. All rights reserved.
