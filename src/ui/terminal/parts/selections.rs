pub mod chooser;
pub mod sel;
pub mod widget;

pub use chooser::{Chooser, Config};
pub use sel::Selection;
pub use widget::BasicWidget;

use std::cell::Cell;
use std::rc::Rc;

pub struct Choice(Rc<Cell<Vec<usize>>>);

impl Choice {
    pub fn new() -> Self {
        Choice(Rc::new(Cell::new(Vec::new())))
    }
    pub fn push(&self, v: Vec<usize>) {
        self.0.set(v);
    }
    pub fn pop(&self) -> Vec<usize> {
        self.0.replace(Vec::new())
    }
    pub fn clone(&self) -> Self {
        Choice(self.0.clone())
    }
}
