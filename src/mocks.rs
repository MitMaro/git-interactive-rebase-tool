
pub mod mockcurses {
	pub use pancurses::{
		COLOR_BLACK,
		COLOR_WHITE,
		COLOR_YELLOW,
		COLOR_BLUE,
		COLOR_GREEN,
		COLOR_CYAN,
		COLOR_MAGENTA,
		COLOR_RED,
		A_UNDERLINE,
		A_BOLD,
		A_DIM,
		A_REVERSE,
		COLOR_PAIR,
		Input,
		chtype
	};

	pub fn curs_set(_visibility: i32) {}
	pub fn def_prog_mode() -> i32 {0}
	pub fn endwin() {}
	pub fn has_colors() -> bool {
		false
	}
	pub fn init_pair(_pair_index: i16, _foreground_color: i16, _background_color: i16) {}
	pub fn initscr() -> Window {
		Window {
			max_y: 2,
			next_char: Input::KeyClear
		}
	}
	pub fn noecho() {}
	pub fn reset_prog_mode() -> i32 {0}
	pub fn resize_term(_nlines: i32, _ncols: i32) {}
	pub fn start_color() {}
	pub fn use_default_colors() {}
	
	#[derive(Debug)]
	pub struct Window {
		pub max_y: i32,
		pub next_char: Input
	}
	
	impl Window {
		pub fn addstr(&self, _string: &str) {}
		pub fn attroff(&self, _attributes: chtype) {}
		pub fn attron(&self, _attributes: chtype) {}
		pub fn attrset(&self, _attributes: chtype) {}
		pub fn clear(&self) {}
		pub fn get_max_y(&self) -> i32 {self.max_y}
		pub fn getch(&self) -> Option<Input> {Some(self.next_char)}
		pub fn keypad(&self, _a: bool) {}
		pub fn mvaddstr(&self, _y: i32, _x: i32, _string: &str) {}
		pub fn refresh(&self) {}
		
	}
}
