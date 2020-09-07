#[cfg(not(test))]
pub use super::ncurses::*;
#[cfg(test)]
pub use super::virtual_curses::*;
