pub mod cell;
pub mod parts;
pub mod rect;
pub mod style;
pub mod widget;

pub use termion::event::Key;

use std::error::Error;
use std::io::{stdout, Write};
use std::iter::Iterator;

use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::{clear, cursor::HideCursor, AsyncReader};

pub type Stdout = HideCursor<RawTerminal<std::io::Stdout>>;
pub type AltStdout = HideCursor<AlternateScreen<RawTerminal<std::io::Stdout>>>;

pub fn new_stdout() -> Result<Stdout, Box<dyn Error>> {
    let stdout = stdout().into_raw_mode()?;
    let mut stdout = HideCursor::from(stdout);
    write!(stdout, "{}", clear::All)?;
    stdout.flush()?;

    Ok(stdout)
}

pub fn new_alt_stdout() -> Result<AltStdout, Box<dyn Error>> {
    let stdout = AlternateScreen::from(stdout().into_raw_mode()?);
    let mut stdout = HideCursor::from(stdout);
    write!(stdout, "{}", clear::All)?;
    stdout.flush()?;

    Ok(stdout)
}

pub struct Stdin(Keys<AsyncReader>);
//pub struct Stdin(mion::Bytes<AsyncReader>);

impl Stdin {
    pub fn new() -> Self {
        //Stdin(termion::async_stdin()) //.keys())
        Stdin(termion::async_stdin().bytes())
    }
}

impl Iterator for Stdin {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        let b = self.0.next();
        match b {
            Some(Err(e)) => panic!("stdin iter error {:?}", e),
            Some(Ok(k)) => {
                let e = termion::event::parse_event(k, &mut self.0);
                match e {
                    //Some(termion::event::Event::Key(x)) => Some(termion::event::Key(x)),
                    _ => None,
                }
            }
        }
    }
}
