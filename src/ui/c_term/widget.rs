pub mod data;
pub mod textlog;
pub mod w_state;

use super::Key;

pub trait Widget {
    fn start(&mut self) -> (&str, bool);
    fn gain_focus(&mut self) -> &str;
    fn lose_focus(&mut self) -> &str;
    fn parse(&mut self, key: Key) -> &str;
}
