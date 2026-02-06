# Template Setup Guide

This guide walks you through customizing this template for your new project.

## Quick Start (Automated)

Run the initialization script to replace all placeholders automatically:

```bash
./scripts/init-template.sh my-project-name "Your Name" "your.email@example.com" "yourusername"
```

This will:
- Replace the project name everywhere
- Update author information
- Configure repository URLs
- Reset the changelog
- Update license copyright

After running, verify with `just check` and commit the changes.

## Manual Setup

If you prefer to customize manually, update the following files:

### Required Changes

| File | What to Change |
|------|----------------|
| `Cargo.toml` | `name`, `authors`, `description`, `repository` |
| `src/cli.rs:16` | `#[command(name = "...")]` |
| `src/cli.rs:61` | Binary name in completions generator |
| `src/lib.rs:1` | Module documentation header |
| `src/cli.rs:1` | Module documentation header |
| `src/logging.rs:1` | Module documentation header |
| `release-plz.toml` | `[[package]]` name |
| `LICENSE-MIT:3` | Copyright holder name |
| `CHANGELOG.md` | Clear template history |
| `README.md` | Project name and description |

### Files with Hardcoded Binary Name

These reference the binary name and need updating:

- `justfile:135` - `BINARY="target/release/..."`
- `justfile:155` - `cargo bloated --bin=...`
- `tests/cli.rs` - `cargo_bin_cmd!("...")`
- `.github/workflows/build.yml:37-38` - Artifact paths
- `.github/workflows/release-plz.yml:72` - Binary path
- `.github/workflows/pr.yml:83` - Binary path
- `scripts/generate-cliff-configs.py:67` - Config header
- `cliff.toml:1` - Comment header

### Optional: Homebrew Tap

If you want Homebrew distribution, edit `dist-workspace.toml`:

```toml
[dist.homebrew]
tap = "yourusername/homebrew-tap"
formula = "your-project-name"
```

## Configure GitHub Repository

### Required Secrets

| Secret | Purpose | Required For |
|--------|---------|--------------|
| `CODECOV_TOKEN` | Coverage upload | coverage.yml |
| `RELEASE_PLZ_TOKEN` | GitHub PAT with repo access | release-plz.yml |
| `CARGO_REGISTRY_TOKEN` | crates.io publishing | release-plz.yml |

### Recommended Settings

1. **Enable template repository** (if forking for your own template)
   - Settings → General → Template repository checkbox

2. **Branch protection for `main`**
   - Require PR reviews
   - Require status checks (ci, audit)
   - Require linear history

3. **Enable Dependabot**
   - Already configured in `.github/dependabot.yml`

## Choose Your CI Tier

This template includes three tiers of CI/CD complexity. Remove what you don't need:

### Tier 1: Essential (Recommended starting point)
Keep only:
- `.github/workflows/ci.yml` - Build, test, lint, format
- `.github/workflows/audit.yml` - Security scanning
- `.github/dependabot.yml` - Dependency updates

Delete:
- `.github/workflows/coverage.yml`
- `.github/workflows/commit-lint.yml`
- `.github/workflows/release-plz.yml`
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`
- `.github/workflows/pr.yml`
- `codecov.yml`
- `release-plz.toml`
- `dist-workspace.toml`

### Tier 2: Standard (Good for libraries)
Keep Tier 1 plus:
- `.github/workflows/coverage.yml` - Code coverage
- `.github/workflows/commit-lint.yml` - Commit message validation
- `.github/workflows/release-plz.yml` - Automated releases
- `codecov.yml`
- `release-plz.toml`

Delete:
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`
- `.github/workflows/pr.yml`
- `dist-workspace.toml`

### Tier 3: Comprehensive (Full-featured CLI tools)
Keep everything. This includes:
- Multi-platform builds
- Binary distribution via cargo-dist
- Supply chain security (SBOM, attestations)
- Binary size tracking

## Post-Setup Verification

After customization:

```bash
# Regenerate derived configs
just generate-configs

# Run all checks
just ci

# Verify the project builds and tests pass
just test
```

## Removing Template Files

After setup, you may want to remove:

- `TEMPLATE_SETUP.md` (this file)
- `scripts/init-template.sh` (if you used it)

## Troubleshooting

### "rust-template" still appears somewhere
Run this to find remaining references:
```bash
rg "rust-template" --type-not lock
```

### Config files out of sync
```bash
just generate-configs
just check-configs
```

### Tests fail after renaming
Update the binary name in `tests/cli.rs` - all `cargo_bin_cmd!()` calls.
