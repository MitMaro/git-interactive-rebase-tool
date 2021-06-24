//! Utilities for writing tests that interact with the display.
pub use super::mockcrossterm::CrossTerm;
use super::*;

/// Assert the the content of the Display is an expected value.
#[inline]
pub fn assert_output(display: &Display<CrossTerm>, expected: &[&str]) {
	assert_eq!(display.tui.get_output().join(""), format!("{}\n", expected.join("\n")));
}
