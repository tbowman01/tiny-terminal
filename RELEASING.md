# Release Process

This document describes the calendar versioning and release process for tiny-terminal.

## Calendar Versioning (CalVer)

We follow [Calendar Versioning](https://calver.org/) using the format `YYYY.MM.MICRO`:

- **YYYY**: Full year (e.g., 2025)
- **MM**: Zero-padded month (e.g., 01 for January, 11 for November)
- **MICRO**: Incrementing number for releases within the same month (0, 1, 2, ...)

### Examples

- `2025.11.0` - First release in November 2025
- `2025.11.1` - Second release (bugfix or feature) in November 2025
- `2025.12.0` - First release in December 2025

### Why CalVer?

1. **Time-based releases**: Makes it clear when a release was published
2. **Predictable versioning**: No subjective decisions about major vs minor changes
3. **Simple communication**: "Get the November 2025 release" is clearer
4. **Continuous delivery friendly**: Supports regular, time-based releases
5. **Breaking changes**: Can occur in any release; clearly documented in CHANGELOG

## Creating a Release

### 1. Determine New Version

Use CalVer format `YYYY.MM.MICRO`:

```bash
# First release in a month: YYYY.MM.0
# Example: 2025.11.0

# Additional releases in same month: YYYY.MM.1, YYYY.MM.2, etc.
# Example: 2025.11.1

# New month resets MICRO to 0
# Example: 2025.12.0
```

### 2. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
version = "2025.11.1"  # Update using CalVer format
```

### 3. Update CHANGELOG.md

Move items from `[Unreleased]` to a new version section:

```markdown
## [2025.11.1] - 2025-11-15

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z

[Unreleased]: https://github.com/tbowman01/tiny-terminal/compare/v2025.11.1...HEAD
[2025.11.1]: https://github.com/tbowman01/tiny-terminal/releases/tag/v2025.11.1
```

### 4. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md Cargo.lock
git commit -m "chore: bump version to 2025.11.1"
```

### 5. Create and Push Tag

```bash
# Use CalVer format: vYYYY.MM.MICRO
git tag -a v2025.11.1 -m "Release v2025.11.1"
git push origin main
git push origin v2025.11.1
```

### 6. Automated Process

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

The release pipeline is triggered by pushing a version tag in CalVer format (e.g., `v2025.11.0`):

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

- **v2025.11.0** (2025-11-12): Initial release with Matrix effect, comprehensive test suite, and CalVer adoption
