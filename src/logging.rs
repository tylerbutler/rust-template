//! Logging initialization for rust-template.
//!
//! Uses `tracing-subscriber` with an environment filter. The verbosity level
//! from the CLI's `-v` flag is mapped to a default tracing level, which can
//! be overridden via the `RUST_LOG` environment variable.

use tracing_subscriber::EnvFilter;

/// Initialize logging with the given verbosity level.
///
/// Verbosity mapping:
/// - 0: warn
/// - 1: info
/// - 2: debug
/// - 3+: trace
///
/// The `RUST_LOG` environment variable takes precedence over the verbosity flag.
pub fn setup_logging(verbosity: u8) {
    let filter = match verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .init();
}
