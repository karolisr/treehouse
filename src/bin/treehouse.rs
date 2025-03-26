use treehouse::App;

fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .antialiasing(false)
        .scale_factor(|_, _| 1.0)
        .run_with(App::new)
}
