pub mod cell;
pub mod decorations;
pub mod rect;
pub mod style;
pub mod widget;

pub use termion::event::Key;

pub trait SyncTermUI {
    fn to_draw(&self) -> &str;
    fn parse(&mut self, key: Key) -> bool;
}

use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor::HideCursor};
pub fn run_ui(mut ui: impl SyncTermUI) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = AlternateScreen::from(stdout().into_raw_mode()?);
    let mut stdout = HideCursor::from(stdout);
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

pub type FailString = String;

impl SyncTermUI for FailString {
    fn to_draw(&self) -> &str {
        println!("{}", self);
        ""
    }
    fn parse(&mut self, _key: Key) -> bool {
        return true;
    }
}
