//! This is the main orchestrator of the application
//! State Management is handled here

use crate::{
    file_io::{read_file, write_file},
    validator::validate_ascii,
};

pub struct Editor {
    pub lines: Vec<String>, // the content, line by line
    pub filename: String,   // current filename
    pub modified: bool,     // unsaved changes
}

/// Main editor logic lies here
/// CRUD
impl Editor {
    /// Create a new instance
    pub fn new(filename: &str) -> Self {
        Self {
            lines: Vec::new(),
            filename: filename.to_string(),
            modified: false,
        }
    }

    /// Load a file from disk
    pub fn load(&mut self) -> Result<(), String> {
        match read_file(&self.filename) {
            Ok(lines) => {
                self.lines = lines;
                self.modified = false;
                Ok(())
            }
            Err(e) => {
                // File doesn't exist - that's okay, we'll create a new one
                if e.contains("Cannot open") {
                    self.lines = Vec::new();
                    self.modified = false;
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Add a new line (append to end)
    pub fn add_line(&mut self, line: &str) -> Result<(), String> {
        // Validate ASCII
        validate_ascii(line)?;

        // Add the line
        self.lines.push(line.to_string());
        self.modified = true;

        Ok(())
    }

    /// Delete a line by line number (1-indexed for user friendliness)
    pub fn delete_line(&mut self, line_num: usize) -> Result<(), String> {
        // Convert to 0-indexed
        let index = line_num - 1;

        if index >= self.lines.len() {
            return Err(format!(
                "Line {} doesn't exist (have {} lines)",
                line_num,
                self.lines.len()
            ));
        }

        self.lines.remove(index);
        self.modified = true;

        Ok(())
    }

    /// Save to the current file
    pub fn save(&mut self) -> Result<(), String> {
        write_file(&self.filename, &self.lines)?;
        self.modified = false;
        Ok(())
    }

    /// Save to a different file
    pub fn save_as(&mut self, filename: &str) -> Result<(), String> {
        write_file(filename, &self.lines)?;
        self.filename = filename.to_string();
        self.modified = false;
        Ok(())
    }
}
