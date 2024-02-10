use crate::{events::Event, input::StandardEvent};

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
pub(crate) fn handle_view_data_scroll(event: Event, view_state: &crate::view::State) -> Option<Event> {
	match event {
		Event::Standard(StandardEvent::ScrollLeft) => view_state.scroll_left(),
		Event::Standard(StandardEvent::ScrollRight) => view_state.scroll_right(),
		Event::Standard(StandardEvent::ScrollDown) => view_state.scroll_down(),
		Event::Standard(StandardEvent::ScrollUp) => view_state.scroll_up(),
		Event::Standard(StandardEvent::ScrollTop) => view_state.scroll_top(),
		Event::Standard(StandardEvent::ScrollBottom) => view_state.scroll_bottom(),
		Event::Standard(StandardEvent::ScrollJumpDown) => view_state.scroll_page_down(),
		Event::Standard(StandardEvent::ScrollJumpUp) => view_state.scroll_page_up(),
		_ => return None,
	};
	Some(event)
}

#[cfg(test)]
mod tests {
	use captur::capture;
	use rstest::rstest;

	use super::*;
	use crate::view::testutil::with_view_state;

	#[rstest]
	#[case::scroll_left(StandardEvent::ScrollLeft, "ScrollLeft")]
	#[case::scroll_right(StandardEvent::ScrollRight, "ScrollRight")]
	#[case::scroll_down(StandardEvent::ScrollDown, "ScrollDown")]
	#[case::scroll_up(StandardEvent::ScrollUp, "ScrollUp")]
	#[case::scroll_top(StandardEvent::ScrollTop, "ScrollTop")]
	#[case::scroll_bottom(StandardEvent::ScrollBottom, "ScrollBottom")]
	#[case::jump_down(StandardEvent::ScrollJumpDown, "PageDown")]
	#[case::jump_up(StandardEvent::ScrollJumpUp, "PageUp")]
	fn handle_view_data_scroll_event(#[case] meta_event: StandardEvent, #[case] action: &str) {
		with_view_state(|context| {
			capture!(action);
			let event = Event::from(meta_event);
			assert_eq!(handle_view_data_scroll(event, &context.state), Some(event));
			context.assert_render_action(&[action]);
		});
	}

	#[test]
	fn handle_view_data_scroll_event_other() {
		with_view_state(|context| {
			let event = Event::from('a');
			assert!(handle_view_data_scroll(event, &context.state).is_none());
			context.assert_render_action(&[]);
		});
	}
}
