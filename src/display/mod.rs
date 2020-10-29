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
mod utils;

use crate::config::theme::Theme;
use crate::display::color_manager::ColorManager;
use crate::display::curses::{chtype, Curses, Input, A_DIM, A_REVERSE, A_UNDERLINE};
use crate::display::display_color::DisplayColor;
use std::cell::RefCell;
use std::convert::TryInto;

pub struct Display<'d> {
	curses: &'d Curses,
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
	pub(crate) fn new(curses: &'d mut Curses, theme: &'d Theme) -> Self {
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
		Self {
			curses,
			height: RefCell::new(curses.get_max_y().try_into().expect("Invalid window height")),
			width: RefCell::new(curses.get_max_x().try_into().expect("Invalid window width")),
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

	pub(crate) fn draw_str(&self, s: &str) {
		self.curses.addstr(s);
	}

	pub(crate) fn clear(&self) {
		self.color(DisplayColor::Normal, false);
		self.set_style(false, false, false);
		self.curses.erase();
	}

	pub(crate) fn refresh(&self) {
		self.curses.refresh();
	}

	pub(crate) fn color(&self, color: DisplayColor, selected: bool) {
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

	pub(crate) fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.set_dim(dim);
		self.set_underline(underline);
		self.set_reverse(reverse);
	}

	fn set_dim(&self, on: bool) {
		if on {
			self.curses.attron(A_DIM);
		}
		else {
			self.curses.attroff(A_DIM);
		}
	}

	fn set_underline(&self, on: bool) {
		// Windows uses blue text for underlined words
		if !cfg!(windows) && on {
			self.curses.attron(A_UNDERLINE);
		}
		else {
			self.curses.attroff(A_UNDERLINE);
		}
	}

	fn set_reverse(&self, on: bool) {
		if on {
			self.curses.attron(A_REVERSE);
		}
		else {
			self.curses.attroff(A_REVERSE);
		}
	}

	pub(crate) fn getch(&self) -> Option<Input> {
		let input = self.curses.getch();

		if let Some(Input::KeyResize) = input {
			self.curses.resize_term(0, 0);
			self.height
				.replace(self.curses.get_max_y().try_into().expect("Invalid window height"));
			self.width
				.replace(self.curses.get_max_x().try_into().expect("Invalid window width"));
		}
		input
	}

	pub(crate) fn get_window_size(&self) -> (usize, usize) {
		(*self.width.borrow(), *self.height.borrow())
	}

	pub(crate) fn fill_end_of_line(&self) {
		self.curses.hline(' ', self.curses.get_max_x());
	}

	pub(crate) fn ensure_at_line_start(&self, y: i32) {
		self.curses.mv(y, 0);
	}

	pub(crate) fn move_from_end_of_line(&self, right: i32) {
		self.curses.mv(self.curses.get_cur_y(), self.curses.get_max_x() - right);
	}

	/// Leaves curses mode, runs the specified callback, and re-enables curses.
	pub(crate) fn leave_temporarily<F, T>(&self, callback: F) -> T
	where F: FnOnce() -> T {
		self.curses.def_prog_mode();
		self.curses.endwin();
		let rv = callback();
		self.curses.reset_prog_mode();
		rv
	}

	pub(crate) fn end(&self) {
		self.curses.endwin();
	}
}

#[cfg(all(windows, test))]
mod tests {
	use super::Display;
	use crate::build_trace;
	use crate::display_module_test;

	display_module_test!(
		set_style_underline_disabled,
		[
			build_trace!("attron", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attron", "128")
		],
		|display: &mut Display<'_>| display.set_style(true, true, true)
	);
}

#[cfg(all(unix, test))]
mod tests {
	use super::display_color::DisplayColor;
	use super::Display;
	use crate::build_trace;
	use crate::display::curses::Input;
	use crate::display_module_test;

	display_module_test!(
		draw_str,
		[build_trace!("addstr", "Test string")],
		|display: &mut Display<'_>| display.draw_str("Test string")
	);

	display_module_test!(
		clear,
		[
			build_trace!("attrset", "16"),
			build_trace!("attroff", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attroff", "128"),
			build_trace!("erase")
		],
		|display: &mut Display<'_>| display.clear()
	);

	display_module_test!(refresh, [build_trace!("refresh")], |display: &mut Display<'_>| {
		display.refresh()
	});

	display_module_test!(
		color_action_break_not_selected,
		[build_trace!("attrset", "20")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionBreak, false)
	);

	display_module_test!(
		color_action_break_selected,
		[build_trace!("attrset", "21")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionBreak, true)
	);

	display_module_test!(
		color_action_drop_not_selected,
		[build_trace!("attrset", "22")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionDrop, false)
	);

	display_module_test!(
		color_action_drop_selected,
		[build_trace!("attrset", "23")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionDrop, true)
	);

	display_module_test!(
		color_action_edit_not_selected,
		[build_trace!("attrset", "24")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionEdit, false)
	);

	display_module_test!(
		color_action_edit_selected,
		[build_trace!("attrset", "25")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionEdit, true)
	);

	display_module_test!(
		color_action_exec_not_selected,
		[build_trace!("attrset", "26")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionExec, false)
	);

	display_module_test!(
		color_action_exec_selected,
		[build_trace!("attrset", "27")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionExec, true)
	);

	display_module_test!(
		color_action_fixup_not_selected,
		[build_trace!("attrset", "28")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionFixup, false)
	);

	display_module_test!(
		color_action_fixup_selected,
		[build_trace!("attrset", "29")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionFixup, true)
	);

	display_module_test!(
		color_action_pick_not_selected,
		[build_trace!("attrset", "30")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionPick, false)
	);

	display_module_test!(
		color_action_pick_selected,
		[build_trace!("attrset", "31")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionPick, true)
	);

	display_module_test!(
		color_action_reword_not_selected,
		[build_trace!("attrset", "32")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionReword, false)
	);

	display_module_test!(
		color_action_reword_selected,
		[build_trace!("attrset", "33")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionReword, true)
	);

	display_module_test!(
		color_action_squash_not_selected,
		[build_trace!("attrset", "34")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionSquash, false)
	);

	display_module_test!(
		color_action_squash_selected,
		[build_trace!("attrset", "35")],
		|display: &mut Display<'_>| display.color(DisplayColor::ActionSquash, true)
	);

	display_module_test!(
		color_normal_not_selected,
		[build_trace!("attrset", "16")],
		|display: &mut Display<'_>| display.color(DisplayColor::Normal, false)
	);

	display_module_test!(
		color_normal_selected,
		[build_trace!("attrset", "17")],
		|display: &mut Display<'_>| display.color(DisplayColor::Normal, true)
	);

	display_module_test!(
		color_indicator_color_not_selected,
		[build_trace!("attrset", "18")],
		|display: &mut Display<'_>| display.color(DisplayColor::IndicatorColor, false)
	);

	display_module_test!(
		color_indiciator_color_selected,
		[build_trace!("attrset", "19")],
		|display: &mut Display<'_>| display.color(DisplayColor::IndicatorColor, true)
	);

	display_module_test!(
		color_diff_add_color_not_selected,
		[build_trace!("attrset", "36")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffAddColor, false)
	);

	display_module_test!(
		color_diff_add_color_selected,
		[build_trace!("attrset", "37")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffAddColor, true)
	);

	display_module_test!(
		color_diff_remove_color_not_selected,
		[build_trace!("attrset", "40")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffRemoveColor, false)
	);

	display_module_test!(
		color_diff_remove_color_selected,
		[build_trace!("attrset", "41")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffRemoveColor, true)
	);

	display_module_test!(
		color_diff_change_color_not_selected,
		[build_trace!("attrset", "38")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffChangeColor, false)
	);

	display_module_test!(
		color_diff_change_color_selected,
		[build_trace!("attrset", "39")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffChangeColor, true)
	);

	display_module_test!(
		color_diff_context_color_not_selected,
		[build_trace!("attrset", "42")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffContextColor, false)
	);

	display_module_test!(
		color_diff_context_color_selected,
		[build_trace!("attrset", "43")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffContextColor, true)
	);

	display_module_test!(
		color_diff_whitespace_color_not_selected,
		[build_trace!("attrset", "44")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffWhitespaceColor, false)
	);

	display_module_test!(
		color_diff_whitespace_color_selected,
		[build_trace!("attrset", "45")],
		|display: &mut Display<'_>| display.color(DisplayColor::DiffWhitespaceColor, true)
	);

	display_module_test!(
		set_style_dim_off_underline_off_reverse_off,
		[
			build_trace!("attroff", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attroff", "128")
		],
		|display: &mut Display<'_>| display.set_style(false, false, false)
	);

	display_module_test!(
		set_style_dim_on_underline_off_reverse_off,
		[
			build_trace!("attron", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attroff", "128")
		],
		|display: &mut Display<'_>| display.set_style(true, false, false)
	);

	display_module_test!(
		set_style_dim_on_underline_off_reverse_on,
		[
			build_trace!("attron", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attron", "128")
		],
		|display: &mut Display<'_>| display.set_style(true, false, true)
	);

	display_module_test!(
		set_style_dim_on_underline_on_reverse_off,
		[
			build_trace!("attron", "64"),
			build_trace!("attron", "256"),
			build_trace!("attroff", "128")
		],
		|display: &mut Display<'_>| display.set_style(true, true, false)
	);

	display_module_test!(
		set_style_dim_on_underline_on_reverse_on,
		[
			build_trace!("attron", "64"),
			build_trace!("attron", "256"),
			build_trace!("attron", "128")
		],
		|display: &mut Display<'_>| display.set_style(true, true, true)
	);

	display_module_test!(
		set_style_dim_off_underline_on_reverse_off,
		[
			build_trace!("attroff", "64"),
			build_trace!("attron", "256"),
			build_trace!("attroff", "128")
		],
		|display: &mut Display<'_>| display.set_style(false, true, false)
	);

	display_module_test!(
		set_style_dim_off_underline_on_reverse_on,
		[
			build_trace!("attroff", "64"),
			build_trace!("attron", "256"),
			build_trace!("attron", "128")
		],
		|display: &mut Display<'_>| display.set_style(false, true, true)
	);

	display_module_test!(
		set_style_dim_off_underline_off_reverse_on,
		[
			build_trace!("attroff", "64"),
			build_trace!("attroff", "256"),
			build_trace!("attron", "128")
		],
		|display: &mut Display<'_>| display.set_style(false, false, true)
	);

	display_module_test!(
		getch_normal_input,
		Input::Character('z'),
		[],
		|display: &mut Display<'_>| assert_eq!(display.getch().unwrap(), Input::Character('z'))
	);

	display_module_test!(
		getch_resize,
		Input::KeyResize,
		[
			build_trace!("resize_term", "0", "0"),
			build_trace!("get_max_y"),
			build_trace!("get_max_x")
		],
		|display: &mut Display<'_>| { assert_eq!(display.getch().unwrap(), Input::KeyResize) }
	);

	display_module_test!(get_window_size, [], |display: &mut Display<'_>| {
		assert_eq!(display.get_window_size(), (77, 66));
	});

	display_module_test!(
		fill_end_of_line,
		[build_trace!("hline", " ", "77")],
		|display: &mut Display<'_>| display.fill_end_of_line()
	);

	display_module_test!(
		ensure_at_line_start,
		[build_trace!("mv", "32", "0")],
		|display: &mut Display<'_>| display.ensure_at_line_start(32)
	);

	display_module_test!(
		move_from_end_of_line,
		[
			build_trace!("get_cur_y"),
			build_trace!("get_max_x"),
			build_trace!("mv", "13", "65")
		],
		|display: &mut Display<'_>| display.move_from_end_of_line(12)
	);

	display_module_test!(
		leave_temporarily,
		[
			build_trace!("def_prog_mode"),
			build_trace!("endwin"),
			build_trace!("reset_prog_mode")
		],
		|display: &mut Display<'_>| {
			let result = display.leave_temporarily(|| "Done");
			assert_eq!(result, "Done");
		}
	);

	display_module_test!(end, [build_trace!("endwin")], |display: &mut Display<'_>| {
		display.end()
	});
}
