//! CLI definition for rust-template.
//!
//! Uses clap derive macros for argument parsing. The CLI supports:
//! - Subcommands via the [`Commands`] enum
//! - Global `--verbose` flag for controlling log verbosity
//! - Hidden `--markdown-help` flag for documentation generation

use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io;

use crate::version::version_string;

/// A Rust project template with comprehensive CI/CD.
#[derive(Parser, Debug)]
#[command(name = "rust-template", version = version_string(), about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = ArgAction::Count)]
    pub verbose: u8,

    /// Print help in markdown format (for documentation generation)
    #[arg(long, hide = true)]
    pub markdown_help: bool,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Greet someone by name
    Greet {
        /// Name of the person to greet
        #[arg(default_value = "World")]
        name: String,
    },

    /// Generate shell completions for the given shell
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
}

impl Cli {
    /// Handle the `--markdown-help` flag. Returns `true` if markdown help was printed.
    #[must_use]
    pub fn handle_markdown_help(&self) -> bool {
        if self.markdown_help {
            clap_markdown::print_help_markdown::<Self>();
            return true;
        }
        false
    }

    /// Generate shell completions to stdout.
    pub fn print_completions(shell: Shell) {
        let mut cmd = Self::command();
        clap_complete::generate(shell, &mut cmd, "rust-template", &mut io::stdout());
    }
}
