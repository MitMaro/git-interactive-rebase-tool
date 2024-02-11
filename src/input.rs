//! Git Interactive Rebase Tool - Input Module
//!
//! # Description
//! This module is used to handle working with input events.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod custom_event;
mod custom_key_binding;
mod event;
mod event_handler;
mod event_provider;
mod input_options;
mod key_bindings;
mod key_event;
mod standard_event;
#[cfg(not(tarpaulin_include))]
pub(crate) mod testutil;
mod thread;

pub(crate) use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind};

pub(crate) use self::{
	custom_event::CustomEvent,
	custom_key_binding::CustomKeybinding,
	event::Event,
	event_handler::EventHandler,
	event_provider::{read_event, EventReaderFn},
	input_options::InputOptions,
	key_bindings::{map_keybindings, KeyBindings},
	key_event::KeyEvent,
	standard_event::StandardEvent,
	thread::{State, Thread, THREAD_NAME},
};
