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
                if e.contains("Cannot open") || e.contains("Invalid file path") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_ryam_path(name: &str) -> String {
        let dir = std::env::temp_dir();
        dir.join(format!("test_{}.ryam", name)).to_str().unwrap().to_string()
    }

    #[test]
    fn test_new_editor() {
        let editor = Editor::new("test.ryam");
        assert!(editor.lines.is_empty());
        assert_eq!(editor.filename, "test.ryam");
        assert!(!editor.modified);
    }

    #[test]
    fn test_add_line() {
        let mut editor = Editor::new("test.ryam");
        assert!(editor.add_line("hello").is_ok());
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "hello");
        assert!(editor.modified);
    }

    #[test]
    fn test_add_line_rejects_non_ascii() {
        let mut editor = Editor::new("test.ryam");
        assert!(editor.add_line("café").is_err());
        assert!(editor.lines.is_empty());
    }

    #[test]
    fn test_delete_line() {
        let mut editor = Editor::new("test.ryam");
        editor.add_line("line1").unwrap();
        editor.add_line("line2").unwrap();
        editor.add_line("line3").unwrap();
        assert!(editor.delete_line(2).is_ok());
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.lines[1], "line3");
    }

    #[test]
    fn test_delete_line_invalid_number() {
        let mut editor = Editor::new("test.ryam");
        editor.add_line("line1").unwrap();
        assert!(editor.delete_line(5).is_err());
        assert_eq!(editor.lines.len(), 1);
    }

    #[test]
    fn test_save_and_load() {
        let path = temp_ryam_path("save_load");
        let _ = fs::remove_file(&path);

        {
            let mut editor = Editor::new(&path);
            editor.add_line("line1").unwrap();
            editor.add_line("line2").unwrap();
            assert!(editor.save().is_ok());
            assert!(!editor.modified);
        }

        {
            let mut editor = Editor::new(&path);
            assert!(editor.load().is_ok());
            assert_eq!(editor.lines.len(), 2);
            assert_eq!(editor.lines[0], "line1");
            assert_eq!(editor.lines[1], "line2");
        }

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_save_as() {
        let path1 = temp_ryam_path("save_as_1");
        let path2 = temp_ryam_path("save_as_2");
        let _ = fs::remove_file(&path1);
        let _ = fs::remove_file(&path2);

        let mut editor = Editor::new(&path1);
        editor.add_line("data").unwrap();
        assert!(editor.save_as(&path2).is_ok());
        assert_eq!(editor.filename, path2);

        let mut reloaded = Editor::new(&path2);
        assert!(reloaded.load().is_ok());
        assert_eq!(reloaded.lines, vec!["data"]);

        let _ = fs::remove_file(&path1);
        let _ = fs::remove_file(&path2);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let path = temp_ryam_path("nonexistent");
        let _ = fs::remove_file(&path);

        let mut editor = Editor::new(&path);
        assert!(editor.load().is_ok());
        assert!(editor.lines.is_empty());
    }
}
