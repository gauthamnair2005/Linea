# Copilot Repository Instructions

## Documentation Policy
- Maintain exactly one README in this repository: root `README.md`.
- Do not create version-specific README files (for example, `README_v4.md`).
- For versioned details, update `CHANGELOG.md` and `RELEASE_NOTES_V*.md` instead of creating extra READMEs.
- Every update must also be reflected in website documentation pages (`docs/*.html`) when relevant, and website quality should be maintained.
- Every feature addition or behavior change must update website pages and markdown documentation, and include/refresh runnable example source code in `examples/` showing import and usage.
- Prefer migrating old syntax to the latest supported syntax across docs, examples, and user-facing references during updates.
- Continuously expand wiki coverage with additional focused wiki pages whenever new or under-documented topics are identified.

## Release Versioning Policy
- For every update, declare the SemVer update type based on the change:
  - **`patch`**: Bug fixes, documentation updates, performance improvements that don't change functionality
  - **`minor`**: New features, additions that don't break existing code (backward compatible)
  - **`major`**: Breaking changes that require user intervention or code changes (requires explicit approval)
- Record the chosen SemVer type in the relevant released `CHANGELOG.md` version entry (no unreleased bucket).
- Version numbering format: `MAJOR.MINOR.PATCH` (e.g., `4.14.0`)
- Any version change must be propagated across ALL version references:
  - `compiler/Cargo.toml` (package version)
  - `CHANGELOG.md` (new version entry)
  - Website pages (`docs/*.html`) - update version numbers in examples and references
  - Markdown documentation files
  - The compiler binary itself reports correct version via `env!("CARGO_PKG_VERSION")` in `compiler/src/main.rs`
- After an update is validated to work for at least 95% of in-scope cases, commit and push the update to GitHub automatically.

## Git Workflow
- Every completed update (bug fix, feature, documentation change) should be committed with a descriptive message
- Commit message format: Include what was changed and why
- Always include Co-authored-by trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`
- After committing, automatically push to the remote repository
- Git authentication is pre-configured; just execute git commands
