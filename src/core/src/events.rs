mod app_key_bindings;
mod meta_event;

pub(crate) use self::{app_key_bindings::AppKeyBindings, meta_event::MetaEvent};
pub(crate) type KeyBindings = crate::input::KeyBindings<AppKeyBindings, MetaEvent>;
pub(crate) type Event = crate::input::Event<MetaEvent>;
pub(crate) type State = crate::input::State<MetaEvent>;
pub(crate) type Thread<EventProvider> = crate::input::Thread<EventProvider, MetaEvent>;
