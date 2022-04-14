mod action_event;
mod app_key_bindings;

pub(crate) use self::{action_event::MetaEvent, app_key_bindings::AppKeyBindings};

pub(crate) type KeyBindings = input::KeyBindings<AppKeyBindings, MetaEvent>;
pub(crate) type Event = input::Event<MetaEvent>;
