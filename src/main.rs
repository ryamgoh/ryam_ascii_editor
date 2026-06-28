// This is the main entry point of our application

mod commands;
mod editor;
mod file_io;
mod ui;
mod validator;
use std::{env, io};

use crate::{
    commands::Command,
    editor::Editor,
    file_io::FILE_EXTENSION,
    ui::{
        clear_screen, confirm_action, show_error, show_header, show_help, show_info, show_lines,
        show_preview, show_prompt, show_success,
    },
};

fn main() {
    println!("🚀 ASCII Editor v0.1.0");
    println!("Loading...");

    start_app();
}

fn start_app() {
    println!("Starting...");
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        args[1].clone()
    } else {
        format!("untitled.{}", FILE_EXTENSION)
    };

    // Create and load editor
    let mut editor = Editor::new(&filename);

    // Try to load file
    match editor.load() {
        Ok(()) => {
            if !editor.lines.is_empty() {
                show_info(&format!(
                    "Loaded {} lines from {}",
                    editor.lines.len(),
                    editor.filename
                ));
            } else {
                show_info("new file created!");
            }
        }
        Err(e) => {
            show_error(&e);
            show_info("Starting with empty file");
        }
    }

    // Wait for user to see message
    wait_for_enter();
    clear_screen();

    // Main loop
    'main: loop {
        // Display UI
        show_header(&editor);
        println!();

        // Show preview
        show_preview(&editor, 40);

        // Show prompt
        show_prompt();

        // Read user input
        let mut input = String::new();
        let bytes_read = match io::stdin().read_line(&mut input) {
            Ok(n) => n,
            Err(_) => {
                show_error("Failed to read input");
                continue;
            }
        };
        if bytes_read == 0 {
            break 'main;
        }
        let input = input.trim().to_string();

        // Parse command
        let command = Command::parse_input(&input);

        // Process command
        match command {
            Command::Help => {
                clear_screen();
                show_help();
                wait_for_enter();
                clear_screen();
            }

            Command::List => {
                clear_screen();
                show_lines(&editor);
                wait_for_enter();
                clear_screen();
            }

            Command::Insert(text) => match editor.add_line(&text) {
                Ok(()) => {
                    show_success(&format!("Added line: {}", text));
                    wait_for_enter();
                    clear_screen();
                }
                Err(e) => {
                    show_error(&e);
                    wait_for_enter();
                }
            },

            Command::Delete(num) => match editor.delete_line(num) {
                Ok(()) => {
                    show_success(&format!("Deleted line {}", num));
                    wait_for_enter();
                    clear_screen();
                }
                Err(e) => {
                    show_error(&e);
                    wait_for_enter();
                }
            },

            Command::Save => match editor.save() {
                Ok(()) => {
                    show_success(&format!("Saved to {}", editor.filename));
                    wait_for_enter();
                    clear_screen();
                }
                Err(e) => {
                    show_error(&e);
                    wait_for_enter();
                }
            },

            Command::SaveAs(new_filename) => match editor.save_as(&new_filename) {
                Ok(()) => {
                    show_success(&format!("Saved to {}", new_filename));
                    wait_for_enter();
                    clear_screen();
                }
                Err(e) => {
                    show_error(&e);
                    wait_for_enter();
                }
            },

            Command::Quit => {
                if editor.modified {
                    show_info("You have unsaved changes!");
                    if confirm_action("Are you sure you want to quit?") {
                        break 'main;
                    } else {
                        clear_screen();
                        continue;
                    }
                } else {
                    break 'main;
                }
            }

            Command::TextLine(text) => match editor.add_line(&text) {
                Ok(()) => {
                    clear_screen();
                }
                Err(e) => {
                    show_error(&e);
                    wait_for_enter();
                    clear_screen();
                }
            },

            Command::Invalid(msg) => {
                show_error(&msg);
                show_info("Type :help for available commands");
                wait_for_enter();
                clear_screen();
            }
        }
    }

    println!("👋 Goodbye!");
}

/// Helper to wait for user to enter input
fn wait_for_enter() {
    println!("\nPress Enter to continue...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).unwrap();
}
