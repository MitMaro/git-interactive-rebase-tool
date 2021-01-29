pub mod color;
mod color_mode;
pub mod display_color;
pub mod size;
mod utils;

#[cfg(not(test))]
mod crossterm;
#[cfg(test)]
mod mockcrossterm;
#[cfg(test)]
pub mod testutil;
#[cfg(not(test))]
use self::crossterm as ct;
#[cfg(test)]
use mockcrossterm as ct;

use crate::config::theme::Theme;
use crate::display::display_color::DisplayColor;
use crate::display::size::Size;
use crate::display::utils::register_selectable_color_pairs;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use anyhow::Result;
use ct::Color as CrosstermColor;
pub use ct::{Colors, CrossTerm, Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};

pub struct Display<'d> {
	crossterm: &'d mut CrossTerm,
	input_handler: InputHandler<'d>,
	action_break: (Colors, Colors),
	action_drop: (Colors, Colors),
	action_edit: (Colors, Colors),
	action_exec: (Colors, Colors),
	action_fixup: (Colors, Colors),
	action_pick: (Colors, Colors),
	action_reword: (Colors, Colors),
	action_squash: (Colors, Colors),
	diff_add: (Colors, Colors),
	diff_change: (Colors, Colors),
	diff_remove: (Colors, Colors),
	diff_context: (Colors, Colors),
	diff_whitespace: (Colors, Colors),
	indicator: (Colors, Colors),
	normal: (Colors, Colors),
}

impl<'d> Display<'d> {
	pub(crate) fn new(input_handler: InputHandler<'d>, crossterm: &'d mut CrossTerm, theme: &'d Theme) -> Self {
		let color_mode = crossterm.get_color_mode();
		let normal = register_selectable_color_pairs(
			color_mode,
			theme.color_foreground,
			theme.color_background,
			theme.color_selected_background,
		);
		let indicator = register_selectable_color_pairs(
			color_mode,
			theme.color_indicator,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_break = register_selectable_color_pairs(
			color_mode,
			theme.color_action_break,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_drop = register_selectable_color_pairs(
			color_mode,
			theme.color_action_drop,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_edit = register_selectable_color_pairs(
			color_mode,
			theme.color_action_edit,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_exec = register_selectable_color_pairs(
			color_mode,
			theme.color_action_exec,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_fixup = register_selectable_color_pairs(
			color_mode,
			theme.color_action_fixup,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_pick = register_selectable_color_pairs(
			color_mode,
			theme.color_action_pick,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_reword = register_selectable_color_pairs(
			color_mode,
			theme.color_action_reword,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_squash = register_selectable_color_pairs(
			color_mode,
			theme.color_action_squash,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_add = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_add,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_change = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_change,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_remove = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_remove,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_context = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_context,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_whitespace = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_whitespace,
			theme.color_background,
			theme.color_selected_background,
		);

		Self {
			crossterm,
			input_handler,
			normal,
			indicator,
			action_break,
			action_drop,
			action_edit,
			action_exec,
			action_fixup,
			action_pick,
			action_reword,
			action_squash,
			diff_add,
			diff_change,
			diff_remove,
			diff_context,
			diff_whitespace,
		}
	}

	pub(crate) fn draw_str(&mut self, s: &str) -> Result<()> {
		self.crossterm.print(s)
	}

	pub(crate) fn clear(&mut self) -> Result<()> {
		self.color(DisplayColor::Normal, false)?;
		self.set_style(false, false, false)?;
		self.crossterm.reset()
	}

	pub(crate) fn refresh(&mut self) -> Result<()> {
		self.crossterm.flush()
	}

	pub(crate) fn color(&mut self, color: DisplayColor, selected: bool) -> Result<()> {
		self.crossterm.set_color(
			if selected {
				match color {
					DisplayColor::ActionBreak => self.action_break.1,
					DisplayColor::ActionDrop => self.action_drop.1,
					DisplayColor::ActionEdit => self.action_edit.1,
					DisplayColor::ActionExec => self.action_exec.1,
					DisplayColor::ActionFixup => self.action_fixup.1,
					DisplayColor::ActionPick => self.action_pick.1,
					DisplayColor::ActionReword => self.action_reword.1,
					DisplayColor::ActionSquash => self.action_squash.1,
					DisplayColor::Normal => self.normal.1,
					DisplayColor::IndicatorColor => self.indicator.1,
					DisplayColor::DiffAddColor => self.diff_add.1,
					DisplayColor::DiffRemoveColor => self.diff_remove.1,
					DisplayColor::DiffChangeColor => self.diff_change.1,
					DisplayColor::DiffContextColor => self.diff_context.1,
					DisplayColor::DiffWhitespaceColor => self.diff_whitespace.1,
				}
			}
			else {
				match color {
					DisplayColor::ActionBreak => self.action_break.0,
					DisplayColor::ActionDrop => self.action_drop.0,
					DisplayColor::ActionEdit => self.action_edit.0,
					DisplayColor::ActionExec => self.action_exec.0,
					DisplayColor::ActionFixup => self.action_fixup.0,
					DisplayColor::ActionPick => self.action_pick.0,
					DisplayColor::ActionReword => self.action_reword.0,
					DisplayColor::ActionSquash => self.action_squash.0,
					DisplayColor::Normal => self.normal.0,
					DisplayColor::IndicatorColor => self.indicator.0,
					DisplayColor::DiffAddColor => self.diff_add.0,
					DisplayColor::DiffRemoveColor => self.diff_remove.0,
					DisplayColor::DiffChangeColor => self.diff_change.0,
					DisplayColor::DiffContextColor => self.diff_context.0,
					DisplayColor::DiffWhitespaceColor => self.diff_whitespace.0,
				}
			},
		)
	}

	pub(crate) fn set_style(&mut self, dim: bool, underline: bool, reverse: bool) -> Result<()> {
		self.set_dim(dim)?;
		self.set_underline(underline)?;
		self.set_reverse(reverse)
	}

	fn set_dim(&mut self, on: bool) -> Result<()> {
		self.crossterm.set_dim(on)
	}

	fn set_underline(&mut self, on: bool) -> Result<()> {
		self.crossterm.set_underline(on)
	}

	fn set_reverse(&mut self, on: bool) -> Result<()> {
		self.crossterm.set_reverse(on)
	}

	pub(crate) fn get_window_size(&self) -> Size {
		self.crossterm.get_size()
	}

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		// TODO remove ignore hack
		loop {
			let input = CrossTerm::read_event().map_or(Input::Other, |input| self.input_handler.get_input(mode, input));
			if input != Input::Ignore {
				return input;
			}
		}
	}

	pub(crate) fn ensure_at_line_start(&mut self) -> Result<()> {
		self.crossterm.move_to_column(1)
	}

	pub(crate) fn move_from_end_of_line(&mut self, right: u16) -> Result<()> {
		let width = self.get_window_size().width();
		self.crossterm.move_to_column(width as u16 - right + 1)
	}

	pub(crate) fn next_line(&mut self) -> Result<()> {
		self.crossterm.move_next_line()
	}

	pub(crate) fn start(&mut self) -> Result<()> {
		self.crossterm.start()?;
		self.crossterm.flush()
	}

	pub(crate) fn end(&mut self) -> Result<()> {
		self.crossterm.end()?;
		self.crossterm.flush()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::create_key_event;
	use crate::display::mockcrossterm::State;
	use crate::display::testutil::{display_module_test, TestContext};
	use crossterm::event::MouseEvent;
	use rstest::rstest;

	#[test]
	#[serial_test::serial]
	fn draw_str() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.draw_str("Test String").unwrap();
			let output = CrossTerm::get_output();
			assert_eq!(output, vec!["Test String"]);
		});
	}

	#[test]
	#[serial_test::serial]
	fn clear() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.crossterm.print("Test String").unwrap();
			test_context.crossterm.set_dim(true).unwrap();
			test_context.crossterm.set_reverse(true).unwrap();
			test_context.crossterm.set_underline(true).unwrap();
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.clear().unwrap();
			assert!(CrossTerm::get_output().is_empty());
			assert!(CrossTerm::get_output().is_empty());
			assert!(!test_context.crossterm.is_dimmed());
			assert!(!test_context.crossterm.is_reverse());
			assert!(!test_context.crossterm.is_underline());
		});
	}

	#[test]
	#[serial_test::serial]
	fn refresh() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.refresh().unwrap();
			assert!(!test_context.crossterm.is_dirty());
		});
	}

	#[rstest(
		display_color,
		selected,
		expected_foreground,
		expected_background,
		case::action_break(DisplayColor::ActionBreak, false, CrosstermColor::White, CrosstermColor::Reset),
		case::action_break_selected(
			DisplayColor::ActionBreak,
			true,
			CrosstermColor::White,
			CrosstermColor::AnsiValue(237)
		),
		case::action_drop(DisplayColor::ActionDrop, false, CrosstermColor::Red, CrosstermColor::Reset),
		case::action_drop_selected(
			DisplayColor::ActionDrop,
			true,
			CrosstermColor::Red,
			CrosstermColor::AnsiValue(237)
		),
		case::action_edit(DisplayColor::ActionEdit, false, CrosstermColor::Blue, CrosstermColor::Reset),
		case::action_edit_selected(
			DisplayColor::ActionEdit,
			true,
			CrosstermColor::Blue,
			CrosstermColor::AnsiValue(237)
		),
		case::action_exec(DisplayColor::ActionExec, false, CrosstermColor::White, CrosstermColor::Reset),
		case::action_exec_selected(
			DisplayColor::ActionExec,
			true,
			CrosstermColor::White,
			CrosstermColor::AnsiValue(237)
		),
		case::action_fixup(DisplayColor::ActionFixup, false, CrosstermColor::Magenta, CrosstermColor::Reset),
		case::action_fixup_selected(
			DisplayColor::ActionFixup,
			true,
			CrosstermColor::Magenta,
			CrosstermColor::AnsiValue(237)
		),
		case::action_pick(DisplayColor::ActionPick, false, CrosstermColor::Green, CrosstermColor::Reset),
		case::action_pick_selected(
			DisplayColor::ActionPick,
			true,
			CrosstermColor::Green,
			CrosstermColor::AnsiValue(237)
		),
		case::action_reword(DisplayColor::ActionReword, false, CrosstermColor::Yellow, CrosstermColor::Reset),
		case::action_reword_selected(
			DisplayColor::ActionReword,
			true,
			CrosstermColor::Yellow,
			CrosstermColor::AnsiValue(237)
		),
		case::action_squash(DisplayColor::ActionSquash, false, CrosstermColor::Cyan, CrosstermColor::Reset),
		case::action_squash_selected(
			DisplayColor::ActionSquash,
			true,
			CrosstermColor::Cyan,
			CrosstermColor::AnsiValue(237)
		),
		case::normal(DisplayColor::Normal, false, CrosstermColor::Reset, CrosstermColor::Reset),
		case::normal_selected(DisplayColor::Normal, true, CrosstermColor::Reset, CrosstermColor::AnsiValue(237)),
		case::indicator(DisplayColor::IndicatorColor, false, CrosstermColor::Cyan, CrosstermColor::Reset),
		case::indicator_selected(
			DisplayColor::IndicatorColor,
			true,
			CrosstermColor::Cyan,
			CrosstermColor::AnsiValue(237)
		),
		case::diff_add(DisplayColor::DiffAddColor, false, CrosstermColor::Green, CrosstermColor::Reset),
		case::diff_add_selected(
			DisplayColor::DiffAddColor,
			true,
			CrosstermColor::Green,
			CrosstermColor::AnsiValue(237)
		),
		case::diff_remove(DisplayColor::DiffRemoveColor, false, CrosstermColor::Red, CrosstermColor::Reset),
		case::diff_remove_selected(
			DisplayColor::DiffRemoveColor,
			true,
			CrosstermColor::Red,
			CrosstermColor::AnsiValue(237)
		),
		case::diff_change(DisplayColor::DiffChangeColor, false, CrosstermColor::Yellow, CrosstermColor::Reset),
		case::diff_change_selected(
			DisplayColor::DiffChangeColor,
			true,
			CrosstermColor::Yellow,
			CrosstermColor::AnsiValue(237)
		),
		case::diff_context(DisplayColor::DiffContextColor, false, CrosstermColor::White, CrosstermColor::Reset),
		case::diff_context_selected(
			DisplayColor::DiffContextColor,
			true,
			CrosstermColor::White,
			CrosstermColor::AnsiValue(237)
		),
		case::diff_whitespace(
			DisplayColor::DiffWhitespaceColor,
			false,
			CrosstermColor::DarkGrey,
			CrosstermColor::Reset
		),
		case::diff_whitespace_selected(
			DisplayColor::DiffWhitespaceColor,
			true,
			CrosstermColor::DarkGrey,
			CrosstermColor::AnsiValue(237)
		)
	)]
	#[serial_test::serial()]
	fn color(
		display_color: DisplayColor,
		selected: bool,
		expected_foreground: CrosstermColor,
		expected_background: CrosstermColor,
	) {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.color(display_color, selected).unwrap();
			assert!(test_context
				.crossterm
				.is_colors_enabled(Colors::new(expected_foreground, expected_background)));
		});
	}

	#[rstest(
		dim,
		underline,
		reverse,
		case::all_off(false, false, false),
		case::reverse(false, false, true),
		case::underline(false, true, false),
		case::underline_reverse(false, true, true),
		case::dim(true, false, false),
		case::dim_reverse(true, false, true),
		case::dim_underline(true, true, false),
		case::all_on(true, true, true)
	)]
	#[serial_test::serial()]
	fn style(dim: bool, underline: bool, reverse: bool) {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.set_style(dim, underline, reverse).unwrap();
			assert_eq!(test_context.crossterm.is_dimmed(), dim);
			assert_eq!(test_context.crossterm.is_underline(), underline);
			assert_eq!(test_context.crossterm.is_reverse(), reverse);
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_input_success() {
		display_module_test(|mut test_context: TestContext<'_>| {
			CrossTerm::set_inputs(vec![create_key_event!('z')]);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			assert_eq!(display.get_input(InputMode::Default), Input::Character('z'));
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_input_fail() {
		display_module_test(|mut test_context: TestContext<'_>| {
			CrossTerm::set_inputs(vec![]);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			assert_eq!(display.get_input(InputMode::Default), Input::Other);
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_input_ignore_hack() {
		display_module_test(|mut test_context: TestContext<'_>| {
			CrossTerm::set_inputs(vec![
				Event::Mouse(MouseEvent {
					kind: MouseEventKind::Moved,
					column: 0,
					row: 0,
					modifiers: KeyModifiers::NONE,
				}),
				create_key_event!('z'),
			]);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			assert_eq!(display.get_input(InputMode::Default), Input::Character('z'));
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_window_size() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.crossterm.set_size(Size::new(12, 10));
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			assert_eq!(display.get_window_size(), Size::new(12, 10));
		});
	}

	#[test]
	#[serial_test::serial]
	fn ensure_at_line_start() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.ensure_at_line_start().unwrap();
			assert_eq!(test_context.crossterm.get_position(), (1, 0));
		});
	}

	#[test]
	#[serial_test::serial]
	fn move_from_end_of_line() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.crossterm.set_size(Size::new(20, 10));
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.move_from_end_of_line(5).unwrap();
			// character after the 15th character (16th)
			assert_eq!(test_context.crossterm.get_position(), (16, 0));
		});
	}

	#[test]
	#[serial_test::serial]
	fn start() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.start().unwrap();
			assert_eq!(test_context.crossterm.get_state(), State::Normal);
		});
	}

	#[test]
	#[serial_test::serial]
	fn end() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.crossterm,
				&test_context.config.theme,
			);
			display.end().unwrap();
			assert_eq!(test_context.crossterm.get_state(), State::Ended);
		});
	}
}
