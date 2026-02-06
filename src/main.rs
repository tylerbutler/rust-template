use clap::Parser;
use rust_template::cli::{Cli, Commands};
use rust_template::logging::setup_logging;

fn main() {
    let cli = Cli::parse();

    // Handle hidden --markdown-help flag
    if cli.handle_markdown_help() {
        return;
    }

    setup_logging(cli.verbose);
    tracing::info!("starting application");

    match cli.command {
        Some(Commands::Greet { name }) => {
            println!("{}", rust_template::greet(&name));
        }
        Some(Commands::Completions { shell }) => {
            Cli::print_completions(shell);
        }
        None => {
            // No subcommand provided: show help
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
        }
    }

    // Check for updates after command execution
    #[cfg(feature = "update-check")]
    check_for_updates();
}

/// Check for updates and print a notification if a new version is available.
///
/// Uses tiny-update-check to query crates.io with caching (24 hours).
#[cfg(feature = "update-check")]
fn check_for_updates() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    if let Ok(Some(update)) = tiny_update_check::check(name, version) {
        eprintln!(
            "Update available: {} → {} (run `cargo install {name}` to update)",
            update.current, update.latest
        );
    }
}
