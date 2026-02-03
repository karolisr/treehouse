// -------------------------------------
#![allow(dead_code)]
#![allow(clippy::excessive_precision)]
// -------------------------------------

use crate::Float;
use riced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub(crate) enum GtsRank {
    Eon,
    Era,
    Period,
    SubPeriod,
    Epoch,
    Age,
}

#[derive(Debug)]
pub(crate) struct GtsRecord {
    pub broader: Option<String>,
    pub name: String,
    pub rank: GtsRank,
    pub beg: Float,
    pub end: Float,
    pub beg_margin_of_error: Option<Float>,
    pub end_margin_of_error: Option<Float>,
    pub color: Color,
}

include!(concat!(env!("OUT_DIR"), "/gts.rs"));
