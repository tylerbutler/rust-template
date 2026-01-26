//! Integration tests for rust-template

use rust_template::greet;

#[test]
fn test_greeting_integration() {
    let result = greet("Integration Test");
    assert!(result.contains("Integration Test"));
    assert!(result.starts_with("Hello"));
}
