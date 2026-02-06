# Development Guide

## Prerequisites

- **Rust** (stable) - https://rustup.rs/
- **just** - Task runner - https://github.com/casey/just
- **cargo-nextest** (optional) - Faster parallel test runner - https://nexte.st/
- **cargo-llvm-cov** (optional) - Code coverage - https://github.com/taiki-e/cargo-llvm-cov
- **cargo-audit** + **cargo-deny** (optional) - Security auditing

## Building

```bash
just build            # Debug build (alias: b)
just build-release    # Release build (alias: br)
```

Or directly with cargo:

```bash
cargo build
cargo build --release
```

## Testing

```bash
just test             # Run all tests (alias: t)
just test-fast        # Run with nextest for parallel execution (alias: tf)
just test-coverage    # Run with coverage, generates lcov.info (alias: tc)
```

Or directly with cargo:

```bash
cargo test
cargo nextest run     # Faster parallel execution
```

### Coverage Reports

```bash
just coverage-html    # Generate HTML coverage report
just coverage-report  # Open coverage report in browser
```

## Linting

Clippy is configured to run with `-D warnings` (warnings are errors).

```bash
just lint             # Run clippy (alias: l)
```

Or directly:

```bash
cargo clippy -- -D warnings
```

## Formatting

```bash
just format           # Format code (alias: f)
just fmt-check        # Check formatting without changes (alias: fc)
```

Or directly:

```bash
cargo fmt --all
cargo fmt --all -- --check
```

## Running All Checks

```bash
just check            # Format check + lint + test (alias: c)
```

## CI

The project uses a tiered CI workflow system:

- **Tier 1 (Essential)**: `ci.yml`, `audit.yml`, `dependabot.yml` - basic CI for any project
- **Tier 2 (Standard)**: + `coverage.yml`, `release-plz.yml`, `commit-lint.yml` - coverage and release automation
- **Tier 3 (Comprehensive)**: + `test.yml`, `build.yml`, `pr.yml`, `release.yml` - cross-platform testing and supply chain security

Run CI checks locally:

```bash
just ci               # Full CI pipeline (format, lint, configs, test, audit, build)
just pr               # PR checks with coverage
```

## Release Process

Releases are automated via [release-plz](https://release-plz.ieni.dev/) and [cargo-dist](https://opensource.axo.dev/cargo-dist/):

1. **Automatic PR**: When commits are pushed to `main`, release-plz creates/updates a release PR with version bumps and changelog updates.
2. **Merge to release**: Merging the release PR creates a git tag (`v<version>`).
3. **Binary distribution**: The tag triggers cargo-dist to build platform-specific binaries and create a GitHub release.

### Conventional Commits

Version bumps are determined from conventional commit messages:
- `fix:` - Patch version bump
- `feat:` - Minor version bump
- `feat!:` or `BREAKING CHANGE:` - Major version bump

### Configuration Management

Commit types are managed via `commit-types.json` (single source of truth):

```bash
just generate-configs  # Regenerate cliff.toml and .commitlintrc.json (alias: gc)
just check-configs     # Verify configs are in sync (alias: cc)
```

## Additional Commands

```bash
just clean            # Clean build artifacts
just install          # Install binary locally
just update           # Update dependencies
just audit            # Security audit with cargo-audit and cargo-deny (alias: a)
just docs             # Build documentation (alias: d)
just docs-open        # Build and open docs in browser
just watch-test       # Watch mode for tests
just watch-lint       # Watch mode for clippy
just changelog        # Generate changelog (alias: cl)
```

### Binary Size Analysis (Linux only)

```bash
just record-size      # Record release binary size (alias: rs)
just bloat            # Analyze binary composition (alias: bl)
```

## Project Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for module structure and design decisions.
