use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum MenuEvent {
    OpenFile,
    SaveAs,
    Quit,
    CloseWindow,
    Undefined,
}

impl Display for MenuEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for MenuEvent {
    fn from(value: String) -> Self {
        match value.as_str() {
            "OpenFile" => MenuEvent::OpenFile,
            "SaveAs" => MenuEvent::SaveAs,
            "CloseWindow" => MenuEvent::CloseWindow,
            "Quit" => MenuEvent::Quit,
            _ => MenuEvent::Undefined,
        }
    }
}
