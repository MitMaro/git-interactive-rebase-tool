use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration,
};

use crossbeam_channel::RecvTimeoutError;

use crate::search::action::Action;

const RECEIVE_TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Clone, Debug)]
pub(crate) struct State {
	ended: Arc<AtomicBool>,
	paused: Arc<AtomicBool>,
	update_receiver: crossbeam_channel::Receiver<Action>,
	update_sender: crossbeam_channel::Sender<Action>,
}

impl State {
	pub(crate) fn new() -> Self {
		let (update_sender, update_receiver) = crossbeam_channel::unbounded();
		Self {
			ended: Arc::new(AtomicBool::from(false)),
			paused: Arc::new(AtomicBool::from(false)),
			update_receiver,
			update_sender,
		}
	}

	pub(crate) fn receive_update(&self) -> Action {
		self.update_receiver
			.recv_timeout(RECEIVE_TIMEOUT)
			.unwrap_or_else(|e: RecvTimeoutError| {
				match e {
					RecvTimeoutError::Timeout => Action::Continue,
					RecvTimeoutError::Disconnected => Action::End,
				}
			})
	}

	pub(crate) fn send_update(&self, action: Action) {
		let _result = self.update_sender.send(action);
	}

	pub(crate) fn is_paused(&self) -> bool {
		self.paused.load(Ordering::Acquire)
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Acquire)
	}

	pub(crate) fn pause(&self) {
		self.paused.store(true, Ordering::Release);
	}

	pub(crate) fn resume(&self) {
		self.paused.store(false, Ordering::Release);
	}

	pub(crate) fn end(&self) {
		self.ended.store(true, Ordering::Release);
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn send_recv_update() {
		let state = State::new();
		state.send_update(Action::Start(String::from("test")));
		assert!(matches!(state.receive_update(), Action::Start(_)));
	}

	#[test]
	fn send_recv_update_timeout() {
		let state = State::new();
		assert!(matches!(state.receive_update(), Action::Continue));
	}

	#[test]
	fn send_recv_disconnect() {
		let (update_sender, _update_receiver) = crossbeam_channel::unbounded();
		let mut state = State::new();
		state.update_sender = update_sender; // replace last reference to sender, to force a disconnect
		assert!(matches!(state.receive_update(), Action::End));
	}

	#[test]
	fn paused() {
		let state = State::new();
		state.pause();
		assert!(state.is_paused());
	}

	#[test]
	fn resumed() {
		let state = State::new();
		state.resume();
		assert!(!state.is_paused());
	}

	#[test]
	fn ended() {
		let state = State::new();
		state.end();
		assert!(state.is_ended());
	}
}
