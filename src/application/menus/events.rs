use std::fmt::Display;

use crate::AppMsg;
use iced::Subscription;
use iced::futures::SinkExt;
use iced::futures::Stream;
use iced::futures::StreamExt;
use iced::futures::channel::mpsc;
use iced::futures::channel::mpsc::Sender;
use iced::stream;

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

pub fn menu_events() -> Subscription<AppMsg> {
    Subscription::run(menu_events_stream)
}

fn menu_events_stream() -> impl Stream<Item = AppMsg> {
    stream::channel(0, async |mut output: Sender<AppMsg>| {
        let (sender, mut receiver) = mpsc::channel::<MenuEventReplyMsg>(0);
        let _ = output.send(AppMsg::MenuEventsSender(sender)).await;
        loop {
            if let Some(MenuEventReplyMsg::Ack) = receiver.next().await {
                break;
            };
        }

        let mut termination_requested = false;
        loop {
            match termination_requested {
                true => termination_requested = false,
                false => {
                    if let Ok(event) = muda::MenuEvent::receiver().recv() {
                        let menu_event_str: &str = &event.id().0;
                        let menu_event: MenuEvent = String::from(menu_event_str).into();
                        termination_requested = matches!(
                            menu_event,
                            MenuEvent::Quit | MenuEvent::CloseWindow | MenuEvent::QuitInternal
                        );
                        let _ = output.send(AppMsg::MenuEvent(menu_event)).await;
                    }
                }
            }

            let event = Some(receiver.select_next_some().await);
            if let Some(MenuEventReplyMsg::Terminate) = event {
                let _ = output.send(AppMsg::TerminationConfirmed).await;
                break;
            };
        }
    })
}
