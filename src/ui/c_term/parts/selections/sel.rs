use super::super::super::{
    parts::text::Text,
    style::{Color, StyleMod},
    Key,
};

pub struct Selection {
    pub text: Text,
    pub base_style: [StyleMod; 2],
    pub hover_style: [StyleMod; 2],
    pub selected_style: [StyleMod; 2],
    pub h_and_s_style: [StyleMod; 2],

    pub selected: bool,
    pub quick_select: Option<Key>,
}

impl Selection {
    pub fn new(text: Text) -> Self {
        Self {
            text: text,
            base_style: [StyleMod::default(), StyleMod::default()],
            hover_style: [StyleMod::default(), StyleMod::default()],
            selected_style: [StyleMod::default(), StyleMod::default()],
            h_and_s_style: [StyleMod::default(), StyleMod::default()],

            selected: false,
            quick_select: None,
        }
    }
    pub fn base_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.base_style[1].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.base_style[1].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn base_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.base_style[0].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.base_style[0].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn hover_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.hover_style[0].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.hover_style[0].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn hover_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.hover_style[1].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.hover_style[1].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn selected_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.selected_style[0].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.selected_style[0].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn selected_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.selected_style[1].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.selected_style[1].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn h_and_s_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.h_and_s_style[0].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.h_and_s_style[0].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }
    pub fn h_and_s_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.h_and_s_style[1].fg = Some(Color::Rgb(fg.0, fg.1, fg.2));
        self.h_and_s_style[1].bg = Some(Color::Rgb(bg.0, bg.1, bg.2));
    }

    pub fn with_quick_select(mut self, k: Key) -> Self {
        self.quick_select = Some(k);
        self
    }
    pub fn copy_styles(mut self, other: &Self) -> Self {
        self.base_style = other.base_style.clone();
        self.hover_style = other.hover_style.clone();
        self.selected_style = other.selected_style.clone();
        self.h_and_s_style = other.h_and_s_style.clone();
        self
    }
    pub fn adjust_style(&mut self, focused: bool, hover: bool) {
        let j = if focused { 1 } else { 0 };
        self.text.style_mods = if self.selected {
            if hover {
                self.h_and_s_style[j].clone()
            } else {
                self.selected_style[j].clone()
            }
        } else {
            if hover {
                self.hover_style[j].clone()
            } else {
                self.base_style[j].clone()
            }
        };
    }
}
