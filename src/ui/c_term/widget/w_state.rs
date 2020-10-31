use super::super::Key;
use super::Widget;
use std::fmt::Write;

pub struct WidgetStateBuilder {
    pub quit_key: Option<Key>,
    pub quick_focus_keys: Vec<Option<Key>>,
    pub cycle_focus_keys: [Option<Key>; 3],
    pub widgets: Vec<Box<dyn Widget>>,
    pub focusable: Vec<bool>,
    pub cur_focus: usize,
    pub update_str: String,
}

impl WidgetStateBuilder {
    pub fn new() -> Self {
        WidgetStateBuilder {
            //quit_key: Some(Key::Esc),
            quit_key: Some(Key::State1),
            quick_focus_keys: Vec::new(),
            //cycle_focus_keys: [None, None, Some(Key::Char(' '))],
            cycle_focus_keys: [None, None, Some(Key::State1)],
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
    pub fn build(mut self, init_focus: usize) -> WidgetState {
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
        WidgetState(self)
    }
}

pub struct WidgetState(WidgetStateBuilder);

mod double_write {
    use super::WidgetState;
    use crate::state::{self, Trans};

    use std::io::Write;

    impl state::State<state::Canvas, state::Data, state::Event> for WidgetState {
        fn on_start(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
            write!(canvas.stdout, "{}", &self.0.update_str).unwrap();
            canvas.stdout.flush().unwrap();
            self.0.update_str.clear();
            Trans::None
        }
        fn on_cycle(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
            if self.0.update_str.is_empty() {
                return Trans::None;
            }
            write!(canvas.stdout, "{}", &self.0.update_str).unwrap();
            canvas.stdout.flush().unwrap();
            self.0.update_str.clear();
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
}
