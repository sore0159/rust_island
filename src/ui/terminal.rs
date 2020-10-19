pub mod border;
pub mod cell;
pub mod mockup;
pub mod rect;
pub mod style;
pub mod text;
pub mod widget;

pub use termion::event::Key;

pub trait SyncTermUI {
    fn to_draw(&self) -> &str;
    fn parse(&mut self, key: Key) -> bool;
}

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
pub fn run_ui(mut ui: impl SyncTermUI) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode()?);
    write!(stdout, "{}", clear::All)?;
    write!(stdout, "{}", ui.to_draw())?;
    stdout.flush()?;
    let stdin = stdin();
    for c in stdin.keys() {
        if ui.parse(c?) {
            break;
        }
        write!(stdout, "{}", ui.to_draw())?;
        stdout.flush()?;
    }
    println!("");
    Ok(())
}

//impl std::fmt::Display for T {
//fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
