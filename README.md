# rust-template

A Rust project template with comprehensive CI/CD, security scanning, and release automation.

## Features

- **Tiered CI/CD** - Essential, Standard, and Comprehensive workflow configurations
- **Security Scanning** - cargo-audit and cargo-deny for vulnerability detection
- **Code Coverage** - Integrated with Codecov via cargo-llvm-cov
- **Release Automation** - release-plz for version management, cargo-dist for binary distribution
- **Conventional Commits** - Enforced via commitlint with git-cliff changelog generation
- **Supply Chain Security** - SBOM generation, attestations, and cargo-auditable

## Quick Start

```bash
# Create a new repository from this template
gh repo create my-project --template yourusername/rust-template
cd my-project

# Initialize with your project details
./scripts/init-template.sh my-project "Your Name" "your@email.com" "yourgithubusername"

# Run all CI checks locally
just ci
```

See [TEMPLATE_SETUP.md](TEMPLATE_SETUP.md) for detailed customization instructions.

## Available Commands

Run `just` to see all available commands. Common ones:

| Command | Alias | Description |
|---------|-------|-------------|
| `just build` | `just b` | Build in debug mode |
| `just build-release` | `just br` | Build in release mode |
| `just test` | `just t` | Run tests |
| `just test-fast` | `just tf` | Run tests with nextest |
| `just lint` | `just l` | Run clippy |
| `just format` | `just f` | Format code |
| `just audit` | `just a` | Security audit |
| `just check` | `just c` | Run all checks |
| `just ci` | - | Full CI pipeline locally |

## Project Structure

```
rust-template/
├── .github/
│   ├── actions/          # Reusable composite actions
│   │   ├── setup-rust/   # Rust toolchain setup
│   │   └── install-tools/# Tool installation
│   ├── workflows/        # CI/CD workflows
│   └── dependabot.yml    # Dependency updates
├── scripts/              # Python config generators
├── src/
│   ├── lib.rs           # Library code
│   └── main.rs          # Binary entrypoint
├── tests/               # Integration tests
├── Cargo.toml           # Project manifest
├── deny.toml            # cargo-deny configuration
├── justfile             # Task runner
└── ...
```

## CI Tiers

This template offers three tiers of CI/CD complexity. Choose based on your project needs:

### Tier 1: Essential

Best for: Personal projects, learning, simple utilities

| Workflow | Purpose |
|----------|---------|
| `ci.yml` | Build, test, lint, format, docs |
| `audit.yml` | Security vulnerability scanning |
| `dependabot.yml` | Automated dependency updates |

**To use Tier 1 only**, delete these files:
- `.github/workflows/coverage.yml`
- `.github/workflows/commit-lint.yml`
- `.github/workflows/release-plz.yml`
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`
- `.github/workflows/pr.yml`
- `codecov.yml`, `release-plz.toml`, `dist-workspace.toml`

### Tier 2: Standard

Best for: Libraries, published crates, team projects

Adds to Tier 1:
| Workflow | Purpose |
|----------|---------|
| `coverage.yml` | Code coverage with Codecov |
| `commit-lint.yml` | Conventional commit enforcement |
| `release-plz.yml` | Automated versioning and releases |

**To use Tier 2**, delete:
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`
- `.github/workflows/pr.yml`
- `dist-workspace.toml`

### Tier 3: Comprehensive (Default)

Best for: CLI tools, production binaries, open source projects

Adds to Tier 2:
| Workflow | Purpose |
|----------|---------|
| `release.yml` | Binary distribution via cargo-dist |
| `build.yml` | Multi-platform builds |
| `pr.yml` | PR validation with binary size tracking |

Includes supply chain security: SBOM, attestations, cargo-auditable.

## Configuration

### Secrets Required

| Secret | Purpose | When Needed |
|--------|---------|-------------|
| `CODECOV_TOKEN` | Code coverage upload | Tier 2+ |
| `RELEASE_PLZ_TOKEN` | GitHub PAT for releases | Tier 2+ |
| `CARGO_REGISTRY_TOKEN` | crates.io publishing | Tier 2+ |

### Customization

**Automated** (recommended):
```bash
./scripts/init-template.sh my-project "Your Name" "you@example.com" "githubuser"
```

**Manual**: See [TEMPLATE_SETUP.md](TEMPLATE_SETUP.md) for the complete list of files to update.

After customization:
1. Choose your CI tier and remove unwanted workflows
2. Run `just generate-configs` to regenerate derived configs
3. Configure GitHub repository secrets
4. Run `just ci` to verify everything works

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please read the contributing guidelines first.
