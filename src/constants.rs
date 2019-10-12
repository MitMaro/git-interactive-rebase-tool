pub const TITLE: &str = "Git Interactive Rebase Tool";
pub const TITLE_LENGTH: i32 = 27;
pub const TITLE_SHORT: &str = "Git Rebase";
pub const TITLE_SHORT_LENGTH: i32 = 10;
pub const TITLE_HELP_INDICATOR_LENGTH: i32 = 7;

pub const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue\n";
pub const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: usize = 45;
pub const SHORT_ERROR_MESSAGE: &str = "Window too small\n";
pub const SHORT_ERROR_MESSAGE_WIDTH: usize = 16;

pub const MINIMUM_WINDOW_HEIGHT: usize = 5; // title + pad top + line + pad bottom + help
pub const MINIMUM_COMPACT_WINDOW_WIDTH: usize = 20; // ">s ccc mmmmmmmmmmmmm".len()
pub const MINIMUM_FULL_WINDOW_WIDTH: usize = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

pub const NAME: &str = "interactive-rebase-tool";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
