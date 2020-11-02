pub mod output;
pub mod parts;
pub mod widget;

//use anyhow::Result;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Key(pub crossterm::event::KeyEvent);
pub type KeyCode = crossterm::event::KeyCode;

impl Key {
    pub fn is_char(&self, c: char) -> bool {
        self.0.code == KeyCode::Char(c)
    }
    pub fn code(&self) -> KeyCode {
        self.0.code
    }
    pub fn from_char(c: char) -> Self {
        Key(crossterm::event::KeyEvent::new(
            KeyCode::Char(c),
            crossterm::event::KeyModifiers::empty(),
        ))
    }
    pub fn from_code(c: KeyCode) -> Self {
        Key(crossterm::event::KeyEvent::new(
            c,
            crossterm::event::KeyModifiers::empty(),
        ))
    }
}

use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
};

pub struct Stdout {
    pub stdout: std::io::Stdout,
    pub alt: bool,
}

impl Stdout {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
        terminal::enable_raw_mode()?;
        let s = Stdout { stdout, alt: true };
        Ok(s)
    }
    pub fn quit_cleanup(&mut self) -> anyhow::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen, Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn set_size(&mut self, size: (u16, u16)) -> anyhow::Result<()> {
        execute!(self.stdout, SetSize(size.0, size.1))?;
        Ok(())
    }
    pub fn get_size(&self) -> anyhow::Result<(u16, u16)> {
        Ok(terminal::size()?)
    }
    pub fn to_alt_screen(&mut self) -> anyhow::Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        self.alt = true;
        Ok(())
    }
    pub fn to_main_screen(&mut self) -> anyhow::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        self.alt = false;
        Ok(())
    }
    pub fn to_raw(&mut self) -> anyhow::Result<()> {
        Ok(terminal::enable_raw_mode()?)
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for Stdout {
    fn drop(&mut self) {
        self.quit_cleanup().expect("error in stdout cleanup oh no!");
    }
}

pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin
    }
}

use crossterm::event::{self, poll, read};
use std::time::Duration;
impl Iterator for Stdin {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        loop {
            match poll(Duration::from_secs(0)) {
                Ok(true) => {
                    match read() {
                        Err(er) => panic!("crossterm read error {}", er),
                        Ok(event::Event::Key(k)) => {
                            return Some(Key(k));
                            //
                        }
                        _ => {}
                    }
                }
                Err(er) => panic!("Crossterm poll error {}", er),
                _ => return None,
            }
        }
    }
}
