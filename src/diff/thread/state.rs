use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use parking_lot::RwLock;

use crate::diff::{
	CommitDiff,
	thread::{Action, LoadStatus},
};

#[derive(Clone, Debug)]
pub(crate) struct State {
	load_status: Arc<RwLock<LoadStatus>>,
	diff: Arc<RwLock<CommitDiff>>,
	ended: Arc<AtomicBool>,
	cancelled: Arc<AtomicBool>,
	update_receiver: crossbeam_channel::Receiver<Action>,
	update_sender: crossbeam_channel::Sender<Action>,
}

impl State {
	pub(crate) fn new(diff: Arc<RwLock<CommitDiff>>) -> Self {
		let (update_sender, update_receiver) = crossbeam_channel::unbounded();
		Self {
			load_status: Arc::new(RwLock::new(LoadStatus::New)),
			diff,
			ended: Arc::new(AtomicBool::from(false)),
			cancelled: Arc::new(AtomicBool::from(false)),
			update_receiver,
			update_sender,
		}
	}

	pub(crate) fn load_status(&self) -> LoadStatus {
		self.load_status.read().clone()
	}

	pub(crate) fn set_load_status(&self, status: LoadStatus) {
		let mut load_status = self.load_status.write();
		*load_status = status;
	}

	pub(crate) fn diff(&self) -> Arc<RwLock<CommitDiff>> {
		Arc::clone(&self.diff)
	}

	pub(crate) fn receive_update(&self) -> Action {
		self.update_receiver.recv().unwrap_or(Action::StatusChange)
	}

	pub(crate) fn start_load(&self, term: &str) {
		self.send_update(Action::Load(String::from(term)));
	}

	pub(crate) fn is_cancelled(&self) -> bool {
		self.cancelled.load(Ordering::Acquire) || self.ended.load(Ordering::Acquire)
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Acquire)
	}

	pub(crate) fn cancel(&self) {
		self.cancelled.store(true, Ordering::Release);
		self.send_update(Action::StatusChange);
	}

	pub(crate) fn resume(&self) {
		self.cancelled.store(false, Ordering::Release);
		self.send_update(Action::StatusChange);
	}

	pub(crate) fn end(&self) {
		self.ended.store(true, Ordering::Release);
		self.send_update(Action::StatusChange);
	}

	fn send_update(&self, action: Action) {
		let _result = self.update_sender.send(action);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_state() -> State {
		State::new(Arc::new(RwLock::new(CommitDiff::new())))
	}

	#[test]
	fn send_recv_update() {
		let state = create_state();
		state.send_update(Action::Load(String::from("abc123")));
		assert_eq!(state.receive_update(), Action::Load(String::from("abc123")));
	}

	#[test]
	fn send_recv_disconnect() {
		let (update_sender, _update_receiver) = crossbeam_channel::unbounded();
		let mut state = create_state();
		state.update_sender = update_sender; // replace last reference to sender, to force a disconnect
		assert_eq!(state.receive_update(), Action::StatusChange);
	}

	#[test]
	fn load_status() {
		let state = create_state();
		state.set_load_status(LoadStatus::DiffComplete);
		assert_eq!(state.load_status(), LoadStatus::DiffComplete);
	}

	#[test]
	fn start_load() {
		let state = create_state();
		state.start_load("term");
		assert_eq!(state.receive_update(), Action::Load(String::from("term")));
	}

	#[test]
	fn diff() {
		// not much to test here
		let state = create_state();
		let _diff = state.diff();
	}

	#[test]
	fn cancelled() {
		let state = create_state();
		state.cancel();
		assert!(state.is_cancelled());
	}

	#[test]
	fn resume() {
		let state = create_state();
		state.cancel();
		state.resume();
		assert!(!state.is_cancelled());
	}

	#[test]
	fn ended() {
		let state = create_state();
		state.end();
		assert!(state.is_ended());
	}
}
