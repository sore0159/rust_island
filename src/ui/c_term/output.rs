use crossterm::style::{ContentStyle, StyledContent};

pub type Style = ContentStyle;

pub fn style_eq(s1: &Style, s2: &Style) -> bool {
    if s1.foreground_color != s2.foreground_color {
        return false;
    }
    if s1.background_color != s2.background_color {
        return false;
    }
    s1.attributes == s2.attributes
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub val: char,
    pub style: Style,
}

impl Cell {
    pub fn set_to(&mut self, other: &Self) {
        self.val = other.val;
        self.style = other.style.clone();
    }
    pub fn same_style(&self, other: &Self) -> bool {
        style_eq(&self.style, &other.style)
    }
    pub fn reset_style(&mut self) {
        self.style = Default::default();
    }
    pub fn clear(&mut self) {
        self.val = ' ';
    }
    pub fn to_default(&mut self) {
        self.clear();
        self.reset_style();
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            val: ' ',
            style: Default::default(),
        }
    }
}

pub struct Line {
    pub cells: Vec<Cell>,
}

impl Line {
    pub fn new() -> Self {
        Line { cells: vec![] }
    }
    pub fn generate_content(&self) -> Vec<StyledContent<String>> {
        let mut stylized = Vec::new();
        let mut s = String::new();
        let mut last_cell: Option<&Cell> = None;
        for cell in &self.cells {
            if let Some(last) = last_cell {
                if !style_eq(&last.style, &cell.style) {
                    stylized.push(StyledContent::new(last.style, s));
                    s = String::new();
                }
            } else {
                last_cell = Some(cell);
            }
            s.push(cell.val);
        }
        stylized
    }
}

use crossterm::{cursor::MoveTo, queue, style::PrintStyledContent};
use std::io::Write;
pub fn queue_write_at(
    mut w: impl Write,
    start: (u16, u16),
    content: Vec<StyledContent<String>>,
) -> anyhow::Result<()> {
    queue!(w, MoveTo(start.0, start.1))?;
    for s in content {
        queue!(w, PrintStyledContent(s))?;
    }
    Ok(())
}

pub struct Rect {
    pub origin: (u16, u16),
    pub size: (u16, u16),
    pub lines: Vec<Line>,
}

impl Rect {
    pub fn new(origin: (u16, u16), size: (u16, u16)) -> Self {
        let mut lines = Vec::with_capacity(size.1 as usize);
        for _ in 0..size.1 {
            lines.push(Line {
                cells: vec![Cell::default(); size.0 as usize],
            });
        }
        Self {
            origin,
            size,
            lines,
        }
    }
    // external 1,1 based coords but since origin is too it's fine
    pub fn external_get_mut(&mut self, ext_coord: (u16, u16)) -> Option<&mut Cell> {
        let x = match ext_coord.0.checked_sub(self.origin.0) {
            None => return None,
            Some(n) => n,
        };
        let y = match ext_coord.1.checked_sub(self.origin.1) {
            None => return None,
            Some(n) => n,
        };
        self.lines
            .get_mut(y as usize)
            .and_then(|row| row.cells.get_mut(x as usize))
    }

    // internal, but still 1,1 based coords
    pub fn get_mut(&mut self, coord: (u16, u16)) -> Option<&mut Cell> {
        self.lines
            .get_mut(coord.1 as usize - 1)
            .and_then(|row| row.cells.get_mut(coord.0 as usize - 1))
    }

    pub fn queue_write(&self, mut w: impl Write) -> anyhow::Result<()> {
        for (i, line) in self.lines.iter().enumerate() {
            let content = line.generate_content();
            queue_write_at(&mut w, (self.origin.0, self.origin.1 + (i as u16)), content)?;
        }
        Ok(())
    }
}
