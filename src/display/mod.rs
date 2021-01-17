#[cfg(not(test))]
mod ncurses;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod virtual_curses;

pub mod color;
mod color_manager;
mod color_mode;
pub mod curses;
pub mod display_color;
mod size;
mod utils;

use crate::config::theme::Theme;
use crate::display::color_manager::ColorManager;
use crate::display::curses::{chtype, Curses, A_DIM, A_REVERSE, A_UNDERLINE};
use crate::display::display_color::DisplayColor;
pub use crate::display::size::Size;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use std::cell::RefCell;
use std::convert::TryInto;

pub struct Display<'d> {
	curses: &'d mut Curses,
	input_handler: InputHandler<'d>,
	height: RefCell<usize>,
	width: RefCell<usize>,
	action_break: (chtype, chtype),
	action_drop: (chtype, chtype),
	action_edit: (chtype, chtype),
	action_exec: (chtype, chtype),
	action_fixup: (chtype, chtype),
	action_pick: (chtype, chtype),
	action_reword: (chtype, chtype),
	action_squash: (chtype, chtype),
	diff_add: (chtype, chtype),
	diff_change: (chtype, chtype),
	diff_remove: (chtype, chtype),
	diff_context: (chtype, chtype),
	diff_whitespace: (chtype, chtype),
	indicator: (chtype, chtype),
	normal: (chtype, chtype),
}

impl<'d> Display<'d> {
	pub(crate) fn new(input_handler: InputHandler<'d>, curses: &'d mut Curses, theme: &'d Theme) -> Self {
		let mut color_manager = ColorManager::new();
		let normal = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_foreground,
			theme.color_background,
			theme.color_selected_background,
		);
		let indicator = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_indicator,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_break = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_break,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_drop = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_drop,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_edit = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_edit,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_exec = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_exec,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_fixup = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_fixup,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_pick = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_pick,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_reword = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_reword,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_squash = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_action_squash,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_add = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_diff_add,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_change = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_diff_change,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_remove = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_diff_remove,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_context = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_diff_context,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_whitespace = color_manager.register_selectable_color_pairs(
			curses,
			theme.color_diff_whitespace,
			theme.color_background,
			theme.color_selected_background,
		);

		let height = curses.get_max_y().try_into().expect("Invalid window height");
		let width = curses.get_max_x().try_into().expect("Invalid window height");

		Self {
			curses,
			input_handler,
			height: RefCell::new(height),
			width: RefCell::new(width),
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

	pub(crate) fn draw_str(&mut self, s: &str) {
		self.curses.addstr(s);
	}

	pub(crate) fn clear(&mut self) {
		self.color(DisplayColor::Normal, false);
		self.set_style(false, false, false);
		self.curses.erase();
	}

	pub(crate) fn refresh(&mut self) {
		self.curses.refresh();
	}

	pub(crate) fn color(&mut self, color: DisplayColor, selected: bool) {
		self.curses.attrset(
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
		);
	}

	pub(crate) fn set_style(&mut self, dim: bool, underline: bool, reverse: bool) {
		self.set_dim(dim);
		self.set_underline(underline);
		self.set_reverse(reverse);
	}

	fn set_dim(&mut self, on: bool) {
		if on {
			self.curses.attron(A_DIM);
		}
		else {
			self.curses.attroff(A_DIM);
		}
	}

	fn set_underline(&mut self, on: bool) {
		// Windows uses blue text for underlined words
		if !cfg!(windows) && on {
			self.curses.attron(A_UNDERLINE);
		}
		else {
			self.curses.attroff(A_UNDERLINE);
		}
	}

	fn set_reverse(&mut self, on: bool) {
		if on {
			self.curses.attron(A_REVERSE);
		}
		else {
			self.curses.attroff(A_REVERSE);
		}
	}

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		self.curses.getch().map_or(Input::Other, |input| {
			let input = self.input_handler.get_input(mode, input);

			if input == Input::Resize {
				self.curses.resize_term(0, 0);
				self.height
					.replace(self.curses.get_max_y().try_into().expect("Invalid window height"));
				self.width
					.replace(self.curses.get_max_x().try_into().expect("Invalid window width"));
			}
			input
		})
	}

	pub(crate) fn get_window_size(&self) -> Size {
		Size::new(*self.width.borrow(), *self.height.borrow())
	}

	pub(crate) fn fill_end_of_line(&mut self) {
		self.curses.hline(' ', self.curses.get_max_x());
	}

	pub(crate) fn ensure_at_line_start(&mut self, y: i32) {
		self.curses.mv(y, 0);
	}

	pub(crate) fn move_from_end_of_line(&mut self, right: i32) {
		self.curses.mv(self.curses.get_cur_y(), self.curses.get_max_x() - right);
	}

	pub(crate) fn def_prog_mode(&self) {
		self.curses.def_prog_mode();
	}

	pub(crate) fn reset_prog_mode(&self) {
		self.curses.reset_prog_mode();
	}

	pub(crate) fn end(&self) {
		self.curses.endwin();
	}
}

#[cfg(all(windows, test))]
mod tests {
	use super::*;
	use crate::display::testutil::{display_module_test, TestContext};

	#[test]
	#[serial_test::serial()]
	fn windows_set_style_underline_disabled() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			display.set_style(true, true, true);
			assert!(test_context.curses.is_dimmed());
			assert!(test_context.curses.is_reverse());
			assert!(!test_context.curses.is_underline());
		});
	}
}

#[cfg(all(unix, test))]
mod tests {
	use super::*;
	use crate::display::curses::Input as CursesInput;
	use crate::display::testutil::{display_module_test, TestContext};
	use crate::display::virtual_curses::State;
	use rstest::rstest;

	#[test]
	#[serial_test::serial]
	fn draw_str() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.draw_str("Test String");
			let output = Curses::get_output();
			assert_eq!(output, vec!["Test String"]);
		});
	}

	#[test]
	#[serial_test::serial]
	fn clear() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.addstr("Test String");
			test_context.curses.attron(curses::A_DIM);
			test_context.curses.attron(curses::A_REVERSE);
			test_context.curses.attron(curses::A_UNDERLINE);

			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.clear();
			assert!(Curses::get_output().is_empty());
			assert!(!test_context.curses.is_dimmed());
			assert!(!test_context.curses.is_reverse());
			assert!(!test_context.curses.is_underline());
		});
	}

	#[test]
	#[serial_test::serial]
	fn reset() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.refresh();
			assert_eq!(test_context.curses.get_state(), State::Refreshed);
		});
	}

	#[rstest(
		display_color,
		selected,
		expected,
		case::action_break(DisplayColor::ActionBreak, false, 20),
		case::action_break_selected(DisplayColor::ActionBreak, true, 21),
		case::action_drop(DisplayColor::ActionDrop, false, 22),
		case::action_drop_selected(DisplayColor::ActionDrop, true, 23),
		case::action_edit(DisplayColor::ActionEdit, false, 24),
		case::action_edit_selected(DisplayColor::ActionEdit, true, 25),
		case::action_exec(DisplayColor::ActionExec, false, 26),
		case::action_exec_selected(DisplayColor::ActionExec, true, 27),
		case::action_fixup(DisplayColor::ActionFixup, false, 28),
		case::action_fixup_selected(DisplayColor::ActionFixup, true, 29),
		case::action_pick(DisplayColor::ActionPick, false, 30),
		case::action_pick_selected(DisplayColor::ActionPick, true, 31),
		case::action_reword(DisplayColor::ActionReword, false, 32),
		case::action_reword_selected(DisplayColor::ActionReword, true, 33),
		case::action_squash(DisplayColor::ActionSquash, false, 34),
		case::action_squash_selected(DisplayColor::ActionSquash, true, 35),
		case::normal(DisplayColor::Normal, false, 16),
		case::normal_selected(DisplayColor::Normal, true, 17),
		case::indicator(DisplayColor::IndicatorColor, false, 18),
		case::indicator_selected(DisplayColor::IndicatorColor, true, 19),
		case::diff_add(DisplayColor::DiffAddColor, false, 36),
		case::diff_add_selected(DisplayColor::DiffAddColor, true, 37),
		case::diff_remove(DisplayColor::DiffRemoveColor, false, 40),
		case::diff_remove_selected(DisplayColor::DiffRemoveColor, true, 41),
		case::diff_change(DisplayColor::DiffChangeColor, false, 38),
		case::diff_change_selected(DisplayColor::DiffChangeColor, true, 39),
		case::diff_context(DisplayColor::DiffContextColor, false, 42),
		case::diff_context_selected(DisplayColor::DiffContextColor, true, 43),
		case::diff_whitespace(DisplayColor::DiffWhitespaceColor, false, 44),
		case::diff_whitespace_selected(DisplayColor::DiffWhitespaceColor, true, 45)
	)]
	#[serial_test::serial()]
	fn color(display_color: DisplayColor, selected: bool, expected: chtype) {
		display_module_test(|mut test_context: TestContext<'_>| {
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.color(display_color, selected);
			assert!(test_context.curses.is_color_enabled(expected));
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
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.set_style(dim, underline, reverse);
			assert_eq!(test_context.curses.is_dimmed(), dim);
			assert_eq!(test_context.curses.is_underline(), underline);
			assert_eq!(test_context.curses.is_reverse(), reverse);
		});
	}

	#[test]
	#[serial_test::serial]
	fn getch_normal_input() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.set_inputs(vec![CursesInput::Character('z')]);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			assert_eq!(display.get_input(InputMode::Default), Input::Character('z'));
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_input_resize() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.set_inputs(vec![CursesInput::KeyResize]);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			assert_eq!(display.get_input(InputMode::Default), Input::Resize);
			assert_eq!(test_context.curses.get_state(), State::Resized);
		});
	}

	#[test]
	#[serial_test::serial]
	fn get_window_size() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.resize_term(10, 12);
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			assert_eq!(display.get_window_size(), Size::new(12, 10));
		});
	}

	#[test]
	#[serial_test::serial]
	fn fill_end_of_line() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.resize_term(10, 23);
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.fill_end_of_line();
			assert_eq!(Curses::get_output()[0], "{HLINE| |23}");
		});
	}

	#[test]
	#[serial_test::serial]
	fn ensure_at_line_start() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.resize_term(5, 25);
			test_context.curses.mv(10, 12);
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.ensure_at_line_start(5);
			assert_eq!(test_context.curses.get_cur_y(), 5);
			assert_eq!(test_context.curses.get_cur_x(), 0);
		});
	}

	#[test]
	#[serial_test::serial]
	fn move_from_end_of_line() {
		display_module_test(|mut test_context: TestContext<'_>| {
			test_context.curses.resize_term(5, 25);
			test_context.curses.mv(5, 20);
			let mut display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.move_from_end_of_line(5);
			assert_eq!(test_context.curses.get_cur_x(), 20);
		});
	}

	#[test]
	#[serial_test::serial]
	fn end() {
		display_module_test(|mut test_context: TestContext<'_>| {
			let display = Display::new(
				test_context.input_handler,
				&mut test_context.curses,
				&test_context.config.theme,
			);
			display.end();
			assert_eq!(test_context.curses.get_state(), State::Ended);
		});
	}
}
