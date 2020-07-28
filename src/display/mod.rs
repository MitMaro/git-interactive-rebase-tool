pub(crate) mod color;
mod color_manager;
mod color_mode;
pub(crate) mod curses;
pub(crate) mod display_color;
mod utils;

use crate::config::Config;
use crate::display::color_manager::ColorManager;
use crate::display::curses::Curses;
use crate::display::display_color::DisplayColor;
use pancurses::Input;
use std::cell::RefCell;
use std::convert::TryInto;

pub(crate) struct Display<'d> {
	color_manager: ColorManager,
	curses: &'d Curses,
	height: RefCell<usize>,
	width: RefCell<usize>,
}

impl<'d> Display<'d> {
	pub(crate) fn new(curses: &'d mut Curses, config: &'d Config) -> Self {
		Self {
			color_manager: ColorManager::new(&config.theme, curses),
			curses,
			height: RefCell::new(curses.get_max_y().try_into().expect("Invalid window height")),
			width: RefCell::new(curses.get_max_x().try_into().expect("Invalid window width")),
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
		self.curses.attrset(self.color_manager.get_color(color, selected));
	}

	pub(crate) fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.set_dim(dim);
		self.set_underline(underline);
		self.set_reverse(reverse);
	}

	fn set_dim(&self, on: bool) {
		if on {
			self.curses.attron(pancurses::A_DIM);
		}
		else {
			self.curses.attroff(pancurses::A_DIM);
		}
	}

	fn set_underline(&self, on: bool) {
		// Windows uses blue text for underlined words
		if !cfg!(windows) && on {
			self.curses.attron(pancurses::A_UNDERLINE);
		}
		else {
			self.curses.attroff(pancurses::A_UNDERLINE);
		}
	}

	fn set_reverse(&self, on: bool) {
		if on {
			self.curses.attron(pancurses::A_REVERSE);
		}
		else {
			self.curses.attroff(pancurses::A_REVERSE);
		}
	}

	pub(crate) fn getch(&self) -> Option<Input> {
		let input = self.curses.getch();

		if let Some(Input::KeyResize) = input {
			pancurses::resize_term(0, 0);
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
