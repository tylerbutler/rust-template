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
# Clone the template
gh repo create my-project --template yourusername/rust-template
cd my-project

# Install dependencies
just deps

# Run tests
just test

# Run all CI checks locally
just ci
```

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

## Configuration

### Secrets Required

| Secret | Purpose | When Needed |
|--------|---------|-------------|
| `CODECOV_TOKEN` | Code coverage upload | Tier 2+ |
| `RELEASE_PLZ_TOKEN` | GitHub PAT for releases | Tier 2+ |
| `CARGO_REGISTRY_TOKEN` | crates.io publishing | Tier 2+ |

### Customization

1. Update `Cargo.toml` with your project details
2. Update `commit-types.json` to customize commit types
3. Run `just generate-configs` to regenerate derived configs
4. Configure GitHub repository secrets

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please read the contributing guidelines first.
