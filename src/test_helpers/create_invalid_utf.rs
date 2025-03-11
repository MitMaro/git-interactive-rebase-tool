use std::sync::LazyLock;

static INVALID_UTF_STRING: LazyLock<String> = LazyLock::new(create_invalid_utf);

#[expect(unsafe_code, reason = "Used for testing invalid UTF8 strings")]
fn create_invalid_utf() -> String {
	// used in tests to create an invalid value in a Git config file, while this is unsafe, it is
	// only ever used in tests to test the handling of invalid input data
	unsafe { String::from_utf8_unchecked(vec![0xC3, 0x28]) }
}

pub(crate) fn invalid_utf() -> &'static str {
	INVALID_UTF_STRING.as_str()
}
