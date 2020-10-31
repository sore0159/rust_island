//pub mod mock4;

use std::io::Write;

use crate::state::{self, Trans};

pub struct NullState;

impl state::State<state::Canvas, state::Data, state::Event> for NullState {
    fn handle_event(
        &mut self,
        k: state::Event,
        _data: &mut state::Data,
        canvas: &mut state::Canvas,
    ) -> Trans {
        if k == state::Event::State1 {
            //if k == state::Event::Char('q') {
            Trans::Quit
        } else {
            write!(canvas.stdout, "BLAH").unwrap();
            canvas.stdout.flush().unwrap();
            Trans::None
        }
    }
}

pub fn gen_mockup() -> Result<state::StateStack, Box<dyn std::error::Error>> {
    let d = crate::data::mockup::gen_mockup();
    Ok(state::stack::StateStack::new(
        NullState,
        state::Canvas::new()?,
        d,
        state::EventStream::new()?,
    ))
}
