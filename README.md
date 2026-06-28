# 📝 ryam_ascii_editor

This is a editor that only works with ascii characters... Why ascii? Because its much lighter!
Also, I wanted to build my own file extension `.ryam` for fun with validation that can only be used on this editor.

A simple, line-based ASCII text editor written in Rust. Perfect for beginners learning Rust or anyone who needs a lightweight, pure-ASCII text editor.

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## ✨ Features

- **Pure ASCII Only** - Automatically validates and rejects non-ASCII characters (ideal for config files, code, plain text)
- **Line-Based Editing** - Simple, intuitive line operations with numbered display
- **File Management** - Create, open, save, and save-as with `.ryam` extension
- **Command System** - Vim-like commands with shortcuts (`:h`, `:l`, `:i`, `:d`, `:s`, `:sa`, `:q`)
- **Live Preview** - Shows first 40 lines automatically in main view
- **Modified Indicator** - Visual warning when unsaved changes exist
- **Unsaved Changes Protection** - Prompts before quitting with unsaved work
- **Zero Dependencies** - Uses only Rust's standard library
- **Cross-Platform** - Works on Windows, macOS, and Linux

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([install Rust](https://rustup.rs/))

### Installation

```bash
# Clone the repository
git clone https://github.com/ryamgoh/ryam_ascii_editor
cd ryam_ascii_editor

# Build the project
cargo build --release

# The binary will be at target/release/ryam_ascii_editor
```

### Alternative: Install via Cargo

```bash
cargo install --git https://github.com/ryamgoh/ryam_ascii_editor
```

## 📖 Usage

### Basic Commands

```bash
# Open/create a file
cargo run -- myfile.ryam

# Or run the binary directly
./target/release/ryam_ascii_editor myfile.ryam

# Create a new file with default name
cargo run
```

### Command Reference

| Command | Shortcut | Description |
|---------|----------|-------------|
| `:help` | `:h` | Show help menu |
| `:list` | `:l` | Display all lines with numbers |
| `:insert TEXT` | `:i TEXT` | Add a new line with TEXT |
| `:delete N` | `:d N` | Delete line number N (1-indexed) |
| `:save` | `:s` | Save to current file |
| `:saveas FILENAME` | `:sa FILENAME` | Save to a different file |
| `:quit` | `:q` | Exit the editor (prompts if unsaved) |

**Just type any text** and press Enter to append it as a new line. All text is automatically validated to ensure it contains only ASCII characters.

### Example Session

```
╔══════════════════════════════════════════════════════════════╗
║  📝 ASCII Editor v0.1.0                                      ║
║  File: notes.ryam                                            ║
╚══════════════════════════════════════════════════════════════╝

📄 Preview (first 40 lines):
  1: Hello World
  2: This is my ASCII file
  3: Only ASCII characters allowed: ABC123!@#

> :insert New line added
✅ Added line: New line added

> :list

📄 File Contents:
  1: Hello World
  2: This is my ASCII file
  3: Only ASCII characters allowed: ABC123!@#
  4: New line added

> :save
✅ Saved to notes.ryam
```

## 📁 File Format

Files are saved as plain text with `.ryam` extension. Each line is separated by a newline character.

```text
Hello World
This is my ASCII file
Only ASCII characters allowed: ABC123!@#
```

## 🏗️ Architecture

The project follows a clean layered architecture:

```
┌─────────────────────────────────────────┐
│   UI LAYER (ui.rs)                      │
│   - Display formatting & rendering      │
│   - User prompts & confirmations        │
│   - Menu & help display                 │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   APPLICATION LAYER (main.rs)           │
│   - Main event loop                     │
│   - Command routing & dispatch          │
│   - Program flow control                │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   DOMAIN LAYER (editor.rs)              │
│   - Editor state management             │
│   - Business logic (CRUD operations)    │
│   - Line operations                     │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   SERVICE LAYER (validator.rs)          │
│   - ASCII validation utilities          │
│   - Input sanitization                  │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   INFRASTRUCTURE LAYER (file_io.rs)     │
│   - File system operations              │
│   - Read/Write with validation          │
│   - Extension handling (.ryam)          │
└─────────────────────────────────────────┘
```

### Module Overview

| Module | Responsibility 
|--------|---------------|
| `main.rs` | Program entry point, main loop, command dispatch |
| `editor.rs` | Core editor state & logic (lines, filename, modified flag) |
| `commands.rs` | Command parsing & interpretation |
| `validator.rs` | ASCII character validation |
| `file_io.rs` | File read/write with extension validation |
| `ui.rs` | All display functions (header, preview, help, prompts) |

## 🛠️ Development

### Project Structure

```
ryam_ascii_editor/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
└── src/
    ├── main.rs          # Entry point & main loop
    ├── editor.rs        # Core editor logic & state
    ├── commands.rs      # Command parsing
    ├── validator.rs     # ASCII validation
    ├── file_io.rs       # File operations
    └── ui.rs            # Display & UI functions
```

### Common Commands

```bash
# Format code
cargo fmt

# Check for errors
cargo check

# Run tests
cargo test

# Build release binary
cargo build --release

# Run with a file
cargo run -- myfile.ryam
```

### Adding Features

The modular architecture makes it easy to extend:

1. **New command**: Add to `commands.rs` enum, handle in `main.rs`
2. **Change validation**: Modify `validator.rs`
3. **New file formats**: Extend `file_io.rs`
4. **UI improvements**: Update `ui.rs`

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with tests
4. Run `cargo fmt` and `cargo check`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Ideas for Contributions

- [ ] Edit/modify existing lines
- [ ] Search/find functionality
- [ ] Undo/Redo support
- [ ] Copy/Paste support
- [ ] Line number toggle
- [ ] Syntax highlighting for ASCII art
- [ ] Terminal UI with `crossterm` or `ratatui`
- [ ] Configuration file support
- [ ] Multiple file tabs/buffers

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with Rust's standard library only
- Inspired by classic Unix text editors (ed, vi)
- Designed as a learning project for Rust beginners

## 📚 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Learn Rust from scratch
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through examples
- [Rust Reference](https://doc.rust-lang.org/reference/) - Complete Rust reference

---

**Made with ❤️ and Rust**
