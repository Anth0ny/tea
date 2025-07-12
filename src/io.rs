use std::fs::File;
use std::io;
use std::path::Path;

use anyhow::Result;
use memmap2::Mmap;
use ropey::Rope;

/// Line ending style in the source file.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineEnding {
    Crlf,
    Lf,
}

/// Detect the line ending style from a byte slice.
fn detect_line_ending(bytes: &[u8]) -> LineEnding {
    if bytes.windows(2).any(|w| w == b"\r\n") {
        LineEnding::Crlf
    } else {
        LineEnding::Lf
    }
}

/// Indicates if the source bytes end with a newline.
fn has_trailing_newline(bytes: &[u8]) -> bool {
    bytes.ends_with(b"\r\n") || bytes.ends_with(b"\n")
}

/// Load a file via mmap into a Rope, returning the rope, detected line ending,
/// and whether the file ended with a trailing newline.
pub fn load_file(path: &Path) -> Result<(Rope, LineEnding, bool)> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let bytes = &mmap[..];
    let line_ending = detect_line_ending(bytes);
    let trailing = has_trailing_newline(bytes);
    // Normalize line endings to '\n' for the rope.
    let text = if line_ending == LineEnding::Crlf {
        String::from_utf8_lossy(bytes).replace("\r\n", "\n")
    } else {
        String::from_utf8_lossy(bytes).to_string()
    };
    let rope = Rope::from_str(&text);
    Ok((rope, line_ending, trailing))
}

/// Save a Rope to a file, applying the given line ending style and trailing newline flag.
#[allow(dead_code)]
pub fn save_file(
    path: &Path,
    rope: &Rope,
    line_ending: LineEnding,
    trailing: bool,
) -> io::Result<()> {
    let sep = match line_ending {
        LineEnding::Crlf => "\r\n",
        LineEnding::Lf => "\n",
    };
    let mut out = String::new();
    for (i, line) in rope.lines().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        let line = line.as_str().unwrap().trim_end_matches('\n');
        out.push_str(line);
    }
    if trailing {
        out.push_str(sep);
    }
    std::fs::write(path, out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_and_save_crlf() -> Result<()> {
        let file = NamedTempFile::new()?;
        fs::write(file.path(), b"a\r\nb\r\n")?;
        let (rope, ending, trailing) = load_file(file.path())?;
        assert_eq!(ending, LineEnding::Crlf);
        assert!(trailing);
        assert_eq!(rope.len_lines(), 3);
        // Modify rope: append 'c'.
        let mut rope = rope;
        rope.insert(rope.len_chars(), "c");
        save_file(file.path(), &rope, ending, trailing).unwrap();
        let buf = fs::read(file.path()).unwrap();
        assert_eq!(buf, b"a\r\nb\r\nc\r\n");
        Ok(())
    }

    #[test]
    fn test_load_and_save_lf_no_trailing() -> Result<()> {
        let file = NamedTempFile::new()?;
        fs::write(file.path(), b"x\ny")?;
        let (rope, ending, trailing) = load_file(file.path())?;
        assert_eq!(ending, LineEnding::Lf);
        assert!(!trailing);
        assert_eq!(rope.len_lines(), 2);
        save_file(file.path(), &rope, ending, trailing).unwrap();
        let buf = fs::read(file.path()).unwrap();
        assert_eq!(buf, b"x\ny");
        Ok(())
    }
}
