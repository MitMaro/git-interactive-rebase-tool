#[allow(clippy::module_inception)]
mod input;
mod input_handler;
mod utils;

pub use self::input::Input;
pub use self::input_handler::InputHandler;
pub use self::utils::curses_input_to_string;
