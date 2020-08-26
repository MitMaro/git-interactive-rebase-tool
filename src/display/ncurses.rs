use crate::display::color_mode::ColorMode;
use crate::display::utils::detect_color_mode;
use pancurses::{chtype, Input};

pub struct Curses {
	color_mode: ColorMode,
	window: pancurses::Window,
}

impl Curses {
	pub(crate) fn new() -> Self {
		let window = pancurses::initscr();
		window.keypad(true);

		pancurses::curs_set(0);
		pancurses::noecho();

		let has_colors = pancurses::has_colors();
		if has_colors {
			pancurses::start_color();
			pancurses::use_default_colors();

			// pair zero should always be default
			pancurses::init_pair(0, -1, -1);
		}

		let color_mode = if has_colors {
			detect_color_mode(pancurses::COLORS() as i16)
		}
		else {
			ColorMode::TwoTone
		};

		Self { window, color_mode }
	}

	#[allow(clippy::unused_self)]
	pub(super) fn init_color(&self, index: i16, red: i16, green: i16, blue: i16) {
		pancurses::init_color(index, red, green, blue);
	}

	#[allow(clippy::unused_self)]
	pub(super) fn init_color_pair(&self, index: i16, foreground: i16, background: i16) -> chtype {
		pancurses::init_pair(index, foreground, background);
		// curses seems to init a pair for i16 but read with u64
		pancurses::COLOR_PAIR(index as chtype)
	}

	pub(super) const fn get_color_mode(&self) -> &ColorMode {
		&self.color_mode
	}

	pub(super) fn erase(&self) {
		self.window.erase();
	}

	pub(super) fn refresh(&self) {
		self.window.refresh();
	}

	pub(super) fn addstr(&self, s: &str) {
		self.window.addstr(s);
	}

	pub(super) fn attrset<T: Into<chtype>>(&self, attributes: T) {
		self.window.attrset(attributes);
	}

	pub(super) fn attron<T: Into<chtype>>(&self, attributes: T) {
		self.window.attron(attributes);
	}

	pub(super) fn attroff<T: Into<chtype>>(&self, attributes: T) {
		self.window.attroff(attributes);
	}

	pub(super) fn getch(&self) -> Option<Input> {
		self.window.getch()
	}

	pub(crate) fn get_cur_y(&self) -> i32 {
		self.window.get_cur_y()
	}

	pub(super) fn get_max_y(&self) -> i32 {
		self.window.get_max_y()
	}

	pub(super) fn get_max_x(&self) -> i32 {
		self.window.get_max_x()
	}

	pub(crate) fn hline(&self, ch: char, width: i32) {
		self.window.hline(ch, width);
	}

	pub(crate) fn mv(&self, y: i32, x: i32) {
		self.window.mv(y, x);
	}

	#[allow(clippy::unused_self)]
	pub(super) fn resize_term(&self, nlines: i32, ncols: i32) {
		pancurses::resize_term(nlines, ncols);
	}

	#[allow(clippy::unused_self)]
	pub(super) fn def_prog_mode(&self) {
		pancurses::def_prog_mode();
	}

	#[allow(clippy::unused_self)]
	pub(super) fn reset_prog_mode(&self) {
		pancurses::reset_prog_mode();
	}

	#[allow(clippy::unused_self)]
	pub(super) fn endwin(&self) {
		pancurses::endwin();
	}
}
