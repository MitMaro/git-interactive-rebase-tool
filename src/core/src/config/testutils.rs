use std::io::Write;

use lazy_static::lazy_static;
use tempfile::NamedTempFile;

use crate::git::Config;

lazy_static! {
	static ref INVALID_UTF_STRING: String = create_invalid_utf();
}

pub(crate) fn with_git_config<F>(lines: &[&str], callback: F)
where F: FnOnce(Config) {
	let tmp_file = NamedTempFile::new().unwrap();
	writeln!(tmp_file.as_file(), "{}", lines.join("\n")).unwrap();
	callback(Config::open(tmp_file.path()).unwrap());
}

#[allow(unsafe_code)]
fn create_invalid_utf() -> String {
	// used in tests to create an invalid value in a Git config file, while this is unsafe, it is
	// only ever used in tests to test the handling of invalid input data
	unsafe { String::from_utf8_unchecked(vec![0xC3, 0x28]) }
}

pub(crate) fn invalid_utf() -> &'static str {
	INVALID_UTF_STRING.as_str()
}
