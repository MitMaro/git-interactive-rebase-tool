use std::time::Duration;

use anyhow::{anyhow, Result};
#[cfg(not(test))]
use crossterm::event::{poll, read};
use crossterm::event::{Event, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};
#[cfg(test)]
use read_event_mocks::{poll, read};

/// Function that returns a event
pub(crate) trait EventReaderFn: Fn() -> Result<Option<Event>> + Send + Sync + 'static {}

impl<FN: Fn() -> Result<Option<Event>> + Send + Sync + 'static> EventReaderFn for FN {}

/// Read the next input event from the terminal interface.
///
/// # Errors
///
/// Errors if the Tui cannot read an event for any reason. In general this should not error, and
/// if this does generate an error, the Tui should be considered to be in a non-recoverable
/// state.
#[inline]
pub(crate) fn read_event() -> Result<Option<Event>> {
	if poll(Duration::from_millis(20)).unwrap_or(false) {
		read()
			.map(|event| {
				match event {
					e @ (Event::Key(KeyEvent {
						kind: KeyEventKind::Press | KeyEventKind::Repeat,
						..
					})
					| Event::Mouse(MouseEvent {
						kind: MouseEventKind::Down(_) | MouseEventKind::ScrollDown | MouseEventKind::ScrollUp,
						..
					})
					| Event::Resize(..)) => Some(e),
					Event::Key(_) | Event::Mouse(_) | Event::Paste(_) | Event::FocusGained | Event::FocusLost => None,
				}
			})
			.map_err(|err| anyhow!("{:#}", err).context("Unexpected Error"))
	}
	else {
		Ok(None)
	}
}

#[cfg(test)]
mod read_event_mocks {
	use std::{mem, time::Duration};

	use crossterm::{
		event::{Event, KeyCode, KeyEvent},
		Result,
	};
	use lazy_static::lazy_static;
	use parking_lot::Mutex;

	lazy_static! {
		pub(crate) static ref HAS_POLLED_EVENT: Mutex<Result<bool>> = Mutex::new(Ok(true));
		pub(crate) static ref NEXT_EVENT: Mutex<Result<Event>> =
			Mutex::new(Ok(Event::Key(KeyEvent::from(KeyCode::Null))));
	}

	pub(crate) fn poll(_: Duration) -> Result<bool> {
		let mut lock = HAS_POLLED_EVENT.lock();
		mem::replace(&mut *lock, Ok(false))
	}

	pub(crate) fn read() -> Result<Event> {
		let mut lock = NEXT_EVENT.lock();
		mem::replace(&mut *lock, Ok(Event::Key(KeyEvent::from(KeyCode::Null))))
	}
}

#[cfg(test)]
mod tests {
	use std::{io, io::ErrorKind};

	use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton};

	use super::*;

	#[test]
	#[serial_test::serial]
	fn read_event_poll_error() {
		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Err(io::Error::from(ErrorKind::Other));
		drop(lock);

		assert!(read_event().unwrap().is_none());
	}

	#[test]
	#[serial_test::serial]
	fn read_event_poll_timeout() {
		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(false);
		drop(lock);

		assert!(read_event().unwrap().is_none());
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_error() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Err(io::Error::from(ErrorKind::Other));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap_err().to_string(), "Unexpected Error");
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_key_press() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Key(KeyEvent::from(KeyCode::Enter)));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), Some(Event::Key(KeyEvent::from(KeyCode::Enter))));
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_key_repeat() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Key(KeyEvent::new_with_kind(
			KeyCode::Enter,
			KeyModifiers::NONE,
			KeyEventKind::Repeat,
		)));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(
			read_event().unwrap(),
			Some(Event::Key(KeyEvent::new_with_kind(
				KeyCode::Enter,
				KeyModifiers::NONE,
				KeyEventKind::Repeat,
			)))
		);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_mouse_down() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Mouse(MouseEvent {
			kind: MouseEventKind::Down(MouseButton::Right),
			column: 0,
			row: 0,
			modifiers: KeyModifiers::NONE,
		}));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(
			read_event().unwrap(),
			Some(Event::Mouse(MouseEvent {
				kind: MouseEventKind::Down(MouseButton::Right),
				column: 0,
				row: 0,
				modifiers: KeyModifiers::NONE
			}))
		);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_mouse_scroll_down() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Mouse(MouseEvent {
			kind: MouseEventKind::ScrollDown,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::NONE,
		}));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(
			read_event().unwrap(),
			Some(Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::NONE
			}))
		);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_mouse_scroll_up() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Mouse(MouseEvent {
			kind: MouseEventKind::ScrollDown,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::NONE,
		}));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(
			read_event().unwrap(),
			Some(Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::NONE
			}))
		);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_resize() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Resize(1, 1));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), Some(Event::Resize(1, 1)));
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_key_other() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Key(KeyEvent::new_with_kind(
			KeyCode::Enter,
			KeyModifiers::NONE,
			KeyEventKind::Release,
		)));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), None);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_mouse_other() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Mouse(MouseEvent {
			kind: MouseEventKind::Moved,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::NONE,
		}));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), None);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_paste() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::Paste(String::from("Foo")));
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), None);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_focus_gained() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::FocusGained);
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), None);
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_focus_lost() {
		let mut lock = read_event_mocks::NEXT_EVENT.lock();
		*lock = Ok(Event::FocusLost);
		drop(lock);

		let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
		*lock = Ok(true);
		drop(lock);

		assert_eq!(read_event().unwrap(), None);
	}
}
