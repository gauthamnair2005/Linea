# Linea Language Specification (v4.14.3)

This document defines the core grammar, typing behavior, module semantics, and error model for Linea.

## 1. Lexical Structure

- Identifiers: start with letter or `_`, followed by alphanumeric or `_`.
- Comments: `# ...` and `// ...` single-line comments.
- Strings: single or double quoted, with common escapes (`\n`, `\t`, `\"`, `\'`).
- Booleans: `true/false` (case-insensitive also accepts `True/False`).

## 2. Declarations and Types

- Typed variable declaration: `var name @ type = expr`
- Object declaration: `obj name @ ClassName = Constructor(...)`
- Class instances must use `obj`, built-in values must use `var`.

Common primitive types:
- `int`, `float`, `str`, `bool`, `ptr`, `any`

Collections:
- Arrays: `[int]`, `[str]`
- Maps: `{str:int}`

## 3. Control Flow

- `if / else`
- One-line if/else statement
- Ternary expression: `cond ? a : b`
- If-expression: `if cond { a } else { b }`
- `for` ranges with `~`: `for i from 1~10 step 2 { ... }`
- `while` loops
- `switch/case/default`

## 4. Functions and Classes

- Function declaration: `func name(param: type, ...) -> type { ... }`
- Return statement: `return expr`
- Class declaration: `class Name { ... }`
- Constructor style object creation: `obj p @ Person = Constructor(...)`
- `this` and `super` are supported in class contexts.

## 5. Module and Import Semantics

- Import module: `import math`
- Import selected symbols: `import math { abs, max }`
- Import module names must be single identifiers (e.g. `import math`).
- Use namespaced calls for module functions: `math::abs(-5)`.
- Compiler validates:
  - Module file exists (e.g. `libs/math.ln`)
  - Imported symbol names exist when using `{ ... }`

## 6. Error Model and Diagnostics

Compiler diagnostics include:
- precise line/column in syntax errors
- expected-vs-found token messages
- actionable hints for common mistakes (missing `@`, missing braces, invalid import form)
- import validation errors with available symbol previews

## 7. CLI Output Semantics

Compiler CLI emits:
- **bold green** success messages
- **bold red** error/failure messages

This includes compile success/failure, runtime errors, parse errors, and file I/O failures surfaced by the CLI.

## 8. Compatibility Policy

- Existing v4 syntax remains supported.
- New diagnostics are backward compatible (message improvements only).
- Import validation introduces earlier failure for invalid imports to avoid late-stage codegen/runtime confusion.
