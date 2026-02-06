//! CLI integration tests using `assert_cmd`.
//!
//! These tests verify CLI behavior by running the compiled binary.
//! Currently the binary is a simple hello-world; these tests will become
//! more meaningful once clap is added for argument parsing.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn runs_successfully() {
    cargo_bin_cmd!("rust-template")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
}

#[test]
fn help_flag_succeeds() {
    // Once clap is added, update this to check for usage information
    cargo_bin_cmd!("rust-template")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn version_flag_succeeds() {
    // Once clap is added, update this to check for version string
    cargo_bin_cmd!("rust-template")
        .arg("--version")
        .assert()
        .success();
}
