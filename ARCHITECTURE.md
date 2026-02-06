# Architecture

<!-- TODO: Replace this with a brief description of your project -->

This document describes the architecture of the project. Update this as the codebase evolves.

## Module Structure

<!-- TODO: Update this table to reflect your actual module layout -->

```
src/
├── main.rs         # CLI entry point
└── lib.rs          # Core library logic
```

### Module Responsibilities

<!-- TODO: Add descriptions for each module -->

| Module | Responsibility |
|--------|---------------|
| `main.rs` | CLI entry point, argument parsing, delegates to library |
| `lib.rs` | Core logic and public API |

## Data Flow

<!-- TODO: Describe the main data flows through your application -->

```
Input → Processing → Output
```

## Key Design Decisions

<!-- TODO: Document important architectural decisions as they are made -->

### Decision Record Template

When making significant architectural decisions, document them here or in `docs/adr/`:

- **Decision**: What was decided
- **Context**: Why the decision was needed
- **Consequences**: What trade-offs were accepted

## Configuration

| File | Purpose |
|------|---------|
| `Cargo.toml` | Project manifest with release profiles |
| `deny.toml` | cargo-deny security and license policy |
| `release-plz.toml` | Version management configuration |
| `dist-workspace.toml` | cargo-dist binary distribution |
| `codecov.yml` | Coverage thresholds and reporting |
| `commit-types.json` | Single source of truth for commit types |
| `justfile` | Task runner recipes |
