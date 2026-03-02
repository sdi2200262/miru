use std::fmt;

/// A 2D character grid for drawing diagrams.
#[derive(Debug, Clone)]
pub struct Canvas {
    cells: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    /// Create a new canvas filled with spaces.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![' '; width]; height],
            width,
            height,
        }
    }

    /// Set a character at (x, y). Out-of-bounds writes are silently ignored.
    pub fn set(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.cells[y][x] = ch;
        }
    }

    /// Get the character at (x, y).
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.cells.get(y)?.get(x).copied()
    }

    /// Write a string starting at (x, y), going right.
    pub fn write_str(&mut self, x: usize, y: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.set(x + i, y, ch);
        }
    }

    /// Draw a horizontal line from (x, y) of the given length.
    pub fn hline(&mut self, x: usize, y: usize, len: usize, ch: char) {
        for i in 0..len {
            self.set(x + i, y, ch);
        }
    }

    /// Draw a vertical line from (x, y) of the given length.
    pub fn vline(&mut self, x: usize, y: usize, len: usize, ch: char) {
        for i in 0..len {
            self.set(x, y + i, ch);
        }
    }

    /// Draw a box with Unicode box-drawing characters.
    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize, ascii: bool) {
        if width < 2 || height < 2 {
            return;
        }

        let (tl, tr, bl, br, h, v) = if ascii {
            ('+', '+', '+', '+', '-', '|')
        } else {
            (
                '\u{250C}', '\u{2510}', '\u{2514}', '\u{2518}', '\u{2500}', '\u{2502}',
            )
            // ┌ ┐ └ ┘ ─ │
        };

        // Corners
        self.set(x, y, tl);
        self.set(x + width - 1, y, tr);
        self.set(x, y + height - 1, bl);
        self.set(x + width - 1, y + height - 1, br);

        // Horizontal edges
        self.hline(x + 1, y, width - 2, h);
        self.hline(x + 1, y + height - 1, width - 2, h);

        // Vertical edges
        self.vline(x, y + 1, height - 2, v);
        self.vline(x + width - 1, y + 1, height - 2, v);
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            let line: String = row.iter().collect();
            let trimmed = line.trim_end();
            write!(f, "{trimmed}")?;
            if i < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_unicode_box() {
        let mut c = Canvas::new(10, 5);
        c.draw_box(1, 1, 6, 3, false);
        let output = c.to_string();
        assert!(output.contains('\u{250C}')); // ┌
        assert!(output.contains('\u{2518}')); // ┘
    }

    #[test]
    fn write_str_on_canvas() {
        let mut c = Canvas::new(20, 1);
        c.write_str(2, 0, "hello");
        assert_eq!(c.to_string(), "  hello");
    }
}
