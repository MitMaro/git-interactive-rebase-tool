use input::StandardEvent;
use view::ViewSender;

use crate::events::Event;
#[macro_export]
macro_rules! select {
	(default $default: expr, $first: expr) => {
		if let Some(value) = $first() {
			value
		}
		else {
			$default()
		}
	};
	(default $default: expr, $first: expr, $($arg:expr),*) => {
		if let Some(value) = $first() {
			value
		}
		$(else if let Some(value) = $arg() {
			value
		})*
		else {
			$default()
		}
	};
}

#[macro_export]
macro_rules! first {
	($first: expr, $($arg:expr),*) => {
		if $first().is_some() {
		}
		$(else if $arg().is_some() {
		})*
	};
}

/// Utility function to handle scroll events.
#[inline]
#[must_use]
#[allow(clippy::wildcard_enum_match_arm)]
pub(crate) fn handle_view_data_scroll(event: Event, view_sender: &ViewSender) -> Option<Event> {
	match event {
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollLeft => view_sender.scroll_left(),
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollRight => view_sender.scroll_right(),
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollDown => view_sender.scroll_down(),
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollUp => view_sender.scroll_up(),
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollJumpDown => view_sender.scroll_page_down(),
		Event::Standard(meta_event) if meta_event == StandardEvent::ScrollJumpUp => view_sender.scroll_page_up(),
		_ => return None,
	};
	Some(event)
}

#[cfg(test)]
mod tests {
	use captur::capture;
	use rstest::rstest;
	use view::testutil::with_view_sender;

	use super::*;

	#[rstest]
	#[case::scroll_left(StandardEvent::ScrollLeft, "ScrollLeft")]
	#[case::scroll_right(StandardEvent::ScrollRight, "ScrollRight")]
	#[case::scroll_down(StandardEvent::ScrollDown, "ScrollDown")]
	#[case::scroll_up(StandardEvent::ScrollUp, "ScrollUp")]
	#[case::jump_down(StandardEvent::ScrollJumpDown, "PageDown")]
	#[case::jump_up(StandardEvent::ScrollJumpUp, "PageUp")]
	fn handle_view_data_scroll_event(#[case] meta_event: StandardEvent, #[case] action: &str) {
		with_view_sender(|context| {
			capture!(action);
			let event = Event::from(meta_event);
			assert_eq!(handle_view_data_scroll(event, &context.sender), Some(event));
			context.assert_render_action(&[action]);
		});
	}

	#[test]
	fn handle_view_data_scroll_event_other() {
		with_view_sender(|context| {
			let event = Event::from('a');
			assert!(handle_view_data_scroll(event, &context.sender).is_none());
			context.assert_render_action(&[]);
		});
	}
}
