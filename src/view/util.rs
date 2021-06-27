use input::{Event, MetaEvent};

use super::ViewSender;

pub fn handle_view_data_scroll(event: Event, view_sender: &ViewSender) -> Option<Event> {
	match event {
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollLeft => view_sender.scroll_left(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollRight => view_sender.scroll_right(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollDown => view_sender.scroll_down(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollUp => view_sender.scroll_up(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollJumpDown => view_sender.scroll_page_down(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollJumpUp => view_sender.scroll_page_up(),
		_ => return None,
	};
	Some(event)
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::view::testutil::with_view_sender;

	#[rstest(
		meta_event,
		action,
		case::scroll_left(MetaEvent::ScrollLeft, "ScrollLeft"),
		case::scroll_right(MetaEvent::ScrollRight, "ScrollRight"),
		case::scroll_down(MetaEvent::ScrollDown, "ScrollDown"),
		case::scroll_up(MetaEvent::ScrollUp, "ScrollUp"),
		case::jump_down(MetaEvent::ScrollJumpDown, "PageDown"),
		case::jump_up(MetaEvent::ScrollJumpUp, "PageUp")
	)]
	fn handle_view_data_scroll_event(meta_event: MetaEvent, action: &str) {
		with_view_sender(|context| {
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
