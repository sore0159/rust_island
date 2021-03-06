use super::super::{output::Rect, output::Style, parts, Key};
use super::Widget;
use border::BorderType;
use parts::{border, text, title};

pub struct WidgetData {
    pub rect: Rect,
    pub borders: border::Borders,
    pub titles: Vec<title::Title>,
    pub texts: Vec<text::Text>,
    pub updates: String,
    pub selected: Vec<((u16, u16), u16)>,
    pub b_styles: [(Style, BorderType); 2],
    //pub i_styles: WidgetStyle,
    pub focusable: bool,
}

impl WidgetData {
    pub fn new(origin: (u16, u16), w: u16, h: u16) -> Self {
        WidgetData::new_with_rect(Rect::new(origin, (w, h)))
    }
    pub fn new_with_rect(r: Rect) -> Self {
        WidgetData {
            rect: r,
            borders: border::Borders::default(),
            titles: Vec::new(),
            texts: Vec::with_capacity(3),
            updates: String::with_capacity(20),
            selected: Vec::new(),
            b_styles: [
                (Default::default(), BorderType::Basic),
                (Default::default(), BorderType::Thick),
            ],
            //i_styles: Default::default(),
            focusable: true,
        }
    }

    pub fn apply_border(&mut self) {
        self.borders.draw(&mut self.rect);
        for t in &self.titles {
            t.draw(&mut self.rect, &self.borders.mods);
        }
    }
    pub fn apply_texts(&mut self) {
        for t in &self.texts {
            t.draw(&mut self.rect)
            //self.rect.apply_text(t);
        }
    }
    pub fn set_focus(&mut self, to_focused: bool) {
        let sty = if to_focused {
            &self.b_styles[1]
        } else {
            &self.b_styles[0]
        };
        self.borders.mods = sty.0.clone();
        self.borders.chartype = sty.1.clone();
        self.apply_border();
    }

    pub fn set_border_rgb(&mut self, fg: (u8, u8, u8), bg: (u8, u8, u8), focus: bool) {
        let mut sty = &mut if focus {
            &mut self.b_styles[1]
        } else {
            &mut self.b_styles[0]
        }
        .0;
        sty.foreground_color = Some(fg.into());
        sty.background_color = Some(bg.into());
    }
    pub fn set_bordertype(&mut self, chartype: border::BorderType, focus: bool) {
        if focus {
            &mut self.b_styles[1]
        } else {
            &mut self.b_styles[0]
        }
        .1 = chartype;
    }

    pub fn new_text(&mut self, val: impl Into<String>, start: (u16, u16)) -> usize {
        self.texts.push(text::Text::new(val, start));
        self.texts.len() - 1
    }

    pub fn add_title(
        &mut self,
        s: impl Into<String>,
        x: u16,
        y: u16,
        vert: bool,
        flair: Option<title::Flair>,
    ) -> usize {
        if let Some(f) = flair {
            self.titles
                .push(title::Title::new(s, x, y, vert).with_flair(f));
        } else {
            self.titles.push(title::Title::new(s, x, y, vert));
        }
        self.titles.len() - 1
    }
    pub fn mod_title_flair(&mut self, f: title::Flair, i: usize) {
        self.titles[i].flair = f
    }
    pub fn mod_title_str(&mut self, s: &str, i: usize) {
        self.titles[i].text.val = s.to_string();
    }
}

impl Widget for WidgetData {
    fn start(&mut self) -> bool {
        self.set_focus(false);
        for text in &self.texts {
            text.draw(&mut self.rect)
        }
        self.focusable
    }
    fn gain_focus(&mut self) {
        self.set_focus(true);
    }
    fn lose_focus(&mut self) {
        self.set_focus(false);
    }
    fn parse(&mut self, _key: Key) -> bool {
        false
    }
    fn queue_write(&mut self, w: &mut crate::ui::c_term::Stdout) -> anyhow::Result<()> {
        self.rect.queue_write(w)
    }
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
