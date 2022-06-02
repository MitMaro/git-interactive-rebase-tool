use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use anyhow::{anyhow, Result};
use parking_lot::Mutex;

use crate::Status;

const WAIT_TIME: Duration = Duration::from_millis(100);

/// Tracker for threads current `Status`s.
#[derive(Debug, Clone)]
pub struct ThreadStatuses {
	statuses: Arc<Mutex<HashMap<String, Status>>>,
}

impl ThreadStatuses {
	/// Create a new instance.
	#[must_use]
	#[inline]
	pub fn new() -> Self {
		Self {
			statuses: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	/// Wait for a thread with a particular name to reach an expected `Status`.
	///
	/// # Errors
	/// Will error if the wait times out.
	#[inline]
	pub fn wait_for_status(&self, thread_name: &str, expected_status: &Status) -> Result<()> {
		let mut attempt = 0;

		loop {
			let lock = self.statuses.lock();
			let current = lock
				.get(thread_name)
				.ok_or_else(|| anyhow!("Thread name not registered"))?;

			if current == expected_status {
				return Ok(());
			}
			drop(lock);

			sleep(WAIT_TIME);
			attempt += 1;

			if attempt > 10 {
				return Err(anyhow!("Timeout waited for thread {} to change status", thread_name));
			}
		}
	}

	pub(crate) fn register_thread(&self, thread_name: &str, status: Status) {
		assert!(
			self.statuses.lock().insert(String::from(thread_name), status).is_none(),
			"Attempt to register more than one threads with name: {}",
			thread_name
		);
	}

	pub(crate) fn update_thread(&self, thread_name: &str, status: Status) {
		let mut lock = self.statuses.lock();
		let current = lock.entry(String::from(thread_name)).or_insert(Status::New);
		if !matches!(*current, Status::Error(..))
			&& !matches!(
				status,
				Status::RequestPause | Status::RequestResume | Status::RequestEnd
			) {
			*current = status;
		}
	}

	pub(crate) fn all_ended(&self) -> bool {
		self.statuses
			.lock()
			.values()
			.all(|status| matches!(status, &(Status::Ended | Status::Error(_))))
	}
}

#[cfg(test)]
mod tests {
	use std::{ops::Mul, thread};

	use super::*;

	#[test]
	fn wait_for_status_success_immediate() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		assert!(!statuses.wait_for_status("name", &Status::New).is_err());
	}

	#[test]
	fn wait_for_status_success_after_wait() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		let thread_statuses = statuses.clone();
		let _ = thread::spawn(move || {
			sleep(WAIT_TIME.mul(4));
			thread_statuses.update_thread("name", Status::Ended);
		});

		assert!(!statuses.wait_for_status("name", &Status::Ended).is_err());
	}

	#[test]
	fn wait_for_status_not_registered_error() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		assert_eq!(
			statuses
				.wait_for_status("not-name", &Status::Ended)
				.unwrap_err()
				.to_string(),
			"Thread name not registered"
		);
	}

	#[test]
	fn wait_for_status_timeout_error() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		assert!(statuses.wait_for_status("name", &Status::Ended).is_err());
	}

	#[test]
	fn register_thread() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		assert!(matches!(statuses.statuses.lock().get("name").unwrap(), Status::New));
	}

	#[test]
	#[should_panic]
	fn register_thread_same_name() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		statuses.register_thread("name", Status::New);
		assert!(matches!(statuses.statuses.lock().get("name").unwrap(), Status::New));
	}

	#[test]
	fn update_thread() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		statuses.update_thread("name", Status::Busy);
		assert!(matches!(statuses.statuses.lock().get("name").unwrap(), Status::Busy));
	}

	#[test]
	fn all_ended_one_not_ended() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::New);
		assert!(!statuses.all_ended());
	}

	#[test]
	fn all_ended_one_ended() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::Ended);
		assert!(statuses.all_ended());
	}

	#[test]
	fn all_ended_multiple_with_one_ended() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name0", Status::New);
		statuses.register_thread("name1", Status::Ended);
		assert!(!statuses.all_ended());
	}

	#[test]
	fn all_ended_multiple_with_all_ended() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name0", Status::Ended);
		statuses.register_thread("name1", Status::Ended);
		assert!(statuses.all_ended());
	}

	#[test]
	fn all_ended_with_error_state() {
		let statuses = ThreadStatuses::new();
		statuses.register_thread("name", Status::Error(anyhow!("Error")));
		assert!(statuses.all_ended());
	}
}
