use pancurses::Input as PancursesInput;

use crate::color::Color;
use crate::config::Config;
use crate::input::Input;
use std::cell::RefCell;

use pancurses;

const COLOR_TABLE: [i16; 8] = [
	pancurses::COLOR_WHITE, // the default foreground color must be the first (see #77)
	pancurses::COLOR_BLACK,
	pancurses::COLOR_BLUE,
	pancurses::COLOR_CYAN,
	pancurses::COLOR_GREEN,
	pancurses::COLOR_MAGENTA,
	pancurses::COLOR_RED,
	pancurses::COLOR_YELLOW,
];

#[derive(Copy, Clone, Debug)]
pub enum WindowColor {
	ActionDrop,
	ActionEdit,
	ActionExec,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	DiffAddColor,
	DiffRemoveColor,
	DiffChangeColor,
	Foreground,
	IndicatorColor,
}

pub struct Window<'w> {
	config: &'w Config,
	pub window: pancurses::Window,
	height: RefCell<i32>,
	width: RefCell<i32>,
}

impl<'w> Window<'w> {
	pub fn new(config: &'w Config) -> Self {
		let window = pancurses::initscr();
		window.keypad(true);

		pancurses::curs_set(0);
		pancurses::noecho();

		if pancurses::has_colors() {
			pancurses::start_color();
		}
		pancurses::use_default_colors();

		for (i, color) in COLOR_TABLE.iter().enumerate() {
			pancurses::init_pair(i as i16, *color, -1);
		}

		let height = window.get_max_y();
		let width = window.get_max_x();

		Window {
			config,
			window,
			height: RefCell::new(height),
			width: RefCell::new(width),
		}
	}

	pub fn draw_str(&self, s: &str) {
		self.window.addstr(s);
	}

	pub fn clear(&self) {
		self.color(WindowColor::Foreground);
		self.set_style(false, false, false);
		self.window.erase();
	}

	pub fn refresh(&self) {
		self.window.refresh();
	}

	fn set_color(&self, color: Color) {
		match color {
			Color::White => self.window.attrset(pancurses::COLOR_PAIR(0)),
			Color::Black => self.window.attrset(pancurses::COLOR_PAIR(1)),
			Color::Blue => self.window.attrset(pancurses::COLOR_PAIR(2)),
			Color::Cyan => self.window.attrset(pancurses::COLOR_PAIR(3)),
			Color::Green => self.window.attrset(pancurses::COLOR_PAIR(4)),
			Color::Magenta => self.window.attrset(pancurses::COLOR_PAIR(5)),
			Color::Red => self.window.attrset(pancurses::COLOR_PAIR(6)),
			Color::Yellow => self.window.attrset(pancurses::COLOR_PAIR(7)),
		};
	}

	pub fn color(&self, color: WindowColor) {
		match color {
			WindowColor::ActionDrop => self.set_color(self.config.drop_color),
			WindowColor::ActionEdit => self.set_color(self.config.edit_color),
			WindowColor::ActionExec => self.set_color(self.config.exec_color),
			WindowColor::ActionFixup => self.set_color(self.config.fixup_color),
			WindowColor::ActionPick => self.set_color(self.config.pick_color),
			WindowColor::ActionReword => self.set_color(self.config.reword_color),
			WindowColor::ActionSquash => self.set_color(self.config.squash_color),
			WindowColor::Foreground => self.set_color(self.config.foreground_color),
			WindowColor::IndicatorColor => self.set_color(self.config.indicator_color),
			WindowColor::DiffAddColor => self.set_color(self.config.diff_add_color),
			WindowColor::DiffRemoveColor => self.set_color(self.config.diff_remove_color),
			WindowColor::DiffChangeColor => self.set_color(self.config.diff_change_color),
		};
	}

	pub fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.set_dim(dim);
		self.set_underline(underline);
		self.set_reverse(reverse);
	}

	fn set_dim(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_DIM);
		}
		else {
			self.window.attroff(pancurses::A_DIM);
		}
	}

	fn set_underline(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_UNDERLINE);
		}
		else {
			self.window.attroff(pancurses::A_UNDERLINE);
		}
	}

	fn set_reverse(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_REVERSE);
		}
		else {
			self.window.attroff(pancurses::A_REVERSE);
		}
	}

	pub fn get_input(&self) -> Input {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) if c == '?' => Input::Help,
			Some(PancursesInput::Character(c)) if c == 'c' => Input::ShowCommit,
			Some(PancursesInput::Character(c)) if c == 'q' => Input::Abort,
			Some(PancursesInput::Character(c)) if c == 'Q' => Input::ForceAbort,
			Some(PancursesInput::Character(c)) if c == 'w' => Input::Rebase,
			Some(PancursesInput::Character(c)) if c == 'W' => Input::ForceRebase,
			Some(PancursesInput::Character(c)) if c == 'p' => Input::Pick,
			Some(PancursesInput::Character(c)) if c == 'r' => Input::Reword,
			Some(PancursesInput::Character(c)) if c == 'e' => Input::Edit,
			Some(PancursesInput::Character(c)) if c == 's' => Input::Squash,
			Some(PancursesInput::Character(c)) if c == 'f' => Input::Fixup,
			Some(PancursesInput::Character(c)) if c == 'd' => Input::Drop,
			Some(PancursesInput::Character(c)) if c == 'j' => Input::SwapSelectedDown,
			Some(PancursesInput::Character(c)) if c == 'k' => Input::SwapSelectedUp,
			Some(PancursesInput::KeyDown) => Input::MoveCursorDown,
			Some(PancursesInput::KeyUp) => Input::MoveCursorUp,
			Some(PancursesInput::KeyPPage) => Input::MoveCursorPageUp,
			Some(PancursesInput::KeyNPage) => Input::MoveCursorPageDown,
			Some(PancursesInput::KeyResize) => {
				pancurses::resize_term(0, 0);
				self.height.replace(self.window.get_max_y());
				self.width.replace(self.window.get_max_x());
				Input::Resize
			},
			Some(PancursesInput::Character(c)) if c == '!' => Input::OpenInEditor,
			_ => Input::Other,
		}
	}

	pub fn get_window_size(&self) -> (i32, i32) {
		(*self.width.borrow(), *self.height.borrow())
	}

	pub fn get_confirm(&self) -> Option<bool> {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) if c == 'y' || c == 'Y' => Some(true),
			Some(PancursesInput::KeyResize) => {
				pancurses::resize_term(0, 0);
				self.height.replace(self.window.get_max_y());
				self.width.replace(self.window.get_max_x());
				None
			},
			_ => Some(false),
		}
	}

	/// Leaves curses mode, runs the specified callback, and re-enables curses.
	pub fn leave_temporarily<F, T>(callback: F) -> T
	where F: FnOnce() -> T {
		pancurses::def_prog_mode();
		pancurses::endwin();
		let rv = callback();
		pancurses::reset_prog_mode();
		rv
	}

	pub fn end(&self) {
		pancurses::endwin();
	}
}
