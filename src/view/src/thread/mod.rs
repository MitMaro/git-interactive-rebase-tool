mod action;
mod state;

use std::{
	borrow::Borrow,
	sync::Arc,
	thread::sleep,
	time::{Duration, Instant},
};

use captur::capture;
use display::Tui;
use parking_lot::Mutex;
use runtime::{Installer, RuntimeError, Threadable};

pub(crate) use self::action::ViewAction;
pub use self::state::State;
use crate::View;

/// The name of the main view thread.
pub const MAIN_THREAD_NAME: &str = "view_main";
/// The name of the view refresh thread.
pub const REFRESH_THREAD_NAME: &str = "view_refresh";

const MINIMUM_TICK_RATE: Duration = Duration::from_millis(20); // ~50 Hz update
const PAUSE_TIME: Duration = Duration::from_millis(230); // 250 ms total pause

/// A thread that updates the rendered view.
#[derive(Debug)]
pub struct Thread<ViewTui: Tui + Send + 'static> {
	state: State,
	view: Arc<Mutex<View<ViewTui>>>,
}

impl<ViewTui: Tui + Send + 'static> Threadable for Thread<ViewTui> {
	#[inline]
	fn install(&self, installer: &Installer) {
		self.install_message_thread(installer);
		self.install_refresh_thread(installer);
	}

	#[inline]
	fn pause(&self) {
		self.state.stop();
	}

	#[inline]
	fn resume(&self) {
		self.state.start();
	}

	#[inline]
	fn end(&self) {
		self.state.end();
	}
}

impl<ViewTui: Tui + Send + 'static> Thread<ViewTui> {
	/// Creates a new thread.
	#[inline]
	pub fn new(view: View<ViewTui>) -> Self {
		Self {
			state: State::new(),
			view: Arc::new(Mutex::new(view)),
		}
	}

	/// Returns a cloned copy of the state of the thread.
	#[inline]
	#[must_use]
	pub fn state(&self) -> State {
		self.state.clone()
	}

	fn install_message_thread(&self, installer: &Installer) {
		let view = Arc::clone(&self.view);
		let state = self.state();

		installer.spawn(MAIN_THREAD_NAME, |notifier| {
			move || {
				capture!(notifier, state);
				notifier.busy();
				state.start();
				notifier.wait();

				let render_slice = state.render_slice();
				let update_receiver = state.update_receiver();
				let mut last_render_time = Instant::now();
				let mut should_render = true;

				for msg in update_receiver {
					notifier.busy();
					match msg {
						ViewAction::Render => should_render = true,
						ViewAction::Start => {
							if let Err(err) = view.lock().start() {
								notifier.error(RuntimeError::ThreadError(err.to_string()));
								break;
							}
						},
						ViewAction::Stop => {
							if let Err(err) = view.lock().end() {
								notifier.error(RuntimeError::ThreadError(err.to_string()));
								break;
							}
						},
						ViewAction::Refresh => {},
						ViewAction::End => break,
					}

					if should_render && Instant::now() >= last_render_time {
						last_render_time += MINIMUM_TICK_RATE;
						should_render = false;
						let render_slice_mutex = render_slice.lock();
						if let Err(err) = view.lock().render(render_slice_mutex.borrow()) {
							notifier.error(RuntimeError::ThreadError(err.to_string()));
							break;
						}
					}
					notifier.wait();
				}

				notifier.request_end();
				notifier.end();
			}
		});
	}

	fn install_refresh_thread(&self, installer: &Installer) {
		let state = self.state();

		installer.spawn(REFRESH_THREAD_NAME, |notifier| {
			move || {
				capture!(notifier, state);
				notifier.wait();
				let sleep_time = MINIMUM_TICK_RATE / 2;
				let mut time = Instant::now();
				while !state.is_ended() {
					notifier.busy();
					state.refresh();
					notifier.wait();
					loop {
						sleep(time.saturating_duration_since(Instant::now()));
						time += sleep_time;
						if !state.is_paused() || state.is_ended() {
							break;
						}
						time += PAUSE_TIME;
					}
				}

				notifier.request_end();
				notifier.end();
			}
		});
	}
}

#[cfg(test)]
mod tests {
	use std::borrow::BorrowMut;

	use config::Theme;
	use display::{
		testutil::{create_unexpected_error, CrossTerm, MockableTui},
		Display,
		DisplayError,
	};
	use runtime::{testutils::ThreadableTester, Status};

	use super::*;
	use crate::ViewData;

	const READ_MESSAGE_TIMEOUT: Duration = Duration::from_secs(1);

	fn with_view<C, CT: MockableTui>(tui: CT, callback: C)
	where C: FnOnce(View<CT>) {
		let theme = Theme::new();
		let display = Display::new(tui, &theme);
		callback(View::new(display, "~", "?"));
	}

	#[test]
	fn set_pause_resume() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			thread.pause();
			assert!(state.is_paused());
			thread.resume();
			assert!(!state.is_paused());
		});
	}

	#[test]
	fn set_end() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			thread.end();
			assert!(state.is_ended());
		});
	}

	#[test]
	fn main_thread_end() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);

			tester.wait_for_status(&Status::Waiting);
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn main_thread_start() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn main_thread_start_error() {
		struct TestCrossTerm;

		impl MockableTui for TestCrossTerm {
			fn start(&mut self) -> Result<(), DisplayError> {
				Err(create_unexpected_error())
			}
		}

		with_view(TestCrossTerm {}, |view| {
			let thread = Thread::new(view);

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_error_status();
		});
	}

	#[test]
	fn main_thread_stop() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			state.stop();
			tester.wait_for_status(&Status::Waiting);
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn main_thread_stop_error() {
		struct TestCrossTerm;

		impl MockableTui for TestCrossTerm {
			fn end(&mut self) -> Result<(), DisplayError> {
				Err(create_unexpected_error())
			}
		}

		with_view(TestCrossTerm {}, |view| {
			let thread = Thread::new(view);
			let state = thread.state();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			state.stop();
			tester.wait_for_error_status();
		});
	}

	#[test]
	fn main_thread_render_with_should_render() {
		struct TestCrossTerm {
			lines: Arc<Mutex<Vec<String>>>,
		}

		impl TestCrossTerm {
			fn new() -> Self {
				Self {
					lines: Arc::new(Mutex::new(vec![])),
				}
			}
		}

		impl MockableTui for TestCrossTerm {
			fn print(&mut self, s: &str) -> Result<(), DisplayError> {
				self.lines.lock().push(String::from(s));
				Ok(())
			}
		}

		let crossterm = TestCrossTerm::new();
		let lines = Arc::clone(&crossterm.lines);

		with_view(crossterm, |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			state.resize(100, 1);
			let view_data = ViewData::new(|updater| {
				updater.push_lines("foo");
			});

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			state.render(&view_data);
			tester.wait_for_status(&Status::Waiting);
			state.end();
			tester.wait_for_status(&Status::Ended);
			for _ in 0..10 {
				let lines_lock = lines.lock();
				let line = lines_lock.first().unwrap();
				if line != "~" {
					assert_eq!(line, "foo");
					break;
				}
			}
		});
	}

	#[test]
	fn main_thread_render_with_should_render_error() {
		struct TestCrossTerm;

		impl MockableTui for TestCrossTerm {
			fn reset(&mut self) -> Result<(), DisplayError> {
				Err(create_unexpected_error())
			}
		}

		with_view(TestCrossTerm {}, |view| {
			let thread = Thread::new(view);
			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_error_status();
		});
	}

	#[test]
	fn main_thread_render_with_refresh() {
		struct TestCrossTerm {
			lines: Arc<Mutex<Vec<String>>>,
		}

		impl TestCrossTerm {
			fn new() -> Self {
				Self {
					lines: Arc::new(Mutex::new(vec![])),
				}
			}
		}

		impl MockableTui for TestCrossTerm {
			fn print(&mut self, s: &str) -> Result<(), DisplayError> {
				self.lines.lock().push(String::from(s));
				Ok(())
			}
		}

		let crossterm = TestCrossTerm::new();
		let lines = Arc::clone(&crossterm.lines);

		with_view(crossterm, |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			state.resize(100, 1);
			let view_data = ViewData::new(|updater| {
				updater.push_lines("foo");
			});
			let render_slice = state.render_slice();
			render_slice.lock().borrow_mut().sync_view_data(&view_data);

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, MAIN_THREAD_NAME);
			tester.wait_for_status(&Status::Waiting);
			sleep(MINIMUM_TICK_RATE); // give the refresh a chance to occur
			state.refresh();
			tester.wait_for_status(&Status::Waiting);
			state.end();
			tester.wait_for_status(&Status::Ended);
			for _ in 0..10 {
				let lines_lock = lines.lock();
				let line = lines_lock.first().unwrap();
				if line != "~" {
					break;
				}
			}
			assert_eq!(*lines.lock().first().unwrap(), "foo");
		});
	}

	#[test]
	fn refresh_thread_receive_and_end() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			let receiver = state.update_receiver();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, REFRESH_THREAD_NAME);

			assert!(matches!(
				receiver.recv_timeout(READ_MESSAGE_TIMEOUT).unwrap(),
				ViewAction::Refresh
			));

			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn refresh_thread_stop_resume() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			let receiver = state.update_receiver();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, REFRESH_THREAD_NAME);
			let _ = receiver.recv_timeout(READ_MESSAGE_TIMEOUT).unwrap();
			state.stop();
			tester.wait_for_status(&Status::Waiting);
			while receiver.recv_timeout(READ_MESSAGE_TIMEOUT).is_ok() {}
			assert!(receiver.recv_timeout(READ_MESSAGE_TIMEOUT).is_err());
			state.start();
			assert!(receiver.recv_timeout(READ_MESSAGE_TIMEOUT).is_ok());
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}

	#[test]
	fn refresh_thread_stop_end() {
		with_view(CrossTerm::new(), |view| {
			let thread = Thread::new(view);
			let state = thread.state();
			let receiver = state.update_receiver();

			let tester = ThreadableTester::new();
			tester.start_threadable(&thread, REFRESH_THREAD_NAME);
			let _ = receiver.recv_timeout(READ_MESSAGE_TIMEOUT).unwrap();
			state.stop();
			while receiver.recv_timeout(READ_MESSAGE_TIMEOUT).is_ok() {}
			state.end();
			tester.wait_for_status(&Status::Ended);
		});
	}
}
