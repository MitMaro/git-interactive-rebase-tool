pub const NAME: &str = "interactive-rebase-tool";

#[cfg(not(feature = "dev"))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "dev")]
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-dev");
