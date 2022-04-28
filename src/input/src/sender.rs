use std::{
	borrow::BorrowMut,
	collections::VecDeque,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration,
};

use anyhow::{anyhow, Error, Result};
use parking_lot::Mutex;

use crate::{event_action::EventAction, Event};

fn map_send_err<CustomEvent: crate::CustomEvent>(_: crossbeam_channel::SendError<EventAction<CustomEvent>>) -> Error {
	anyhow!("Unable to send data")
}

const EVENT_POLL_TIMEOUT: Duration = Duration::from_secs(1);

/// Represents a message sender and receiver for passing actions between threads.
#[derive(Clone, Debug)]
pub struct Sender<CustomEvent: crate::CustomEvent> {
	event_queue: Arc<Mutex<VecDeque<Event<CustomEvent>>>>,
	poisoned: Arc<AtomicBool>,
	paused: Arc<AtomicBool>,
	receiver: crossbeam_channel::Receiver<()>,
	sender: crossbeam_channel::Sender<EventAction<CustomEvent>>,
}

impl<CustomEvent: crate::CustomEvent> Sender<CustomEvent> {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub fn new(
		sender: crossbeam_channel::Sender<EventAction<CustomEvent>>,
		receiver: crossbeam_channel::Receiver<()>,
	) -> Self {
		Self {
			event_queue: Arc::new(Mutex::new(VecDeque::new())),
			poisoned: Arc::new(AtomicBool::new(false)),
			paused: Arc::new(AtomicBool::new(false)),
			receiver,
			sender,
		}
	}

	/// Clone the poisoned flag.
	#[inline]
	#[must_use]
	pub fn clone_poisoned(&self) -> Arc<AtomicBool> {
		Arc::clone(&self.poisoned)
	}

	/// Is the sender poisoned, and not longer accepting actions.
	#[inline]
	#[must_use]
	pub fn is_poisoned(&self) -> bool {
		self.poisoned.load(Ordering::Relaxed)
	}

	/// Is the sender paused from reading events.
	#[inline]
	#[must_use]
	pub fn is_paused(&self) -> bool {
		self.paused.load(Ordering::Relaxed)
	}

	#[inline]
	pub(crate) fn clone_event_queue(&self) -> Arc<Mutex<VecDeque<Event<CustomEvent>>>> {
		Arc::clone(&self.event_queue)
	}

	/// Queue an end action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn end(&self) -> Result<()> {
		self.sender.send(EventAction::End).map_err(map_send_err)
	}

	/// Read an event from the queue
	#[inline]
	pub fn read_event(&mut self) -> Event<CustomEvent> {
		// clear existing message since last read
		while self.receiver.try_recv().is_ok() {}
		loop {
			if let Some(event) = self.event_queue.lock().borrow_mut().pop_front() {
				return event;
			}

			// if there is no event available on the queue, instead of returning early, we can wait
			// for the new event message and try again.
			if self.receiver.recv_timeout(EVENT_POLL_TIMEOUT).is_ok() {
				continue;
			}

			// We always return if the above recv call times out, to ensure this does not block
			// forever
			return Event::None;
		}
	}

	/// Add an event after existing events.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn enqueue_event(&self, event: Event<CustomEvent>) -> Result<()> {
		self.sender.send(EventAction::EnqueueEvent(event)).map_err(map_send_err)
	}

	/// Add an event before existing events.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn push_event(&self, event: Event<CustomEvent>) -> Result<()> {
		self.sender.send(EventAction::PushEvent(event)).map_err(map_send_err)
	}

	/// Pause the event read thread.
	#[inline]
	pub fn pause(&self) {
		self.paused.store(true, Ordering::Relaxed);
	}

	/// Resume the event read thread.
	#[inline]
	pub fn resume(&self) {
		self.paused.store(false, Ordering::Relaxed);
	}
}

#[cfg(test)]
mod tests {
	use std::thread::{sleep, spawn};

	use crossbeam_channel::bounded;

	use super::*;
	use crate::testutil::local::TestEvent;

	type Sender = super::Sender<TestEvent>;

	#[test]
	fn end() {
		let (sender, receiver) = bounded(1);
		let (_, new_event_receiver) = bounded(1);

		let sender = Sender::new(sender, new_event_receiver);
		sender.end().unwrap();
		let event = receiver.try_recv().expect("Unable to recv event");
		assert_eq!(event, EventAction::End);
	}

	#[test]
	fn end_error() {
		let (sender, _) = bounded(0);
		let (_, new_event_receiver) = bounded(0);

		let sender = Sender::new(sender, new_event_receiver);
		assert_eq!(sender.end().unwrap_err().to_string(), "Unable to send data");
	}

	#[test]
	fn read_event_empty() {
		let (sender, _) = bounded(1);
		let (_, new_event_receiver) = bounded(1);

		let mut sender = Sender::new(sender, new_event_receiver);
		assert_eq!(sender.read_event(), Event::None);
	}

	#[test]
	fn read_event_ready() {
		let (sender, _) = bounded(1);
		let (_, new_event_receiver) = bounded(1);

		let mut sender = Sender::new(sender, new_event_receiver);
		let event_queue = sender.clone_event_queue();
		event_queue.lock().push_back(Event::from('a'));
		assert_eq!(sender.read_event(), Event::from('a'));
	}

	#[test]
	fn read_event_clear_existing_events() {
		let (sender, _) = bounded(1);
		let (new_event_sender, new_event_receiver) = bounded(1);
		new_event_sender.send(()).unwrap();

		let mut sender = Sender::new(sender, new_event_receiver);
		let event_queue = sender.clone_event_queue();
		event_queue.lock().push_back(Event::from('a'));
		let _event = sender.read_event();
		assert_eq!(new_event_sender.len(), 0);
	}

	#[test]
	fn read_event_wait_for_event() {
		let (sender, _) = bounded(1);
		let (new_event_sender, new_event_receiver) = bounded(1);

		let mut sender = Sender::new(sender, new_event_receiver);
		let event_queue = sender.clone_event_queue();

		let ready = Arc::new(AtomicBool::new(false));
		let event = Arc::new(Mutex::new(Event::None));
		let thread_ready = ready.clone();
		let thread_event = event.clone();
		let handle = spawn(move || {
			for _ in 0..100 {
				thread_ready.store(true, Ordering::Release);
				let event = sender.read_event();

				if event == Event::None {
					sleep(Duration::from_millis(10));
					continue;
				}

				*thread_event.lock() = event;
				break;
			}
		});

		while !ready.load(Ordering::Acquire) {}
		sleep(Duration::from_millis(10)); // this is probably fragile

		event_queue.lock().push_back(Event::from('a'));
		new_event_sender.send(()).unwrap();
		handle.join().unwrap();

		assert_eq!(*event.lock(), Event::from('a'));
	}

	#[test]
	fn enqueue_event() {
		let (sender, receiver) = bounded(1);
		let (_, new_event_receiver) = bounded(1);

		let sender = Sender::new(sender, new_event_receiver);
		sender.enqueue_event(Event::from('a')).unwrap();
		let event = receiver.try_recv().expect("Unable to recv event");
		assert_eq!(event, EventAction::EnqueueEvent(Event::from('a')));
	}

	#[test]
	fn enqueue_event_error() {
		let (sender, _) = bounded(0);
		let (_, new_event_receiver) = bounded(0);

		let sender = Sender::new(sender, new_event_receiver);
		assert_eq!(
			sender.enqueue_event(Event::from('a')).unwrap_err().to_string(),
			"Unable to send data"
		);
	}

	#[test]
	fn push_event() {
		let (sender, receiver) = bounded(1);
		let (_, new_event_receiver) = bounded(1);

		let sender = Sender::new(sender, new_event_receiver);
		sender.push_event(Event::from('a')).unwrap();
		let event = receiver.try_recv().expect("Unable to recv event");
		assert_eq!(event, EventAction::PushEvent(Event::from('a')));
	}

	#[test]
	fn push_event_error() {
		let (sender, _) = bounded(0);
		let (_, new_event_receiver) = bounded(0);

		let sender = Sender::new(sender, new_event_receiver);
		assert_eq!(
			sender.push_event(Event::from('a')).unwrap_err().to_string(),
			"Unable to send data"
		);
	}
}
