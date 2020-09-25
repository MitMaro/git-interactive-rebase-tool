use crate::input::Input;
use crate::view::view_data::ViewData;

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
