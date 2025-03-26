mod main;
pub use main::{MainWin, MainWinMsg, main_win_settings};

pub enum AppWin {
    MainWin(Box<MainWin>),
}
