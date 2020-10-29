extern crate rand;
extern crate termion;

#[cfg(test)]
mod tests;

pub mod data;
pub mod state;
pub mod ui;

fn main() {
    let mut stack = ui::mockup::mock4::new_mock4();
    stack.run();
}
