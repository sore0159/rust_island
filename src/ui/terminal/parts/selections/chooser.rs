use crate::ui::terminal::rect::Rect;
use crate::ui::terminal::Key;

use super::{Choice, Selection};

pub struct Chooser {
    pub options: Vec<Selection>,
    pub hover: Option<usize>,
    pub choice: Choice,
    pub conf: Config,
}

pub struct Config {
    pub hover_eq_select: bool, // hover_style never used, only h_and_s
    pub select_eq_submit: bool,

    pub can_multi_select: bool,
    pub can_zero_select: bool,

    pub hover_keys: [Option<Key>; 3],
    pub select_key: Option<Key>,
    pub submit_key: Option<Key>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hover_eq_select: false,
            select_eq_submit: false,
            can_multi_select: false,
            can_zero_select: true,
            hover_keys: [Some(Key::Up), Some(Key::Down), None],
            select_key: Some(Key::Char('\n')),
            submit_key: Some(Key::Char(' ')),
        }
    }
}

impl Chooser {
    pub fn new(v: Vec<Selection>, conf: Config) -> Self {
        Chooser {
            options: v,
            hover: None,
            choice: Choice::new(),
            conf: conf,
        }
    }
    pub fn prev_key(mut self, k: Key) -> Self {
        self.conf.hover_keys[0] = Some(k);
        self
    }
    pub fn next_key(mut self, k: Key) -> Self {
        self.conf.hover_keys[1] = Some(k);
        self
    }
    pub fn loop_key(mut self, k: Key) -> Self {
        self.conf.hover_keys[2] = Some(k);
        self
    }
    pub fn select_key(mut self, k: Key) -> Self {
        self.conf.select_key = Some(k);
        self
    }
    pub fn submit_key(mut self, k: Key) -> Self {
        self.conf.submit_key = Some(k);
        self
    }
    pub fn start_selected(mut self, selected: Vec<usize>) -> Self {
        for i in selected {
            self.options[i].selected = true;
        }
        self
    }
    pub fn start_hover(mut self, i: usize) -> Self {
        self.hover = Some(i);
        if self.conf.hover_eq_select {
            self.set_select(i, true);
        }
        self.choice.retrieve();
        self
    }

    pub fn adjust_styles(&mut self, focused: bool) {
        for (i, o) in self.options.iter_mut().enumerate() {
            o.adjust_style(focused, Some(i) == self.hover);
        }
    }
    pub fn get_selected(&self) -> Vec<usize> {
        if self.conf.hover_eq_select {
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
    pub fn submit(&mut self) {
        self.choice.set(self.get_selected());
    }

    pub fn set_select(&mut self, j: usize, to_selected: bool) -> bool {
        let mut made_change = false;
        if to_selected {
            for (i, o) in self.options.iter_mut().enumerate() {
                made_change = made_change || (i == j && !o.selected);
                o.selected = (o.selected && self.conf.can_multi_select) || i == j;
            }
            if self.conf.select_eq_submit {
                self.submit();
                made_change = true;
            }
        } else {
            if !self.conf.can_zero_select && 1 == self.options.iter().filter(|x| x.selected).count()
            {
                return false;
            }
            let o = &mut self.options[j];
            made_change = o.selected;
            o.selected = false;
        }
        made_change
    }

    pub fn set_hover(&mut self, h: Option<usize>) -> bool {
        if self.hover == h {
            return false;
        }
        let old = self.hover.take();
        self.hover = h;
        if self.conf.hover_eq_select {
            if let Some(i) = old {
                self.set_select(i, false);
            }
            if let Some(j) = h {
                self.set_select(j, true);
            }
        }
        true
    }

    pub fn draw(&self, r: &mut Rect) {
        for o in &self.options {
            o.text.draw(r);
        }
    }
    pub fn set_focus(&mut self, to_focused: bool) {
        self.adjust_styles(to_focused);
    }
    pub fn parse_key(&mut self, k: Key) -> bool {
        let mut parsed = false;
        let mut made_change = false;
        if Some(k) == self.conf.submit_key {
            made_change = self.get_selected().len() > 0;
            self.submit();
            parsed = true;
        }
        if !parsed && Some(k) == self.conf.select_key && !self.conf.hover_eq_select {
            if let Some(i) = self.hover {
                made_change = self.set_select(i, !self.options[i].selected) || made_change;
            }
            parsed = true;
        }
        if !parsed {
            for (i, k2) in self.conf.hover_keys.iter().enumerate() {
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
