mod color_manager;
mod curses;
#[allow(clippy::module_inception)]
mod display;
mod display_color;

pub use self::curses::Curses;
pub use self::display::Display;
pub use self::display_color::DisplayColor;
