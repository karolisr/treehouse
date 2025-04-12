#![allow(unused_imports, unused_mut, unused_variables)]
use super::super::{MenuEvent, MenuEventReplyMsg};
use crate::AppMsg;

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
    stream::channel(0, async |mut output: Sender<AppMsg>| {})
}
