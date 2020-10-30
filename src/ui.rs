pub mod mockup;
pub mod terminal;

use std::error::Error;

pub struct Canvas {
    //pub stdout: terminal::Stdout,
    pub stdout: terminal::AltStdout,
}

impl Canvas {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        //let stdout = terminal::new_stdout()?;
        let stdout = terminal::new_alt_stdout()?;
        Ok(Canvas { stdout: stdout })
    }
}

pub type Event = termion::event::Key;

pub struct EventStream {
    pub stdin: terminal::Stdin,
}

impl EventStream {
    pub fn new() -> Result<EventStream, Box<dyn Error>> {
        let stdin = terminal::Stdin::new();
        Ok(EventStream { stdin: stdin })
    }
}

impl Iterator for EventStream {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        self.stdin.next()
    }
}
