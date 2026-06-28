//! This is the ASCII validator for our app.
//! We will be processing any text from the user here

/// Checks if a string contains only ASCII characters
fn is_ascii_only(text: &str) -> bool {
    text.is_ascii()
}

/// Validate ASCII and return a helpful error if invalid
pub fn validate_ascii(s: &str) -> Result<(), String> {
    if is_ascii_only(s) {
        Ok(())
    } else {
        // Then we want to find out which char
        // TODO: Add function to find out where it is
        Err("Contains non-ASCII characters".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii() {
        assert!(validate_ascii("Hello World").is_ok());
        assert!(validate_ascii("ABC123!@#").is_ok());
        assert!(validate_ascii("").is_ok());
    }

    #[test]
    fn test_invalid_unicode() {
        assert!(validate_ascii("café").is_err());
        assert!(validate_ascii("日本語").is_err());
        assert!(validate_ascii("emoji 😀").is_err());
    }

    #[test]
    fn test_error_message() {
        let err = validate_ascii("café").unwrap_err();
        assert!(err.contains("non-ASCII"));
    }
}
