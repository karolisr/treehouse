use super::super::{MenuEvent, MenuEventReplyMsg};
use crate::app::AppMsg;
use iced::{
    Subscription,
    futures::{
        SinkExt, Stream, StreamExt,
        channel::mpsc::{self, Sender},
    },
    stream,
};

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
