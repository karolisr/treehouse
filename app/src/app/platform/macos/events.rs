use crate::app::AppMsg;
use iced::{
    Subscription,
    futures::{
        channel::mpsc::{UnboundedSender, unbounded},
        stream::StreamExt,
    },
    task::{Never, Sipper, sipper},
};
use std::sync::OnceLock;

static SENDER: OnceLock<UnboundedSender<AppMsg>> = OnceLock::new();

pub fn send_os_event(app_msg: AppMsg) {
    SENDER
        .get()
        .expect("Failed to get 'sender': 'app::platform::macos::events::send_os_event'.")
        .unbounded_send(app_msg)
        .expect("Failed: 'app::platform::macos::events::send_os_event'.");
}

pub fn os_events() -> Subscription<AppMsg> {
    Subscription::run(macos_events_sipper)
}

fn macos_events_sipper() -> impl Sipper<Never, AppMsg> {
    sipper(async |mut output| {
        let (sender, mut receiver) = unbounded();
        SENDER.set(sender).expect("SENDER for os_events_sipper was set once previously.");
        loop {
            let app_msg = receiver.select_next_some().await;
            output.send(app_msg).await
        }
    })
}
