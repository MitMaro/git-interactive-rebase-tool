use crate::{
	input::{Event, MetaEvent},
	view::view_data::ViewData,
};

pub fn handle_view_data_scroll(event: Event, view_data: &mut ViewData) -> Option<Event> {
	match event {
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollLeft => view_data.scroll_left(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollRight => view_data.scroll_right(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollDown => view_data.scroll_down(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollUp => view_data.scroll_up(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollJumpDown => view_data.page_down(),
		Event::Meta(meta_event) if meta_event == MetaEvent::ScrollJumpUp => view_data.page_up(),
		_ => return None,
	}
	Some(event)
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{assert_rendered_output, view::view_line::ViewLine};

	#[rstest(
		meta_event,
		result,
		case::scroll_left(MetaEvent::ScrollLeft, "12345"),
		case::scroll_right(MetaEvent::ScrollRight, "34567")
	)]
	fn handle_view_data_scroll_horizontal(meta_event: MetaEvent, result: &str) {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::from("012345678"));
		view_data.set_view_size(5, 10);
		view_data.scroll_right();
		view_data.scroll_right();
		assert_eq!(
			handle_view_data_scroll(Event::from(meta_event), &mut view_data),
			Some(Event::from(meta_event))
		);
		assert_rendered_output!(&mut view_data, "{BODY}", format!("{{Normal}}{}", result));
	}

	#[rstest(
	meta_event,
		result,
		case::scroll_down(MetaEvent::ScrollDown, ["3", "4", "5", "6"]),
		case::scroll_up(MetaEvent::ScrollUp, ["1", "2", "3", "4"]),
		case::jump_down(MetaEvent::ScrollJumpDown, ["4", "5", "6", "7"]),
		case::jump_up(MetaEvent::ScrollJumpUp, ["0", "1", "2", "3"]),
	)]
	fn handle_view_data_scroll_vertical(meta_event: MetaEvent, result: [&str; 4]) {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::from("0"));
		view_data.push_line(ViewLine::from("1"));
		view_data.push_line(ViewLine::from("2"));
		view_data.push_line(ViewLine::from("3"));
		view_data.push_line(ViewLine::from("4"));
		view_data.push_line(ViewLine::from("5"));
		view_data.push_line(ViewLine::from("6"));
		view_data.push_line(ViewLine::from("7"));
		view_data.push_line(ViewLine::from("8"));
		view_data.push_line(ViewLine::from("9"));
		view_data.set_view_size(10, 4);
		view_data.scroll_down();
		view_data.scroll_down();
		assert_eq!(
			handle_view_data_scroll(Event::from(meta_event), &mut view_data),
			Some(Event::from(meta_event))
		);
		assert_rendered_output!(
			&mut view_data,
			"{BODY}",
			format!("{{Normal}}{}", result[0]),
			format!("{{Normal}}{}", result[1]),
			format!("{{Normal}}{}", result[2]),
			format!("{{Normal}}{}", result[3])
		);
	}

	#[test]
	fn handle_view_data_scroll_other_input() {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::from("012345678"));
		view_data.set_view_size(5, 10);
		assert_eq!(handle_view_data_scroll(Event::from('a'), &mut view_data), None);
		assert_rendered_output!(&mut view_data, "{BODY}", "{Normal}01234");
	}
}
