//! Utilities for writing tests that interact with input events.
use std::cell::RefCell;

use super::{Event, EventHandler, KeyBindings, KeyCode, KeyEvent, KeyModifiers, MetaEvent};

#[allow(clippy::match_same_arms)]
fn map_event_to_crossterm(event: Event) -> crossterm::event::Event {
	match event {
		Event::Meta(meta_event) => {
			let key_event = match meta_event {
				MetaEvent::Abort => KeyEvent::from(KeyCode::Char('q')),
				MetaEvent::ActionBreak => KeyEvent::from(KeyCode::Char('b')),
				MetaEvent::ActionDrop => KeyEvent::from(KeyCode::Char('d')),
				MetaEvent::ActionEdit => KeyEvent::from(KeyCode::Char('e')),
				MetaEvent::ActionFixup => KeyEvent::from(KeyCode::Char('f')),
				MetaEvent::ActionPick => KeyEvent::from(KeyCode::Char('p')),
				MetaEvent::ActionReword => KeyEvent::from(KeyCode::Char('r')),
				MetaEvent::ActionSquash => KeyEvent::from(KeyCode::Char('s')),
				MetaEvent::Delete => KeyEvent::from(KeyCode::Delete),
				MetaEvent::Edit => KeyEvent::from(KeyCode::Char('E')),
				MetaEvent::Exit => {
					KeyEvent {
						code: KeyCode::Char('d'),
						modifiers: KeyModifiers::CONTROL,
					}
				},
				MetaEvent::ForceAbort => KeyEvent::from(KeyCode::Char('Q')),
				MetaEvent::ForceRebase => KeyEvent::from(KeyCode::Char('W')),
				MetaEvent::Help => KeyEvent::from(KeyCode::Char('?')),
				MetaEvent::InsertLine => KeyEvent::from(KeyCode::Char('I')),
				MetaEvent::Kill => {
					KeyEvent {
						code: KeyCode::Char('c'),
						modifiers: KeyModifiers::CONTROL,
					}
				},
				MetaEvent::MoveCursorDown => KeyEvent::from(KeyCode::Down),
				MetaEvent::MoveCursorEnd => KeyEvent::from(KeyCode::End),
				MetaEvent::MoveCursorHome => KeyEvent::from(KeyCode::Home),
				MetaEvent::MoveCursorLeft => KeyEvent::from(KeyCode::Left),
				MetaEvent::MoveCursorPageDown => KeyEvent::from(KeyCode::PageDown),
				MetaEvent::MoveCursorPageUp => KeyEvent::from(KeyCode::PageUp),
				MetaEvent::MoveCursorRight => KeyEvent::from(KeyCode::Right),
				MetaEvent::MoveCursorUp => KeyEvent::from(KeyCode::Up),
				MetaEvent::No => KeyEvent::from(KeyCode::Char('n')),
				MetaEvent::OpenInEditor => KeyEvent::from(KeyCode::Char('!')),
				MetaEvent::Rebase => KeyEvent::from(KeyCode::Char('w')),
				MetaEvent::Redo => {
					KeyEvent {
						code: KeyCode::Char('y'),
						modifiers: KeyModifiers::CONTROL,
					}
				},
				MetaEvent::ScrollBottom => KeyEvent::from(KeyCode::End),
				MetaEvent::ScrollDown => KeyEvent::from(KeyCode::Down),
				MetaEvent::ScrollJumpDown => KeyEvent::from(KeyCode::PageDown),
				MetaEvent::ScrollJumpUp => KeyEvent::from(KeyCode::PageUp),
				MetaEvent::ScrollLeft => KeyEvent::from(KeyCode::Left),
				MetaEvent::ScrollRight => KeyEvent::from(KeyCode::Right),
				MetaEvent::ScrollTop => KeyEvent::from(KeyCode::Home),
				MetaEvent::ScrollUp => KeyEvent::from(KeyCode::Up),
				MetaEvent::ShowCommit => KeyEvent::from(KeyCode::Char('c')),
				MetaEvent::ShowDiff => KeyEvent::from(KeyCode::Char('d')),
				MetaEvent::SwapSelectedDown => KeyEvent::from(KeyCode::Char('j')),
				MetaEvent::SwapSelectedUp => KeyEvent::from(KeyCode::Char('k')),
				MetaEvent::ToggleVisualMode => KeyEvent::from(KeyCode::Char('v')),
				MetaEvent::Undo => {
					KeyEvent {
						code: KeyCode::Char('z'),
						modifiers: KeyModifiers::CONTROL,
					}
				},
				MetaEvent::Yes => KeyEvent::from(KeyCode::Char('y')),
				MetaEvent::ExternalCommandSuccess => KeyEvent::from(KeyCode::Null),
				MetaEvent::ExternalCommandError => KeyEvent::from(KeyCode::Null),
			};
			crossterm::event::Event::Key(key_event)
		},
		Event::Key(key_event) => crossterm::event::Event::Key(key_event),
		Event::Mouse(mouse_event) => crossterm::event::Event::Mouse(mouse_event),
		Event::Resize(width, height) => crossterm::event::Event::Resize(width, height),
		Event::None => crossterm::event::Event::Key(KeyEvent::from(KeyCode::Null)),
	}
}

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
	/// The number of known available events.
	pub number_events: usize,
}

impl TestContext {
	/// For each known event, call the callback with the `EventHandler` instance.
	#[inline]
	pub fn for_each_event<C, T>(&self, mut callback: C) -> Vec<T>
	where C: FnMut(&EventHandler) -> T {
		let mut results = vec![];
		for _ in 0..self.number_events {
			results.push(callback(&self.event_handler));
		}
		results
	}
}

/// Provide an `EventHandler` instance for use within a test.
///
/// ```
/// use input::{testutil::with_event_handler, Event, MetaEvent};
///
/// with_event_handler(&[Event::Meta(MetaEvent::Abort)], |context| {
/// 	context.for_each_event(|_event_handler| {});
/// });
/// ```
#[inline]
pub fn with_event_handler<C>(events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	let crossterm_events = RefCell::new(
		events
			.iter()
			.map(|input| map_event_to_crossterm(*input))
			.collect::<Vec<crossterm::event::Event>>(),
	);

	crossterm_events.borrow_mut().reverse();
	let event_handler = EventHandler::new(
		move || {
			let mut ct_events = crossterm_events.borrow_mut();
			Ok(ct_events.pop())
		},
		create_test_keybindings(),
	);

	callback(TestContext {
		event_handler,
		number_events: events.len(),
	});
}
