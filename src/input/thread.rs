mod state;

use std::{
	sync::Arc,
	thread::sleep,
	time::{Duration, Instant},
};

use captur::capture;

pub(crate) use self::state::State;
use crate::{
	input::{Event, EventReaderFn},
	runtime::{Installer, Threadable},
};

/// The name of the input thread.
pub(crate) const THREAD_NAME: &str = "input";
const MINIMUM_PAUSE_RATE: Duration = Duration::from_millis(250);

/// A thread for reading and handling input events.
#[derive(Debug)]
pub(crate) struct Thread<EventProvider, CustomEvent>
where
	EventProvider: EventReaderFn,
	CustomEvent: crate::input::CustomEvent + 'static,
{
	event_provider: Arc<EventProvider>,
	state: State<CustomEvent>,
}

impl<EventProvider, CustomEvent> Threadable for Thread<EventProvider, CustomEvent>
where
	EventProvider: EventReaderFn,
	CustomEvent: crate::input::CustomEvent + Send + Sync + 'static,
{
	fn install(&self, installer: &Installer) {
		let state = self.state();
		let event_provider = Arc::clone(&self.event_provider);

		installer.spawn(THREAD_NAME, |notifier| {
			move || {
				capture!(notifier, state, event_provider);
				let mut time = Instant::now();
				notifier.busy();
				while !state.is_ended() {
					while state.is_paused() {
						notifier.wait();
						sleep(time.saturating_duration_since(Instant::now()));
						time += MINIMUM_PAUSE_RATE;
					}
					notifier.busy();
					if let Ok(Some(event)) = (event_provider)() {
						state.enqueue_event(Event::from(event));
					}
				}

				notifier.end();
				notifier.request_end();
			}
		});
	}

	fn pause(&self) {
		self.state.pause();
	}

	fn resume(&self) {
		self.state.resume();
	}

	fn end(&self) {
		self.state.end();
	}
}

impl<EventProvider, CustomEvent> Thread<EventProvider, CustomEvent>
where
	EventProvider: EventReaderFn,
	CustomEvent: crate::input::CustomEvent + 'static,
{
	/// Create a new instance of a thread.
	pub(crate) fn new(event_provider: EventProvider) -> Self {
		Self {
			event_provider: Arc::new(event_provider),
			state: State::new(),
		}
	}

	/// Get a cloned copy of the state of the thread.
	#[must_use]
	pub(crate) fn state(&self) -> State<CustomEvent> {
		self.state.clone()
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;
	use crossterm::event::{KeyCode, KeyModifiers};

	use super::*;
	use crate::{
		input::{
			testutil::local::{create_event_reader, TestEvent},
			KeyEvent,
		},
		runtime::{testutils::ThreadableTester, Status},
	};

	#[test]
	fn set_pause_resume() {
		let event_provider = create_event_reader(|| Ok(None));
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();
		thread.pause();
		assert!(state.is_paused());
		thread.resume();
		assert!(!state.is_paused());
	}

	#[test]
	fn set_end() {
		let event_provider = create_event_reader(|| Ok(None));
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();
		thread.end();
		assert!(state.is_ended());
	}

	#[test]
	fn read_event_from_event_provider() {
		let event_provider = create_event_reader(|| {
			Ok(Some(Event::Key(KeyEvent::new(
				KeyCode::Char('a'),
				KeyModifiers::empty(),
			))))
		});
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);

		let event_received;
		loop {
			let event = state.read_event();
			if event != Event::None {
				event_received = event;
				break;
			}
		}
		state.end();

		assert_eq!(event_received, Event::from('a'));
	}

	#[test]
	fn read_none_event() {
		let event_provider = create_event_reader(|| Ok(None));
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Busy);
		let event_received = state.read_event();
		state.end();
		tester.wait_for_finished();
		assert_eq!(event_received, Event::None);
	}

	#[test]
	fn read_error() {
		let event_provider = create_event_reader(|| Err(anyhow!("Err")));
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Busy);
		let event_received = state.read_event();
		state.end();
		tester.wait_for_finished();
		assert_eq!(event_received, Event::None);
	}

	#[test]
	fn pause_resume() {
		let event_provider = create_event_reader(|| Ok(None));
		let thread: Thread<_, TestEvent> = Thread::new(event_provider);
		let state = thread.state();

		let tester = ThreadableTester::new();
		tester.start_threadable(&thread, THREAD_NAME);
		tester.wait_for_status(&Status::Busy);
		state.pause();
		tester.wait_for_status(&Status::Waiting);
		state.resume();
		tester.wait_for_status(&Status::Busy);
		state.end();
		tester.wait_for_finished();
	}
}
