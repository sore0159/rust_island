use std::fmt::Write;
use termion::color;

#[derive(PartialEq, Clone)]
pub struct Style {
    pub deco: Decoration,
    pub fg: Color,
    pub bg: Color,
}

impl Style {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            deco: Decoration::None,
            fg: Color::White,
            bg: Color::Black,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Decoration {
    None,
    Bold,
    Italic,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
}

type WriteErr = std::result::Result<(), std::fmt::Error>;

impl Decoration {
    pub fn start(&self, mut w: impl Write) -> WriteErr {
        match self {
            Decoration::None => Ok(()),
            Decoration::Bold => write!(w, "{}", termion::style::Bold),
            Decoration::Italic => write!(w, "{}", termion::style::Italic),
        }
    }

    pub fn stop(&self, mut w: impl Write) -> WriteErr {
        match self {
            Decoration::None => Ok(()),
            Decoration::Bold => write!(w, "{}", termion::style::NoBold),
            Decoration::Italic => write!(w, "{}", termion::style::NoItalic),
        }
    }
}

impl Color {
    pub fn start_fg(&self, mut w: impl Write) -> WriteErr {
        match self {
            Color::Black => write!(w, "{}", color::Fg(color::Black)),
            Color::White => write!(w, "{}", color::Fg(color::White)),
            Color::Red => write!(w, "{}", color::Fg(color::Red)),
            Color::Blue => write!(w, "{}", color::Fg(color::Blue)),
            Color::Green => write!(w, "{}", color::Fg(color::Green)),
        }
    }
    pub fn start_bg(&self, mut w: impl Write) -> WriteErr {
        match self {
            Color::Black => write!(w, "{}", color::Bg(color::Black)),
            Color::White => write!(w, "{}", color::Bg(color::White)),
            Color::Red => write!(w, "{}", color::Bg(color::Red)),
            Color::Blue => write!(w, "{}", color::Bg(color::Blue)),
            Color::Green => write!(w, "{}", color::Bg(color::Green)),
        }
    }
}

#[derive(Clone)]
pub struct StyleMod {
    pub deco: Option<Decoration>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl StyleMod {
    pub fn new() -> Self {
        StyleMod {
            deco: None,
            fg: None,
            bg: None,
        }
    }
    pub fn apply(&self, style: &mut Style) {
        if let Some(s) = &self.deco {
            style.deco = s.clone();
        }
        if let Some(cl) = &self.fg {
            style.fg = cl.clone();
        }
        if let Some(cl) = &self.bg {
            style.bg = cl.clone();
        }
    }
    pub fn color_swap(&mut self) {
        let holder = self.fg.clone();
        self.fg = self.bg.clone();
        self.bg = holder;
    }
}

impl Default for StyleMod {
    fn default() -> Self {
        StyleMod {
            deco: Some(Decoration::None),
            fg: Some(Color::White),
            bg: Some(Color::Black),
        }
    }
}
