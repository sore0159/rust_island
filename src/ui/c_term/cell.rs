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

    pub fn style_delta(&self, o: &Self) -> StyleMod {
        let mut s = StyleMod::new();
        if self.style.deco != o.style.deco {
            s.deco = Some(o.style.deco.clone());
        }
        if self.style.fg != o.style.fg {
            s.fg = Some(o.style.fg.clone());
        }
        if self.style.bg != o.style.bg {
            s.bg = Some(o.style.bg.clone());
        }
        s
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
