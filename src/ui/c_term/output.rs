use crossterm::style::{ContentStyle, StyledContent};

pub type Style = ContentStyle;

pub use crossterm::style::Color;

pub fn new_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r, g, b }
}

pub fn style_eq(s1: &Style, s2: &Style) -> bool {
    if s1.foreground_color != s2.foreground_color {
        return false;
    }
    if s1.background_color != s2.background_color {
        return false;
    }
    s1.attributes == s2.attributes
}
pub fn add_style(source: &Style, target: &mut Style) {
    if source.foreground_color.is_some() {
        target.foreground_color = source.foreground_color;
    }
    if source.background_color.is_some() {
        target.background_color = source.background_color;
    }
    target.attributes.extend(source.attributes);
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub val: char,
    style_mods: Style,
    pub base_fg: Color,
    pub base_bg: Color,
}

impl Cell {
    pub fn set_to(&mut self, other: &Self) {
        self.val = other.val;
        self.style_mods = other.style_mods.clone();
    }
    pub fn to_base_style(&mut self) {
        self.style_mods = Default::default();
    }
    pub fn clear_char(&mut self) {
        self.val = ' ';
    }
    pub fn to_default(&mut self) {
        self.clear_char();
        self.to_base_style();
    }
    pub fn set_style_mods(&mut self, s: &Style) {
        self.style_mods = s.clone();
    }
    pub fn add_style_mods(&mut self, s: &Style) {
        add_style(s, &mut self.style_mods);
    }
    pub fn set_base(&mut self, fg: Color, bg: Color) {
        self.base_fg = fg;
        self.base_bg = bg;
    }
    pub fn get_output_style(&self) -> Style {
        let mut s = self.style_mods.clone();
        if s.foreground_color.is_none() {
            s.foreground_color = Some(self.base_fg);
        }
        if s.background_color.is_none() {
            s.background_color = Some(self.base_bg);
        }
        s
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            val: ' ',
            style_mods: Default::default(),
            base_fg: Color::White,
            base_bg: Color::Black,
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
        let mut last_style: Option<Style> = None;
        for cell in &self.cells {
            let o_style = cell.get_output_style();
            if let Some(last) = last_style {
                if !style_eq(&last, &o_style) {
                    stylized.push(StyledContent::new(last, s));
                    s = String::new();
                }
            }
            last_style = Some(o_style);
            s.push(cell.val);
        }
        if let Some(last) = last_style {
            stylized.push(StyledContent::new(last, s));
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
    pub fn imprint_at(&mut self, at: (u16, u16), c: char, s: &Style) {
        let cell = &mut self.lines[at.1 as usize].cells[at.0 as usize];
        cell.val = c;
        cell.set_style_mods(s);
    }
    pub fn add_at(&mut self, at: (u16, u16), c: char, s: &Style) {
        let cell = &mut self.lines[at.1 as usize].cells[at.0 as usize];
        cell.val = c;
        cell.add_style_mods(s);
    }
    pub fn str_imprint_at(&mut self, start: (u16, u16), st: &str, sty: &Style) {
        for (i, c) in st.chars().enumerate() {
            self.imprint_at((start.0 + i as u16, start.1), c, sty);
        }
    }
    pub fn str_add_at(&mut self, start: (u16, u16), st: &str, sty: &Style) {
        for (i, c) in st.chars().enumerate() {
            self.add_at((start.0 + i as u16, start.1), c, sty);
        }
    }
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

    pub fn get_mut(&mut self, coord: (u16, u16)) -> Option<&mut Cell> {
        self.lines
            .get_mut(coord.1 as usize)
            .and_then(|row| row.cells.get_mut(coord.0 as usize))
    }

    pub fn queue_write(&self, mut w: impl Write) -> anyhow::Result<()> {
        for (i, line) in self.lines.iter().enumerate() {
            let content = line.generate_content();
            queue_write_at(&mut w, (self.origin.0, self.origin.1 + (i as u16)), content)?;
        }
        Ok(())
    }
    pub fn all_cells(&mut self, f: impl Fn(&mut Cell)) {
        for line in self.lines.iter_mut() {
            for cell in &mut line.cells {
                f(cell);
            }
        }
    }
}
