# Release Process

This document describes the semantic versioning and release process for tiny-terminal.

## Semantic Versioning

We follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** version (X.0.0): Incompatible API changes
- **MINOR** version (0.X.0): Add functionality in a backward compatible manner
- **PATCH** version (0.0.X): Backward compatible bug fixes

## Creating a Release

### 1. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this
```

### 2. Update CHANGELOG.md

Move items from `[Unreleased]` to a new version section:

```markdown
## [0.2.0] - 2025-11-13

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z
```

### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"
```

### 4. Create and Push Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 5. Automated Process

Once the tag is pushed, GitHub Actions will automatically:

1. Run the full CI pipeline (build, test, clippy, fmt, audit)
2. Create a GitHub Release with the version notes
3. Build binaries for multiple platforms:
   - Linux (x86_64 GNU and musl)
   - macOS (x86_64 and ARM64)
   - Windows (x86_64)
4. Upload binaries as release artifacts
5. Publish to crates.io (if `CARGO_TOKEN` secret is configured)

## CI/CD Pipeline

### Continuous Integration (CI)

The CI pipeline runs on every push and pull request:

- **Build**: Compiles the project on Linux, macOS, and Windows
- **Test**: Runs all unit and integration tests
- **Clippy**: Lints the code for common mistakes and improvements
- **Format**: Checks code formatting with rustfmt
- **Audit**: Scans dependencies for security vulnerabilities

### Release Pipeline

The release pipeline is triggered by pushing a version tag (e.g., `v0.2.0`):

1. Creates a GitHub Release
2. Builds optimized binaries for all supported platforms
3. Strips debug symbols for smaller binary size
4. Uploads binaries as release artifacts
5. Publishes to crates.io

## Setting Up Secrets

To enable crates.io publishing, add a `CARGO_TOKEN` secret:

1. Generate a token at https://crates.io/me
2. Go to GitHub repository Settings → Secrets and variables → Actions
3. Add new secret: `CARGO_TOKEN` with your crates.io token

## Version History

- **v0.1.0** (2025-11-12): Initial release with Matrix effect
