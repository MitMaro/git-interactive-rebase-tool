use crate::exit::Exit;

#[cfg(not(feature = "dev"))]
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "dev")]
pub(crate) const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-dev");

fn build_version() -> String {
	let mut parts = vec![];

	if let Some(hash) = option_env!("GIRT_BUILD_GIT_HASH") {
		parts.push(String::from(hash));
	}

	parts.push(String::from(env!("GIRT_BUILD_DATE")));

	format!("interactive-rebase-tool {VERSION} ({})", parts.join(" "))
}

pub(crate) fn run() -> Exit {
	Exit::from(build_version())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[serial_test::serial]
	fn test_run() {
		assert!(
			run()
				.get_message()
				.as_ref()
				.unwrap()
				.starts_with("interactive-rebase-tool")
		)
	}

	#[test]
	#[serial_test::serial]
	fn build_version_default() {
		let version = build_version();
		assert!(version.starts_with("interactive-rebase-tool"));
	}

	#[test]
	#[serial_test::serial]
	fn build_version_with_env() {
		let version = build_version();
		let expected_meta = format!("({} {})", env!("GIRT_BUILD_GIT_HASH"), env!("GIRT_BUILD_DATE"));
		assert!(version.starts_with("interactive-rebase-tool"));
		assert!(version.ends_with(expected_meta.as_str()));
	}
}
