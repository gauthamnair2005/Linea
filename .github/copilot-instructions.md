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
- For every update, declare the SemVer update type: `patch`, `minor`, or `major`.
- Record the chosen SemVer type in the relevant released `CHANGELOG.md` version entry (no unreleased bucket).
- Any version change must be propagated across website pages and markdown documentation files so version references remain consistent everywhere.
- After an update is validated to work for at least 95% of in-scope cases, commit and push the update to GitHub.
