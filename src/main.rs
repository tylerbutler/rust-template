use clap::Parser;
use rust_template::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    // Handle hidden --markdown-help flag
    if cli.handle_markdown_help() {
        return;
    }

    // TODO: wire up verbosity to tracing subscriber
    #[allow(clippy::no_effect_underscore_binding)]
    let _verbosity = cli.verbose;

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
}
