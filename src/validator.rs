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
