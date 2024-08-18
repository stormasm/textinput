mod colors;
mod event;
mod icon;
mod styled;

pub mod button;
pub mod checkbox;
pub mod divider;
pub mod history;
pub mod indicator;
pub mod input;
pub mod label;
pub mod prelude;
pub mod scroll;
pub mod tab;
pub mod theme;
pub mod tooltip;

// re-export
pub use wry;

pub use crate::Disableable;
pub use colors::*;
pub use event::InteractiveElementExt;
pub use icon::*;
pub use styled::*;

/// Initialize the UI module.
pub fn init(cx: &mut gpui::AppContext) {
    input::init(cx);
}
