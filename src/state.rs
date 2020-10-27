// C is any UI output, D is game data
pub struct StateData<C, D> {
    pub data: D,
    pub canvas: C,
}

// E is event type provided by UI
pub trait State<C, D, E> {
    fn on_start(&mut self, d: &mut StateData<C, D>) -> Trans<C, D, E>;
    fn on_end(&mut self, d: &mut StateData<C, D>);
    fn on_pause(&mut self, d: &mut StateData<C, D>);
    fn on_resume(&mut self, d: &mut StateData<C, D>) -> Trans<C, D, E>;
    fn handle_event(&mut self, e: E);
    fn tic(&mut self) -> Trans<C, D, E>;
    fn shadow_tic(&mut self);
}

pub enum Trans<C, D, E> {
    None,
    PopMe,
    Push(Box<dyn State<C, D, E>>),
    PushMulti(Vec<Box<dyn State<C, D, E>>>),
    PopAll,
    Sequence(Vec<Trans<C, D, E>>),
    Quit,
}
