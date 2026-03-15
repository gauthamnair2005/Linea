# The Linea Programming Language - Compiler Edition

✅ **Version 3.0.1 'Avocado'** - Compiled Language Release

> **Latest Update**: Added module system support, -V/--version flags, and importable libraries written in Linea. The compiler now supports modular development with reusable library files!

## 🚀 What's New in Linea 3.0.1 'Avocado Patch'

**Patch Release - New Features & Improvements:**

* ✅ **Module System** - Import libraries with `import module { items }` syntax
* ✅ **Importable Libraries** - Create reusable `.ln` library files in `libs/` directory
* ✅ **Version Flags** - Use `linea -V` or `linea --version` to check compiler version
* ✅ **Fixed String Concatenation** - String + number operations now work correctly
* ✅ **Fixed Type Casting** - String to integer conversion uses proper `.parse()` method
* ✅ **Clean Rust Code Generation** - Generated code compiles without warnings
* ✅ **Interpreter Parity** - Interpreter and compiled output produce identical results
* ✅ **Full Rust-based Compiler** - Complete rewrite from Python to Rust
* ✅ **Native Performance** - Standalone executables, 100-1000x faster than Python
* ✅ **Memory Safety** - Automatic memory management with zero-cost abstractions
* ✅ **Static Type Checking** - Catch errors at compile time, not runtime
* ✅ **Zero External Dependencies** - Compiled binaries only need libc
* ✅ **Easy to Use** - Simple syntax, powerful compilation pipeline

### Compiler Commands:
- `linea -V` → Show version information (NEW!)
- `linea --version` → Show version information (NEW!)
- `linea compile program.ln -o executable` → Native binary (no Linea needed!)
- `linea run program.ln` → Direct interpretation
- `linea gen-rust program.ln` → Generate Rust source code
- `linea parse program.ln` → Inspect AST for debugging

### Module System:

Linea now supports importable libraries! Create `.ln` files in the `libs/` directory and import them in your programs:

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
linea 3.0.1

$ linea --version
linea 3.0.1

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

Linea 3.0.0 "Avocado" - A complete compilation of the Linea language from Python interpreter to Rust-based compiled language with full memory safety.

Copyright © 2025 Gautham Nair. All rights reserved.
