# CLAUDE.md

This file provides guidance to Claude Code when working with this Rust project template.

## Project Overview

This is a Rust project template with comprehensive CI/CD configuration, featuring:

- **Tiered CI workflows**: Essential (Tier 1), Standard (Tier 2), Comprehensive (Tier 3)
- **Security scanning**: cargo-audit, cargo-deny
- **Code coverage**: cargo-llvm-cov with Codecov integration
- **Release automation**: release-plz for versioning, cargo-dist for binary distribution
- **Conventional commits**: commitlint with git-cliff changelog generation
- **Supply chain security**: SBOM, attestations, cargo-auditable

## Common Commands

```bash
# Development
just build          # Build debug
just test           # Run tests
just lint           # Run clippy
just format         # Format code
just check          # All local checks

# CI simulation
just ci             # Full CI pipeline locally
just pr             # PR checks with coverage

# Configuration
just generate-configs  # Regenerate cliff.toml and .commitlintrc.json
just check-configs     # Verify configs are in sync
```

## Architecture

### Single Source of Truth Pattern

`commit-types.json` is the single source of truth for:
- Conventional commit types
- git-cliff changelog categories
- commitlint validation rules

Python scripts in `scripts/` generate:
- `cliff.toml` from commit-types.json
- `.commitlintrc.json` from commit-types.json

CI validates that generated files stay in sync via `check-configs-sync.py`.

### GitHub Actions Structure

```
.github/
├── actions/
│   ├── setup-rust/      # Toolchain setup with caching
│   └── install-tools/   # Tool installation via taiki-e/install-action
├── workflows/
│   ├── ci.yml           # Core CI (test, lint, format, docs)
│   ├── audit.yml        # Security scanning
│   ├── coverage.yml     # Code coverage
│   ├── commit-lint.yml  # Commit message validation
│   ├── release-plz.yml  # Version management
│   ├── release.yml      # Binary distribution (cargo-dist)
│   ├── test.yml         # Cross-platform testing
│   ├── build.yml        # Multi-target builds
│   └── pr.yml           # PR validation with semver checks
└── dependabot.yml       # Dependency updates
```

### Workflow Tiers

**Tier 1 (Essential)**: ci.yml, audit.yml, dependabot.yml
- Basic CI for any project

**Tier 2 (Standard)**: + coverage.yml, release-plz.yml, commit-lint.yml
- Professional-grade with coverage and release automation

**Tier 3 (Comprehensive)**: + test.yml, build.yml, pr.yml, release.yml
- Full-featured with cross-platform testing and supply chain security

## Key Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Project manifest with release profiles |
| `deny.toml` | cargo-deny security/license configuration |
| `release-plz.toml` | Version management configuration |
| `dist-workspace.toml` | cargo-dist binary distribution |
| `codecov.yml` | Coverage thresholds and reporting |
| `commit-types.json` | Single source of truth for commit types |
| `justfile` | Task runner with shortcuts |

## Making Changes

### Adding a new commit type

1. Edit `commit-types.json` - add the type with description and changelog_group
2. Run `just generate-configs` to regenerate cliff.toml and .commitlintrc.json
3. Commit the changes

### Updating workflows

- Always use SHA pinning with ratchet comments: `@sha # ratchet:action@version`
- Test changes locally with `act` if available
- Update dependabot.yml if adding new ecosystem

### Adding dependencies

1. Add to `Cargo.toml`
2. Run `cargo build` to update Cargo.lock
3. Verify license compatibility: `cargo deny check licenses`

## Secrets Required

| Secret | Purpose |
|--------|---------|
| `CODECOV_TOKEN` | Coverage upload |
| `RELEASE_PLZ_TOKEN` | GitHub PAT for releases |
| `CARGO_REGISTRY_TOKEN` | crates.io publishing |
