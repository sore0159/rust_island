use crate::ui::terminal::{
    rect::Rect,
    style::{Color, StyleMod},
};

#[derive(Clone, Debug)]
pub struct Bar {
    pub vert: bool,
    pub at: (u16, bool),
    pub padding: (u16, u16),
    pub start_type: BorderChar,
    pub end_type: BorderChar,
}

impl Bar {
    pub fn new(at: u16) -> Self {
        Bar {
            vert: false,
            at: (at, true),
            padding: (0, 0),
            start_type: BorderChar::VSide,
            end_type: BorderChar::VSide,
        }
    }
    pub fn from_end(mut self) -> Self {
        self.at.1 = false;
        self
    }
    pub fn vert(mut self, vert: bool) -> Self {
        self.vert = vert;
        self
    }
    pub fn with_pads(mut self, start: u16, end: u16) -> Self {
        self.padding = (start, end);
        self
    }
    pub fn with_ends(mut self, start: BorderChar, end: BorderChar) -> Self {
        self.start_type = start;
        self.end_type = end;
        self
    }
    pub fn draw(&self, r: &mut Rect, m: &StyleMod, chartype: BorderType) {
        let start = self.padding.0;
        let (end, mid_char, at) = if self.vert {
            (
                r.size.1 - self.padding.1,
                char_for(BorderChar::HSide, chartype),
                if self.at.1 {
                    self.at.0
                } else {
                    r.size.0 - self.at.0 + 1
                },
            )
        } else {
            (
                r.size.0 - self.padding.1,
                char_for(BorderChar::VSide, chartype),
                if self.at.1 {
                    self.at.0
                } else {
                    r.size.1 - self.at.0 + 1
                },
            )
        };
        for i in start..end {
            let cell = if self.vert {
                &mut r.cells[i as usize][at as usize - 1]
            } else {
                &mut r.cells[at as usize - 1][i as usize]
            };
            m.apply(&mut cell.style);
            match i {
                j if j == start => cell.val = char_for(self.start_type.clone(), chartype),
                j if j == end - 1 => cell.val = char_for(self.end_type.clone(), chartype),
                _ => cell.val = mid_char.clone(),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Borders {
    pub bars: Vec<Bar>,
    pub crosses: Vec<(u16, u16)>,
    pub mods: StyleMod,
    pub chartype: BorderType,
}

impl Default for Borders {
    fn default() -> Self {
        use BorderChar::*;
        let mut v = Vec::with_capacity(4);
        v.push(Bar::new(1).with_ends(TL, TR));
        v.push(Bar::new(1).from_end().with_ends(BL, BR));
        v.push(Bar::new(1).vert(true).with_ends(TL, BL));
        v.push(Bar::new(1).vert(true).from_end().with_ends(TR, BR));
        Borders {
            bars: v,
            mods: StyleMod {
                fg: Some(Color::White),
                bg: Some(Color::Black),
                deco: None,
            },
            crosses: Vec::new(),
            chartype: BorderType::Basic,
        }
    }
}

impl Borders {
    pub fn draw(&self, rect: &mut Rect) {
        for bar in &self.bars {
            bar.draw(rect, &self.mods, self.chartype.clone());
        }
        for coord in &self.crosses {
            let cell = rect.get_mut(*coord).unwrap();
            self.mods.apply(&mut cell.style);
            cell.val = char_for(BorderChar::XSplit, self.chartype.clone());
        }
    }
    pub fn add_bar(&mut self, at: u16, vert: bool, pad_start: u16, pad_end: u16) -> usize {
        let mut b = Bar::new(at).vert(vert).with_pads(pad_start, pad_end);
        use BorderChar::*;
        if vert {
            b = b.with_ends(TSplit, BSplit);
        } else {
            b = b.with_ends(LSplit, RSplit);
        }
        self.bars.push(b);
        self.bars.len() - 1
    }
    pub fn add_cross(&mut self, x: u16, y: u16) {
        self.crosses.push((x, y));
    }
}

#[derive(Clone, Debug)]
pub enum BorderChar {
    TL,
    TR,
    BL,
    BR,
    HSide,
    VSide,
    LSplit,
    RSplit,
    TSplit,
    BSplit,
    XSplit,
}

#[derive(Copy, Clone, Debug)]
pub enum BorderType {
    Basic,
    Double,
    Rounded,
    Thick,
}

pub fn char_for(bc: BorderChar, st: BorderType) -> char {
    match st {
        BorderType::Basic => match bc {
            BorderChar::TL => '\u{250c}',
            BorderChar::TR => '\u{2510}',
            BorderChar::BL => '\u{2514}',
            BorderChar::BR => '\u{2518}',

            BorderChar::HSide => '\u{2502}',
            BorderChar::VSide => '\u{2500}',

            BorderChar::LSplit => '\u{251c}',
            BorderChar::RSplit => '\u{2524}',
            BorderChar::TSplit => '\u{252C}',
            BorderChar::BSplit => '\u{2534}',

            BorderChar::XSplit => '\u{253C}',
        },
        BorderType::Double => match bc {
            BorderChar::TL => '\u{2554}',
            BorderChar::TR => '\u{2557}',
            BorderChar::BL => '\u{255A}',
            BorderChar::BR => '\u{255D}',

            BorderChar::HSide => '\u{2551}',
            BorderChar::VSide => '\u{2550}',

            BorderChar::LSplit => '\u{2560}',
            BorderChar::RSplit => '\u{2563}',
            BorderChar::TSplit => '\u{2566}',
            BorderChar::BSplit => '\u{2569}',

            BorderChar::XSplit => '\u{256C}',
        },
        BorderType::Thick => match bc {
            BorderChar::TL => '\u{250F}',
            BorderChar::TR => '\u{2513}',
            BorderChar::BL => '\u{2517}',
            BorderChar::BR => '\u{251B}',

            BorderChar::HSide => '\u{2503}',
            BorderChar::VSide => '\u{2501}',

            BorderChar::LSplit => '\u{2523}',
            BorderChar::RSplit => '\u{252B}',
            BorderChar::TSplit => '\u{2533}',
            BorderChar::BSplit => '\u{253B}',

            BorderChar::XSplit => '\u{254a}',
        },

        BorderType::Rounded => match bc {
            BorderChar::TL => '\u{256D}',
            BorderChar::TR => '\u{256E}',
            BorderChar::BL => '\u{2570}',
            BorderChar::BR => '\u{256F}',
            _ => char_for(bc, BorderType::Basic),
        },
    }
}
