#[cfg(not(test))]
pub use super::ncurses::Curses;
#[cfg(test)]
pub use super::virtual_curses::Curses;

pub use pancurses::{
	chtype,
	Input,
	A_DIM,
	A_REVERSE,
	A_UNDERLINE,
	COLOR_BLACK,
	COLOR_BLUE,
	COLOR_CYAN,
	COLOR_GREEN,
	COLOR_MAGENTA,
	COLOR_RED,
	COLOR_WHITE,
	COLOR_YELLOW,
};
