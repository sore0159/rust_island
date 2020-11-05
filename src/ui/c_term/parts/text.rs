use super::super::output::{Rect, Style};

#[derive(Debug)]
pub struct Text {
    pub val: String,
    pub start: (u16, u16),
    pub style_mods: Style,
}

impl Text {
    pub fn from_str(s: impl Into<String>) -> Self {
        Self::new(s, (0, 0))
    }
    pub fn new(s: impl Into<String>, start: (u16, u16)) -> Self {
        Text {
            val: s.into(),
            style_mods: Style::default(),
            start: start,
        }
    }
    pub fn trim(&mut self) {
        self.val = self.val.trim().to_string();
    }
    pub fn trim_char(&mut self, c: char) {
        self.val = self.val.trim_matches(c).to_string();
    }
    pub fn len(&self) -> u16 {
        self.val.chars().count() as u16
    }
    pub fn fit(&mut self, f: &Fitter, size: u16) {
        self.val = f.fit(&self.val.trim_matches(f.val), size as usize);
    }
    pub fn draw(&self, r: &mut Rect) {
        for (i, ch) in self.val.chars().enumerate() {
            r.imprint_at(
                (self.start.0 + i as u16, self.start.1),
                ch,
                &self.style_mods,
            );
        }
    }
}

#[derive(Debug)]
pub struct Fitter {
    pub val: char,
    pub alignment: Alignment,
    pub truncate: bool,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Middle,
    MiddleR,
}

impl Fitter {
    pub fn new(val: char, alignment: Alignment, truncate: bool) -> Self {
        Fitter {
            val,
            alignment,
            truncate,
        }
    }
    pub fn left(mut self) -> Self {
        self.alignment = Alignment::Left;
        self
    }
    pub fn right(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }
    pub fn middle(mut self) -> Self {
        self.alignment = Alignment::Middle;
        self
    }
    pub fn middle_r(mut self) -> Self {
        self.alignment = Alignment::MiddleR;
        self
    }
    pub fn set_left(&mut self) {
        self.alignment = Alignment::Left;
    }
    pub fn set_right(&mut self) {
        self.alignment = Alignment::Right;
    }
    pub fn set_middle(&mut self) {
        self.alignment = Alignment::Middle;
    }
    pub fn set_middle_r(&mut self) {
        self.alignment = Alignment::MiddleR;
    }
    pub fn fit(&self, s: &str, mut size: usize) -> String {
        let l = s.chars().count(); // assumes all chars 1 width;
        let mut fitted = String::with_capacity(size);

        if l > size {
            let skip: usize;
            if self.truncate {
                match self.alignment {
                    Alignment::Left => skip = 0,
                    Alignment::Right => skip = l - size,
                    Alignment::Middle => skip = (l - size) / 2,
                    Alignment::MiddleR => skip = (l - size + 1) / 2,
                }
            } else {
                skip = 0;
                size = l;
            }
            for c in s.chars().skip(skip).take(size) {
                fitted.push(c);
            }
        } else if l == size {
            for c in s.chars() {
                fitted.push(c);
            }
        } else if l < size {
            let diff = size - l;
            let (lpad, rpad): (usize, usize);
            match self.alignment {
                Alignment::Left => {
                    lpad = 0;
                    rpad = diff;
                }
                Alignment::Right => {
                    lpad = diff;
                    rpad = 0;
                }
                Alignment::Middle => {
                    lpad = diff / 2;
                    rpad = (diff + 1) / 2;
                }
                Alignment::MiddleR => {
                    lpad = (diff + 1) / 2;
                    rpad = diff / 2;
                }
            }
            let pad = std::iter::repeat(self.val);
            let pad2 = std::iter::repeat(self.val);
            for c in (pad).take(lpad).chain(s.chars()).chain((pad2).take(rpad)) {
                fitted.push(c);
            }
        }
        fitted
    }
}

impl Default for Fitter {
    fn default() -> Self {
        Fitter {
            val: ' ',
            alignment: Alignment::Left,
            truncate: true,
        }
    }
}
