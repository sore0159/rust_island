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
            crossterm::queue!(
                canvas.stdout,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(4, 4)
            )
            .unwrap();
            write!(canvas.stdout, "EVENT:{:?}", k).unwrap();
            crossterm::queue!(canvas.stdout, crossterm::cursor::MoveTo(4, 5)).unwrap();
            write!(canvas.stdout, "Press q to quit").unwrap();
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
