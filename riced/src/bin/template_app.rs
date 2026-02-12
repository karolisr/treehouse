use riced::{
    Alignment, BTN_H1, Clr, Element, Horizontal, IcedResult, Key,
    KeyboardEvent, Length, Modifiers, SF, Size, Subscription, Theme, Vertical,
    WindowSettings, btn_txt, center, iced_col, iced_row, keyboard_events,
    txt_i64, txt_input,
};

const PADDING: f32 = 3e1 * SF;
const MIN_WIN_DIM: f32 = BTN_H1 * 2e0 + PADDING * 5e0;
const MIN_WIN_SIZE: Size = Size { width: MIN_WIN_DIM, height: MIN_WIN_DIM };

#[derive(Debug)]
struct App {
    theme: Theme,
    title: String,
    counter: i64,
    explain: bool,
}

#[derive(Debug, Clone)]
enum Msg {
    SetTitle(String),
    Decrement,
    Increment,
    OnKeyPress(Key, Modifiers),
    Other(Option<String>),
}

impl App {
    fn view(&self) -> Element<'_, Msg> {
        match self.explain {
            true => Element::explain(view(self), Clr::RED),
            false => view(self),
        }
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SetTitle(title) => self.title = title,
            Msg::Decrement => self.counter -= 1,
            Msg::Increment => self.counter += 1,
            Msg::OnKeyPress(key, modifiers) => {
                handle_keyboard_press_events(self, key, modifiers);
            }
            Msg::Other(opt_msg) => {
                if let Some(msg) = opt_msg {
                    println!("Msg::Other({msg})");
                }
            }
        }
    }

    fn boot() -> Self {
        Self {
            theme: Theme::Light,
            title: "".to_string(),
            counter: 0,
            explain: false,
        }
    }

    fn scale_factor(&self) -> f32 {
        1e0 / SF
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> Subscription<Msg> {
        let mut subs: Vec<Subscription<Msg>> = Vec::with_capacity(4);
        subs.push(keyboard_events().map(|e| match e {
            KeyboardEvent::KeyPressed { key, modifiers, .. } => {
                Msg::OnKeyPress(key, modifiers)
            }
            _ => Msg::Other(None),
        }));
        Subscription::batch(subs)
    }
}

fn handle_keyboard_press_events(app: &mut App, key: Key, modifiers: Modifiers) {
    if let Key::Character(char) = key {
        let c = char.as_str();
        if modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT) {
            if c == "e" {
                app.explain = !app.explain;
            }
        } else {
            match c {
                "-" => app.counter -= 1,
                "=" => app.counter += 1,
                _ => {}
            }
        }
    }
}

fn view(app: &App) -> Element<'_, Msg> {
    center(
        iced_col![
            txt_input(
                "Window Title",
                &app.title,
                "txt_input_title",
                Msg::SetTitle
            )
            .width(Length::Fixed(MIN_WIN_DIM - PADDING * 2e0))
            .align_x(Horizontal::Center),
            iced_row![
                btn_txt("-", Some(Msg::Decrement)),
                txt_i64(app.counter)
                    .width(Length::Fixed(PADDING))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                btn_txt("+", Some(Msg::Increment)),
            ]
            .align_y(Vertical::Center)
            .spacing(PADDING),
            btn_txt(
                "Clear Window Title",
                match app.title.as_str() {
                    "" => None,
                    _ => Some(Msg::SetTitle("".to_string())),
                }
            )
            .width(Length::Fixed(MIN_WIN_DIM - PADDING * 2e0)),
        ]
        .align_x(Horizontal::Center)
        .spacing(PADDING),
    )
    .into()
}

fn main() -> IcedResult {
    iced::application(App::boot, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .antialiasing(true)
        .scale_factor(App::scale_factor)
        .window(WindowSettings {
            size: MIN_WIN_SIZE,
            min_size: Some(MIN_WIN_SIZE),
            position: riced::WindowPosition::Centered,
            ..Default::default()
        })
        .subscription(App::subscription)
        .run()
}
