use crate::display::display_color::DisplayColor;
use crate::input::input_handler::InputMode;
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::util::handle_view_data_scroll;
use crate::todo_file::TodoFile;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use unicode_segmentation::UnicodeSegmentation;

fn get_max_help_key_length(lines: &[(String, String)]) -> usize {
	let mut max_length = 0;
	for &(ref key, _) in lines {
		let len = UnicodeSegmentation::graphemes(key.as_str(), true).count();
		if len > max_length {
			max_length = len;
		}
	}
	max_length
}

pub struct Help {
	return_state: Option<State>,
	no_help_view_data: ViewData,
	view_data: Option<ViewData>,
}

impl ProcessModule for Help {
	fn activate(&mut self, _: &TodoFile, return_state: State) -> ProcessResult {
		if self.return_state.is_none() {
			self.return_state = Some(return_state);
		}
		ProcessResult::new()
	}

	fn deactivate(&mut self) {
		self.clear()
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
		let view_width = view.get_view_size().width();
		let view_height = view.get_view_size().height();
		let view_data = self.view_data.as_mut().unwrap_or(&mut self.no_help_view_data);
		view_data.set_view_size(view_width, view_height);
		view_data.rebuild();
		view_data
	}

	fn handle_input(&mut self, view: &mut View<'_>, _: &mut TodoFile) -> ProcessResult {
		let input = view.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);
		let mut view_data = self.view_data.as_mut().unwrap_or(&mut self.no_help_view_data);
		if handle_view_data_scroll(input, &mut view_data).is_none() && input != Input::Resize {
			result = result.state(self.return_state.unwrap_or(State::List));
		}
		result
	}
}

impl Help {
	pub fn new() -> Self {
		let mut no_help_view_data = ViewData::new();
		no_help_view_data.push_line(ViewLine::from("Help not available"));
		no_help_view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Press any key to close",
			DisplayColor::IndicatorColor,
		)]));

		Self {
			return_state: None,
			view_data: None,
			no_help_view_data,
		}
	}

	pub fn clear(&mut self) {
		self.return_state = None;
		self.view_data = None;
	}

	pub fn update_from_keybindings_descriptions(&mut self, keybindings: &[(String, String)]) {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);

		let max_key_length = get_max_help_key_length(keybindings);

		view_data.push_leading_line(
			ViewLine::new_pinned(vec![LineSegment::new_with_color_and_style(
				format!(" {0:width$} Action", "Key", width = max_key_length).as_str(),
				DisplayColor::Normal,
				false,
				true,
				false,
			)])
			.set_padding_color_and_style(DisplayColor::Normal, false, true, false),
		);

		for line in keybindings {
			view_data.push_line(ViewLine::new_with_pinned_segments(
				vec![
					LineSegment::new_with_color(
						format!(" {0:width$}", line.0, width = max_key_length).as_str(),
						DisplayColor::IndicatorColor,
					),
					LineSegment::new_with_color_and_style("|", DisplayColor::Normal, true, false, false),
					LineSegment::new(line.1.as_str()),
				],
				2,
			));
		}

		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Press any key to close",
			DisplayColor::IndicatorColor,
		)]));

		self.view_data = Some(view_data);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::display::size::Size;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};

	#[test]
	#[serial_test::serial]
	fn empty() {
		process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
			let mut module = Help::new();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{BODY}",
				"{Normal}Help not available",
				"{TRAILING}",
				"{IndicatorColor}Press any key to close"
			);
		});
	}

	#[test]
	#[serial_test::serial]
	fn from_key_bindings() {
		process_module_test(
			&[],
			ViewState {
				size: Size::new(22, 100),
				..ViewState::default()
			},
			&[],
			|test_context: TestContext<'_>| {
				let mut module = Help::new();
				module.update_from_keybindings_descriptions(&[
					(String::from("a"), String::from("Description A")),
					(String::from("b"), String::from("Description B")),
				]);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal,Underline} Key Action{Normal,Underline}{Pad  ,11}",
					"{BODY}",
					"{IndicatorColor} a{Normal,Dimmed}|{Normal}Description A",
					"{IndicatorColor} b{Normal,Dimmed}|{Normal}Description B",
					"{TRAILING}",
					"{IndicatorColor}Press any key to close"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn clear() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Character('a')],
			|_: TestContext<'_>| {
				let mut module = Help::new();
				module.clear();
				assert!(module.return_state.is_none());
				assert!(module.view_data.is_none());
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn deactivate() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Character('a')],
			|_: TestContext<'_>| {
				let mut module = Help::new();
				module.deactivate();
				assert!(module.return_state.is_none());
				assert!(module.view_data.is_none());
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn return_state() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Character('a')],
			|mut test_context: TestContext<'_>| {
				let mut module = Help::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Character('a'),
					state = State::ConfirmRebase
				)
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn resize() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = Help::new();
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize)
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_events() {
		process_module_test(
			&[],
			ViewState::default(),
			&[
				Input::ScrollLeft,
				Input::ScrollRight,
				Input::ScrollDown,
				Input::ScrollUp,
				Input::ScrollJumpDown,
				Input::ScrollJumpUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Help::new();
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollLeft);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollRight);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollDown);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollUp);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpDown);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpUp);
			},
		);
	}
}
