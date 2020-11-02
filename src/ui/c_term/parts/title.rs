use super::super::output::{new_rgb, Rect, Style};
use super::text;

pub struct Title {
    pub text: text::Text,
    pub vert: bool,
    pub flair: Flair,
}

impl Title {
    pub fn new(s: impl Into<String>, x: u16, y: u16, vert: bool) -> Self {
        let flair = if !vert {
            if y == 0 {
                Flair::WedgeDown
            } else {
                Flair::WedgeUp
            }
        } else {
            if x == 0 {
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
        self.text.style_mods.foreground_color = Some(new_rgb(r, g, b));
    }
    pub fn draw(&self, r: &mut Rect, border_style: &Style) {
        let m = Style {
            foreground_color: border_style.background_color,
            background_color: border_style.foreground_color,
            attributes: border_style.attributes,
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
            let chars = self.flair.chars();
            let cell = r.get_mut(coord).unwrap();
            let mut m2 = m.clone();
            if m.background_color.is_none() {
                m2.background_color = Some(cell.base_fg);
            }
            if m.foreground_color.is_none() {
                m2.foreground_color = Some(cell.base_bg);
            }
            if spacer.is_some() {
                match i {
                    0 => {
                        r.imprint_at(coord, chars.0, border_style);
                    }
                    j if j == ln => {
                        r.imprint_at(coord, chars.1, border_style);
                    }
                    _ => {
                        r.imprint_at(coord, c, &m2);
                    }
                }
            } else {
                r.imprint_at(coord, c, &m2);
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
