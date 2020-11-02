pub mod mock5;

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
        if k.is_char('q') {
            Trans::Quit
        } else {
            write!(canvas.stdout, "EVENT:{:?}", k).unwrap();
            canvas.stdout.flush().unwrap();
            Trans::None
        }
    }
}

pub fn gen_mockup() -> anyhow::Result<state::StateStack> {
    let d = crate::data::mockup::gen_mockup();
    Ok(state::stack::StateStack::new(
        NullState,
        state::Canvas::new()?,
        d,
        state::EventStream::new()?,
    ))
}
