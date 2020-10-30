pub mod cell;
pub mod parts;
pub mod rect;
pub mod style;
pub mod widget;

pub use termion::event::Key;

use std::error::Error;
use std::io::{stdout, Read, Write};
use std::iter::Iterator;

//use termion::input::{Keys, TermRead};
//use termion::input::TermRead;
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

//pub struct Stdin(Keys<AsyncReader>);
pub struct Stdin(std::io::Bytes<AsyncReader>);

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
            Some(Err(er)) => panic!("stdin iter error: {:?}", er),
            Some(Ok(k)) => {
                let e = termion::event::parse_event(k, &mut self.0);
                match e {
                    Ok(termion::event::Event::Key(x)) => Some(x),
                    //Err(er) => panic!("stdin iter inner error: {:?}", er),
                    _ => {
                        //if let Some(Ok(x)) = b {
                        //if x == 27 {
                        //return Some(Key::Esc);
                        //}
                        //}
                        //println!("OTHER EVENT: {:?}", k);
                        None
                    } //_ => None,
                }
            }
            _ => None,
        }
    }
}
