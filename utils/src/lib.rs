// -------------------------------------
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// -------------------------------------

mod colors;
mod math;
mod text_width;

pub use colors::Clr;
pub use math::lerp;
pub use text_width::text_width;
