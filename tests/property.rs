//! Property-based tests using `proptest`.
//!
//! These tests generate random inputs to verify that functions maintain
//! their invariants across a wide range of values.

use proptest::prelude::*;
use rust_template::greet;

proptest! {
    /// The `greet` function should always wrap the input in the greeting format.
    #[test]
    fn greet_always_contains_input(name in "\\PC{0,100}") {
        let result = greet(&name);
        prop_assert!(result.contains(&name),
            "greeting {:?} should contain input {:?}", result, name);
    }

    /// The greeting should always start with "Hello, " and end with "!".
    #[test]
    fn greet_has_correct_prefix_and_suffix(name in "[a-zA-Z0-9 ]{0,50}") {
        let result = greet(&name);
        prop_assert!(result.starts_with("Hello, "),
            "greeting {:?} should start with 'Hello, '", result);
        prop_assert!(result.ends_with('!'),
            "greeting {:?} should end with '!'", result);
    }

    /// The greeting length should always equal "Hello, " + name + "!".
    #[test]
    fn greet_length_is_predictable(name in "\\PC{0,100}") {
        let result = greet(&name);
        let expected_len = "Hello, ".len() + name.len() + "!".len();
        prop_assert_eq!(result.len(), expected_len,
            "greeting length {} should be {}", result.len(), expected_len);
    }
}
