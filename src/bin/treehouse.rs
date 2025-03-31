use treehouse::App;

fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .antialiasing(false)
        .scale_factor(App::scale_factor)
        .run_with(App::new)
}
