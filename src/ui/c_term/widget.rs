pub mod data;
pub mod textlog;
pub mod w_state;

use super::Key;
use crate::ui::c_term::Stdout;

pub trait Widget {
    fn start(&mut self) -> bool;
    fn gain_focus(&mut self);
    fn lose_focus(&mut self);
    fn parse(&mut self, key: Key) -> bool;
    fn queue_write(&mut self, s: &mut Stdout) -> anyhow::Result<()>;
}
