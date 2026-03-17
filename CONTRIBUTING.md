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

## Documentation and Versioning Policy

*   **Single README Rule**: Keep only one README file at the repository root (`README.md`).
*   **No Version-Specific READMEs**: Do not add files like `README_vX.md`. Use `CHANGELOG.md` and `RELEASE_NOTES_V*.md` for versioned release notes.
*   **Mandatory SemVer Type per Update**: Every update must declare a SemVer type (`patch`, `minor`, or `major`) in `CHANGELOG.md` under the relevant released version entry.
*   **Version Propagation Requirement**: Any version change must also be reflected across website pages and all markdown documentation files.
*   **Website Documentation Required**: For relevant changes, update website documentation in `docs/*.html` and keep website content consistent and maintained.
*   **Feature Documentation + Examples Required**: Every feature addition or behavior change must update website docs and markdown docs, and include updated runnable example source code in `examples/` showing how to import and use it.
*   **Syntax Modernization**: Prefer updating old syntax to the latest supported syntax in docs, examples, and user-facing references.
*   **Wiki Expansion**: Add and maintain wiki pages for new features, workflows, and commonly requested topics whenever possible.
*   **Commit and Push Discipline**: After validating an update works for at least 95% of in-scope cases, commit and push it to GitHub.

## Reporting Issues

If you find a bug or have a feature request, please open an issue on GitHub with:
1.  A clear description of the problem.
2.  Steps to reproduce (including code snippets).
3.  Expected vs. actual behavior.
4.  Environment details (OS, Linea version).

Thank you for helping build the future of systems programming!
