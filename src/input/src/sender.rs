use std::{
	borrow::BorrowMut,
	collections::VecDeque,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use anyhow::{anyhow, Error, Result};
use parking_lot::Mutex;

use crate::{event_action::EventAction, Event};

fn map_send_err(_: crossbeam_channel::SendError<EventAction>) -> Error {
	anyhow!("Unable to send data")
}

/// Represents a message sender and receiver for passing actions between threads.
#[derive(Clone, Debug)]
pub struct Sender {
	poisoned: Arc<AtomicBool>,
	sender: crossbeam_channel::Sender<EventAction>,
	event_queue: Arc<Mutex<VecDeque<Event>>>,
}

impl Sender {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub fn new(sender: crossbeam_channel::Sender<EventAction>) -> Self {
		Self {
			poisoned: Arc::new(AtomicBool::new(false)),
			sender,
			event_queue: Arc::new(Mutex::new(VecDeque::new())),
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

	#[inline]
	pub(crate) fn clone_event_queue(&self) -> Arc<Mutex<VecDeque<Event>>> {
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
	pub fn read_event(&mut self) -> Event {
		self.event_queue.lock().borrow_mut().pop_front().unwrap_or(Event::None)
	}

	/// Add an event after existing events.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn enqueue_event(&self, event: Event) -> Result<()> {
		self.sender.send(EventAction::EnqueueEvent(event)).map_err(map_send_err)
	}

	/// Add an event before existing events.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn push_event(&self, event: Event) -> Result<()> {
		self.sender.send(EventAction::PushEvent(event)).map_err(map_send_err)
	}
}
