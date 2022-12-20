use std::{
	sync::Arc,
	thread::sleep,
	time::{Duration, Instant},
};

use captur::capture;
use runtime::{Installer, Threadable};

use crate::search::{
	action::Action,
	interrupter::Interrupter,
	search_result::SearchResult,
	searchable::Searchable,
	State,
};

pub(crate) const THREAD_NAME: &str = "search";
const MINIMUM_PAUSE_RATE: Duration = Duration::from_millis(50);
const SEARCH_INTERRUPT_TIME: Duration = Duration::from_millis(10);

#[derive(Debug)]
pub(crate) struct Thread<UpdateHandler: Fn() + Sync + Send> {
	state: State,
	search_update_handler: Arc<UpdateHandler>,
}

impl<UpdateHandler> Threadable for Thread<UpdateHandler>
where UpdateHandler: Fn() + Sync + Send + 'static
{
	#[inline]
	fn install(&self, installer: &Installer) {
		let state = self.state();
		let update_handler = Arc::clone(&self.search_update_handler);

		installer.spawn(THREAD_NAME, |notifier| {
			move || {
				capture!(notifier, state);
				let mut active_searchable: Option<Box<dyn Searchable>> = None;
				let mut search_term = String::new();
				let mut search_complete = false;

				notifier.wait();
				let mut time = Instant::now();

				loop {
					notifier.wait();
					if state.is_ended() {
						break;
					}
					while state.is_paused() {
						sleep(time.saturating_duration_since(Instant::now()));
						time += MINIMUM_PAUSE_RATE;
					}

					let msg = state.receive_update();
					notifier.busy();
					match msg {
						Action::Cancel => {
							if let Some(searchable) = active_searchable.as_mut() {
								searchable.reset();
							};
							search_complete = true;
							search_term.clear();
						},
						Action::SetSearchable(searchable) => {
							search_complete = true;
							active_searchable = Some(searchable);
						},
						Action::Start(term) => {
							// avoid calling update handler when there is no change in the search term
							if term == search_term {
								continue;
							}
							search_complete = false;
							search_term = term;
						},
						Action::Continue => {},
						Action::End => break,
					}
					if search_complete || search_term.is_empty() {
						continue;
					}

					let Some(searchable) = active_searchable.as_mut() else {
						continue;
					};

					match searchable.search(Interrupter::new(SEARCH_INTERRUPT_TIME), search_term.as_str()) {
						SearchResult::None | SearchResult::Updated => {},
						SearchResult::Complete => search_complete = true,
					}
					update_handler();
				}

				notifier.request_end();
				notifier.end();
			}
		});
	}

	#[inline]
	fn pause(&self) {
		self.state.pause();
	}

	#[inline]
	fn resume(&self) {
		self.state.resume();
	}

	#[inline]
	fn end(&self) {
		self.state.end();
	}
}

impl<UpdateHandler> Thread<UpdateHandler>
where UpdateHandler: Fn() + Sync + Send
{
	pub(crate) fn new(search_update_handler: UpdateHandler) -> Self {
		Self {
			state: State::new(),
			search_update_handler: Arc::new(search_update_handler),
		}
	}

	pub(crate) fn state(&self) -> State {
		self.state.clone()
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};

	use parking_lot::Mutex;
	use runtime::{testutils::ThreadableTester, Status};

	use super::*;

	#[derive(Clone)]
	struct MockedSearchable {
		calls: Arc<Mutex<Vec<String>>>,
		search_result: Arc<Mutex<SearchResult>>,
	}

	impl MockedSearchable {
		fn new() -> Self {
			Self {
				calls: Arc::new(Mutex::new(vec![])),
				search_result: Arc::new(Mutex::new(SearchResult::None)),
			}
		}
	}

	impl Searchable for MockedSearchable {
		fn reset(&mut self) {
			self.calls.lock().push(String::from("Reset"));
		}

		fn search(&mut self, _: Interrupter, term: &str) -> SearchResult {
			self.calls.lock().push(format!("Search({term})"));
			*self.search_result.lock()
		}
	}

	#[test]
	fn set_pause_resume() {
		let thread = Thread::new(|| {});
		let state = thread.state();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Waiting);
		thread.pause();
		assert!(state.is_paused());
		// give thread time to pause
		sleep(Duration::from_secs(1));
		state.send_update(Action::Continue);
		thread.resume();
		assert!(!state.is_paused());
		state.end();
		tester.wait_for_status(&Status::Ended);
	}

	#[test]
	fn set_end() {
		let thread = Thread::new(|| {});
		let state = thread.state();
		thread.end();
		assert!(state.is_ended());
	}

	#[test]
	fn thread_end_from_state() {
		let thread = Thread::new(|| {});
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Waiting);
		state.end();
		tester.wait_for_status(&Status::Ended);
	}

	#[test]
	fn thread_end_from_action() {
		let thread = Thread::new(|| {});
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Waiting);
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);
	}

	#[test]
	fn thread_start_search() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Waiting);
		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)")]);
	}

	#[test]
	fn thread_start_cancel() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::Cancel);
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Relaxed), 1);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Reset")]);
	}

	#[test]
	fn thread_start_continue() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::Continue);
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 2);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Search(foo)")]);
	}

	#[test]
	fn thread_no_updates_after_complete() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		*searchable.search_result.lock() = SearchResult::Complete;
		state.send_update(Action::Continue);
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)")]);
	}

	#[test]
	fn thread_no_updates_on_empty_term() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::new()));
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 0);
		let calls = searchable.calls.lock();
		assert!(calls.is_empty());
	}

	#[test]
	fn thread_no_additional_updates_on_start_with_same_term() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 1);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)")]);
	}

	#[test]
	fn thread_no_updates_on_no_searchable() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::Start(String::from("foo")));
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 0);
	}

	#[test]
	fn thread_updates_after_timeout() {
		let update_handler_calls = Arc::new(AtomicUsize::new(0));
		let update_handler_calls_thread = Arc::clone(&update_handler_calls);
		let thread = Thread::new(move || {
			let _ = update_handler_calls_thread.fetch_add(1, Ordering::Release);
		});
		let state = thread.state();

		let searchable = MockedSearchable::new();
		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		state.send_update(Action::SetSearchable(Box::new(searchable.clone())));
		state.send_update(Action::Start(String::from("foo")));
		sleep(Duration::from_millis(750)); // will timeout after 500ms
		state.send_update(Action::End);
		tester.wait_for_status(&Status::Ended);

		assert_eq!(update_handler_calls.load(Ordering::Acquire), 2);
		let calls = searchable.calls.lock();
		assert_eq!(*calls, vec![String::from("Search(foo)"), String::from("Search(foo)")]);
	}
}
