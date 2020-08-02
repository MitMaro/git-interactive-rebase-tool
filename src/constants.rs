pub const TITLE: &str = "Git Interactive Rebase Tool";
pub const TITLE_LENGTH: usize = 27;
pub const TITLE_SHORT: &str = "Git Rebase";
pub const TITLE_SHORT_LENGTH: usize = 10;
pub const TITLE_HELP_INDICATOR_LENGTH: usize = 6;

pub const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: usize = 45;

pub const MINIMUM_WINDOW_HEIGHT: usize = 5; // title + pad top + line + pad bottom + help
pub const MINIMUM_COMPACT_WINDOW_WIDTH: usize = 20; // ">s ccc mmmmmmmmmmmmm".len()
pub const MINIMUM_FULL_WINDOW_WIDTH: usize = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

pub const NAME: &str = "interactive-rebase-tool";

#[cfg(not(feature = "nightly"))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "nightly")]
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-nightly");
