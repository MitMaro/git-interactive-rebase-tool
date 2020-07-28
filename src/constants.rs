pub(crate) const TITLE: &str = "Git Interactive Rebase Tool";
pub(crate) const TITLE_LENGTH: usize = 27;
pub(crate) const TITLE_SHORT: &str = "Git Rebase";
pub(crate) const TITLE_SHORT_LENGTH: usize = 10;
pub(crate) const TITLE_HELP_INDICATOR_LENGTH: usize = 6;

pub(crate) const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue\n";
pub(crate) const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: usize = 45;
pub(crate) const SHORT_ERROR_MESSAGE: &str = "Window too small\n";
pub(crate) const SHORT_ERROR_MESSAGE_WIDTH: usize = 16;

pub(crate) const MINIMUM_WINDOW_HEIGHT: usize = 5; // title + pad top + line + pad bottom + help
pub(crate) const MINIMUM_COMPACT_WINDOW_WIDTH: usize = 20; // ">s ccc mmmmmmmmmmmmm".len()
pub(crate) const MINIMUM_FULL_WINDOW_WIDTH: usize = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

pub(crate) const NAME: &str = "interactive-rebase-tool";

#[cfg(not(feature = "nightly"))]
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "nightly")]
pub(crate) const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-nightly");
