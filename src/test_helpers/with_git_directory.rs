use std::path::{Path, PathBuf};

use crate::test_helpers::{EnvVarAction, with_env_var};

pub(crate) fn with_git_directory<C>(repo: &str, callback: C)
where C: FnOnce(&str) {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join(repo)
		.canonicalize()
		.unwrap_or(PathBuf::from("does-not-exist"));
	with_env_var(
		&[EnvVarAction::Set("GIT_DIR", String::from(path.to_str().unwrap()))],
		|| callback(path.to_str().unwrap()),
	);
}
