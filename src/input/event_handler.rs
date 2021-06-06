use std::{cell::RefCell, collections::VecDeque};

use anyhow::Result;

use super::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::{
	display::{CrossTerm, Tui},
	input::{key_bindings::KeyBindings, InputOptions, MetaEvent},
};

pub struct EventHandler {
	event_provider: Box<dyn Fn() -> Result<Option<crossterm::event::Event>>>,
	event_queue: RefCell<VecDeque<Event>>,
	key_bindings: KeyBindings,
}

impl EventHandler {
	pub(crate) fn new(key_bindings: KeyBindings) -> Self {
		Self {
			event_provider: Box::new(CrossTerm::read_event),
			event_queue: RefCell::new(VecDeque::new()),
			key_bindings,
		}
	}

	#[cfg(test)]
	pub fn set_event_provider<F: 'static>(&mut self, event_provider: F)
	where F: Fn() -> Result<Option<crossterm::event::Event>> {
		self.event_provider = Box::new(event_provider);
	}

	pub fn poll_event(&self) -> Event {
		if let Some(event) = self.event_queue.borrow_mut().pop_front() {
			event
		}
		else if let Ok(Some(event)) = (self.event_provider)() {
			Event::from(event)
		}
		else {
			Event::None
		}
	}

	pub(crate) fn push_event(&self, event: Event) {
		self.event_queue.borrow_mut().push_back(event);
	}

	pub(crate) fn read_event<F>(&self, input_options: &InputOptions, callback: F) -> Event
	where F: FnOnce(Event, &KeyBindings) -> Event {
		let event = self.poll_event();
		if event == Event::None {
			return event;
		}

		if let Some(e) = Self::handle_standard_inputs(event) {
			return e;
		}

		if input_options.resize {
			if let Event::Resize(..) = event {
				return event;
			}
		}

		if input_options.movement {
			if let Some(evt) = Self::handle_movement_inputs(event) {
				return evt;
			}
		}

		if input_options.help && self.key_bindings.help.contains(&event) {
			return Event::from(MetaEvent::Help);
		}

		if input_options.undo_redo {
			if let Some(evt) = Self::handle_undo_redo(&self.key_bindings, event) {
				return evt;
			}
		}

		callback(event, &self.key_bindings)
	}

	fn handle_standard_inputs(event: Event) -> Option<Event> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(MetaEvent::Kill)),
			Event::Key(KeyEvent {
				code: KeyCode::Char('d'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(MetaEvent::Exit)),
			_ => None,
		}
	}

	fn handle_movement_inputs(event: Event) -> Option<Event> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Up,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollUp)),
			Event::Key(KeyEvent {
				code: KeyCode::Down,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollLeft)),
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollRight)),
			Event::Key(KeyEvent {
				code: KeyCode::PageUp,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollJumpUp)),
			Event::Key(KeyEvent {
				code: KeyCode::PageDown,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollJumpDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollTop)),
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollBottom)),
			_ => None,
		}
	}

	fn handle_undo_redo(key_bindings: &KeyBindings, event: Event) -> Option<Event> {
		if key_bindings.undo.contains(&event) {
			return Some(Event::from(MetaEvent::Undo));
		}
		else if key_bindings.redo.contains(&event) {
			return Some(Event::from(MetaEvent::Redo));
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;
	use rstest::rstest;

	use super::*;
	use crate::input::testutil::{create_event_handler, with_event_handler};

	#[test]
	fn poll_event_ok() {
		with_event_handler(&[Event::from('a')], |context| {
			assert_eq!(context.event_handler.poll_event(), Event::from('a'));
		});
	}

	#[test]
	fn poll_event_miss() {
		with_event_handler(&[], |context| {
			assert_eq!(context.event_handler.poll_event(), Event::None);
		});
	}

	#[test]
	fn poll_event_error() {
		let mut event_handler = create_event_handler();
		event_handler.set_event_provider(move || Err(anyhow!("Read Event Error")));
		assert_eq!(event_handler.poll_event(), Event::None);
	}

	#[rstest(
		event,
		handled,
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('c'),
			modifiers: KeyModifiers::CONTROL,
		}), true),
		case::resize(Event::Resize(100, 100), false),
		case::movement(Event::from(KeyCode::Up), false),
		case::help(Event::from('?'), false),
		case::undo_redo(Event::Key(KeyEvent {
			code: KeyCode::Char('z'),
			modifiers: KeyModifiers::CONTROL,
		}), false),
		case::other(Event::from('a'), false),
	)]
	fn read_event_options_disabled(event: Event, handled: bool) {
		with_event_handler(&[event], |context| {
			let result = context
				.event_handler
				.read_event(&InputOptions::new().resize(false), |_, _| Event::from(KeyCode::Null));

			if handled {
				assert_ne!(result, Event::from(KeyCode::Null));
			}
			else {
				assert_eq!(result, Event::from(KeyCode::Null));
			}
		});
	}

	#[rstest(
		event,
		handled,
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('c'),
			modifiers: KeyModifiers::CONTROL,
		}), true),
		case::resize(Event::Resize(100, 100), true),
		case::movement(Event::from(KeyCode::Up), true),
		case::help(Event::from('?'), true),
		case::undo_redo(Event::Key(KeyEvent {
			code: KeyCode::Char('z'),
			modifiers: KeyModifiers::CONTROL,
		}), true),
		case::other(Event::from('a'), false),
	)]
	fn read_event_enabled(event: Event, handled: bool) {
		with_event_handler(&[event], |context| {
			let result = context.event_handler.read_event(
				&InputOptions::new().movement(true).help(true).undo_redo(true),
				|_, _| Event::from(KeyCode::Null),
			);

			if handled {
				assert_ne!(result, Event::from(KeyCode::Null));
			}
			else {
				assert_eq!(result, Event::from(KeyCode::Null));
			}
		});
	}

	#[rstest(
		event,
		expected,
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('c'),
			modifiers: KeyModifiers::CONTROL,
		}), Event::from(MetaEvent::Kill)),
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('d'),
			modifiers: KeyModifiers::CONTROL,
		}), Event::from(MetaEvent::Exit)),
		case::other(Event::from('a'), Event::from(KeyCode::Null)),
	)]
	fn standard_inputs(event: Event, expected: Event) {
		with_event_handler(&[event], |context| {
			let result = context
				.event_handler
				.read_event(&InputOptions::new(), |_, _| Event::from(KeyCode::Null));

			assert_eq!(result, expected);
		});
	}

	#[rstest(
		event,
		expected,
		case::standard(Event::from(KeyCode::Up), Event::from(MetaEvent::ScrollUp)),
		case::standard(Event::from(KeyCode::Down), Event::from(MetaEvent::ScrollDown)),
		case::standard(Event::from(KeyCode::Left), Event::from(MetaEvent::ScrollLeft)),
		case::standard(Event::from(KeyCode::Right), Event::from(MetaEvent::ScrollRight)),
		case::standard(Event::from(KeyCode::PageUp), Event::from(MetaEvent::ScrollJumpUp)),
		case::standard(Event::from(KeyCode::PageDown), Event::from(MetaEvent::ScrollJumpDown)),
		case::standard(Event::from(KeyCode::Home), Event::from(MetaEvent::ScrollTop)),
		case::standard(Event::from(KeyCode::End), Event::from(MetaEvent::ScrollBottom)),
		case::other(Event::from('a'), Event::from(KeyCode::Null))
	)]
	fn movement_inputs(event: Event, expected: Event) {
		with_event_handler(&[event], |context| {
			let result = context
				.event_handler
				.read_event(&InputOptions::new().movement(true), |_, _| Event::from(KeyCode::Null));

			assert_eq!(result, expected);
		});
	}

	#[rstest(
		event,
		expected,
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('z'),
			modifiers: KeyModifiers::CONTROL,
		}), Event::from(MetaEvent::Undo)),
		case::standard(Event::Key(KeyEvent {
			code: KeyCode::Char('y'),
			modifiers: KeyModifiers::CONTROL,
		}), Event::from(MetaEvent::Redo)),
		case::other(Event::from('a'), Event::from(KeyCode::Null))
	)]
	fn undo_redo_inputs(event: Event, expected: Event) {
		with_event_handler(&[event], |context| {
			let result = context
				.event_handler
				.read_event(&InputOptions::new().undo_redo(true), |_, _| Event::from(KeyCode::Null));

			assert_eq!(result, expected);
		});
	}
}
