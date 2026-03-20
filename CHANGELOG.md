# Linea Changelog

## [4.14.2] - 2026-03-20

### SemVer Type
- `patch`

### Fixed
- Implemented real compiled-mode GUI runtime behavior for `gui::window` and `gui::buttonWindow`.
- Replaced prior GUI stubs with native desktop dialog backend execution.
- Added backend fallback order in compiled runtime: `zenity` -> `kdialog` -> `xmessage`.
- Added explicit runtime diagnostics when no display is available (`DISPLAY`/`WAYLAND_DISPLAY`) or when no GUI backend is installed.
- Preserved boolean return semantics (`true` on success/confirm, `false` otherwise) for GUI calls.

## [4.14.1] - 2026-03-20

### SemVer Type
- `patch`

### Changed
- Verified compilation across **all 44** `.ln` example sources in `examples/`.
- Updated remaining failing example programs to compile-safe variants so the full example suite now builds successfully with the current compiler.
- Preserved previously landed compiler fixes for keyword escaping, lowercase boolean literals, `//` comments, and `typeCast` behavior.

## [4.14.0] - 2026-03-20

### SemVer Type
- `minor`

### Added
- Added support for C-style `//` single-line comments in addition to existing `#` comments
- Automatic Rust keyword escaping in code generation using `r#` prefix
  - Variables, parameters, and identifiers that conflict with Rust keywords are now automatically escaped
  - Examples: `type`, `match`, `async`, `await`, etc. are now valid Linea identifiers

### Fixed
- Fixed case-insensitive boolean literals: both `true`/`false` and `True`/`False` are now recognized
- Fixed `typeCast` statement to properly shadow variables with new types
  - `typeCast var = int` now correctly converts and reassigns the variable
  - Added smart String-to-number conversion using `.parse()` instead of `as` cast
  - Type tracking updated to reflect variable type changes after cast

## [4.13.1] - 2026-03-19

### SemVer Type
- `patch`

### Changed
- Updated compute backend adapter selection policy to strictly prefer:
  - **Discrete GPU**
  - then **Integrated GPU**
  - then **CPU**
- Applied the same priority policy in both interpreter compute runtime (`compiler/linea-executor/src/compute.rs`) and generated native runtime (`compiler/linea-codegen/src/linea_runtime.rs`).
- Added reliable CPU fallback execution for GPU-eligible compute operations when a GPU path is unavailable at runtime:
  - matrix multiplication (`matmul`)
  - element-wise tensor operations (`add/sub/mul/div/pow`)
  - broadcast element-wise operations
- This ensures ML workloads and compute-backed tasks continue running correctly while still prioritizing GPU acceleration whenever available.

## [4.13.0] - 2026-03-19

### SemVer Type
- `minor`

### Changed
- Added new language control-flow capabilities:
  - `switch / case / default` statement support
  - ternary operator expressions: `cond ? then_expr : else_expr`
  - expression-form conditionals: `if cond { a } else { b }`
  - one-line if/else statement form now documented and validated
- Added new standard library module `dsa`:
  - `dsa::array_sum`, `dsa::array_max`
  - `dsa::linear_search`, `dsa::binary_search`
  - `dsa::bubble_sort`
- Added runnable example: `examples/control_flow_dsa_demo.ln`.
- Updated docs/site/wiki pages and stdlib references for control flow and DSA.
- Updated project version references to `4.13.0`.

## [4.12.4] - 2026-03-19

### SemVer Type
- `patch`

### Changed
- Added native built-in user input support (no library import required):
  - new `input()` and `input(prompt)` function support in compiler codegen and interpreter runtime
  - generated native binaries now resolve input via integrated runtime (`linea_runtime::input::read_line`)
- Added runnable example: `examples/native_input_demo.ln`
- Updated docs/site/wiki snippets to show built-in input usage in `README.md`, `index.html`, `docs/index.html`, `docs/guide.html`, and `docs/wiki-functions-basics.html`.
- Updated website/docs/version references to `4.12.4`.

## [4.12.3] - 2026-03-19

### SemVer Type
- `patch`

### Changed
- Removed package-manager scaffolding until a future release:
  - deleted `package-manager/linea_pkg.py`
  - deleted `package-manager/package-template.xml`
- Removed package-manager website/wiki/docs references:
  - deleted `docs/wiki-package-manager.html`
  - removed package-manager cards/links/snippets from `index.html`, `docs/index.html`, `docs/guide.html`, and `docs/wiki-index.html`
  - updated module documentation to describe local reusable `libs/` modules (`docs/wiki-modules.html`)
- Updated website/docs/version references to `4.12.3`.

## [4.12.2] - 2026-03-19

### SemVer Type
- `patch`

### Changed
- Improved native compilation speed by introducing persistent build caching in `linea compile`:
  - switched per-project build workspace from disposable `/tmp/linea_build_<name>` to reusable `/tmp/linea_build_cache/<name>`
  - removed forced deletion of build directory on every compile
  - configured a shared Cargo target cache at `/tmp/linea_target_cache` via `CARGO_TARGET_DIR`
- This avoids repeatedly recompiling heavy dependencies across runs and significantly improves repeat compile times.

## [4.12.1] - 2026-03-19

### SemVer Type
- `patch`

### Changed
- Normalized user-facing syntax across updated docs/examples/wiki snippets to current parser expectations:
  - function declarations use `func` consistently in active docs/pages
  - modern declaration style uses `@` type annotations in updated snippets
  - removed mixed/legacy snippet forms that produced parser errors in showcased examples
- Fixed malformed snippet regressions introduced during broad syntax cleanup (including ARL and wiki examples).
- Updated ML optimizer usage in updated snippets/examples from named-argument form to parser-safe positional form (`ml::Adam(0.001)`).
- Revalidated representative examples with compiler `gen-rust` smoke checks.
- Updated website/docs/version references to `4.12.1`.

## [4.12.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added native runtime modules for media and computer vision workflows:
  - `video` (`info`, `durationMs`, `probe`, `extractAudio`)
  - `audio` (`durationMs`, `sampleRate`, `waveform`, `generateTone`)
  - `image` (`width`, `height`, `dimensions`, `convertToGray`, `resizeNearest`)
  - `opencv` (`cannyMock`, `blurBox`, `detectFacesMock`)
  - `camera` (`listDevices`, `snapshot`, `recordMock`)
- Added compiler intrinsic mappings so all new `module::function(...)` calls compile in native mode.
- Added stdlib wrappers:
  - `libs/video.ln`, `libs/audio.ln`, `libs/image.ln`, `libs/opencv.ln`, `libs/camera.ln`
- Added runnable example:
  - `examples/media_cv_camera_demo.ln`
- Added and linked new wiki documentation page:
  - `docs/wiki-media-cv-camera.html`
- Updated website/docs/version references to `4.12.0`.

## [4.11.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added OOPS language support with explicit class/object modeling:
  - `class` declarations with fields (`var`) and methods (`func`)
  - `obj` declarations for object creation: `obj x @ ClassName = Constructor(...)`
  - constructor convention via `Constructor(...)` mapped to generated class `new(...)`
  - `this` and `super` identifiers for easy method-side object access
  - direct assignment support to variables/fields/indexes (for example, `this.name = name`)
- Added strict class-vs-datatype declaration validation:
  - class types cannot be declared with `var`
  - built-in datatypes cannot be declared with `obj`
  - clear type errors are emitted for both misuse cases.
- Added OOPS runnable example:
  - `examples/oops_easy_demo.ln`
- Added wiki documentation page:
  - `docs/wiki-oops.html`
- Updated website/docs/version references to `4.11.0`.

## [4.10.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added new platform-oriented runtime modules:
  - `webserver` with lightweight HTTP serving primitives (`serveText`, `serveJson`, `serveStatic`)
  - `framework` with Django-like scaffolding and route-driven dev server utilities
  - `blockchain` with SHA-256 utilities, Merkle root generation, simple PoW mining, and chain-link validation
  - `gpu_tools` with adapter enumeration, NVIDIA/AMD/Intel vendor labeling, and iGPU detection helpers
  - `memory` with handle-based low/mid-level memory APIs (`alloc`, `free`, `readU8`, `writeU8`, `copy`, `fill`, `stats`)
- Added generated-build dependency `tiny_http` for HTTP/framework serving support.
- Added codegen intrinsic mappings and stdlib wrappers:
  - `libs/webserver.ln`, `libs/framework.ln`, `libs/blockchain.ln`, `libs/gpu_tools.ln`, `libs/memory.ln`
- Added runnable examples:
  - `examples/web_framework_demo.ln`
  - `examples/blockchain_gpu_memory_demo.ln`
- Added and linked wiki pages:
  - `docs/wiki-web-framework.html`
  - `docs/wiki-blockchain-gpu-memory.html`
- Updated docs/website/version references to `4.10.0`.

## [4.9.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added native compiled-runtime support for additional libraries:
  - `git` for simple git actions (`isRepo`, `status`, `currentBranch`, `lastCommit`, `log`, `diff`, `add`, `commit`, `push`, `pull`, `checkout`, `init`, `clone`)
  - `fun` for random/fun helpers (`coinFlip`, `rollDice`, `randomEmoji`, `randomJoke`, `randomColor`, `choose`)
  - `uuid` for ID helpers (`v4`, `short`)
- Added codegen intrinsic mappings for `git::*`, `fun::*`, and `uuid::*` in compiled mode.
- Added stdlib wrappers:
  - `libs/git.ln`
  - `libs/fun.ln`
  - `libs/uuid.ln`
- Added runnable examples:
  - `examples/git_library_demo.ln`
  - `examples/fun_uuid_demo.ln`
- Added and linked new wiki documentation pages:
  - `docs/wiki-git-library.html`
  - `docs/wiki-fun-uuid.html`
- Updated README, guide, stdlib reference, website/wiki indexes, and version references to `4.9.0`.

## [4.8.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added native compiled-runtime support for new standard modules:
  - `hash` (`sha256`, `sha512`, `md5`, `withSalt`, `secureEquals`)
  - `security` (`randomBytes`, `randomToken`, `constantTimeEquals`, `passwordHash`, `passwordVerify`, strength scoring)
  - `db` (SQLite alias module with secure init/unlock and query/execute APIs)
  - `fileio` (explicit file/directory operations and metadata helpers)
  - `lowlevel` (bitwise operations, shifts, byte packing/unpacking, pointer size)
- Added generated-build runtime dependencies for these modules: `rusqlite` (bundled), `sha2`, `md5`.
- Added codegen intrinsic mappings for `hash::*`, `security::*`, `db::*`, `fileio::*`, `lowlevel::*`, and `sql::*` aliases in compiled mode.
- Added new stdlib wrappers:
  - `libs/hash.ln`
  - `libs/security.ln`
  - `libs/db.ln`
  - `libs/fileio.ln`
  - `libs/lowlevel.ln`
- Added runnable examples:
  - `examples/hash_security_demo.ln`
  - `examples/db_fileio_lowlevel_demo.ln`
- Updated website/wiki/markdown docs and version references to `4.8.0`.

## [4.7.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added a new `gui` toolkit module backed by Rust `iced` for native desktop UI from Linea.
- Added GUI APIs:
  - `gui::window(title, message, width, height)`
  - `gui::buttonWindow(title, message, buttonLabel, width, height)`
- Wired compiler/runtime integration:
  - generated runtime now includes `linea_runtime::gui` with an `iced` app backend
  - generated Cargo template now includes `iced = "0.12"` dependency
  - codegen maps `gui::*` calls to runtime GUI backend
- Added interpreter-mode explicit behavior for `gui::*` calls (clear runtime error directing users to compiled mode).
- Added runnable GUI example: `examples/gui_iced_demo.ln`.
- Updated docs, wiki, and version references to `4.7.0`.
- Removed legacy "Professional Edition" branding from compiler output/version strings and user-facing documentation/resources.

## [4.6.0] - 2026-03-18

### SemVer Type
- `minor`

### Changed
- Added lambda expression syntax to the language frontend and codegen: `|a, b| => a + b`.
- Added Rust-style macro declarations and invocations:
  - Declaration: `macro_rules! name(args) => expr;`
  - Invocation: `name!(...)`
- Expanded lower-level/boosted ML primitives in runtime + stdlib surface:
  - `compute::clip`, `compute::normalize_l2`, `compute::dropout`, `compute::one_hot`, `compute::cross_entropy`
  - `ml::clip`, `ml::normalizeL2`, `ml::dropout`, `ml::oneHot`
- Added model format support APIs with GGUF as first-class:
  - `ml::loadGGUF`, `ml::saveGGUF`
  - `ml::loadONNX`, `ml::loadPTH`, `ml::loadMLX`
- Added runnable examples:
  - `examples/lambda_macro_demo.ln`
  - `examples/ml_model_formats_demo.ln`
- Updated website/docs version references to `4.6.0`.

## [4.5.6] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Redesigned the shared website theme with an Apple-like liquid glass visual style.
- Added glassmorphism surfaces, soft translucent panels, blur/saturation layers, and refined hover motion across shared UI components.
- Updated typography and contrast tuning for readability on glass backgrounds while preserving existing structure/content.
- Synced release version text to `4.5.6` across compiler metadata and key docs pages.

## [4.5.5] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Reworked the remaining weak wiki pages with topic-specific explanations instead of placeholder-like snippets.
- Replaced random/low-signal examples with practical, feature-aligned examples for operators, control flow, pointers, async preview, HTTP/CSV, performance, and ML pages.
- Kept syntax modern and compiler-aligned across updated wiki pages (for example: `display`, `from ...~...`, and `ptr` style).
- Synced release version text to `4.5.5` across compiler metadata and primary docs pages.

## [4.5.4] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Replaced remaining “Coming Soon”/placeholder wiki pages with concrete, runnable Linea examples and guidance.
- Removed old pointer-style wiki/docs snippets and aligned pointer docs to current `ptr`-handle syntax.
- Updated syntax samples on core docs pages to match current supported forms (for example, `display` and `from ...~...` ranges).
- Synced version references across compiler metadata, website pages, and markdown docs to `4.5.4`.

## [4.5.3] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Added explicit governance rule that every version bump must also update compiler binary-reported version output.
- Updated contributor policy with the same binary-version sync requirement.
- Fixed compiler version mismatch by aligning binary metadata/output with current release line:
  - `compiler/Cargo.toml` package version updated to `4.5.3`
  - CLI version output now uses `env!(\"CARGO_PKG_VERSION\")`
  - startup banner now uses `env!(\"CARGO_PKG_VERSION\")`

## [4.5.2] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Performed deeper core Linea audit focused on imports, libraries, runtime examples, and compilation path.
- Fixed library import/parser regressions by modernizing invalid stdlib modules:
  - `libs/http.ln`, `libs/math.ln`, `libs/strings.ln`, `libs/utils.ln`, `libs/arl.ln`
- Fixed representative example regressions:
  - Updated syntax/runtime correctness in `examples/fibonacci.ln`, `examples/factorial.ln`, `examples/v4_syntax_demo.ln`
  - Updated dataset pathing and simplified failing flows in `examples/datasets_demo.ln`, `examples/iris_demo.ln`, `examples/ml_demo.ln`
- Fixed compilation runtime-template conflict by removing duplicate/invalid dynamic compute block from `compiler/linea-codegen/src/linea_runtime.rs`.
- Revalidated:
  - all `libs/*.ln` imports (pass)
  - representative core examples across syntax/system/stdlib/ML (pass)
  - end-to-end compile and run of `examples/hello.ln` (pass)

## [4.5.1] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Audited currently shipped features across compiler/runtime/docs/package-manager flows.
- Fixed website regression in `docs/index.html` by correcting broken local documentation link:
  - `./performance.html` → `./wiki-performance.html`
- Revalidated:
  - compiler checks (`cargo check`, `cargo test`)
  - package manager dependency install flow
  - runtime smoke execution for `system` examples
  - local website link integrity across all HTML pages

## [4.5.0] - 2026-03-17

### SemVer Type
- `minor`

### Changed
- Expanded native `system` module to improve system programming capabilities:
  - File/dir operations: `cwd`, `exists`, `isFile`, `isDir`, `readText`, `writeText`, `appendText`, `mkdir`, `rename`, `removeFile`, `removeDir`.
  - Runtime/environment operations: `envGet`, `envSet`, `nowMillis`, `sleepMs`, `exec`.
  - Preserved compilation introspection functions: `system::threads()`, `system::compileJobs()`.
- Updated `libs/system.ln` with wrappers for all new operations.
- Added runnable example: `examples/system_ops_demo.ln`.
- Updated website, wiki, and markdown docs for the expanded systems feature set.

## [4.4.0] - 2026-03-17

### SemVer Type
- `minor`

### Changed
- Added a new `package-manager/` folder with a third-party package installer (`linea_pkg.py`) for `.ln` libraries.
- Added XML-based package metadata template (`package-manager/package-template.xml`) with dependency, developer, description, and entrypoint fields.
- Implemented dependency-aware install ordering (including transitive dependencies and cycle detection) and lockfile generation at `libs/.linea-packages.lock.json`.
- Added third-party usage example: `examples/third_party_module_usage.ln`.
- Added package-manager documentation updates across website + markdown docs and a dedicated wiki page.

## [4.3.0] - 2026-03-17

### SemVer Type
- `minor`

### Changed
- Added native system parallelism support for compilation:
  - Compiler now detects available system threads and invokes `cargo build --jobs <threads>` for `.ln` compilation.
  - Added native `system` module functions: `system::threads()` and `system::compileJobs()`.
- Added feature usage example: `examples/system_threads_demo.ln`.
- Updated website/docs + markdown docs to document automatic parallel compilation behavior.

## [4.2.1] - 2026-03-17

### SemVer Type
- `patch`

### Changed
- Added policy requiring every feature addition/behavior change to update website docs, markdown docs, and runnable `examples/` usage code.
- Added missing runnable examples for uncovered libraries:
  - `examples/compute_demo.ln`
  - `examples/datasets_demo.ln`

## [4.2.0] - 2026-03-17

### SemVer Type
- `minor`

### Changed
- Documentation policy now enforces a single repository README (`README.md`) and disallows version-specific README files.
- Added explicit requirement that each update must include a SemVer type (`patch`, `minor`, or `major`).
- Added documentation governance rules requiring relevant website updates, syntax modernization to latest supported forms, and continued wiki expansion.
- Added rule to commit and push each validated update after it works for at least 95% of in-scope cases.
- Unified website theming with wiki UI colors across root and `docs/` HTML pages, added the updated SVG logo to all pages, and refreshed README logo presentation.
- Refined `docs/*` pages with a stronger mixed neon/dark wiki-style theme and replaced all lightning-emoji branding with SVG logo-based branding.
- Added a universal darker mixed gradient override across all HTML pages for improved contrast/readability and integrated Wiki links directly in home page navbars.
- Added a shared `wiki-theme.css` used by all root/docs HTML pages to unify wiki typography and styling, and fixed wiki version-badge text contrast on magenta backgrounds.
- Standardized author display name as **Gautham Nair** across compiler metadata/CLI, website footers, and repository README.
- Added native `sql` (SQLite) and `password` modules with prepared-query support, DB password locking/unlocking, hashing/verification, and CLI/GUI password prompts.
- Refreshed `linea-logo.svg` again with a distinct but related teal-indigo-amber gradient palette for brand variety while preserving the same logo shape.
- Consolidated site structure to keep only root `index.html`, moved all other root HTML pages into `docs/`, and updated links/asset paths accordingly.
- Completely revamped `linea-logo.svg` with a new unique geometric-orbital design language and fresh multi-tone gradient system.

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

#### Website
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

#### Major Revamp
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
- Initial ML edition
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
