use super::super::{Key, KeyCode};
use super::Widget;
use std::io::Write;

pub struct WidgetStateBuilder {
    pub quit_key: Option<Key>,
    pub quick_focus_keys: Vec<Option<Key>>,
    pub cycle_focus_keys: [Option<Key>; 3],
    pub widgets: Vec<Box<dyn Widget>>,
    pub focusable: Vec<bool>,
    pub cur_focus: usize,
    pub changed: Vec<bool>,
}

impl WidgetStateBuilder {
    pub fn new() -> Self {
        WidgetStateBuilder {
            quit_key: Some(Key::from_code(KeyCode::Esc)),
            quick_focus_keys: Vec::new(),
            cycle_focus_keys: [None, None, Some(Key::from_code(KeyCode::Tab))],
            //cycle_focus_keys: [None, None, Some(Key::from_char('\t'))],
            widgets: Vec::new(),
            focusable: Vec::new(),
            cur_focus: 0,
            changed: Vec::new(),
        }
    }
    pub fn stop_cur_focus(&mut self) {
        self.widgets[self.cur_focus].lose_focus();
        self.changed[self.cur_focus] = true;
    }
    pub fn start_focus(&mut self, i: usize) {
        self.widgets[i].gain_focus();
        self.changed[i] = true;
        self.cur_focus = i;
    }
    pub fn send_focus_key(&mut self, k: Key) {
        self.changed[self.cur_focus] =
            self.widgets[self.cur_focus].parse(k) || self.changed[self.cur_focus];
    }
    pub fn add_widget(&mut self, w: impl Widget + 'static) -> usize {
        self.widgets.push(Box::new(w));
        self.focusable.push(true);
        self.changed.push(true);
        self.widgets.len() - 1
    }
    pub fn build(mut self, init_focus: usize) -> WidgetState {
        self.cur_focus = init_focus;
        for (i, w) in self.widgets.iter_mut().enumerate() {
            let f = w.start();
            self.focusable[i] = f;
            if i == init_focus {
                w.gain_focus();
            }
        }
        WidgetState(self)
    }
}

pub struct WidgetState(WidgetStateBuilder);

impl WidgetState {
    pub fn write_flush_full(&mut self, s: &mut crate::ui::c_term::Stdout) -> anyhow::Result<()> {
        for (i, w) in self.0.widgets.iter_mut().enumerate() {
            w.queue_write(s)?;
            self.0.changed[i] = false;
        }
        s.flush()?;
        Ok(())
    }
    pub fn write_flush(&mut self, s: &mut crate::ui::c_term::Stdout) -> anyhow::Result<()> {
        let list: Vec<usize> = self
            .0
            .changed
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i)
            .collect();
        for i in list {
            self.0.widgets[i].queue_write(s)?;
            self.0.changed[i] = false;
        }
        s.flush()?;
        Ok(())
    }
}

use crate::state::{self, Trans};

impl state::State<state::Canvas, state::Data, state::Event> for WidgetState {
    fn on_start(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.write_flush_full(&mut canvas.stdout).unwrap();
        Trans::None
    }
    fn on_resume(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.write_flush_full(&mut canvas.stdout).unwrap();
        Trans::None
    }
    fn on_cycle(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.write_flush(&mut canvas.stdout).unwrap();
        Trans::None
    }

    fn handle_event(
        &mut self,
        key: state::Event,
        _data: &mut state::Data,
        _canvas: &mut state::Canvas,
    ) -> Trans {
        if self.0.quit_key.as_ref() == Some(&key) {
            return Trans::Quit;
        }
        for (i, k) in self.0.quick_focus_keys.iter().enumerate() {
            if k.as_ref() == Some(&key) {
                if i == self.0.cur_focus {
                    return Trans::None;
                }
                self.0.stop_cur_focus();
                self.0.start_focus(i);
                return Trans::None;
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
                return Trans::None;
            }
        }
        self.0.send_focus_key(key);
        Trans::None
    }
}
