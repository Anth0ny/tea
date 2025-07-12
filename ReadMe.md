

# tea

**tea** is a terminal-based text file viewer built with [Ratatui](https://docs.rs/ratatui/) and [Crossterm](https://docs.rs/crossterm/), written in Rust.

It loads a text file into memory, displays it in a scrollable viewport, and is the foundation for a performant, cross-platform terminal text editor.

---

## Features

- 📜 View large plain-text files in a styled TUI
- 🧵 Uses `Ropey` for efficient text representation
- 🧼 Automatic handling of line endings (LF, CRLF)
- 🚪 Exit with `q`
- 🧱 Clean separation of UI rendering and file I/O logic

---

## Installation

```bash
# Clone and build
$ git clone git@github.com:anth0ny/tea.git
$ cd tea
$ cargo build --release
```

---

## Usage

```bash
$ cargo run -- path/to/your_file.txt
```

- Use in any ANSI-compatible terminal
- Press `q` to exit

---

## Roadmap

- [x] Initial TUI viewport with Ratatui
- [ ] Scroll with arrow keys / PgUp / PgDn
- [ ] Syntax highlighting
- [ ] File editing
- [ ] Search (forward/backward)
- [ ] Configurable themes
- [ ] Plugin system

---

## Development

```bash
# Run formatter and linter
$ make lint

# Run tests
$ make test
```

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Author

**Anthony Lemus**  
<https://github.com/anth0ny>

---

Built with ❤️ and caffeine (and AI).