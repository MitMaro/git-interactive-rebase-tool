pub const TITLE: &str = "Git Interactive Rebase Tool";
pub const TITLE_LENGTH: usize = 27;
pub const TITLE_SHORT: &str = "Git Rebase";
pub const TITLE_SHORT_LENGTH: usize = 10;
pub const TITLE_HELP_INDICATOR_LENGTH: usize = 6;

pub const NAME: &str = "interactive-rebase-tool";

#[cfg(not(feature = "dev"))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "dev")]
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-dev");
