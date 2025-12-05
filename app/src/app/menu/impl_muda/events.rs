use super::MenuItemId;
use crate::AppMsg;
use riced::{Never, Sipper, StreamExt, Subscription, sipper};

pub fn menu_events() -> Subscription<AppMsg> {
    Subscription::run(menu_events_sipper)
}

fn menu_events_sipper() -> impl Sipper<Never, AppMsg> {
    sipper(async |mut output| {
        let muda_receiver: &mut muda::MenuEventReceiver =
            muda::MenuEvent::receiver();
        loop {
            let event = muda_receiver.select_next_some().await;
            let mid: MenuItemId = String::from(&event.id().0).into();
            let app_msg = AppMsg::MenuEvent(mid);
            output.send(app_msg).await;
        }
    })
}
