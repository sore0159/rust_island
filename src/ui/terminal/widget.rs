use crate::ui::terminal::{border, rect, style, text};
use style::Style;

pub struct WidgetData {
    pub rect: rect::Rect,
    pub borders: border::Borders,
    pub texts: Vec<((u16, u16), text::Text)>,
    pub updates: String,
    pub selected: Vec<((u16, u16), u16)>,
    pub b_styles: WidgetStyle,
    pub i_styles: WidgetStyle,
}

#[derive(Debug, Clone)]
pub struct WidgetStyle {
    pub plain: Style,
    pub focused: Style,
    pub selected: Style,
    pub f_and_s: Style,
}

impl Default for WidgetStyle {
    fn default() -> Self {
        WidgetStyle {
            plain: Style::default(),
            focused: Style::default(),
            selected: Style::default(),
            f_and_s: Style::default(),
        }
    }
}

impl WidgetData {
    pub fn new(r: rect::Rect) -> Self {
        WidgetData {
            rect: r,
            borders: border::Borders::new(),
            texts: Vec::with_capacity(3),
            updates: String::with_capacity(20),
            selected: Vec::new(),
            b_styles: Default::default(),
            i_styles: Default::default(),
        }
    }

    pub fn set_focus(&mut self, to_focused: bool) {
        //self.borders.mods.fg = self.focus_mods[if to_focused { 0 } else { 1 }].fg.clone();
        self.borders.draw(&mut self.rect);
    }
}
