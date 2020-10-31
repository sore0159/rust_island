use super::Widget;
use crate::ui::terminal::{Key, SyncTermUI};
use std::fmt::Write;

pub struct WidgetUIBuilder {
    pub quit_key: Option<Key>,
    pub quick_focus_keys: Vec<Option<Key>>,
    pub cycle_focus_keys: [Option<Key>; 3],
    pub widgets: Vec<Box<dyn Widget>>,
    pub focusable: Vec<bool>,
    pub cur_focus: usize,
    pub update_str: String,
}

impl WidgetUIBuilder {
    pub fn new() -> Self {
        WidgetUIBuilder {
            quit_key: Some(Key::Esc),
            quick_focus_keys: Vec::new(),
            cycle_focus_keys: [None, None, Some(Key::Char(' '))],
            widgets: Vec::new(),
            focusable: Vec::new(),
            update_str: String::new(),
            cur_focus: 0,
        }
    }
    pub fn stop_cur_focus(&mut self) {
        write!(
            self.update_str,
            "{}",
            self.widgets[self.cur_focus].lose_focus()
        )
        .unwrap();
    }
    pub fn start_focus(&mut self, i: usize) {
        write!(self.update_str, "{}", self.widgets[i].gain_focus()).unwrap();
        self.cur_focus = i;
    }
    pub fn send_focus_key(&mut self, k: Key) {
        write!(self.update_str, "{}", self.widgets[self.cur_focus].parse(k)).unwrap();
    }
    pub fn add_widget(&mut self, w: impl Widget + 'static) -> usize {
        self.widgets.push(Box::new(w));
        self.focusable.push(true);
        self.widgets.len() - 1
    }
    pub fn build(mut self, init_focus: usize) -> WidgetUI {
        self.cur_focus = init_focus;
        for (i, w) in self.widgets.iter_mut().enumerate() {
            let (s, f) = w.start();
            self.focusable[i] = f;
            if i == init_focus {
                write!(self.update_str, "{}", w.gain_focus()).unwrap();
            } else {
                write!(self.update_str, "{}", s).unwrap();
            }
        }
        WidgetUI(self)
    }
}

pub struct WidgetUI(WidgetUIBuilder);
impl SyncTermUI for WidgetUI {
    fn to_draw(&self) -> &str {
        &self.0.update_str
    }
    fn parse(&mut self, key: Key) -> bool {
        self.0.update_str.clear();
        if self.0.quit_key.as_ref() == Some(&key) {
            return true;
        }
        for (i, k) in self.0.quick_focus_keys.iter().enumerate() {
            if k.as_ref() == Some(&key) {
                if i == self.0.cur_focus {
                    return false;
                }
                self.0.stop_cur_focus();
                self.0.start_focus(i);
                return false;
            }
        }
        let ln = self.0.widgets.len();
        for (i, k) in self.0.cycle_focus_keys.iter().enumerate() {
            if k.as_ref() == Some(&key) {
                if ln > 1 {
                    let mut next: Option<usize> = None;
                    match i {
                        0 => {
                            for j in 1..self.0.focusable.len() {
                                let j: isize = self.0.cur_focus as isize - j as isize;
                                let j: usize = if j < 0 {
                                    break;
                                //self.0.focusable.len() + (-j) as usize
                                } else {
                                    j as usize
                                };
                                if self.0.focusable[j] {
                                    next = Some(j);
                                    break;
                                }
                            }
                        }
                        1 => {
                            for j in 1..self.0.focusable.len() {
                                let j = self.0.cur_focus + j;
                                if j >= self.0.focusable.len() {
                                    break;
                                }
                                if self.0.focusable[j] {
                                    next = Some(j);
                                    break;
                                }
                            }
                        }
                        _ => {
                            for j in 1..self.0.focusable.len() {
                                let j = (self.0.cur_focus + j) % self.0.focusable.len();
                                if self.0.focusable[j] {
                                    next = Some(j);
                                    break;
                                }
                            }
                        }
                    }
                    if let Some(j) = next {
                        if j != self.0.cur_focus {
                            self.0.stop_cur_focus();
                            self.0.start_focus(j);
                        }
                    }
                }
                return false;
            }
        }
        self.0.send_focus_key(key);
        false
    }
}
