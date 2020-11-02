use super::super::super::{
    output::{new_rgb, Style},
    parts::text::Text,
    Key,
};

pub struct Selection {
    pub text: Text,
    pub base_style: [Style; 2],
    pub hover_style: [Style; 2],
    pub selected_style: [Style; 2],
    pub h_and_s_style: [Style; 2],

    pub selected: bool,
    pub quick_select: Option<Key>,
}

impl Selection {
    pub fn new(text: Text) -> Self {
        Self {
            text: text,
            base_style: [Style::default(), Style::default()],
            hover_style: [Style::default(), Style::default()],
            selected_style: [Style::default(), Style::default()],
            h_and_s_style: [Style::default(), Style::default()],

            selected: false,
            quick_select: None,
        }
    }
    pub fn base_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.base_style[1].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.base_style[1].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn base_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.base_style[0].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.base_style[0].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn hover_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.hover_style[0].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.hover_style[0].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn hover_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.hover_style[1].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.hover_style[1].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn selected_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.selected_style[0].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.selected_style[0].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn selected_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.selected_style[1].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.selected_style[1].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn h_and_s_no_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.h_and_s_style[0].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.h_and_s_style[0].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
    }
    pub fn h_and_s_f(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.h_and_s_style[1].foreground_color = Some(new_rgb(fg.0, fg.1, fg.2));
        self.h_and_s_style[1].background_color = Some(new_rgb(bg.0, bg.1, bg.2));
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
