use super::events::MenuEvent;
use crate::app::AppMsg;
use iced::{
    Subscription,
    futures::stream::StreamExt,
    task::{Never, Sipper, sipper},
};

pub fn menu_events() -> Subscription<AppMsg> {
    Subscription::run(menu_events_sipper)
}

fn menu_events_sipper() -> impl Sipper<Never, AppMsg> {
    sipper(async |mut output| {
        let muda_receiver: &mut muda::MenuEventReceiver = muda::MenuEvent::receiver();
        loop {
            let event = muda_receiver.select_next_some().await;
            let menu_event_str: &str = &event.id().0;
            let menu_event: MenuEvent = String::from(menu_event_str).into();
            let app_msg = AppMsg::MenuEvent(menu_event);
            output.send(app_msg).await;
        }
    })
}
