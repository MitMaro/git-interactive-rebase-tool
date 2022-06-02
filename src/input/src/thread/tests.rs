use std::thread::JoinHandle;

use super::*;
use crate::testutil::local::TestEvent;

fn spawn_event_thread<F: Send + 'static>(event_provider: F) -> (crossbeam_channel::Sender<TestEvent>, JoinHandle<()>)
where F: Fn() -> Result<Option<crossterm::event::Event>> {
	super::spawn_event_thread(event_provider)
}

#[test]
fn thread_pause_resume() {
	// setup event provider to continuously provide a key event
	let (mut sender, _thread) = spawn_event_thread(|| {
		Ok(Some(crossterm::event::Event::Key(crossterm::event::KeyEvent::new(
			crossterm::event::KeyCode::Char('a'),
			crossterm::event::KeyModifiers::empty(),
		))))
	});

	sender.pause();
	sender.clone_event_queue().lock().clear(); // remove any events that were already enqueued
	assert_eq!(sender.read_event(), Event::None); // sadly this will pause for a second
	sender.resume();
	assert_eq!(sender.read_event(), Event::from('a'));
	sender.end().unwrap();
	while !sender.is_poisoned() {}
}
