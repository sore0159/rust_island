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
