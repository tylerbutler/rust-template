//! CLI integration tests using `assert_cmd`.
//!
//! These tests verify CLI behavior by running the compiled binary.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn no_args_shows_help() {
    cargo_bin_cmd!("rust-template")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn help_flag_shows_usage() {
    cargo_bin_cmd!("rust-template")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("greet"));
}

#[test]
fn version_flag_shows_version() {
    cargo_bin_cmd!("rust-template")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn greet_default_name() {
    cargo_bin_cmd!("rust-template")
        .arg("greet")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn greet_custom_name() {
    cargo_bin_cmd!("rust-template")
        .args(["greet", "Rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Rust!"));
}

#[test]
fn completions_bash() {
    cargo_bin_cmd!("rust-template")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust-template"));
}

#[test]
fn markdown_help_produces_output() {
    cargo_bin_cmd!("rust-template")
        .arg("--markdown-help")
        .assert()
        .success()
        .stdout(predicate::str::contains("# Command-Line Help"));
}
