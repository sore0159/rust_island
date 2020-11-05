use crate::state::{self, Trans};
use crate::ui::c_term::{output, parts, widget, Key, KeyCode};
use output::new_rgb;
use parts::{
    border::{self, Bar},
    selections::{self, Choice, ChoiceBool, Chooser, Selection},
    text::{self, Text},
    title::Flair,
};
use std::io::Write;

use widget::{data::WidgetData, textlog::TextLog, w_state::WidgetStateBuilder};

pub fn new_mock5() -> state::StateStack {
    let mut w1 = WidgetData::new((0, 0), 88, 30);
    let mut w3 = WidgetData::new((40, 30), 88, 26);
    let mut w4 = WidgetData::new((0, 30), 40, 26);
    w1.add_title("Box One", 0, 3, true, None);
    w3.add_title("Box Three", 22, 0, false, None);
    w3.add_title("Really Box Three", 29, 2, false, None);
    w3.add_title("For real", 9, 5, true, Some(Flair::VDiamond2));
    w3.add_title("I mean THREE REAL!", 39, 25, false, None);
    let i = w4.add_title("B4 Life", 39, 2, true, Some(Flair::VDiamond1));
    //w4.title.as_mut().unwrap().force_fg(250, 250, 250);
    w4.mod_title_flair(Flair::VDiamond1, i);
    w4.focusable = false;

    w1.borders.bars.push(Bar::new(24));
    w1.borders.add_bar(24, false, 0, 0);
    //w1.set_border_rgb((000, 255, 255), (0, 0, 0), true);
    w1.set_border_rgb((0, 0, 255), (0, 0, 0), true);
    w1.set_border_rgb((150, 150, 255), (0, 0, 0), false);
    //w1.set_border_rgb((000, 000, 150), (0, 0, 0), false);

    let mut s1 = Selection::new(Text::new("Choice 0", (2, 3)));
    s1.base_no_f((100, 0, 0), (30, 0, 0));
    s1.base_f((250, 0, 0), (30, 0, 0));
    s1.hover_no_f((250, 250, 250), (0, 0, 100));
    s1.hover_f((250, 250, 250), (0, 0, 250));
    s1.selected_no_f((250, 250, 250), (100, 0, 0));
    s1.selected_f((250, 250, 250), (250, 0, 0));
    s1.h_and_s_no_f((250, 250, 250), (100, 0, 100));
    s1.h_and_s_f((250, 250, 250), (250, 0, 250));

    let s2 = Selection::new(Text::new("Choice 1", (2, 4))).copy_styles(&s1);
    let s3 = Selection::new(Text::new("Choice 2", (2, 5))).copy_styles(&s1);
    let mut conf = selections::Config::default();
    //conf.options[0].selected = true;
    //conf.hover_eq_select = true;
    conf.select_eq_submit = true;
    //conf.can_multi_select = true;
    conf.can_zero_select = false;
    let chooser = Chooser::new(vec![s1, s2, s3], conf)
        .prev_key(Key::from_code(KeyCode::Up))
        .next_key(Key::from_code(KeyCode::Down))
        //.start_selected(vec![1, 2])
        .start_hover(2);

    let mut w2 = selections::BasicWidget::new((88, 0), (40, 30), chooser);
    w2.w_data
        .add_title("Box Two", 21, 29, false, Some(Flair::HDiamond2));
    w2.w_data.borders.add_bar(2, false, 0, 0);
    w2.w_data.set_bordertype(border::BorderType::Double, false);
    w2.w_data.set_border_rgb((250, 000, 000), (30, 0, 0), true);
    w2.w_data.set_border_rgb((255, 255, 255), (30, 0, 0), false);
    w2.w_data
        .rect
        .all_cells(|cell| cell.base_bg = new_rgb(30, 0, 0));
    let choice = w2.clone_choice();

    w3.borders.add_bar(2, false, 0, 0);
    w3.borders.add_bar(9, true, 2, 0);
    w4.borders.add_bar(4, false, 0, 0);
    w3.set_bordertype(border::BorderType::Rounded, false);
    w3.set_border_rgb((000, 250, 000), (0, 30, 0), true);
    w3.set_border_rgb((255, 255, 255), (0, 30, 0), false);
    w3.rect.all_cells(|cell| cell.base_bg = new_rgb(0, 30, 0));
    w4.set_bordertype(border::BorderType::Rounded, false);
    w4.set_border_rgb((200, 100, 200), (0, 0, 0), false);

    let mut m = output::Style::new();
    m.foreground_color = Some(output::Color::Black);
    m.background_color = Some(output::Color::White);
    //w1.new_text("Box One", (2, 2));
    let mut i = w2.w_data.new_text("    Box Two    ", (1, 1));
    w2.w_data.texts[i].style_mods = m.clone();

    let mut f3 = text::Fitter::default().middle();
    f3.val = '-';
    i = w3.new_text("12Box6Three23", (2, 1));
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

    let mut builder = WidgetStateBuilder::new();
    builder.add_widget(tl1);
    builder.add_widget(w2);
    builder.add_widget(w3);
    builder.add_widget(w4);

    builder.quick_focus_keys = vec![
        Some(Key::from_char('1')),
        Some(Key::from_char('2')),
        Some(Key::from_char('3')),
    ];
    builder.quit_key = Some(Key::from_code(KeyCode::Esc));
    //builder.cycle_focus_keys[2] = None;
    builder.cycle_focus_keys[2] = Some(Key::from_code(KeyCode::Tab));
    builder.cycle_focus_keys[0] = Some(Key::from_code(KeyCode::Left));
    builder.cycle_focus_keys[1] = Some(Key::from_code(KeyCode::Right));

    let ui = builder.build(0);
    let s = MockState {
        ui,
        choice,
        quit_confirmed: ChoiceBool::new(),
        count: [0, 0, 0],
    };

    let c = state::Canvas::new().unwrap();
    let d = state::Data::new();
    let i = state::EventStream::new().unwrap();

    state::StateStack::new(s, c, d, i)
}

pub struct MockState {
    ui: widget::w_state::WidgetState,
    choice: Choice,
    quit_confirmed: ChoiceBool,
    count: [usize; 3],
}

impl state::State<state::Canvas, state::Data, state::Event> for MockState {
    fn on_start(&mut self, data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.ui.on_start(data, canvas)
    }
    fn on_resume(&mut self, data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        if let Some(true) = self.quit_confirmed.pop() {
            Trans::Quit
        } else {
            self.ui.on_resume(data, canvas)
        }
    }
    fn on_tic(&mut self, _data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        let (mut w, mut h) = canvas.stdout.get_size().unwrap();
        if w < 128 || h < 56 {
            if w < 128 {
                w = 128
            }
            if h < 56 {
                h = 56
            }
            canvas.stdout.set_size((w, h)).unwrap();
            self.ui.write_flush_full(&mut canvas.stdout).unwrap();
        }
        Trans::None
    }
    fn on_cycle(&mut self, data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.ui.on_cycle(data, canvas)
    }
    fn handle_event(
        &mut self,
        key: state::Event,
        data: &mut state::Data,
        canvas: &mut state::Canvas,
    ) -> Trans {
        let q = self.ui.handle_event(key, data, canvas);
        crossterm::queue!(canvas.stdout, crossterm::cursor::MoveTo(3, 38),).unwrap();
        println!("                            ");
        crossterm::queue!(canvas.stdout, crossterm::cursor::MoveTo(3, 38),).unwrap();
        println!("KEY:{:?} T:{:?}", key.0.code, q);
        for (j, i) in self.choice.pop().into_iter().enumerate() {
            self.count[i] += 1;
            crossterm::queue!(canvas.stdout, crossterm::cursor::MoveTo(3, 35 + j as u16),).unwrap();
            println!("MADE CHOICE {}; TIME:{}", i, self.count[i]);
        }
        match q {
            Trans::Quit => Trans::Push(Box::new(ConfirmState::new(
                "quit",
                (20, 20),
                (80, 7),
                self.quit_confirmed.clone(),
            ))),
            _ => q,
        }
    }
}

pub fn mock_text() -> &'static str {
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
}

pub struct ConfirmState {
    ui: widget::w_state::WidgetState,
    confirm_choice: Choice,
    confirmed: ChoiceBool,
}

impl ConfirmState {
    pub fn new(choice_desc: &str, origin: (u16, u16), size: (u16, u16), pipe: ChoiceBool) -> Self {
        //
        let mut conf = selections::Config::default();
        conf.hover_eq_select = true;
        conf.can_zero_select = false;
        conf.select_key = None;
        conf.submit_key = Some(Key::from_code(KeyCode::Enter));
        let mut t1 = Text::new(format!("Do you really want to {}?", choice_desc), (2, 2));
        t1.fit(&text::Fitter::default().middle(), size.0 - 2);
        let mut s1 = Selection::new(Text::new(" Yes ", ((size.0 - 2) / 4 - 1, 4)));
        s1.h_and_s_f((255, 255, 255), (255, 0, 0));
        let s2 = Selection::new(Text::new(" No ", ((size.0 - 2) * 3 / 4 - 3, 4))).copy_styles(&s1);
        let chooser = Chooser::new(vec![s1, s2], conf)
            .prev_key(Key::from_code(KeyCode::Left))
            .next_key(Key::from_code(KeyCode::Right))
            .start_hover(0);
        let mut w = selections::BasicWidget::new(origin, size, chooser);
        w.w_data.texts.push(t1);
        let choice = w.clone_choice();
        let mut builder = WidgetStateBuilder::new();
        //builder.quit_key = None;
        builder.add_widget(w);
        ConfirmState {
            ui: builder.build(0),
            confirm_choice: choice,
            confirmed: pipe,
        }
    }
}
impl state::State<state::Canvas, state::Data, state::Event> for ConfirmState {
    fn on_start(&mut self, data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.ui.on_start(data, canvas)
    }
    fn on_cycle(&mut self, data: &mut state::Data, canvas: &mut state::Canvas) -> Trans {
        self.ui.on_cycle(data, canvas)
    }
    fn handle_event(
        &mut self,
        key: state::Event,
        data: &mut state::Data,
        canvas: &mut state::Canvas,
    ) -> Trans {
        if key.code() == KeyCode::Esc {
            return Trans::Pop;
        }
        let q = self.ui.handle_event(key, data, canvas);
        for i in self.confirm_choice.pop() {
            if i == 0 {
                println!("CONFIRMED CHOICE");
                self.confirmed.push(Some(true));
            } else {
                println!("DID NOT CONFIRM CHOICE");
                self.confirmed.push(Some(false));
            }
            return Trans::Pop;
        }
        q
    }
}
