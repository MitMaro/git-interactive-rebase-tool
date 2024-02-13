use std::{
	env::set_var,
	path::{Path, PathBuf},
};

pub(crate) fn set_git_directory(repo: &str) -> String {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join(repo)
		.canonicalize()
		.unwrap_or(PathBuf::from("does-not-exist"));
	set_var("GIT_DIR", path.to_str().unwrap());
	String::from(path.to_str().unwrap())
}
