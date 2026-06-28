//! Command parsing - Application Layer

/// All possible commands the user can issue
#[derive(Debug, PartialEq)]
pub enum Command {
    Help,             // Show help
    List,             // Show all lines
    Insert(String),   // Insert a new line
    Delete(usize),    // Delete a line by number
    Save,             // Save to file
    SaveAs(String),   // Save to different file
    Quit,             // Exit the editor
    TextLine(String), // Regular text (append as line)
    Invalid(String),  // Invalid command with error message
}

impl Command {
    /// Parse input
    pub fn parse_input(input: &str) -> Self {
        // 1. Clean
        let input = input.trim();

        // 2. Deal with edge cases: Empty
        if input.is_empty() {
            return Command::Invalid("Empty input".to_string());
        }

        // 3. Check if its a command (starts with ":") safely supporting Unicode
        if let Some(rest) = input.strip_prefix(':') {
            let trimmed_cmd = rest.trim();
            if trimmed_cmd.is_empty() {
                return Command::Invalid("Empty command".to_string());
            }

            // Split into 2 parts max at the first whitespace character
            let mut parts = trimmed_cmd.splitn(2, char::is_whitespace);
            let command = parts.next().unwrap_or("");
            let args = parts.next().map(|s| s.trim()).unwrap_or("");

            match command {
                "help" | "h" => Command::Help,
                "list" | "l" => Command::List,
                "quit" | "q" => Command::Quit,
                "save" | "s" => Command::Save,

                "insert" | "i" => {
                    if args.is_empty() {
                        Command::Invalid("Missing text for insert".to_string())
                    } else {
                        // Preserves exact spacing typed by the user
                        Command::Insert(args.to_string())
                    }
                }

                "delete" | "d" => {
                    if args.is_empty() {
                        Command::Invalid("Missing line number for delete".to_string())
                    } else {
                        // Isolate the first token in args if user typed extra args
                        let num_str = args.split_whitespace().next().unwrap_or("");
                        match num_str.parse::<usize>() {
                            Ok(num) => Command::Delete(num),
                            Err(_) => {
                                Command::Invalid(format!("Invalid line number: '{}'", num_str))
                            }
                        }
                    }
                }

                "saveas" | "sa" => {
                    if args.is_empty() {
                        Command::Invalid("Missing filename for saveas".to_string())
                    } else {
                        Command::SaveAs(args.to_string())
                    }
                }

                cmd => Command::Invalid(format!("Unknown command: '{}'", cmd)),
            }
        } else {
            // Regular text
            Command::TextLine(input.to_string())
        }
    }
}
