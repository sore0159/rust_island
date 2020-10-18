use std::fmt::Display;
use termion::cursor;

use super::style::{Style, StyleMod};

#[derive(Clone)]
pub struct Cell {
    pub val: char,
    pub style: Style,
}

impl Cell {
    pub fn set_to(&mut self, other: &Self) {
        self.val = other.val;
        self.style = other.style.clone();
    }
    pub fn mod_style(&mut self, m: &StyleMod) {
        m.apply(&mut self.style);
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

pub struct Rect {
    pub origin: (u16, u16),
    pub size: (u16, u16),
    pub cells: Vec<Vec<Cell>>,
}

impl Rect {
    pub fn new(origin: (u16, u16), w: u16, h: u16) -> Self {
        Self::new_with(origin, w, h, Cell::default())
    }
    pub fn new_with(origin: (u16, u16), w: u16, h: u16, default: Cell) -> Self {
        let mut rows = Vec::with_capacity(h as usize);
        for _ in 0..h {
            let mut row = Vec::with_capacity(w as usize);
            for _ in 0..w {
                row.push(default.clone());
            }
            rows.push(row);
        }
        Rect {
            origin: origin,
            size: (w, h),
            cells: rows,
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
        self.cells
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
    }

    // internal, but still 1,1 based coords
    pub fn get_mut(&mut self, coord: (u16, u16)) -> Option<&mut Cell> {
        self.cells
            .get_mut(coord.1 as usize - 1)
            .and_then(|row| row.get_mut(coord.0 as usize - 1))
    }

    // Assumes text can fit, sep utils for wrapping/sizing text
    pub fn set_cells(&mut self, start: (u16, u16), text: &str, m: Option<&StyleMod>) {
        for (i, ch) in text.chars().enumerate() {
            let mut cell = self.get_mut((start.0 + i as u16, start.1)).unwrap();
            if let Some(m) = m {
                m.apply(&mut cell.style);
            }
            cell.val = ch;
            //cell.val = format!("{}", i).chars().next().unwrap();
        }
    }
    pub fn mod_cells(&mut self, start: (u16, u16), len: u16, m: &StyleMod) {
        for i in 0..len {
            let cell = self.get_mut((start.0 + i as u16, start.1)).unwrap();
            m.apply(&mut cell.style);
        }
    }
    pub fn clear_style(&mut self, start: (u16, u16), len: u16) {
        let m = StyleMod::default();
        self.mod_cells(start, len, &m);
    }
    pub fn set_text(&mut self, start: (u16, u16), text: &str) {
        for (i, ch) in text.chars().enumerate() {
            let mut cell = self.get_mut((start.0 + i as u16, start.1)).unwrap();
            cell.val = ch;
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity((self.size.0 + 10) as usize);
        for (y, row) in self.cells.iter().enumerate() {
            s.clear();
            let mut last_style = super::style::Style::default();
            for (x, cell) in row.iter().enumerate() {
                if x == 0 {
                    last_style = cell.style.clone();
                    cell.style.deco.start(&mut s)?;
                    cell.style.fg.start_fg(&mut s)?;
                    cell.style.bg.start_bg(&mut s)?;
                } else {
                    if last_style.fg != cell.style.fg {
                        cell.style.fg.start_fg(&mut s)?;
                        last_style.fg = cell.style.fg.clone();
                    }
                    if last_style.bg != cell.style.bg {
                        cell.style.bg.start_bg(&mut s)?;
                        last_style.bg = cell.style.bg.clone();
                    }
                    if last_style.deco != cell.style.deco {
                        last_style.deco.stop(&mut s)?;
                        cell.style.deco.start(&mut s)?;
                        last_style.deco = cell.style.deco.clone();
                    }
                }
                s.push(cell.val);
            }
            last_style.deco.stop(&mut s)?;

            write!(
                f,
                "{}{}",
                cursor::Goto(self.origin.0, self.origin.1 + y as u16),
                s
            )?
        }
        Ok(())
    }
}

/*
rect::file_debug(&mut std::fs::File::create("test.txt").unwrap(), &rect2);
use std::fmt::Write;
pub fn file_debug(mut w: impl std::io::Write, rect: &Rect) {
    let mut s = String::with_capacity((rect.size.0 + 10) as usize);
    for row in &rect.cells {
        s.clear();
        let mut last_cell = Cell::default();
        for (x, cell) in row.iter().enumerate() {
            if x == 0 {
                last_cell = cell.clone();
                //cell.start_style(&mut s)?;
                //cell.start_fg(&mut s)?;
                //cell.start_bg(&mut s)?;
                write!(s, "{:?}{:?}", cell.fg_color, cell.bg_color).unwrap();
            } else {
                if last_cell.fg_color != cell.fg_color {
                    //cell.start_fg(&mut s)?;
                    write!(s, "{:?}", cell.fg_color).unwrap();
                    last_cell.fg_color = cell.fg_color.clone();
                }
                if last_cell.bg_color != cell.bg_color {
                    //cell.start_bg(&mut s)?;
                    write!(s, "{:?}", cell.bg_color).unwrap();
                    last_cell.bg_color = cell.bg_color.clone();
                }
                if last_cell.style != cell.style {
                    //last_cell.end_style(&mut s)?;
                    //cell.start_style(&mut s)?;
                }
            }
            s.push(cell.val);
        }
        s.push('\n');

        write!(w, "{}", s).unwrap();
    }
}
*/
