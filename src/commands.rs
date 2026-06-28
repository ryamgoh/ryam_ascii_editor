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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(Command::parse_input(""), Command::Invalid("Empty input".to_string()));
        assert_eq!(Command::parse_input("   "), Command::Invalid("Empty input".to_string()));
    }

    #[test]
    fn test_help_command() {
        assert_eq!(Command::parse_input(":help"), Command::Help);
        assert_eq!(Command::parse_input(":h"), Command::Help);
        assert_eq!(Command::parse_input("  :help  "), Command::Help);
    }

    #[test]
    fn test_list_command() {
        assert_eq!(Command::parse_input(":list"), Command::List);
        assert_eq!(Command::parse_input(":l"), Command::List);
    }

    #[test]
    fn test_quit_command() {
        assert_eq!(Command::parse_input(":quit"), Command::Quit);
        assert_eq!(Command::parse_input(":q"), Command::Quit);
    }

    #[test]
    fn test_save_command() {
        assert_eq!(Command::parse_input(":save"), Command::Save);
        assert_eq!(Command::parse_input(":s"), Command::Save);
    }

    #[test]
    fn test_insert_command() {
        assert_eq!(Command::parse_input(":insert Hello"), Command::Insert("Hello".to_string()));
        assert_eq!(Command::parse_input(":i Hello World"), Command::Insert("Hello World".to_string()));
        assert_eq!(Command::parse_input(":insert"), Command::Invalid("Missing text for insert".to_string()));
    }

    #[test]
    fn test_delete_command() {
        assert_eq!(Command::parse_input(":delete 5"), Command::Delete(5));
        assert_eq!(Command::parse_input(":d 1"), Command::Delete(1));
        assert_eq!(Command::parse_input(":delete"), Command::Invalid("Missing line number for delete".to_string()));
        assert_eq!(Command::parse_input(":delete abc"), Command::Invalid("Invalid line number: 'abc'".to_string()));
    }

    #[test]
    fn test_saveas_command() {
        assert_eq!(Command::parse_input(":saveas new.ryam"), Command::SaveAs("new.ryam".to_string()));
        assert_eq!(Command::parse_input(":sa other.ryam"), Command::SaveAs("other.ryam".to_string()));
        assert_eq!(Command::parse_input(":saveas"), Command::Invalid("Missing filename for saveas".to_string()));
    }

    #[test]
    fn test_unknown_command() {
        assert_eq!(Command::parse_input(":unknown"), Command::Invalid("Unknown command: 'unknown'".to_string()));
    }

    #[test]
    fn test_text_line() {
        assert_eq!(Command::parse_input("hello world"), Command::TextLine("hello world".to_string()));
        assert_eq!(Command::parse_input("some random text"), Command::TextLine("some random text".to_string()));
    }

    #[test]
    fn test_empty_command() {
        assert_eq!(Command::parse_input(":"), Command::Invalid("Empty command".to_string()));
        assert_eq!(Command::parse_input(":   "), Command::Invalid("Empty command".to_string()));
    }
}
