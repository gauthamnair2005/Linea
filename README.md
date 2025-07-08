# The Linea Programming Language Interpreter

⚠️ Note : Old Linea 1.x.x code might be incompatible with Linea 2

## What's new in Linea Interpreter 25.07.08?

* New versioning scheme, the Linea Interpreter (this repository) and LibLinea.
* Any updates to LibLinea doesn't mean Linea Interpreter will get update.
* Linea Interpreter will only be updated when there's syntax update or new built-in function enablement in Linea Language.

## What was new in Linea 2.2.0 'Mango'?

* Introduced dataframe support.
* Introduced data handling features with the `liblinea_data` module [UNSTABLE].
* Introduced AI/ML features with the `liblinea_ai` module [UNSTABLE].

## What was new in Linea 2.1.0 'Coconut'?

* Fixed known bugs.
* Added support for network module under `liblinea_network`.

## What was new in Linea 2.0 'Coconut'?

* Revamped the entire codebase to improve performance and maintainability.
* New style and syntax.
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
