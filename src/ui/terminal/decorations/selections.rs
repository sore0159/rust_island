use crate::ui::terminal::decorations::text::Text;
use crate::ui::terminal::rect::Rect;
use crate::ui::terminal::style::StyleMod;
use crate::ui::terminal::Key;

pub struct Selections {
    pub options: Vec<Selection>,
    pub hover: Option<usize>,

    pub hover_equals_select: bool,
    pub can_multi_select: bool,
    pub can_zero_select: bool,
    pub hover_keys: [Option<Key>; 3],
    pub select_key: Option<Key>,
}

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

impl Selections {
    pub fn new(v: Vec<Selection>) -> Self {
        Selections {
            options: v,
            hover: None,

            hover_equals_select: false,
            can_multi_select: false,
            can_zero_select: true,
            hover_keys: [None; 3],
            select_key: None,
        }
    }
    pub fn adjust_styles(&mut self, focused: bool) {
        for (i, o) in self.options.iter_mut().enumerate() {
            o.adjust_style(focused, Some(i) == self.hover);
        }
    }

    pub fn set_select(&mut self, j: usize, to_selected: bool) -> bool {
        let mut already_set = false;
        if to_selected {
            for (i, o) in self.options.iter_mut().enumerate() {
                already_set = already_set || (i == j && o.selected);
                o.selected = (o.selected && self.can_multi_select) || i == j;
            }
        } else {
            if !self.can_zero_select && 1 == self.options.iter().filter(|x| x.selected).count() {
                return false;
            }
            let o = &mut self.options[j];
            already_set = !o.selected;
            o.selected = false;
        }
        !already_set
    }
    pub fn set_hover(&mut self, h: Option<usize>) -> bool {
        let mut made_change = false;
        if self.hover_equals_select {
            if let Some(i) = self.hover {
                made_change = made_change || self.set_select(i, false);
            }
            if let Some(j) = h {
                made_change = made_change || self.set_select(j, true);
            }
        }
        made_change = made_change || self.hover != h;
        self.hover = h;
        made_change
    }

    pub fn draw(&self, r: &mut Rect) {
        for o in &self.options {
            r.apply_text(&o.text);
        }
    }
    pub fn set_focus(&mut self, to_focused: bool) {
        self.adjust_styles(to_focused);
    }
    pub fn parse_key(&mut self, k: Key) -> bool {
        let mut parsed = false;
        let mut made_change = false;
        if Some(k) == self.select_key && !self.hover_equals_select {
            if let Some(i) = self.hover {
                made_change |= self.set_select(i, !self.options[i].selected);
            }
            parsed = true;
        }
        if !parsed {
            for (i, k2) in self.hover_keys.iter().enumerate() {
                if &Some(k) == k2 {
                    let h: Option<usize>;
                    match i {
                        0 => {
                            h = if let Some(x) = self.hover {
                                if x > 1 {
                                    Some(x - 1)
                                } else {
                                    Some(0)
                                }
                            } else {
                                Some(0)
                            };
                        }
                        1 => {
                            h = if let Some(x) = self.hover {
                                if x < self.options.len() - 1 {
                                    Some(x + 1)
                                } else {
                                    Some(x)
                                }
                            } else {
                                Some(0)
                            };
                        }
                        _ => {
                            h = if let Some(x) = self.hover {
                                Some((x + 1) % self.options.len())
                            } else {
                                Some(0)
                            };
                        }
                    }
                    made_change |= self.set_hover(h);
                    parsed = true;
                    break;
                }
            }
        }
        if !parsed {
            for o in &mut self.options {
                if o.quick_select == Some(k) {
                    made_change |= !o.selected;
                    o.selected = true;
                    parsed = true;
                    break;
                }
            }
        }
        if made_change {
            self.adjust_styles(true); // assumes always focused when parsing keys
        }
        parsed
    }
}