mod app_key_bindings;
mod meta_event;

pub(crate) use self::{app_key_bindings::AppKeyBindings, meta_event::MetaEvent};

pub(crate) type KeyBindings = input::KeyBindings<AppKeyBindings, MetaEvent>;
pub(crate) type Event = input::Event<MetaEvent>;
