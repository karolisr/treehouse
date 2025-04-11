use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum MenuEvent {
    OpenFile,
    Save,
    Quit,
    CloseWindow,
    QuitInternal,
    Undefined(String),
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
            "Save" => MenuEvent::Save,
            "CloseWindow" => MenuEvent::CloseWindow,
            "Quit" => MenuEvent::Quit,
            "QuitInternal" => MenuEvent::QuitInternal,
            s => MenuEvent::Undefined(s.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MenuEventReplyMsg {
    Ack,
    Terminate,
}
