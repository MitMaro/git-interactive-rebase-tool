use crossbeam_channel::Sender;

use crate::runtime::{RuntimeError, Status};

/// A thread status notifier, that allows a thread to notify the `Runtime` of the current status of the thread.
#[derive(Debug)]
pub(crate) struct Notifier {
	thread_name: String,
	sender: Sender<(String, Status)>,
}

impl Notifier {
	pub(crate) fn new(thread_name: &str, sender: Sender<(String, Status)>) -> Self {
		Self {
			thread_name: String::from(thread_name),
			sender,
		}
	}

	/// Notify the `Runtime` that the thread is busy processing.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn busy(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::Busy))
			.unwrap();
	}

	/// Notify the `Runtime` to request that the `Runtime` and all other registered thread pause processing.
	#[allow(clippy::missing_panics_doc, unused)]
	pub(crate) fn request_pause(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::RequestPause))
			.unwrap();
	}

	/// Notify the `Runtime` to request that the `Runtime` and all other registered thread resume processing.
	#[allow(clippy::missing_panics_doc, unused)]
	pub(crate) fn request_resume(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::RequestResume))
			.unwrap();
	}

	/// Notify the `Runtime` to request that the `Runtime` and all other registered thread end processing.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn request_end(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::RequestEnd))
			.unwrap();
	}

	/// Notify the `Runtime` that the thread is waiting for new data or messages to process.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn wait(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::Waiting))
			.unwrap();
	}

	/// Notify the `Runtime` that the thread is in a permanent error state.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn error(&self, err: RuntimeError) {
		self.sender
			.send((String::from(&self.thread_name), Status::Error(err)))
			.unwrap();
	}

	/// Notify the `Runtime` that the thread has ended processing.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn end(&self) {
		self.sender
			.send((String::from(&self.thread_name), Status::Ended))
			.unwrap();
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_ok_eq;
	use crossbeam_channel::unbounded;

	use super::*;

	#[test]
	fn busy() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.busy();
		assert_ok_eq!(receiver.recv(), (String::from("name"), Status::Busy));
	}

	#[test]
	fn request_pause() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.request_pause();
		assert_ok_eq!(receiver.recv(), (String::from("name"), Status::RequestPause));
	}

	#[test]
	fn request_resume() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.request_resume();
		assert_ok_eq!(receiver.recv(), (String::from("name"), Status::RequestResume));
	}

	#[test]
	fn request_end() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.request_end();
		assert_ok_eq!(receiver.recv(), (String::from("name"), Status::RequestEnd));
	}

	#[test]
	fn error() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.error(RuntimeError::ThreadError(String::from("error")));
		assert_ok_eq!(
			receiver.recv(),
			(
				String::from("name"),
				Status::Error(RuntimeError::ThreadError(String::from("error")))
			)
		);
	}

	#[test]
	fn end() {
		let (sender, receiver) = unbounded();
		let notifier = Notifier::new("name", sender);
		notifier.end();
		assert_ok_eq!(receiver.recv(), (String::from("name"), Status::Ended));
	}
}
