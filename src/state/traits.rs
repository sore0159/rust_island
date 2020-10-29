// C is where you tell the UI what to output, D is game data
// E is event type provided by UI
pub trait State<C, D, E> {
    fn on_start(&mut self, _data: &mut D, _canvas: &mut C) -> Trans<C, D, E> {
        Trans::None
    }
    fn on_end(&mut self, _data: &mut D, _canvas: &mut C) {}
    fn on_pause(&mut self, _data: &mut D, _canvas: &mut C) {}
    fn on_resume(&mut self, _data: &mut D, _canvas: &mut C) -> Trans<C, D, E> {
        Trans::None
    }
    fn handle_event(&mut self, _e: E, _data: &mut D, _canvas: &mut C) -> Trans<C, D, E> {
        Trans::None
    }
    fn on_tic(&mut self, _data: &mut D, _canvas: &mut C) -> Trans<C, D, E> {
        Trans::None
    }
    fn on_shadow_tic(&mut self, _data: &mut D, _canvas: &mut C) {}
    fn on_cycle(&mut self, _data: &mut D, _canvas: &mut C) -> Trans<C, D, E> {
        Trans::None
    }
    fn on_shadow_cycle(&mut self, _data: &mut D, _canvas: &mut C) {}
}

pub enum Trans<C, D, E> {
    None,
    Pop,
    Push(Box<dyn State<C, D, E>>),
    Switch(Box<dyn State<C, D, E>>),
    PushMulti(Vec<Box<dyn State<C, D, E>>>),
    PopAll,
    Sequence(Vec<Trans<C, D, E>>),
    Quit,
}

use std::fmt;
impl<C, D, E> fmt::Debug for Trans<C, D, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Trans::None => "None",
            Trans::Quit => "Quit",
            Trans::Push(_) => "Push",
            Trans::Switch(_) => "Switch",
            _ => "Other",
        };
        write!(f, "Trans::{}", t)
    }
}
