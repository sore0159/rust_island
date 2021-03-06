use termion::event::Key;
use termion::terminal_size;

use crate::ui::terminal::{
    self,
    decorations::{border, text},
    rect, style,
};

use border::Borders;
use rect::Rect;
use terminal::SyncTermUI;
pub struct Mockup2 {
    pub b1: (Rect, Borders),
    pub b2: (Rect, Borders),
    pub b3: (Rect, Borders),
    pub b4: (Rect, Borders),
    selected: u8,
    pub update_str: String,
}

impl Mockup2 {
    pub fn new() -> Self {
        println!("{:?}", terminal_size().unwrap());
        // assumes 128, 54; maybe check?
        let mut rect1 = rect::Rect::new((1, 1), 88, 30);
        let mut bdr1 = border::Borders::new();
        let mut rect2 = rect::Rect::new((89, 1), 40, 30);
        let mut bdr2 = border::Borders::new();

        let mut rect4 = rect::Rect::new((1, 31), 40, 24);
        let mut bdr4 = border::Borders::new();
        let mut rect3 = rect::Rect::new((41, 31), 88, 24);
        let mut bdr3 = border::Borders::new();

        bdr1.h_bars.push(25);
        bdr1.mods.fg = Some(style::Color::Rgb(200, 100, 200));
        bdr1.mods.color_swap();
        bdr2.h_bars.push(5);
        bdr2.mods.fg = Some(style::Color::Blue);

        //bdr3.v_bars.push(15);
        bdr3.h_bars.push(3);
        bdr3.mods.fg = Some(style::Color::Green);
        bdr4.h_bars.push(5);

        let mut m = style::StyleMod::new();
        m.fg = Some(style::Color::Black);
        m.bg = Some(style::Color::White);
        rect1.apply_str((2, 2), "Box One", None);
        let mut t2 = text::Text::new("    Box Two    ", (2, 2));
        t2.style_mods = m.clone();

        let mut f3 = text::Fitter::default().middle();
        f3.val = '-';
        let mut t3 = text::Text::new("12Box6Three23", (3, 2));
        t3.fit(&f3, 20);
        //println!("{:?}", t3);
        t3.style_mods = m.clone();
        t3.start = (3, 2);
        rect2.apply_text(&t2);
        rect3.apply_text(&t3);
        bdr1.draw(&mut rect1);
        bdr2.draw(&mut rect2);
        bdr3.draw(&mut rect3);
        bdr4.draw(&mut rect4);
        let s = format!("{}{}{}{}", rect1, rect2, rect3, rect4);
        Mockup2 {
            b1: (rect1, bdr1),
            b2: (rect2, bdr2),
            b3: (rect3, bdr3),
            b4: (rect4, bdr4),
            selected: 0,
            update_str: s,
        }
    }
}

use std::fmt::Write;
impl SyncTermUI for Mockup2 {
    fn to_draw(&self) -> &str {
        &self.update_str
    }
    fn parse(&mut self, key: Key) -> bool {
        self.update_str.clear();
        match key {
            Key::Char('q') => true,
            Key::Char('n') => {
                self.selected = (self.selected + 1) % 3;
                match self.selected {
                    0 => {
                        self.b1.1.mods.color_swap();
                        self.b3.1.mods.color_swap();
                        self.b1.1.draw(&mut self.b1.0);
                        self.b3.1.draw(&mut self.b3.0);
                        write!(self.update_str, "{}{}", self.b1.0, self.b3.0).unwrap();
                    }
                    1 => {
                        self.b1.1.mods.color_swap();
                        self.b2.1.mods.color_swap();
                        self.b1.1.draw(&mut self.b1.0);
                        self.b2.1.draw(&mut self.b2.0);
                        write!(self.update_str, "{}{}", self.b1.0, self.b2.0).unwrap();
                    }
                    _ => {
                        self.b2.1.mods.color_swap();
                        self.b3.1.mods.color_swap();
                        self.b2.1.draw(&mut self.b2.0);
                        self.b3.1.draw(&mut self.b3.0);
                        write!(self.update_str, "{}{}", self.b2.0, self.b3.0).unwrap();
                    }
                }
                false
            }
            _ => false,
        }
    }
}
