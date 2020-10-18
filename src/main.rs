extern crate rand;
extern crate termion;

#[cfg(test)]
mod tests;

pub mod data;
pub mod ui;

fn main() {
    //let g = data::mockup::gen_mockup();
    ui::terminal::mockup::draw_boxes().unwrap();
}
