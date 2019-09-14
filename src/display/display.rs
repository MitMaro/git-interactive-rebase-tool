use crate::config::Config;
use crate::display::color_manager::ColorManager;
use crate::display::display_color::DisplayColor;
use crate::display::Curses;
use pancurses::{chtype, Input};
use std::cell::RefCell;

pub struct Display<'d> {
	color_manager: ColorManager,
	curses: &'d Curses,
	height: RefCell<i32>,
	width: RefCell<i32>,
}

impl<'d> Display<'d> {
	pub fn new(curses: &'d mut Curses, config: &'d Config) -> Self {
		Self {
			color_manager: ColorManager::new(&config.theme, curses),
			curses,
			height: RefCell::new(curses.get_max_y()),
			width: RefCell::new(curses.get_max_x()),
		}
	}

	pub fn draw_str(&self, s: &str) {
		self.curses.addstr(s);
	}

	pub fn clear(&self) {
		self.color(DisplayColor::Normal, false);
		self.set_style(false, false, false);
		self.curses.erase();
	}

	pub fn refresh(&self) {
		self.curses.refresh();
	}

	pub fn color(&self, color: DisplayColor, selected: bool) {
		let selected = selected && self.curses.number_of_colors > 8;
		self.curses
			.attrset(self.color_manager.get_color(color, selected) as chtype);
	}

	pub fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.set_dim(dim);
		self.set_underline(underline);
		self.set_reverse(reverse);
	}

	pub fn set_dim(&self, on: bool) {
		if on {
			self.curses.attron(pancurses::A_DIM);
		}
		else {
			self.curses.attroff(pancurses::A_DIM);
		}
	}

	pub fn set_underline(&self, on: bool) {
		if on {
			self.curses.attron(pancurses::A_UNDERLINE);
		}
		else {
			self.curses.attroff(pancurses::A_UNDERLINE);
		}
	}

	pub fn set_reverse(&self, on: bool) {
		if on {
			self.curses.attron(pancurses::A_REVERSE);
		}
		else {
			self.curses.attroff(pancurses::A_REVERSE);
		}
	}

	pub fn getch(&self) -> Option<Input> {
		let input = self.curses.getch();

		if let Some(Input::KeyResize) = input {
			pancurses::resize_term(0, 0);
			self.height.replace(self.curses.get_max_y());
			self.width.replace(self.curses.get_max_x());
		}
		input
	}

	pub fn get_window_size(&self) -> (i32, i32) {
		(*self.width.borrow(), *self.height.borrow())
	}

	/// Leaves curses mode, runs the specified callback, and re-enables curses.
	pub fn leave_temporarily<F, T>(&self, callback: F) -> T
	where F: FnOnce() -> T {
		self.curses.def_prog_mode();
		self.curses.endwin();
		let rv = callback();
		self.curses.reset_prog_mode();
		rv
	}

	pub fn end(&self) {
		self.curses.endwin();
	}
}
