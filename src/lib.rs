//! # rust-template
//!
//! A Rust project template with comprehensive CI/CD.

pub mod cli;
pub mod errors;

/// Returns a greeting message.
///
/// # Examples
///
/// ```
/// use rust_template::greet;
/// assert_eq!(greet("World"), "Hello, World!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
