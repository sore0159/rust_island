use super::cell::Rect;
use super::style::Color;
use super::style::StyleMod;

pub struct Borders {
    pub v_bars: Vec<u16>,
    pub h_bars: Vec<u16>,
    pub mods: StyleMod,
}

impl Borders {
    pub fn new() -> Self {
        Borders {
            v_bars: Vec::with_capacity(3),
            h_bars: Vec::with_capacity(3),
            mods: StyleMod {
                fg: Some(Color::White),
                bg: Some(Color::Black),
                deco: None,
            },
        }
    }

    pub fn draw(&self, rect: &mut Rect) {
        let m = Some(&self.mods);
        draw_box(rect, m);
        for x in &self.v_bars {
            draw_v_split(rect, *x, m);
        }
        for y in &self.h_bars {
            draw_h_split(rect, *y, m);
            for x in &self.v_bars {
                let mut cell = rect.get_mut((*x as u16, *y as u16)).unwrap();
                cell.val = XSPLIT;
            }
        }
    }
}

pub fn draw_box(rec: &mut Rect, m: Option<&StyleMod>) {
    let (w, h) = (rec.size.0 as usize - 1, rec.size.1 as usize - 1);
    for (j, row) in rec.cells.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            match (i, j) {
                (0, 0) => cell.val = TL,
                (x, y) if x == w && y == h => cell.val = BR,
                (0, y) if y == h => cell.val = BL,
                (x, 0) if x == w => cell.val = TR,

                (_, y) if y == h => cell.val = VSIDE,
                (_, 0) => cell.val = VSIDE,
                (0, _) => cell.val = HSIDE,
                (x, _) if x == w => cell.val = HSIDE,
                _ => continue,
            }
            if let Some(m) = m {
                m.apply(&mut cell.style);
            }
        }
    }
}

pub fn draw_h_split(rec: &mut Rect, y: u16, m: Option<&StyleMod>) {
    let row = &mut rec.cells[y as usize - 1];
    let ln = row.len() - 1;
    for (i, cell) in row.iter_mut().enumerate() {
        cell.val = match i {
            0 => LSPLIT,
            n if n == ln => RSPLIT,
            _ => VSIDE,
        };
        if let Some(m) = m {
            m.apply(&mut cell.style);
        }
    }
}
pub fn draw_v_split(rec: &mut Rect, x: u16, m: Option<&StyleMod>) {
    let ln = rec.cells.len() - 1;
    for (j, row) in rec.cells.iter_mut().enumerate() {
        let cell = &mut row[x as usize - 1];
        cell.val = match j {
            0 => TSPLIT,
            n if n == ln => BSPLIT,
            _ => HSIDE,
        };
        if let Some(m) = m {
            m.apply(&mut cell.style);
        }
    }
}

const TL: char = '\u{250c}';
const TR: char = '\u{2510}';
const BL: char = '\u{2514}';
const BR: char = '\u{2518}';

const HSIDE: char = '\u{2502}';
const VSIDE: char = '\u{2500}';

const LSPLIT: char = '\u{251c}';
const RSPLIT: char = '\u{2524}';
const TSPLIT: char = '\u{252C}';
const BSPLIT: char = '\u{2534}';

const XSPLIT: char = '\u{253C}';
