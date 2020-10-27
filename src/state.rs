use crate::data::Game;

pub trait State {
    fn on_start(&mut self, g: &mut Game) -> Trans;
    fn on_end(&mut self, g: &mut Game);
    fn on_pause(&mut self, g: &mut Game);
    fn on_resume(&mut self, g: &mut Game) -> Trans;
    fn handle_event(&mut self, e: Event);
    fn tic(&mut self) -> Trans;
    fn shadow_tic(&mut self);
}

pub enum Event {
    Key,
}

pub enum Trans {
    None,
    PopMe,
    Push(Box<dyn State>),
    PushMulti(Vec<Box<dyn State>>),
    PopAll,
    Sequence(Vec<Trans>),
    Quit,
}
