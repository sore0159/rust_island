use crate::ui::terminal::decorations::text::Text;
use crate::ui::terminal::rect::Rect;
use crate::ui::terminal::style::StyleMod;
use crate::ui::terminal::widget::{data, Widget};
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
    pub fn get_selected(&self) -> Vec<usize> {
        if self.hover_equals_select {
            self.hover.iter().map(|x| x.clone()).collect()
        } else {
            self.options
                .iter()
                .enumerate()
                .filter(|(_, x)| x.selected)
                .map(|(i, _)| i)
                .collect()
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
        if self.hover == h {
            return false;
        }
        if self.hover_equals_select {
            if let Some(i) = self.hover {
                made_change = self.set_select(i, false) || made_change;
            }
            if let Some(j) = h {
                made_change = self.set_select(j, true) || made_change;
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
                made_change = self.set_select(i, !self.options[i].selected) || made_change;
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
                    made_change = self.set_hover(h) || made_change;
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

use std::cell::Cell;
use std::rc::Rc;

pub struct Choice(Rc<Cell<Vec<usize>>>);

impl Choice {
    pub fn new() -> Self {
        Choice(Rc::new(Cell::new(Vec::new())))
    }
    pub fn set(&self, v: Vec<usize>) {
        self.0.set(v);
    }
    pub fn retrieve(&self) -> Vec<usize> {
        self.0.replace(Vec::new())
    }
    pub fn clone(&self) -> Self {
        Choice(self.0.clone())
    }
}

pub struct BasicWidget {
    pub w_data: data::WidgetData,
    pub selections: Selections,
    pub choice: Choice,
    pub confirm_key: Key,
}

impl BasicWidget {
    pub fn new(
        origin: (u16, u16),
        size: (u16, u16),
        sels: Vec<Selection>,
        confirm: Key,
        choice: Choice,
    ) -> Self {
        BasicWidget {
            w_data: data::WidgetData::new(origin, size.0, size.1),
            selections: Selections::new(sels),
            choice: choice,
            confirm_key: confirm,
        }
    }
}
impl Widget for BasicWidget {
    fn start(&mut self) -> (&str, bool) {
        self.w_data.set_focus(false);
        self.selections.set_focus(false);
        for text in &self.w_data.texts {
            self.w_data.rect.apply_text(text);
        }
        self.selections.draw(&mut self.w_data.rect);
        self.w_data.gen_drawstring();
        (&self.w_data.updates, self.w_data.focusable)
    }
    fn gain_focus(&mut self) -> &str {
        self.w_data.set_focus(true);
        self.selections.set_focus(true);
        self.selections.draw(&mut self.w_data.rect);
        self.w_data.gen_drawstring();
        &self.w_data.updates
    }
    fn lose_focus(&mut self) -> &str {
        self.w_data.set_focus(false);
        self.selections.set_focus(false);
        self.selections.draw(&mut self.w_data.rect);
        self.w_data.gen_drawstring();
        &self.w_data.updates
    }
    fn parse(&mut self, key: Key) -> &str {
        self.w_data.updates.clear();
        if key == self.confirm_key {
            self.choice.set(self.selections.get_selected());
        } else if self.selections.parse_key(key) {
            self.selections.draw(&mut self.w_data.rect);
            self.w_data.gen_drawstring();
        }
        &self.w_data.updates
    }
}
