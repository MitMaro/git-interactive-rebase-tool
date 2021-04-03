use crate::{input::Input, view::view_data::ViewData};

pub fn handle_view_data_scroll(input: Input, view_data: &mut ViewData) -> Option<Input> {
	match input {
		Input::MoveCursorLeft | Input::ScrollLeft => view_data.scroll_left(),
		Input::MoveCursorRight | Input::ScrollRight => view_data.scroll_right(),
		Input::MoveCursorDown | Input::ScrollDown => view_data.scroll_down(),
		Input::MoveCursorUp | Input::ScrollUp => view_data.scroll_up(),
		Input::MoveCursorPageDown | Input::ScrollJumpDown => view_data.page_down(),
		Input::MoveCursorPageUp | Input::ScrollJumpUp => view_data.page_up(),
		_ => return None,
	}
	Some(input)
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{assert_rendered_output, view::view_line::ViewLine};

	#[rstest(
		input,
		result,
		case::move_cursor_left(Input::MoveCursorLeft, "12345"),
		case::scroll_left(Input::ScrollLeft, "12345"),
		case::move_cursor_right(Input::MoveCursorRight, "34567"),
		case::scroll_right(Input::ScrollRight, "34567")
	)]
	fn handle_view_data_scroll_horizontal(input: Input, result: &str) {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::from("012345678"));
		view_data.set_view_size(5, 10);
		view_data.scroll_right();
		view_data.scroll_right();
		assert_eq!(handle_view_data_scroll(input, &mut view_data), Some(input));
		assert_rendered_output!(&mut view_data, "{BODY}", format!("{{Normal}}{}", result));
	}

	#[rstest(
		input,
		result,
		case::move_cursor_down(Input::MoveCursorDown, ["3", "4", "5", "6"]),
		case::scroll_down(Input::ScrollDown, ["3", "4", "5", "6"]),
		case::move_cursor_up(Input::MoveCursorUp, ["1", "2", "3", "4"]),
		case::scroll_up(Input::ScrollUp, ["1", "2", "3", "4"]),
		case::move_cursor_page_down(Input::MoveCursorPageDown, ["4", "5", "6", "7"]),
		case::jump_down(Input::ScrollJumpDown, ["4", "5", "6", "7"]),
		case::move_cursor_page_up(Input::MoveCursorPageUp, ["0", "1", "2", "3"]),
		case::jump_up(Input::ScrollJumpUp, ["0", "1", "2", "3"]),
	)]
	fn handle_view_data_scroll_vertical(input: Input, result: [&str; 4]) {
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
		assert_eq!(handle_view_data_scroll(input, &mut view_data), Some(input));
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
