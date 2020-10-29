pub mod stack;
pub mod traits;

pub use traits::State;

use crate::{data, ui};
pub use data::Data;
pub use ui::Canvas;
pub use ui::Event;
pub use ui::EventStream;

pub type Trans = traits::Trans<Canvas, Data, Event>;

pub type StateStack = stack::StateStack<Canvas, Data, Event, EventStream>;
