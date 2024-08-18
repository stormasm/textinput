mod colors;
mod event;
mod focusable;
mod icon;
mod root;
mod styled;

pub mod button;
pub mod checkbox;
pub mod divider;
pub mod drawer;
pub mod history;
pub mod indicator;
pub mod input;
pub mod label;
pub mod list;
pub mod modal;
pub mod notification;
pub mod popover;
pub mod popup_menu;
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
pub use focusable::FocusableCycle;
pub use icon::*;
pub use root::{ContextModal, Root};
pub use styled::*;

/// Initialize the UI module.
pub fn init(cx: &mut gpui::AppContext) {
    input::init(cx);
    list::init(cx);
    popover::init(cx);
    popup_menu::init(cx);
}

rust_i18n::i18n!("locales", fallback = "en");
use std::ops::Deref;
pub fn locale() -> impl Deref<Target = str> {
    rust_i18n::locale()
}

pub fn set_locale(locale: &str) {
    rust_i18n::set_locale(locale)
}
