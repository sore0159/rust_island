use super::{State, Trans};
use std::iter::Iterator;

pub struct StateStack<C, D, E, I>
where
    I: Iterator<Item = E>,
{
    pub stack: Vec<Box<dyn State<C, D, E>>>,
    pub canvas: C,
    pub data: D,
    pub event_stream: I,

    pub tic_len: std::time::Duration,
}

impl<C, D, E, I> StateStack<C, D, E, I>
where
    I: Iterator<Item = E>,
{
    pub fn run(&mut self) {
        //
    }

    pub fn handle_trans(&mut self, mut t: Trans<C, D, E>) {
        loop {
            let mut next_t = Trans::None;
            match t {
                Trans::None => {
                    return;
                }
                Trans::Pop => {
                    let ln = self.stack.len();
                    self.stack[ln - 1].on_end(&mut self.data, &mut self.canvas);
                    self.stack.pop();
                    if !self.stack.is_empty() {
                        next_t = self.stack[ln - 2].on_resume(&mut self.data, &mut self.canvas);
                    }
                }
                Trans::Push(s) => {
                    self.stack.push(s);
                    let ln = self.stack.len();
                    if ln > 1 {
                        self.stack[ln - 2].on_pause(&mut self.data, &mut self.canvas);
                    }
                    next_t = self.stack[ln - 1].on_start(&mut self.data, &mut self.canvas)
                }
                Trans::Switch(s) => {
                    let ln = self.stack.len();
                    self.stack[ln - 1].on_end(&mut self.data, &mut self.canvas);
                    self.stack.pop();
                    self.stack.push(s);
                    next_t = self.stack[ln - 1].on_start(&mut self.data, &mut self.canvas)
                }
                Trans::PushMulti(v) => {
                    for s in v {
                        match next_t {
                            Trans::None => {}
                            _ => panic!("State attempted transition in the middle of a multi push"),
                        }
                        self.stack.push(s);
                        let ln = self.stack.len();
                        if ln > 1 {
                            self.stack[ln - 2].on_pause(&mut self.data, &mut self.canvas);
                        }
                        next_t = self.stack[ln - 1].on_start(&mut self.data, &mut self.canvas);
                    }
                }
                Trans::PopAll => {
                    while self.stack.len() > 0 {
                        let ln = self.stack.len();
                        self.stack[ln - 1].on_end(&mut self.data, &mut self.canvas);
                        self.stack.pop();
                    }
                }
                Trans::Sequence(v) => {
                    for t in v {
                        self.handle_trans(t);
                    }
                }
                Trans::Quit => {
                    panic!("Attempted quit transititon in handling function");
                }
            }
            t = next_t;
        }
    }
}
