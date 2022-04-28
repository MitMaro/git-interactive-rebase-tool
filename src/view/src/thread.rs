use std::{
	borrow::Borrow,
	sync::atomic::Ordering,
	thread::{sleep, spawn, JoinHandle},
	time::{Duration, Instant},
};

use crossbeam_channel::unbounded;
use display::Tui;

use super::{action::ViewAction, sender::Sender, View};

const MINIMUM_TICK_RATE: Duration = Duration::from_millis(20); // ~50 Hz update

/// Spawn a thread that will handle rendering contents to the `View`.
///
/// # Panics
/// This may panic if there is an unexpected error in the processing of the `View`, i.e. a bug.
#[inline]
pub fn spawn_view_thread<T: Tui + Send + 'static>(mut view: View<T>) -> (Sender, JoinHandle<()>) {
	let (sender, receiver) = unbounded();
	let view_sender = Sender::new(sender.clone());
	let refresh_thread_view_sender = view_sender.clone();
	let view_render_slice = view_sender.clone_render_slice();
	let crashed = view_sender.clone_poisoned();

	let thread = spawn(move || {
		let mut last_render_time = Instant::now() + MINIMUM_TICK_RATE;
		let mut should_render = true;
		for msg in receiver {
			let mut err = false;
			match msg {
				ViewAction::Render => should_render = true,
				ViewAction::Start => {
					if view.start().is_err() {
						err = true;
					}
				},
				ViewAction::Stop => {
					if view.end().is_err() {
						err = true;
					}
				},
				ViewAction::Refresh => {},
				ViewAction::End => break,
			}
			if should_render && Instant::now() >= last_render_time {
				last_render_time += MINIMUM_TICK_RATE;
				should_render = false;
				let render_slice = view_render_slice.lock();
				if view.render(render_slice.borrow()).is_err() {
					err = true;
				}
			}
			if err {
				crashed.store(true, Ordering::Release);
			}
		}
	});

	let _refresh_thread = spawn(move || {
		let sleep_time = MINIMUM_TICK_RATE / 2;
		let mut time = Instant::now();
		while sender.send(ViewAction::Refresh).is_ok() {
			loop {
				sleep(time.saturating_duration_since(Instant::now()));
				time += sleep_time;
				if !refresh_thread_view_sender.is_paused() {
					break;
				}
			}
		}
	});

	(view_sender, thread)
}
