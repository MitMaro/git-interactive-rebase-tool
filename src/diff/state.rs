use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use parking_lot::RwLock;

use crate::diff::{Action, CommitDiff};

// TODO move
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum LoadStatus {
	New,
	QuickDiff(usize, usize),
	CompleteQuickDiff,
	Diff(usize, usize),
	DiffComplete,
}

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

	pub(crate) fn load_status(&self) -> Arc<RwLock<LoadStatus>> {
		Arc::clone(&self.load_status)
	}

	pub(crate) fn diff(&self) -> Arc<RwLock<CommitDiff>> {
		Arc::clone(&self.diff)
	}

	pub(crate) fn receive_update(&self) -> Action {
		self.update_receiver.recv().unwrap_or(Action::End)
	}

	pub(crate) fn send_update(&self, action: Action) {
		let _result = self.update_sender.send(action);
	}

	pub(crate) fn is_cancelled(&self) -> bool {
		self.cancelled.load(Ordering::Acquire) || self.ended.load(Ordering::Acquire)
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Acquire)
	}

	pub(crate) fn cancel(&self) {
		self.cancelled.store(true, Ordering::Release);
	}

	pub(crate) fn resume(&self) {
		self.cancelled.store(false, Ordering::Release);
	}

	pub(crate) fn end(&self) {
		self.ended.store(true, Ordering::Release);
		self.send_update(Action::End);
	}
}
// #[cfg(test)]
// mod tests {
//
// 	use super::*;
//
// 	#[test]
// 	fn send_recv_update() {
// 		let state = State::new();
// 		state.send_update(Action::Start(String::from("test")));
// 		assert!(matches!(state.receive_update(), Action::Start(_)));
// 	}
//
// 	#[test]
// 	fn send_recv_update_timeout() {
// 		let state = State::new();
// 		assert!(matches!(state.receive_update(), Action::Continue));
// 	}
//
// 	#[test]
// 	fn send_recv_disconnect() {
// 		let (update_sender, _update_receiver) = crossbeam_channel::unbounded();
// 		let mut state = State::new();
// 		state.update_sender = update_sender; // replace last reference to sender, to force a disconnect
// 		assert!(matches!(state.receive_update(), Action::End));
// 	}
//
// 	#[test]
// 	fn paused() {
// 		let state = State::new();
// 		state.pause();
// 		assert!(state.is_paused());
// 	}
//
// 	#[test]
// 	fn resumed() {
// 		let state = State::new();
// 		state.resume();
// 		assert!(!state.is_paused());
// 	}
//
// 	#[test]
// 	fn ended() {
// 		let state = State::new();
// 		state.end();
// 		assert!(state.is_ended());
// 	}
// }
