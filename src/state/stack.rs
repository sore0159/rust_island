use super::traits::{State, Trans};
use std::iter::Iterator;
use std::time::{Duration, Instant};

pub struct StateStack<C, D, E, I>
where
    I: Iterator<Item = E>,
{
    pub stack: Vec<Box<dyn State<C, D, E>>>,
    pub canvas: C,
    pub data: D,
    pub event_stream: I,

    pub tic_len: Duration,
}

impl<C, D, E, I> StateStack<C, D, E, I>
where
    I: Iterator<Item = E>,
{
    pub fn new(first_state: impl State<C, D, E> + 'static, canvas: C, data: D, stream: I) -> Self {
        StateStack {
            stack: vec![Box::new(first_state)],
            canvas: canvas,
            data: data,
            event_stream: stream,
            tic_len: Duration::from_secs(1).div_f64(60.0),
        }
    }
    pub fn run(&mut self) {
        let mut next_tic = Instant::now() + self.tic_len;
        'OUTER: loop {
            let now = Instant::now();
            if self.stack.is_empty() {
                break 'OUTER;
            }
            if let Some(_) = next_tic.checked_duration_since(now) {
                let ln = self.stack.len();
                for s in self.stack.iter_mut().take(ln - 1) {
                    s.on_shadow_tic(&mut self.data, &mut self.canvas);
                }
                let t = self.stack[ln - 1].on_tic(&mut self.data, &mut self.canvas);
                if self.handle_trans(t) {
                    break 'OUTER;
                }
                next_tic += self.tic_len;
            }
            let ln = self.stack.len();
            for s in self.stack.iter_mut().take(ln - 1) {
                s.on_shadow_cycle(&mut self.data, &mut self.canvas);
            }
            let t = self.stack[ln - 1].on_cycle(&mut self.data, &mut self.canvas);
            if self.handle_trans(t) {
                break 'OUTER;
            }
            while let Some(e) = self.event_stream.next() {
                let ln = self.stack.len();
                let t = self.stack[ln - 1].handle_event(e, &mut self.data, &mut self.canvas);
                if self.handle_trans(t) {
                    break 'OUTER;
                }
            }
        }
    }

    pub fn handle_trans(&mut self, mut t: Trans<C, D, E>) -> bool {
        loop {
            let mut next_t = Trans::None;
            match t {
                Trans::None => {
                    return self.stack.is_empty();
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
                        if self.handle_trans(t) {
                            return true;
                        }
                    }
                }
                Trans::Quit => {
                    return true;
                }
            }
            t = next_t;
        }
    }
}
