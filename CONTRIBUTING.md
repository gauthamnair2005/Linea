# Contributing to Linea

Thank you for your interest in contributing to Linea! As a professional systems language for the AI era, we value high-quality contributions that enhance performance, stability, and developer experience.

## Getting Started

1.  **Fork the Repository**: Start by forking the [Linea repository](https://github.com/gauthamnair2005/Linea) to your GitHub account.
2.  **Clone Locally**: Clone your fork to your local machine.
    ```bash
    git clone https://github.com/YOUR_USERNAME/Linea.git
    cd Linea
    ```
3.  **Setup Environment**: Ensure you have Rust and Cargo installed. Run `install.sh` to set up dependencies.

## Development Workflow

### Building the Compiler
Linea is built with Rust. Use standard cargo commands:
```bash
cd compiler
cargo build --release
```

### Running Tests
We maintain a high standard of code quality. Please run all tests before submitting a PR.
```bash
cargo test
```
To run Linea integration tests:
```bash
./linea test examples/
```

## Contribution Guidelines

*   **Code Style**: Follow standard Rust formatting (`cargo fmt`) for compiler code. For Linea code (`.ln`), follow the style of existing libraries.
*   **Documentation**: Ensure all new features are documented in `docs/` and have corresponding examples in `examples/`.
*   **Performance**: Linea prioritizes performance. Benchmarking is recommended for critical path changes.
*   **Commits**: Use clear, descriptive commit messages.

## Reporting Issues

If you find a bug or have a feature request, please open an issue on GitHub with:
1.  A clear description of the problem.
2.  Steps to reproduce (including code snippets).
3.  Expected vs. actual behavior.
4.  Environment details (OS, Linea version).

Thank you for helping build the future of systems programming!
