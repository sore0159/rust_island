pub mod chooser;
pub mod sel;
pub mod widget;

pub use chooser::{Chooser, Config};
pub use sel::Selection;
pub use widget::BasicWidget;

pub type Choice = choice::Choice<Vec<usize>>;
pub type ChoiceBool = choice::Choice<Option<bool>>;

pub mod choice {
    use std::cell::Cell;
    use std::rc::Rc;
    pub struct Choice<T: Default>(Rc<Cell<T>>);

    impl<T: Default> Choice<T> {
        pub fn new() -> Self {
            Choice(Rc::new(Cell::new(T::default())))
        }
        pub fn push(&self, v: T) {
            self.0.set(v);
        }
        pub fn pop(&self) -> T {
            self.0.replace(T::default())
        }
        pub fn clone(&self) -> Self {
            Choice(self.0.clone())
        }
    }
}
