use std::{
	borrow::BorrowMut,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use crossbeam_channel::unbounded;
use parking_lot::Mutex;

use crate::{RenderSlice, ViewAction, ViewData};

/// Represents a message sender and receiver for passing actions between threads.
#[derive(Clone, Debug)]
pub struct State {
	ended: Arc<AtomicBool>,
	paused: Arc<AtomicBool>,
	render_slice: Arc<Mutex<RenderSlice>>,
	pub(crate) update_receiver: crossbeam_channel::Receiver<ViewAction>,
	update_sender: crossbeam_channel::Sender<ViewAction>,
}

impl State {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		let (update_sender, update_receiver) = unbounded();
		Self {
			ended: Arc::new(AtomicBool::from(false)),
			paused: Arc::new(AtomicBool::from(false)),
			render_slice: Arc::new(Mutex::new(RenderSlice::new())),
			update_receiver,
			update_sender,
		}
	}

	fn send_update(&self, action: ViewAction) {
		self.update_sender.send(action).unwrap();
	}

	pub(crate) fn update_receiver(&self) -> crossbeam_channel::Receiver<ViewAction> {
		self.update_receiver.clone()
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Relaxed)
	}

	pub(crate) fn is_paused(&self) -> bool {
		self.paused.load(Ordering::Relaxed)
	}

	/// Clone the render slice.
	#[inline]
	#[must_use]
	pub fn render_slice(&self) -> Arc<Mutex<RenderSlice>> {
		Arc::clone(&self.render_slice)
	}

	/// Queue a start action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn start(&self) {
		self.paused.store(false, Ordering::Release);
		self.send_update(ViewAction::Start);
	}

	/// Pause the event read thread.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn stop(&self) {
		self.paused.store(true, Ordering::Release);
		self.send_update(ViewAction::Stop);
	}

	/// Queue an end action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn end(&self) {
		self.ended.store(true, Ordering::Release);
		self.stop();
		self.send_update(ViewAction::End);
	}

	/// Queue a refresh action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn refresh(&self) {
		self.send_update(ViewAction::Refresh);
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_top(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_top();
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_bottom(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_bottom();
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_up(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_up();
	}

	/// Queue a scroll down action.
	#[inline]
	pub fn scroll_down(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_down();
	}

	/// Queue a scroll left action.
	#[inline]
	pub fn scroll_left(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_left();
	}

	/// Queue a scroll right action.
	#[inline]
	pub fn scroll_right(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_right();
	}

	/// Queue a scroll up a page action.
	#[inline]
	pub fn scroll_page_up(&self) {
		self.render_slice.lock().borrow_mut().record_page_up();
	}

	/// Queue a scroll down a page action.
	#[inline]
	pub fn scroll_page_down(&self) {
		self.render_slice.lock().borrow_mut().record_page_down();
	}

	/// Queue a resize action.
	#[inline]
	pub fn resize(&self, width: u16, height: u16) {
		self.render_slice
			.lock()
			.borrow_mut()
			.record_resize(width as usize, height as usize);
	}

	/// Sync the `ViewData` and queue a render action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn render(&self, view_data: &ViewData) {
		self.render_slice.lock().borrow_mut().sync_view_data(view_data);
		self.send_update(ViewAction::Render);
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		testutil::{render_view_line, with_view_state},
		ViewData,
		ViewLine,
	};

	#[test]
	fn start() {
		with_view_state(|context| {
			context.state.start();
			context.assert_sent_messages(vec!["Start"]);
			assert!(!context.state.is_paused());
		});
	}

	#[test]
	fn stop() {
		with_view_state(|context| {
			context.state.stop();
			context.assert_sent_messages(vec!["Stop"]);
			assert!(context.state.is_paused());
		});
	}

	#[test]
	fn end() {
		with_view_state(|context| {
			context.state.end();
			context.assert_sent_messages(vec!["Stop", "End"]);
		});
	}

	#[test]
	fn refresh() {
		with_view_state(|context| {
			context.state.refresh();
			context.assert_sent_messages(vec!["Refresh"]);
		});
	}

	#[test]
	fn scroll_top() {
		with_view_state(|context| {
			context.state.scroll_top();
			context.assert_render_action(&["ScrollTop"]);
		});
	}

	#[test]
	fn scroll_bottom() {
		with_view_state(|context| {
			context.state.scroll_bottom();
			context.assert_render_action(&["ScrollBottom"]);
		});
	}

	#[test]
	fn scroll_up() {
		with_view_state(|context| {
			context.state.scroll_up();
			context.assert_render_action(&["ScrollUp"]);
		});
	}

	#[test]
	fn scroll_down() {
		with_view_state(|context| {
			context.state.scroll_down();
			context.assert_render_action(&["ScrollDown"]);
		});
	}

	#[test]
	fn scroll_left() {
		with_view_state(|context| {
			context.state.scroll_left();
			context.assert_render_action(&["ScrollLeft"]);
		});
	}

	#[test]
	fn scroll_right() {
		with_view_state(|context| {
			context.state.scroll_right();
			context.assert_render_action(&["ScrollRight"]);
		});
	}

	#[test]
	fn scroll_page_up() {
		with_view_state(|context| {
			context.state.scroll_page_up();
			context.assert_render_action(&["PageUp"]);
		});
	}

	#[test]
	fn scroll_page_down() {
		with_view_state(|context| {
			context.state.scroll_page_down();
			context.assert_render_action(&["PageDown"]);
		});
	}

	#[test]
	fn resize() {
		with_view_state(|context| {
			context.state.resize(10, 20);
			context.assert_render_action(&["Resize(10, 20)"]);
		});
	}

	#[test]
	fn render() {
		with_view_state(|context| {
			context.state.resize(300, 100);
			context
				.state
				.render(&ViewData::new(|updater| updater.push_line(ViewLine::from("Foo"))));
			assert_eq!(
				render_view_line(context.state.render_slice().lock().get_lines().first().unwrap(), None),
				"Foo"
			);
		});
	}
}
