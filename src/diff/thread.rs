use std::sync::Arc;

use captur::capture;
use parking_lot::Mutex;

use crate::{
	diff::{Action, CommitDiffLoader, State, UpdateHandlerFn, state::LoadStatus},
	runtime::{Installer, RuntimeError, Threadable},
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

				let load_status = state.load_status();
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
								let mut ls = load_status.write();
								*ls = s;
								update_handler();
								return state.is_cancelled();
							}) {
								notifier.error(RuntimeError::ThreadError(e.to_string()));
								continue;
							}
						},
						Action::End => continue,
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

// #[cfg(test)]
// mod tests {
// 	use std::sync::atomic::{AtomicUsize, Ordering};
//
// 	use parking_lot::Mutex;
//
// 	use super::*;
// 	use crate::{runtime::Status, test_helpers::testers};
//
// 	#[derive(Clone)]
// 	struct MockedSearchable {
// 		calls: Arc<Mutex<Vec<String>>>,
// 		search_result: Arc<Mutex<SearchResult>>,
// 	}
//
// 	impl MockedSearchable {
// 		fn new() -> Self {
// 			Self {
// 				calls: Arc::new(Mutex::new(vec![])),
// 				search_result: Arc::new(Mutex::new(SearchResult::None)),
// 			}
// 		}
// 	}
//
// 	impl Searchable for MockedSearchable {
// 		fn reset(&mut self) {
// 			self.calls.lock().push(String::from("Reset"));
// 		}
//
// 		fn search(&mut self, _: Interrupter, term: &str) -> SearchResult {
// 			self.calls.lock().push(format!("Search({term})"));
// 			*self.search_result.lock()
// 		}
// 	}
//
// 	#[test]
// 	fn set_pause_resume() {
// 		let thread = Thread::new(|| {});
// 		let state = thread.state();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
// 		tester.wait_for_status(&Status::Waiting);
// 		thread.pause();
// 		assert!(state.is_paused());
// 		// give thread time to pause
// 		sleep(Duration::from_secs(1));
// 		state.send_update(Action::Continue);
// 		thread.resume();
// 		assert!(!state.is_paused());
// 		state.end();
// 		tester.wait_for_status(&Status::Ended);
// 	}
//
// 	#[test]
// 	fn set_end() {
// 		let thread = Thread::new(|| {});
// 		let state = thread.state();
// 		thread.end();
// 		assert!(state.is_ended());
// 	}
//
// 	#[test]
// 	fn thread_end_from_state() {
// 		let thread = Thread::new(|| {});
// 		let state = thread.state();
//
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
// 		tester.wait_for_status(&Status::Waiting);
// 		state.end();
// 		tester.wait_for_status(&Status::Ended);
// 	}
//
// 	#[test]
// 	fn thread_end_from_action() {
// 		let thread = Thread::new(|| {});
// 		let state = thread.state();
//
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
// 		tester.wait_for_status(&Status::Waiting);
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
// 	}
//
// 	#[test]
// 	fn thread_start_search() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
//
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
// 		tester.wait_for_status(&Status::Waiting);
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)")]);
// 	}
//
// 	#[test]
// 	fn thread_start_cancel() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
//
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::Cancel);
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Relaxed), 1);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Reset")]);
// 	}
//
// 	#[test]
// 	fn thread_start_continue() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::Continue);
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 2);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Search(foo)")]);
// 	}
//
// 	#[test]
// 	fn thread_no_updates_after_complete() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		*searchable.search_result.lock() = SearchResult::Complete;
// 		state.send_update(Action::Continue);
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)")]);
// 	}
//
// 	#[test]
// 	fn thread_no_updates_on_empty_term() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::new()));
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 0);
// 		let calls = searchable.calls.lock();
// 		assert!(calls.is_empty());
// 	}
//
// 	#[test]
// 	fn thread_no_additional_updates_on_start_with_same_term() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)")]);
// 	}
//
// 	#[test]
// 	fn thread_no_updates_on_no_searchable() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::Start(String::from("foo")));
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 0);
// 	}
//
// 	#[test]
// 	fn thread_updates_after_timeout() {
// 		let update_handler_calls = Arc::new(AtomicUsize::new(0));
// 		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
// 		let thread = Thread::new(move || {
// 			_ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
// 		});
// 		let state = thread.state();
//
// 		let searchable = MockedSearchable::new();
// 		let tester = testers::Threadable::new();
// 		tester.start_threadable(&thread, THREAD_NAME);
//
// 		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
// 		state.send_update(Action::Start(String::from("foo")));
// 		sleep(Duration::from_millis(750)); // will timeout after 500ms
// 		state.send_update(Action::End);
// 		tester.wait_for_status(&Status::Ended);
//
// 		assert_eq!(update_handler_calls.load(Ordering::Acquire), 2);
// 		let calls = searchable.calls.lock();
// 		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Search(foo)")]);
// 	}
// }
