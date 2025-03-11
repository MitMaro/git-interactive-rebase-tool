use std::io::Write as _;

use tempfile::NamedTempFile;

use crate::git::Config;

pub(crate) fn with_git_config<F>(lines: &[&str], callback: F)
where F: FnOnce(Config) {
	let tmp_file = NamedTempFile::new().unwrap();
	writeln!(tmp_file.as_file(), "{}", lines.join("\n")).unwrap();
	callback(Config::open(tmp_file.path()).unwrap());
}
