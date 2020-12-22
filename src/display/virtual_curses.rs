use crate::display::color_mode::ColorMode;
use crate::display::utils::detect_color_mode;
pub use pancurses::{chtype, Input};
use std::cell::RefCell;

pub const A_DIM: chtype = 64;
pub const A_REVERSE: chtype = 128;
pub const A_UNDERLINE: chtype = 256;
pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
	Normal,
	Saved,
	Ended,
	Refreshed,
	Resized,
}

pub struct Curses {
	attributes: RefCell<chtype>,
	color_mode: ColorMode,
	color_pairs: [(i16, i16); 255],
	colors: [(i16, i16, i16); 255],
	input: RefCell<Vec<Input>>,
	output: RefCell<Vec<String>>,
	position: RefCell<(i32, i32)>,
	size: RefCell<(i32, i32)>,
	state: RefCell<State>,
}

impl Curses {
	pub(crate) fn new() -> Self {
		Self {
			attributes: RefCell::new(0),
			color_mode: detect_color_mode(16),
			color_pairs: [(0, 0); 255],
			colors: [(0, 0, 0); 255],
			input: RefCell::new(vec![Input::KeyExit]),
			output: RefCell::new(vec![]),
			position: RefCell::new((0, 0)),
			size: RefCell::new((10, 10)),
			state: RefCell::new(State::Normal),
		}
	}

	// Start mock access functions

	pub(crate) const fn get_colors(&self) -> &[(i16, i16, i16); 255] {
		&self.colors
	}

	pub(crate) fn get_output(&self) -> Vec<String> {
		self.output.borrow().clone()
	}

	pub(crate) const fn get_color_pairs(&self) -> &[(i16, i16); 255] {
		&self.color_pairs
	}

	pub(crate) fn get_state(&self) -> State {
		*self.state.borrow()
	}

	pub(crate) fn is_color_enabled(&self, color_index: chtype) -> bool {
		(*self.attributes.borrow() & color_index) == color_index
	}

	pub(crate) fn is_dimmed(&self) -> bool {
		(*self.attributes.borrow() & A_DIM) == A_DIM
	}

	pub(crate) fn is_reverse(&self) -> bool {
		(*self.attributes.borrow() & A_REVERSE) == A_REVERSE
	}

	pub(crate) fn is_underline(&self) -> bool {
		(*self.attributes.borrow() & A_UNDERLINE) == A_UNDERLINE
	}

	pub(crate) fn set_inputs(&self, mut input: Vec<Input>) {
		input.reverse();
		self.input.replace(input);
	}

	pub(crate) fn set_color_mode(&mut self, color_mode: ColorMode) {
		self.color_mode = color_mode;
	}

	// End mock access functions

	pub(super) fn init_color(&mut self, index: i16, red: i16, green: i16, blue: i16) {
		self.colors[index as usize] = (red, green, blue);
	}

	pub(super) fn init_color_pair(&mut self, index: i16, foreground: i16, background: i16) -> chtype {
		self.color_pairs[index as usize] = (foreground, background);
		index as chtype
	}

	pub(super) const fn get_color_mode(&self) -> &ColorMode {
		&self.color_mode
	}

	pub(super) fn erase(&self) {
		self.output.borrow_mut().clear();
	}

	pub(super) fn refresh(&self) {
		self.state.replace(State::Refreshed);
	}

	pub(super) fn addstr(&self, s: &str) {
		self.output.borrow_mut().push(String::from(s));
	}

	pub(super) fn attrset<T: Into<chtype>>(&self, attributes: T) {
		let attrs = attributes.into();
		self.attributes.replace(attrs);
	}

	pub(super) fn attron<T: Into<chtype>>(&self, attribute: T) {
		let attr = attribute.into();
		let old_attr = *self.attributes.borrow();
		self.attributes.replace(old_attr | attr);
	}

	pub(super) fn attroff<T: Into<chtype>>(&self, attribute: T) {
		let attr = attribute.into();
		let old_attr = *self.attributes.borrow();
		self.attributes.replace(old_attr & !attr);
	}

	pub(super) fn getch(&self) -> Option<Input> {
		self.input.borrow_mut().pop()
	}

	pub(crate) fn get_cur_y(&self) -> i32 {
		(*self.position.borrow()).1
	}

	pub(crate) fn get_cur_x(&self) -> i32 {
		(*self.position.borrow()).0
	}

	pub(super) fn get_max_x(&self) -> i32 {
		(*self.size.borrow()).0
	}

	pub(super) fn get_max_y(&self) -> i32 {
		(*self.size.borrow()).1
	}

	pub(crate) fn hline(&self, ch: char, width: i32) {
		self.output.borrow_mut().push(format!("{{HLINE|{}|{}}}", ch, width));
	}

	pub(crate) fn mv(&self, y: i32, x: i32) {
		self.position.replace((x, y));
	}

	pub(crate) fn resize_term(&self, nlines: i32, ncols: i32) {
		if nlines == 0 && ncols == 0 {
			self.state.replace(State::Resized);
		}
		else {
			self.size.replace((ncols, nlines));
		}
	}

	pub(super) fn def_prog_mode(&self) {
		self.state.replace(State::Saved);
	}

	pub(super) fn reset_prog_mode(&self) {
		self.state.replace(State::Normal);
	}

	pub(super) fn endwin(&self) {
		self.state.replace(State::Ended);
	}
}
