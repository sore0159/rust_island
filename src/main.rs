extern crate anyhow;
extern crate crossterm;
extern crate rand;

#[cfg(test)]
mod tests;

pub mod data;
pub mod state;
pub mod ui;

use anyhow::Result;
//use crossterm::ExecutableCommand;

fn main() -> Result<()> {
    let mut stack = ui::mockup::mock5::new_mock5();
    stack.run();
    Ok(())
}

use std::io::Write;
pub fn emergency_goto(x: u16, y: u16) {
    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y)).unwrap();
}
