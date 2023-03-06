use std::{
	collections::VecDeque,
	mem,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration,
};

use parking_lot::Mutex;

use crate::Event;

const MAXIMUM_EVENTS: usize = 100;
const EVENT_POLL_TIMEOUT: Duration = Duration::from_secs(1);

/// Input thread state.
#[derive(Clone, Debug)]
pub struct State<CustomEvent: crate::CustomEvent> {
	ended: Arc<AtomicBool>,
	event_queue: Arc<Mutex<VecDeque<Event<CustomEvent>>>>,
	paused: Arc<AtomicBool>,
	update_receiver: crossbeam_channel::Receiver<()>,
	update_sender: crossbeam_channel::Sender<()>,
}

impl<CustomEvent: crate::CustomEvent> State<CustomEvent> {
	pub(crate) fn new() -> Self {
		let (update_sender, update_receiver) = crossbeam_channel::unbounded();
		Self {
			ended: Arc::new(AtomicBool::from(false)),
			event_queue: Arc::new(Mutex::new(VecDeque::new())),
			paused: Arc::new(AtomicBool::from(false)),
			update_receiver,
			update_sender,
		}
	}

	fn send_update(&self) {
		let _result = self.update_sender.send(());
	}

	pub(crate) fn is_paused(&self) -> bool {
		self.paused.load(Ordering::Acquire)
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Acquire)
	}

	/// Pause the event read thread.
	#[inline]
	pub fn pause(&self) {
		self.paused.store(true, Ordering::Release);
	}

	/// Resume the event read thread.
	#[inline]
	pub fn resume(&self) {
		self.paused.store(false, Ordering::Release);
	}

	/// Permanently End the event read thread.
	#[inline]
	pub fn end(&self) {
		self.ended.store(true, Ordering::Release);
	}

	/// Add an event after existing events.
	#[inline]
	pub fn enqueue_event(&self, event: Event<CustomEvent>) {
		let mut events = self.event_queue.lock();
		let last_resize_event_maybe = matches!(event, Event::Resize(..))
			.then(|| events.back_mut().filter(|e| matches!(*e, &mut Event::Resize(..))))
			.flatten();

		if let Some(last_resize_event) = last_resize_event_maybe {
			let _old = mem::replace(last_resize_event, event);
		}
		else if events.len() < MAXIMUM_EVENTS {
			events.push_back(event);
		}
		self.send_update();
	}

	/// Add an event before existing events.
	#[inline]
	pub fn push_event(&self, event: Event<CustomEvent>) {
		let mut events = self.event_queue.lock();
		if events.len() >= MAXIMUM_EVENTS {
			_ = events.pop_back();
		}
		events.push_front(event);
		self.send_update();
	}

	/// Read an event from the queue. This function will block for a while until an event is
	/// available. And if no event is available, it will return `Event::None`.
	#[inline]
	#[must_use]
	pub fn read_event(&self) -> Event<CustomEvent> {
		// clear existing message since last read
		while self.update_receiver.try_recv().is_ok() {}
		loop {
			if let Some(event) = self.event_queue.lock().pop_front() {
				return event;
			}

			// if there is no event available on the queue, instead of returning early, we can wait
			// for the new event message and try again.
			if self.update_receiver.recv_timeout(EVENT_POLL_TIMEOUT).is_ok() {
				continue;
			}

			// We always return if the above recv call times out, to ensure this does not block
			// forever
			return Event::None;
		}
	}
}

#[cfg(test)]
mod tests {
	use std::{
		sync::atomic::AtomicUsize,
		thread::{sleep, spawn},
	};

	use super::*;
	use crate::testutil::local::{Event, TestEvent};

	fn create_state() -> State<TestEvent> {
		State::new()
	}

	#[test]
	fn paused() {
		let state = create_state();
		state.pause();
		assert!(state.is_paused());
	}

	#[test]
	fn resumed() {
		let state = create_state();
		state.resume();
		assert!(!state.is_paused());
	}

	#[test]
	fn ended() {
		let state = create_state();
		state.end();
		assert!(state.is_ended());
	}

	#[test]
	fn enqueue_event() {
		let state = create_state();
		state.enqueue_event(Event::from('a'));
		state.enqueue_event(Event::from('b'));

		assert_eq!(state.read_event(), Event::from('a'));
		assert_eq!(state.read_event(), Event::from('b'));
	}

	#[test]
	fn enqueue_event_resize_last_follow_by_non_resize() {
		let state = create_state();
		state.enqueue_event(Event::Resize(1, 1));
		state.enqueue_event(Event::from('a'));

		assert_eq!(state.read_event(), Event::Resize(1, 1));
		assert_eq!(state.read_event(), Event::from('a'));
	}

	#[test]
	fn enqueue_event_resize_last_follow_by_new_resize() {
		let state = create_state();
		state.enqueue_event(Event::Resize(1, 1));
		state.enqueue_event(Event::Resize(2, 2));

		assert_eq!(state.read_event(), Event::Resize(2, 2));
		assert_eq!(state.read_event(), Event::None);
	}

	#[test]
	fn enqueue_event_overflow() {
		let state = create_state();
		// fill queue
		for _ in 0..MAXIMUM_EVENTS {
			state.enqueue_event(Event::from('a'));
		}
		state.enqueue_event(Event::from('b'));

		let mut events_received = vec![];
		loop {
			let event = state.read_event();
			if event == Event::None {
				break;
			}
			events_received.push(event);
		}

		assert_eq!(state.read_event(), Event::None);
		assert_eq!(events_received.len(), MAXIMUM_EVENTS);
		assert_eq!(events_received.first().unwrap(), &Event::from('a'));
		assert_eq!(events_received.last().unwrap(), &Event::from('a'));
	}

	#[test]
	fn push_event() {
		let state = create_state();
		state.push_event(Event::from('a'));
		state.push_event(Event::from('b'));

		assert_eq!(state.read_event(), Event::from('b'));
		assert_eq!(state.read_event(), Event::from('a'));
	}

	#[test]
	fn push_event_overflow() {
		let state = create_state();
		// fill queue
		for _ in 0..MAXIMUM_EVENTS {
			state.push_event(Event::from('a'));
		}
		state.push_event(Event::from('b'));

		let mut events_received = vec![];
		loop {
			let event = state.read_event();
			if event == Event::None {
				break;
			}
			events_received.push(event);
		}

		assert_eq!(state.read_event(), Event::None);
		assert_eq!(events_received.len(), MAXIMUM_EVENTS);
		assert_eq!(events_received.first().unwrap(), &Event::from('b'));
		assert_eq!(events_received.last().unwrap(), &Event::from('a'));
	}

	#[test]
	fn read_event() {
		// STEPS:
		// 0 -> thread: initial event read with timeout, returns None
		//      test: waits for initial event read with timeout to occur (moves to step 1)
		// 1 -> thread: waits for step 1 to complete
		//      test: enqueues new event (moves to step 2)
		// 2 -> tread: reads enqueued event (moves to step 3)
		//      test: waits for step 2 to complete
		// 3 -> thead: ended, no action
		//      test: assert events read match

		let state = create_state();

		let step: Arc<Mutex<AtomicUsize>> = Arc::new(Mutex::new(AtomicUsize::new(0)));
		let events_read: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(vec![]));

		let thread_state = state.clone();
		let thread_step = Arc::clone(&step);
		let thread_events_read = Arc::clone(&events_read);
		_ = spawn(move || {
			loop {
				let mut thread_events_read_lock = thread_events_read.lock();
				let thread_step_lock = thread_step.lock();
				match thread_step_lock.load(Ordering::Acquire) {
					0 => {
						let event = thread_state.read_event();
						thread_events_read_lock.push(event);
						thread_step_lock.store(1, Ordering::Release);
					},
					1 => {
						sleep(Duration::from_millis(10));
					},
					2 => {
						let event = thread_state.read_event();
						thread_events_read_lock.push(event);
						thread_step_lock.store(3, Ordering::Release);
						break;
					},
					_ => unreachable!(),
				}
			}
		});

		while step.lock().load(Ordering::Acquire) != 1 {
			sleep(Duration::from_millis(10));
		}
		state.enqueue_event(Event::from('a'));
		step.lock().store(2, Ordering::Release);

		while step.lock().load(Ordering::Acquire) == 2 {
			sleep(Duration::from_millis(10));
		}

		let mut events_read_lock = events_read.lock();
		assert_eq!(events_read_lock.pop().unwrap(), Event::from('a'));
		assert_eq!(events_read_lock.pop().unwrap(), Event::None);
	}
}
