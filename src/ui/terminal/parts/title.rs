use super::text;
use crate::ui::terminal::{
    rect::Rect,
    style::{Color, StyleMod},
};

pub struct Title {
    pub text: text::Text,
    pub vert: bool,
    pub flair: Flair,
}

impl Title {
    pub fn new(s: impl Into<String>, x: u16, y: u16, vert: bool) -> Self {
        let flair = if !vert {
            if y == 1 {
                Flair::WedgeDown
            } else {
                Flair::WedgeUp
            }
        } else {
            if x == 1 {
                Flair::WedgeRight
            } else {
                Flair::WedgeLeft
            }
        };
        Title {
            text: text::Text::new(s, (x, y)),
            vert: vert,
            flair: flair,
        }
    }
    pub fn with_flair(mut self, f: Flair) -> Self {
        self.flair = f;
        self
    }
    pub fn force_fg(&mut self, r: u8, g: u8, b: u8) {
        self.text.style_mods.fg = Some(Color::Rgb(r, g, b));
    }
    pub fn draw(&self, r: &mut Rect, border_style: &StyleMod) {
        let m = StyleMod {
            deco: None,
            fg: self
                .text
                .style_mods
                .fg
                .clone()
                .or(border_style.bg.clone())
                .or(Some(r.default_style.bg.clone())),
            bg: self
                .text
                .style_mods
                .bg
                .clone()
                .or(border_style.fg.clone())
                .or(Some(r.default_style.fg.clone())),
        };
        let mut ln = self.text.len() as usize;
        let spacer = if self.flair == Flair::None {
            ln -= 1;
            None
        } else {
            ln += 1;
            Some(' ')
        };
        for (i, c) in spacer
            .clone()
            .into_iter()
            .chain(self.text.val.chars())
            .chain(spacer.clone())
            .enumerate()
        {
            let coord = if self.vert {
                (self.text.start.0, self.text.start.1 + i as u16 - 1)
            } else {
                (self.text.start.0 + i as u16 - 1, self.text.start.1)
            };
            let mut cell = r.get_mut(coord).unwrap();
            let chars = self.flair.chars();
            if spacer.is_some() {
                match i {
                    0 => {
                        cell.val = chars.0;
                        border_style.apply(&mut cell.style);
                    }
                    j if j == ln => {
                        cell.val = chars.1;
                        border_style.apply(&mut cell.style);
                    }
                    _ => {
                        cell.val = c;
                        m.apply(&mut cell.style);
                    }
                }
            } else {
                cell.val = c;
                m.apply(&mut cell.style);
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Flair {
    None,
    WedgeLeft,
    WedgeRight,
    WedgeUp,
    WedgeDown,
    Space,
    VDiamond1,
    HDiamond1,
    HDiamond2,
    VDiamond2,
}

impl Flair {
    pub fn chars(&self) -> (char, char) {
        match &self {
            Flair::None => (' ', ' '),
            Flair::WedgeLeft => ('\u{25E2}', '\u{25E5}'),
            Flair::WedgeRight => ('\u{25E3}', '\u{25E4}'),
            Flair::WedgeUp => ('\u{25E2}', '\u{25E3}'),
            Flair::WedgeDown => ('\u{25E5}', '\u{25E4}'),
            Flair::Space => (' ', ' '),
            Flair::VDiamond1 => ('\u{25E3}', '\u{25E5}'),
            Flair::HDiamond1 => ('\u{25E5}', '\u{25E3}'),
            Flair::HDiamond2 => ('\u{25E2}', '\u{25E4}'),
            Flair::VDiamond2 => ('\u{25E2}', '\u{25E4}'),
        }
    }
}
