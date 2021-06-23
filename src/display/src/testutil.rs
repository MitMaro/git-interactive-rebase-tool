pub use super::mockcrossterm::CrossTerm;
use super::*;

#[inline]
pub fn assert_output(display: &Display<CrossTerm>, expected: &[&str]) {
	assert_eq!(display.tui.get_output().join(""), format!("{}\n", expected.join("\n")));
}
