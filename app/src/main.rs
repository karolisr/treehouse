#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// -------------------------------------
// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(clippy::single_match)]
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(clippy::needless_range_loop)]
// -------------------------------------

mod app;
use app::App;

fn main() -> riced::IcedResult {
    // tracing_subscriber::fmt::init();
    riced::daemon(App::boot, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .scale_factor(App::scale_factor)
        .theme(App::theme)
        .settings(App::settings())
        .run()
}
