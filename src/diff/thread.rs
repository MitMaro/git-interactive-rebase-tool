mod action;
mod load_status;
mod state;
mod update_handler;

use std::sync::Arc;

use captur::capture;
use parking_lot::Mutex;

pub(crate) use self::{action::Action, load_status::LoadStatus, state::State, update_handler::UpdateHandlerFn};
use crate::{
	diff::CommitDiffLoader,
	runtime::{Installer, Threadable},
};

pub(crate) const THREAD_NAME: &str = "diff";

#[derive(Debug)]
pub(crate) struct Thread<UpdateHandler: UpdateHandlerFn> {
	state: State,
	commit_diff_loader: Arc<Mutex<CommitDiffLoader>>,
	update_handler: Arc<UpdateHandler>,
}

impl<UpdateHandler> Threadable for Thread<UpdateHandler>
where UpdateHandler: UpdateHandlerFn + 'static
{
	fn install(&self, installer: &Installer) {
		let state = self.state();

		installer.spawn(THREAD_NAME, |notifier| {
			let update_handler = Arc::clone(&self.update_handler);
			let commit_diff_loader = Arc::clone(&self.commit_diff_loader);
			move || {
				capture!(notifier, state);

				notifier.wait();

				loop {
					notifier.wait();
					if state.is_ended() {
						break;
					}

					if state.is_cancelled() {
						commit_diff_loader.lock().reset();
					}

					let msg = state.receive_update();
					notifier.busy();
					match msg {
						Action::Load(hash) => {
							state.resume();
							let mut loader = commit_diff_loader.lock();
							if let Err(e) = loader.load_diff(hash.as_str(), |s: LoadStatus| {
								state.set_load_status(s);
								update_handler();
								state.is_cancelled()
							}) {
								state.set_load_status(LoadStatus::Error {
									msg: e.to_string(),
									code: e.code(),
								});
								state.cancel();
								update_handler();
							}
						},
						Action::StatusChange => {},
					}
				}

				notifier.request_end();
				notifier.end();
			}
		});
	}

	fn pause(&self) {
		self.state.cancel();
	}

	fn resume(&self) {
		self.state.resume();
	}

	fn end(&self) {
		self.state.end();
	}
}

impl<UpdateHandler> Thread<UpdateHandler>
where UpdateHandler: UpdateHandlerFn
{
	pub(crate) fn new(commit_diff_loader: CommitDiffLoader, update_handler: UpdateHandler) -> Self {
		let commit_diff = commit_diff_loader.commit_diff();
		Self {
			state: State::new(commit_diff),
			commit_diff_loader: Arc::new(Mutex::new(commit_diff_loader)),
			update_handler: Arc::new(update_handler),
		}
	}

	pub(crate) fn state(&self) -> State {
		self.state.clone()
	}
}

#[cfg(test)]
mod tests {
	use std::{thread::sleep, time::Duration};

	use super::*;
	use crate::{
		diff::CommitDiffLoaderOptions,
		runtime::Status,
		test_helpers::{testers, with_temp_repository},
	};

	#[test]
	fn set_pause_resume() {
		with_temp_repository(|repository| {
			let diff_loader = CommitDiffLoader::new(repository, CommitDiffLoaderOptions::new());
			let diff = diff_loader.commit_diff();
			diff.write().update(vec![], 1, 2, 3);
			let thread = Thread::new(diff_loader, || {});

			let state = thread.state();
			let tester = testers::Threadable::new();
			tester.start_threadable(&thread, THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			thread.pause();
			assert!(state.is_cancelled());
			let mut pass = false;
			for _ in 0..10 {
				if diff.read().number_deletions() == 0 {
					pass = true;
					break;
				}
				sleep(Duration::from_millis(10));
			}
			assert!(pass);
			thread.resume();
			assert!(!state.is_cancelled());
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn set_end() {
		with_temp_repository(|repository| {
			let thread = Thread::new(CommitDiffLoader::new(repository, CommitDiffLoaderOptions::new()), || {});

			let state = thread.state();
			let tester = testers::Threadable::new();
			tester.start_threadable(&thread, THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			state.end();
			assert!(state.is_ended());
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn diff_load_error() {
		with_temp_repository(|repository| {
			let thread = Thread::new(CommitDiffLoader::new(repository, CommitDiffLoaderOptions::new()), || {});

			let state = thread.state();
			let tester = testers::Threadable::new();
			tester.start_threadable(&thread, THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);

			state.start_load("abc123");

			let mut pass = false;
			for _ in 0..10 {
				if let LoadStatus::Error { .. } = state.load_status() {
					pass = true;
					break;
				}
				sleep(Duration::from_millis(10));
			}
			assert!(pass);
			assert!(state.is_cancelled());

			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}
}
