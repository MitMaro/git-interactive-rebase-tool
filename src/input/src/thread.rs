use std::{
	sync::atomic::Ordering,
	thread::{spawn, JoinHandle},
};

use anyhow::Result;
use crossbeam_channel::{bounded, unbounded};

use crate::{event::Event, event_action::EventAction, sender::Sender};

const MAXIMUM_EVENTS: usize = 100;

/// Spawn a thead for handling events.
///
/// # Panics
/// This may panic if there is an unexpected error in the processing of an event, i.e. a bug.
#[inline]
#[allow(clippy::module_name_repetitions)]
pub fn spawn_event_thread<F: Send + 'static, CustomEvent: crate::CustomEvent + Send + 'static>(
	event_provider: F,
) -> (Sender<CustomEvent>, JoinHandle<()>)
where F: Fn() -> Result<Option<crossterm::event::Event>> {
	let (sender, receiver) = bounded(0);
	let (new_event_sender, new_event_receiver) = unbounded();
	let event_sender = Sender::new(sender, new_event_receiver);
	let event_queue = event_sender.clone_event_queue();
	let push_thread_event_sender = event_sender.clone();
	let poisoned = event_sender.clone_poisoned();
	let thread = spawn(move || {
		for msg in receiver {
			match msg {
				EventAction::End => {
					poisoned.store(true, Ordering::Relaxed);
					break;
				},
				EventAction::EnqueueEvent(event) => {
					let mut events = event_queue.lock();
					if events.len() < MAXIMUM_EVENTS {
						events.push_back(event);
					}
					let _send_result = new_event_sender.send(());
				},
				EventAction::PushEvent(event) => {
					let mut events = event_queue.lock();
					if events.len() >= MAXIMUM_EVENTS {
						let _ = events.pop_back();
					}
					events.push_front(event);
					let _send_result = new_event_sender.send(());
				},
			}
		}
	});

	let _push_events_thread = spawn(move || {
		while !push_thread_event_sender.is_poisoned() {
			if let Ok(Some(event)) = (event_provider)() {
				let _result = push_thread_event_sender.enqueue_event(Event::from(event));
			}
		}
	});

	(event_sender, thread)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil::local::TestEvent;

	fn spawn_event_thread<F: Send + 'static>(event_provider: F) -> (Sender<TestEvent>, JoinHandle<()>)
	where F: Fn() -> Result<Option<crossterm::event::Event>> {
		super::spawn_event_thread(event_provider)
	}

	#[test]
	fn thread_enqueue_event_from_provider() {
		let (mut sender, _thread) = spawn_event_thread(|| {
			Ok(Some(crossterm::event::Event::Key(crossterm::event::KeyEvent::new(
				crossterm::event::KeyCode::Char('a'),
				crossterm::event::KeyModifiers::empty(),
			))))
		});

		let received = loop {
			let event = sender.read_event();
			if event != Event::None {
				break event;
			}
		};
		// end after reading event loop, so that the _push_events_thread has a chance to read the
		// event before the thread is closed
		sender.end().unwrap();
		while !sender.is_poisoned() {}

		assert_eq!(received, Event::from('a'));
	}

	#[test]
	fn thread_enqueue_event() {
		let (mut sender, _thread) = spawn_event_thread(|| Ok(None));

		sender.enqueue_event(Event::from('a')).unwrap();
		sender.enqueue_event(Event::from('b')).unwrap();
		sender.end().unwrap();
		while !sender.is_poisoned() {}

		let mut events_received = vec![];
		loop {
			let event = sender.read_event();
			if event != Event::None {
				events_received.push(event);
			}

			if events_received.len() == 2 {
				break;
			}
		}

		assert_eq!(events_received, vec![Event::from('a'), Event::from('b')]);
	}

	#[test]
	fn thread_enqueue_event_overflow() {
		let (mut sender, _thread) = spawn_event_thread(|| Ok(None));

		for _ in 0..150 {
			sender.enqueue_event(Event::from('a')).unwrap();
		}
		sender.enqueue_event(Event::from('b')).unwrap();
		sender.end().unwrap();
		while !sender.is_poisoned() {}

		let mut events_received = vec![];
		loop {
			let event = sender.read_event();
			if event != Event::None {
				events_received.push(event);
			}

			if events_received.len() == 100 {
				break;
			}
		}

		assert_eq!(sender.read_event(), Event::None);
		assert_ne!(events_received.last().unwrap(), &Event::from('b'));
	}

	#[test]
	fn thread_push_event() {
		let (mut sender, _thread) = spawn_event_thread(|| Ok(None));

		sender.push_event(Event::from('a')).unwrap();
		sender.push_event(Event::from('b')).unwrap();
		sender.end().unwrap();
		while !sender.is_poisoned() {}

		let mut events_received = vec![];
		loop {
			let event = sender.read_event();
			if event != Event::None {
				events_received.push(event);
			}

			if events_received.len() == 2 {
				break;
			}
		}

		assert_eq!(events_received, vec![Event::from('b'), Event::from('a')]);
	}

	#[test]
	fn thread_push_event_overflow() {
		let event_provider = || Ok(None);
		let (mut sender, _thread) = spawn_event_thread(event_provider);

		for _ in 0..100 {
			sender.push_event(Event::from('a')).unwrap();
		}
		sender.push_event(Event::from('b')).unwrap();
		sender.end().unwrap();
		while !sender.is_poisoned() {}

		let mut events_received = vec![];
		loop {
			let event = sender.read_event();
			if event != Event::None {
				events_received.push(event);
			}

			if events_received.len() == 100 {
				break;
			}
		}

		assert_eq!(sender.read_event(), Event::None);
		assert_eq!(events_received.first().unwrap(), &Event::from('b'));
		assert_eq!(events_received.last().unwrap(), &Event::from('a'));
	}
}
