//! UI display functions (Presentation Layer)
use crate::editor::Editor;
use std::io::Write;

/// Display the main header
pub fn show_header(editor: &Editor) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  📝 ASCII Editor v0.1.0                                      ║");
    println!(
        "║  File: {}                                                    ║",
        editor.filename
    );
    if editor.modified {
        println!("║  ⚠️  MODIFIED - unsaved changes                              ║");
    }
    println!("╚══════════════════════════════════════════════════════════════╝");
}

/// Show the help menu
pub fn show_help() {
    println!("\n📖 Available Commands:");
    println!("  :help, :h    - Show this help");
    println!("  :list, :l    - Show all lines with numbers");
    println!("  :insert, :i  - Add a new line (e.g., :insert Hello)");
    println!("  :delete, :d N - Delete line number N");
    println!("  :save, :s    - Save to current file");
    println!("  :saveas, :sa - Save to different file");
    println!("  :quit, :q    - Exit (prompts if unsaved changes)");
    println!("\n  Just type any text to append it as a new line.");
    println!("  Only ASCII characters are allowed!\n");
}

/// Show lines with numbers
pub fn show_lines(editor: &Editor) {
    if editor.lines.is_empty() {
        println!("📄 (empty file)");
        return;
    }

    println!("\n📄 File Contents:");
    for (i, line) in editor.lines.iter().enumerate() {
        println!("{:>3}: {}", i + 1, line);
    }
    println!();
}

/// Show a preview of the file (first N lines)
pub fn show_preview(editor: &Editor, max_lines: usize) {
    if editor.lines.is_empty() {
        println!("📄 (empty file)");
        return;
    }

    println!("📄 Preview (first {} lines):", max_lines);
    for (i, line) in editor.lines.iter().enumerate().take(max_lines) {
        println!("{:>3}: {}", i + 1, line);
    }
    if editor.lines.len() > max_lines {
        println!("  ... and {} more lines", editor.lines.len() - max_lines);
    }
    println!();
}

/// Show a success message
pub fn show_success(message: &str) {
    println!("✅ {}", message);
}

/// Show an error message
pub fn show_error(message: &str) {
    println!("❌ Error: {}", message);
}

/// Show an info message
pub fn show_info(message: &str) {
    println!("ℹ️  {}", message);
}

/// Show the prompt
pub fn show_prompt() {
    print!("> ");
    std::io::stdout().flush().unwrap();
}

/// Confirm action with user
pub fn confirm_action(prompt: &str) -> bool {
    print!("{} (y/n): ", prompt);
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();

    response.trim().to_lowercase() == "y"
}

/// Clear the screen (simple ANSI escape)
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
