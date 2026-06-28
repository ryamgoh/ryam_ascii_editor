//! This deals with all File IO operations
//! This includes reading and writing
//! Also checks if the file exists

use std::{
    ffi::OsStr,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use crate::validator::validate_ascii;

const FILE_EXTENSION: &str = "ryam";

fn has_validated_file_extension(ext: &OsStr) -> bool {
    match ext.to_str() {
        Some(FILE_EXTENSION) => true,
        _ => false,
    }
}

fn get_validated_path_buffer(file_name: &str, require_exists: bool) -> Option<PathBuf> {
    let path = Path::new(file_name);

    // 1. Check if exists
    if require_exists && !path.is_file() {
        return None;
    }

    // 2. Check if the extension is supported
    let ext = path.extension()?;
    if has_validated_file_extension(ext) {
        // 3. Return the path buffer
        Some(path.to_path_buf())
    } else {
        None
    }
}

/// Read lines from a file
pub fn read_file(filename: &str) -> Result<Vec<String>, String> {
    // 1. Check if buffer exists and unwrap the Option safely
    // The "?" operator is used to exit early if any errors
    let buf = get_validated_path_buffer(filename, true)
        .ok_or_else(|| format!("Invalid file path or extension for: {}", filename))?;

    // 2. Read content and convert io::Error to String
    // Likewise, it exits early with an error
    let content = read_to_string(buf).map_err(|e| format!("Failed to read file: {}", e))?;

    // 3. Split into lines and collect into a Vector
    let lines: Vec<String> = content.lines().map(String::from).collect();

    // 4. Validate all lines are ASCII
    for line in &lines {
        match validate_ascii(line) {
            Ok(_) => (),             // Do nothing, line is valid
            Err(e) => return Err(e), // Early return the error string if invalid
        }
    }

    Ok(lines)
}

/// Write lines to a file
pub fn write_file(filename: &str, lines: &[String]) -> Result<(), String> {
    // 1. Check if extension makes sense (should not check if exists right?? We're just writing
    //    whether exists or not)
    let buf = get_validated_path_buffer(filename, false)
        .ok_or_else(|| format!("Invalid extension for: {}", filename))?;

    // 2. Join lines with newlines
    let content = lines.join("\n");

    // 3. Validate content
    match validate_ascii(&content) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    write(buf, content).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}
