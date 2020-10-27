extern crate rand;
extern crate termion;

#[cfg(test)]
mod tests;

pub mod data;
pub mod state;
pub mod ui;

fn main() {
    //let g = data::mockup::gen_mockup();
    //let mock = ui::terminal::mockup::Mockup2::new();
    let mock = ui::mockup::new_mock3();
    ui::terminal::run_ui(mock).unwrap()
}
