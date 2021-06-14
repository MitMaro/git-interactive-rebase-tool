use crate::core::exit::Exit;

#[cfg(not(feature = "dev"))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "dev")]
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-dev");

fn build_version() -> String {
	let mut parts = vec![];

	if let Some(hash) = option_env!("GIRT_BUILD_GIT_HASH") {
		parts.push(String::from(hash));
	}

	parts.push(String::from(env!("GIRT_BUILD_DATE")));

	format!("interactive-rebase-tool {} ({})", VERSION, parts.join(" "))
}

pub fn run() -> Exit {
	Exit::from(build_version())
}
