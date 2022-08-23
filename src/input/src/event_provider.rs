use std::time::Duration;

use anyhow::{anyhow, Result};
use crossterm::event::Event;
#[cfg(not(test))]
use crossterm::event::{poll, read};
#[cfg(test)]
use read_event_mocks::{poll, read};

/// Function that returns a event
pub trait EventReaderFn: Fn() -> Result<Option<Event>> + Send + Sync + 'static {}

impl<FN: Fn() -> Result<Option<Event>> + Send + Sync + 'static> EventReaderFn for FN {}

/// Read the next input event from the terminal interface.
///
/// # Errors
///
/// Errors if the Tui cannot read an event for any reason. In general this should not error, and
/// if this does generate an error, the Tui should be considered to be in a non-recoverable
/// state.
#[inline]
pub fn read_event() -> Result<Option<Event>> {
	if poll(Duration::from_millis(20)).unwrap_or(false) {
		read()
			.map(Some)
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
		pub static ref HAS_POLLED_EVENT: Mutex<Result<bool>> = Mutex::new(Ok(true));
		pub static ref NEXT_EVENT: Mutex<Result<Event>> = Mutex::new(Ok(Event::Key(KeyEvent::from(KeyCode::Null))));
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

	use crossterm::event::{KeyCode, KeyEvent};

	use super::*;

	#[test]
	#[serial_test::serial]
	fn read_event_poll_error() {
		{
			let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
			*lock = Err(io::Error::from(ErrorKind::Other));
		}

		assert!(read_event().unwrap().is_none());
	}

	#[test]
	#[serial_test::serial]
	fn read_event_poll_timeout() {
		{
			let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
			*lock = Ok(false);
		}

		assert!(read_event().unwrap().is_none());
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_error() {
		{
			let mut lock = read_event_mocks::NEXT_EVENT.lock();
			*lock = Err(io::Error::from(ErrorKind::Other));

			let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
			*lock = Ok(true);
		}

		assert_eq!(read_event().unwrap_err().to_string(), "Unexpected Error");
	}

	#[test]
	#[serial_test::serial]
	fn read_event_read_success() {
		{
			let mut lock = read_event_mocks::NEXT_EVENT.lock();
			*lock = Ok(Event::Key(KeyEvent::from(KeyCode::Enter)));

			let mut lock = read_event_mocks::HAS_POLLED_EVENT.lock();
			*lock = Ok(true);
		}

		assert_eq!(read_event().unwrap(), Some(Event::Key(KeyEvent::from(KeyCode::Enter))));
	}
}
