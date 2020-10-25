use termion::terminal_size;

use crate::ui::terminal::{self, decorations, style, widget, Key};
use decorations::{
    border::{self, Bar},
    selections::{self, Choice, Selection},
    text::{self, Text},
    title::Flair,
};
use style::Color;
use terminal::SyncTermUI;
use widget::{data::WidgetData, textlog::TextLog, ui::WidgetUIBuilder};
pub fn new_mock3() -> impl SyncTermUI {
    let (w, h) = terminal_size().unwrap();
    // assumes 128, 54; maybe check?
    println!("Term Size: {}w,{}h", w, h);
    if w < 128 || h < 56 {
        panic!("Term not properly sized; need (128, 56), have {:?}", (w, h));
        //return format!("Term not properly sized; need (128, 56), have {:?}", (w, h));
    }
    let mut w1 = WidgetData::new((1, 1), 88, 30);
    let mut w3 = WidgetData::new((41, 31), 88, 26);
    let mut w4 = WidgetData::new((1, 31), 40, 26);
    w1.add_title("Box One", 1, 4, true, None);
    w3.add_title("Box Three", 23, 1, false, None);
    w3.add_title("Really Box Three", 30, 3, false, None);
    w3.add_title("For real", 10, 6, true, Some(Flair::VDiamond2));
    w3.add_title("I mean THREE REAL!", 40, 26, false, None);
    let i = w4.add_title("B4 Life", 40, 3, true, Some(Flair::VDiamond1));
    //w4.title.as_mut().unwrap().force_fg(250, 250, 250);
    w4.mod_title_flair(Flair::VDiamond1, i);
    w4.focusable = false;

    w1.borders.bars.push(Bar::new(25));
    w1.borders.add_bar(25, false, 0, 0);
    w1.set_border_rgb(000, 000, 250, true);

    let choice = selections::Choice::new();
    let mut s1 = Selection::new(Text::new("Choice 0", (3, 4)));

    s1.base_style[0].fg = Some(Color::Rgb(100, 0, 0));
    s1.base_style[1].fg = Some(Color::Rgb(250, 0, 0));

    s1.hover_style[0].fg = Some(Color::Rgb(250, 250, 250));
    s1.hover_style[0].bg = Some(Color::Rgb(0, 0, 100));
    s1.hover_style[1].fg = Some(Color::Rgb(250, 250, 250));
    s1.hover_style[1].bg = Some(Color::Rgb(0, 0, 250));

    s1.selected_style[0].fg = Some(Color::Rgb(250, 250, 250));
    s1.selected_style[0].bg = Some(Color::Rgb(100, 0, 0));
    s1.selected_style[1].fg = Some(Color::Rgb(250, 250, 250));
    s1.selected_style[1].bg = Some(Color::Rgb(250, 0, 0));

    s1.h_and_s_style[0].fg = Some(Color::Rgb(250, 250, 250));
    s1.h_and_s_style[0].bg = Some(Color::Rgb(100, 0, 100));

    s1.h_and_s_style[1].fg = Some(Color::Rgb(250, 250, 250));
    s1.h_and_s_style[1].bg = Some(Color::Rgb(250, 0, 250));
    let s2 = Selection::new(Text::new("Choice 1", (3, 5))).copy_styles(&s1);
    let s3 = Selection::new(Text::new("Choice 2", (3, 6))).copy_styles(&s1);

    let mut w2 = selections::BasicWidget::new(
        (89, 1),
        (40, 30),
        vec![s1, s2, s3],
        Key::Char(' '),
        choice.clone(),
    );
    w2.w_data
        .add_title("Box Two", 21, 30, false, Some(Flair::HDiamond2));
    w2.w_data.borders.add_bar(3, false, 0, 0);
    w2.w_data.set_bordertype(border::BorderType::Double, false);
    w2.w_data.set_border_rgb(250, 000, 000, true);
    w2.selections.hover_keys = [Some(Key::Up), Some(Key::Down), None];
    w2.selections.select_key = Some(Key::Char('\n'));

    //w2.selections.hover = Some(0);
    //w2.selections.options[0].selected = true;
    //w2.selections.hover_equals_select = true;
    w2.selections.can_multi_select = true;
    //w2.selections.can_zero_select = false;

    w3.borders.add_bar(3, false, 0, 0);
    w3.borders.add_bar(10, true, 2, 0);
    w4.borders.add_bar(5, false, 0, 0);
    w3.set_bordertype(border::BorderType::Rounded, false);
    w3.set_border_rgb(000, 250, 000, true);
    w4.set_bordertype(border::BorderType::Rounded, false);
    w4.set_border_rgb(200, 100, 200, false);

    let mut m = style::StyleMod::new();
    m.fg = Some(style::Color::Black);
    m.bg = Some(style::Color::White);
    //w1.new_text("Box One", (2, 2));
    let mut i = w2.w_data.new_text("    Box Two    ", (2, 2));
    w2.w_data.texts[i].style_mods = m.clone();

    let mut f3 = text::Fitter::default().middle();
    f3.val = '-';
    i = w3.new_text("12Box6Three23", (3, 2));
    w3.texts[i].fit(&f3, 20);

    let mut tl1 = TextLog::from_widgetdata(w1);
    tl1.padding = (1, 4);
    for _ in 0..10 {
        for line in mock_text().lines() {
            tl1.add_event(line);
        }
        //break;
    }
    tl1.add_event(mock_text().repeat(10));
    //tl1.add_event(mock_text());

    let mut builder = WidgetUIBuilder::new();
    builder.add_widget(tl1);
    builder.add_widget(w2);
    builder.add_widget(w3);
    builder.add_widget(w4);

    builder.quick_focus_keys = vec![
        Some(Key::Char('1')),
        Some(Key::Char('2')),
        Some(Key::Char('3')),
    ];
    //builder.cycle_focus_keys[2] = None;
    builder.cycle_focus_keys[2] = Some(Key::Char('\t'));
    builder.cycle_focus_keys[0] = Some(Key::Left);
    builder.cycle_focus_keys[1] = Some(Key::Right);

    let ui = builder.build(0);
    MockUI {
        ui,
        choice,
        count: [0, 0, 0],
    }
}

pub struct MockUI {
    ui: widget::ui::WidgetUI,
    choice: Choice,
    count: [usize; 3],
}

impl SyncTermUI for MockUI {
    fn to_draw(&self) -> &str {
        self.ui.to_draw()
    }
    fn parse(&mut self, key: Key) -> bool {
        let q = self.ui.parse(key);
        for j in 0..4 {
            println!(
                "{}                           ",
                termion::cursor::Goto(4, 36 + j as u16)
            );
        }
        for (j, i) in self.choice.retrieve().into_iter().enumerate() {
            self.count[i] += 1;
            println!(
                "{}MADE CHOICE {}; TIME:{}",
                termion::cursor::Goto(4, 36 + j as u16),
                i,
                self.count[i]
            );
        }
        q
    }
}

pub fn mock_text() -> &'static str {
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
}
