//! Utilities for writing tests that interact with input events.

use super::{Event, EventHandler, KeyBindings, KeyCode, KeyEvent, KeyModifiers};
use crate::{event_action::EventAction, Sender};

/// Create a mocked version of `KeyBindings`.
#[inline]
#[must_use]
pub fn create_test_keybindings() -> KeyBindings {
	KeyBindings {
		abort: vec![Event::from(KeyCode::Char('q'))],
		action_break: vec![Event::from(KeyCode::Char('b'))],
		action_drop: vec![Event::from(KeyCode::Char('d'))],
		action_edit: vec![Event::from(KeyCode::Char('e'))],
		action_fixup: vec![Event::from(KeyCode::Char('f'))],
		action_pick: vec![Event::from(KeyCode::Char('p'))],
		action_reword: vec![Event::from(KeyCode::Char('r'))],
		action_squash: vec![Event::from(KeyCode::Char('s'))],
		confirm_yes: vec![Event::from(KeyCode::Char('y'))],
		edit: vec![Event::from(KeyCode::Char('E'))],
		force_abort: vec![Event::from(KeyCode::Char('Q'))],
		force_rebase: vec![Event::from(KeyCode::Char('W'))],
		help: vec![Event::from(KeyCode::Char('?'))],
		insert_line: vec![Event::from(KeyCode::Char('I'))],
		move_down: vec![Event::from(KeyCode::Down)],
		move_down_step: vec![Event::from(KeyCode::PageDown)],
		move_end: vec![Event::from(KeyCode::End)],
		move_home: vec![Event::from(KeyCode::Home)],
		move_left: vec![Event::from(KeyCode::Left)],
		move_right: vec![Event::from(KeyCode::Right)],
		move_selection_down: vec![Event::from(KeyCode::Char('j'))],
		move_selection_up: vec![Event::from(KeyCode::Char('k'))],
		move_up: vec![Event::from(KeyCode::Up)],
		move_up_step: vec![Event::from(KeyCode::PageUp)],
		open_in_external_editor: vec![Event::from(KeyCode::Char('!'))],
		rebase: vec![Event::from(KeyCode::Char('w'))],
		redo: vec![Event::Key({
			KeyEvent {
				code: KeyCode::Char('y'),
				modifiers: KeyModifiers::CONTROL,
			}
		})],
		remove_line: vec![Event::from(KeyCode::Delete)],
		show_commit: vec![Event::from(KeyCode::Char('c'))],
		show_diff: vec![Event::from(KeyCode::Char('d'))],
		toggle_visual_mode: vec![Event::from(KeyCode::Char('v'))],
		undo: vec![Event::Key({
			KeyEvent {
				code: KeyCode::Char('z'),
				modifiers: KeyModifiers::CONTROL,
			}
		})],
	}
}

/// Context for a `EventHandler` based test.
#[allow(missing_debug_implementations)]
#[non_exhaustive]
pub struct TestContext {
	/// The `EventHandler` instance.
	pub event_handler: EventHandler,
	/// The sender instance.
	pub sender: Sender,
	/// The receiver instance.
	pub receiver: crossbeam_channel::Receiver<EventAction>,
	/// The number of known available events.
	pub number_events: usize,
}

/// Provide an `EventHandler` instance for use within a test.
#[inline]
#[allow(clippy::missing_panics_doc)]
pub fn with_event_handler<C>(events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	let event_handler = EventHandler::new(create_test_keybindings());
	let (sender, receiver) = crossbeam_channel::bounded(10);
	let event_sender = Sender::new(sender);
	let event_queue = event_sender.clone_event_queue();

	for event in events {
		event_queue.lock().push_back(*event);
	}

	callback(TestContext {
		event_handler,
		sender: event_sender,
		receiver,
		number_events: events.len(),
	});
}
