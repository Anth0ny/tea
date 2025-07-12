# Tea Editor – Product Requirements Document (v0.1)

## 1. Purpose & Differentiator

Tea is a terminal‑native text editor that *feels* like a GUI editor. It delivers mouse‑driven menus, scroll‑wheel navigation, and familiar VS Code key‑bindings while remaining a single self‑contained binary that runs in any POSIX shell—local or over SSH.

## 2. Target Personas & Use‑Cases

| Persona               | Scenario                                                                  |
| --------------------- | ------------------------------------------------------------------------- |
| **Remote Developer**  | Editing source files on a Linux server over SSH and missing GUI comforts. |
| **Ops / SRE**         | Scrolling and editing 100 MB log files directly in production terminals.  |
| **Locked‑Down Admin** | Needs a modern editor where Python/Node runtimes are disallowed.          |

## 3. MVP Functional Requirements

1. **Open & Display Local File**\
   • Memory‑mapped read into a *rope* data structure (via `ropey`).
2. **Viewport Navigation**\
   • Scroll wheel, arrow keys, PgUp/PgDn.\
   • Status bar shows *Ln x, Col y*.
3. **Cursor & Selection**\
   • Mouse click to move insertion point.\
   • Mouse drag for single‑range selection (inverse colours).
4. **Menu Bar (row 0)**\
   *File · Edit · View* menus with dropdowns; mouse‑click and accelerator keys.
   | Menu | Item        | Key | Note             |
   | ---- | ----------- | --- | ---------------- |
   | File | Open…       | ⌘O  | Path prompt      |
   |      | Quit        | ⌘Q  | Confirms unsaved |
   | Edit | Select All  | ⌘A  |                  |
   |      | Cut         | ⌘X  | OSC 52 clipboard |
   |      | Copy        | ⌘C  | OSC 52 clipboard |
   |      | Paste       | ⌘V  | OSC 52 clipboard |
   | View | Word Wrap ✓ | ⌘⇧W | Soft‑wrap toggle |
5. **Line Ending Handling**\
   • Detect CRLF/LF on open; preserve on save; indicator in status bar.
6. **Config File**\
   • JSON at `~/.tea/config.json` (Linux/Mac/WSL).\
   • Auto‑generated on first run; live‑reloaded.
7. **Colour & Unicode**\
   • 24‑bit TrueColor + full UTF‑8.\
   • Fallback to 256‑colour when `COLORTERM` lacks `truecolor`.

## 4. Non‑Functional Requirements

| Requirement               | Target                                                    |
| ------------------------- | --------------------------------------------------------- |
| **Binary size**           | ≤ 6 MB stripped                                           |
| **Launch to first paint** | ≤ 100 ms for <1 MB file                                   |
| **Keystroke latency**     | < 5 ms on 100 MB file (2‑core VM)                         |
| **Cross‑platform**        | macOS, Linux, WSL; xterm‑compatible terms (xterm, iTerm2) |

## 5. Technical Stack

| Layer           | Choice                                                    |
| --------------- | --------------------------------------------------------- |
| Language        | **Rust**                                                  |
| Terminal I/O    | `crossterm`                                               |
| UI Widgets      | `ratatui`                                                 |
| Text Buffer     | `ropey` (rope)                                            |
| Clipboard (MVP) | OSC 52 escapes                                            |
| Build           | `cargo`, static linking (`-C target-feature=+crt-static`) |

### Architecture Diagram

```
Terminal ⇄ Crossterm ⇄ Event Loop
                    ↓
              Input Mapper (keys/mouse)
                    ↓
      Rope Buffer ← Edit Engine → Undo/Redo
                    ↓
          Ratatui Renderer (menu, status, text)
```

## 6. Packaging & Distribution

- **Binary name:** `tea`
- **Homebrew:** `brew install tea-editor` formula.
- **Linux:** `.deb`, `.rpm`, AUR, and standalone `tea-x86_64-unknown-linux-gnu`.
- **Config directory:** `~/.tea/` on all platforms.
- **Logo/Icon:** Generic SVG of Times Roman lowercase **t** (placeholder in repo `/assets/logo.svg`).
- **License:** MIT.

## 7. Road‑Map (post‑MVP)

- Context menu, multiple cursors, block selection.
- Syntax highlighting & LSP support.
- Tabs / split panes.
- Git & debugger integration.
- Native clipboards (pbcopy / xclip / wl‑copy).
- Plugin system (JS or Python).
- Theme engine and user‑defined colour palettes.

## 8. Success Metrics

- Installs via Homebrew without additional dependencies.
- Opens GNU GPL v3 (180 KB) in <0.1 s.
- Passes integration test suite of all MVP key combinations.

## 9. Risks & Mitigations

| Risk                             | Mitigation                                          |
| -------------------------------- | --------------------------------------------------- |
| Terminal variance (tmux, screen) | `crossterm` feature probes; OSC52 fallback warnings |
| OSC 52 disabled over SSH         | Status‑bar warning; allow manual copy prompt        |
| Large file RAM usage             | mmap + rope chunking keeps memory bounded           |

## 10. Milestone Plan

| Phase             | Deliverable                             | ETA   |
| ----------------- | --------------------------------------- | ----- |
| 0. Bootstrap      | Cargo workspace, CI, Hello World binary | +2 d  |
| 1. Core I/O       | mmap loader, rope adapter, saving       | +1 wk |
| 2. Terminal shell | Event loop & incremental renderer       | +2 wk |
| 3. Mouse & Keys   | Selection + VS Code bindings            | +3 wk |
| 4. Menu Bar       | Interactive pulldown menus              | +4 wk |
| 5. Clipboard      | OSC 52 copy/paste                       | +5 wk |
| 6. Packaging      | Homebrew & Linux bundles                | +6 wk |
| **Beta**          | v0.1.0 tagged release                   | +7 wk |

---

*Last updated: 2025‑07‑11*

