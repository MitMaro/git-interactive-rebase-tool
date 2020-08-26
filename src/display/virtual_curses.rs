#![allow(non_camel_case_types)]

use crate::build_trace;
use crate::display::color_mode::ColorMode;
use crate::display::utils::detect_color_mode;
use crate::display::{chtype, Input};
use std::cell::RefCell;

pub struct Curses {
	attributes: RefCell<chtype>,
	color_mode: ColorMode,
	color_pairs: [(i16, i16); 255],
	colors: [(i16, i16, i16); 255],
	function_call_trace: RefCell<Vec<(String, Vec<String>)>>,
	output: RefCell<Vec<String>>,
	input: RefCell<Vec<Input>>,
	position: RefCell<(i32, i32)>,
	size: RefCell<(i32, i32)>,
}

impl Curses {
	pub(crate) fn new() -> Self {
		Self {
			attributes: RefCell::new(0),
			color_mode: detect_color_mode(16),
			color_pairs: [(0, 0); 255],
			colors: [(0, 0, 0); 255],
			function_call_trace: RefCell::new(vec![build_trace!("new")]),
			output: RefCell::new(vec![]),
			input: RefCell::new(vec![]),
			position: RefCell::new((0, 0)),
			size: RefCell::new((10, 10)),
		}
	}

	pub(crate) fn get_function_trace(&self) -> Vec<(String, Vec<String>)> {
		self.function_call_trace.borrow().clone()
	}

	pub(crate) fn push_input(&mut self, input: Input) {
		self.input.borrow_mut().push(input);
	}

	pub(super) fn init_color(&mut self, index: i16, red: i16, green: i16, blue: i16) {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("init_color", index, red, green, blue));
		self.colors[index as usize] = (red, green, blue);
	}

	pub(super) fn init_color_pair(&mut self, index: i16, foreground: i16, background: i16) -> chtype {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("init_color_pair", index, foreground, background));
		self.color_pairs[index as usize] = (foreground, background);
		index as chtype
	}

	pub(super) fn get_color_mode(&self) -> &ColorMode {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("get_color_mode"));
		&self.color_mode
	}

	pub(super) fn erase(&self) {
		self.function_call_trace.borrow_mut().push(build_trace!("erase"));
		self.output.borrow_mut().clear();
	}

	pub(super) fn refresh(&self) {
		self.function_call_trace.borrow_mut().push(build_trace!("refresh"));
	}

	pub(super) fn addstr(&self, s: &str) {
		self.function_call_trace.borrow_mut().push(build_trace!("addstr", s));
		self.output.borrow_mut().push(String::from(s));
	}

	pub(super) fn attrset<T: Into<chtype>>(&self, attributes: T) {
		let attributes = attributes.into();
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("attrset", attributes));
		self.attributes.replace(attributes);
	}

	pub(super) fn attron<T: Into<chtype>>(&self, attributes: T) {
		let attributes = attributes.into();
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("attron", attributes));
		let attr = *self.attributes.borrow();
		self.attributes.replace(attr | attributes);
	}

	pub(super) fn attroff<T: Into<chtype>>(&self, attributes: T) {
		let attributes = attributes.into();
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("attroff", attributes));
		let attr = *self.attributes.borrow();
		self.attributes.replace(attr & !attributes);
	}

	pub(super) fn getch(&self) -> Option<Input> {
		self.function_call_trace.borrow_mut().push(build_trace!("getch"));
		self.input.borrow_mut().pop()
	}

	pub(crate) fn get_cur_y(&self) -> i32 {
		self.function_call_trace.borrow_mut().push(build_trace!("get_cur_y"));
		(*self.position.borrow()).1
	}

	pub(super) fn get_max_x(&self) -> i32 {
		self.function_call_trace.borrow_mut().push(build_trace!("get_max_x"));
		(*self.size.borrow()).0
	}

	pub(super) fn get_max_y(&self) -> i32 {
		self.function_call_trace.borrow_mut().push(build_trace!("get_max_y"));
		(*self.size.borrow()).1
	}

	pub(crate) fn hline(&self, ch: char, width: i32) {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("hline", ch, width));
	}

	pub(crate) fn mv(&self, y: i32, x: i32) {
		self.function_call_trace.borrow_mut().push(build_trace!("mv", y, x));
		self.position.replace((x, y));
	}

	pub(crate) fn resize_term(&self, nlines: i32, ncols: i32) {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("resize_term", nlines, ncols));
		self.size.replace((ncols, nlines));
	}

	pub(super) fn def_prog_mode(&self) {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("def_prog_mode"));
	}

	pub(super) fn reset_prog_mode(&self) {
		self.function_call_trace
			.borrow_mut()
			.push(build_trace!("reset_prog_mode"));
	}

	pub(super) fn endwin(&self) {
		self.function_call_trace.borrow_mut().push(build_trace!("endwin"));
	}
}
