//! Integration tests for rust-template

use rstest::{fixture, rstest};
use rust_template::greet;

#[test]
fn test_greeting_integration() {
    let result = greet("Integration Test");
    assert!(result.contains("Integration Test"));
    assert!(result.starts_with("Hello"));
}

/// Fixture that provides a set of sample names for testing.
#[fixture]
fn sample_names() -> Vec<&'static str> {
    vec!["Alice", "Bob", "Charlie"]
}

/// Demonstrates rstest fixtures: the `sample_names` fixture is automatically
/// injected as an argument.
#[rstest]
fn greet_all_sample_names(sample_names: Vec<&'static str>) {
    for name in sample_names {
        let greeting = greet(name);
        assert_eq!(greeting, format!("Hello, {name}!"));
    }
}

/// Demonstrates rstest parametrized tests with `#[case]`.
#[rstest]
#[case("World", "Hello, World!")]
#[case("Rust", "Hello, Rust!")]
#[case("", "Hello, !")]
fn greet_produces_expected_output(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(greet(input), expected);
}
