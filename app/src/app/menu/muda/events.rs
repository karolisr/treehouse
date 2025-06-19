use super::AppMenuItemId;
use crate::app::AppMsg;
use riced::{Never, Sipper, StreamExt, Subscription, sipper};

pub fn menu_events() -> Subscription<AppMsg> { Subscription::run(menu_events_sipper) }

fn menu_events_sipper() -> impl Sipper<Never, AppMsg> {
    sipper(async |mut output| {
        let muda_receiver: &mut muda::MenuEventReceiver = muda::MenuEvent::receiver();
        loop {
            let event = muda_receiver.select_next_some().await;
            let menu_event: AppMenuItemId = String::from(&event.id().0).into();
            let app_msg = AppMsg::MenuEvent(menu_event);
            output.send(app_msg).await;
        }
    })
}
