pub mod stack;

// C is any UI output, D is game data
// E is event type provided by UI
pub trait State<C, D, E> {
    fn on_start(&mut self, data: &mut D, canvas: &mut C) -> Trans<C, D, E>;
    fn on_end(&mut self, data: &mut D, canvas: &mut C);
    fn on_pause(&mut self, data: &mut D, canvas: &mut C);
    fn on_resume(&mut self, data: &mut D, canvas: &mut C) -> Trans<C, D, E>;
    fn handle_event(&mut self, e: E, data: &mut D, canvas: &mut C) -> Trans<C, D, E>;
    fn tic(&mut self) -> Trans<C, D, E>;
    fn shadow_tic(&mut self);
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
