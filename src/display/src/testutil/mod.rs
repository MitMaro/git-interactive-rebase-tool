//! Utilities for writing tests that interact with the display.
mod mockcrossterm;
mod state;

pub use self::{mockcrossterm::CrossTerm, state::State};
use crate::Display;

/// Assert the the content of the Display is an expected value.
#[inline]
pub fn assert_output(display: &Display<CrossTerm>, expected: &[&str]) {
	assert_eq!(display.tui.get_output().join(""), format!("{}\n", expected.join("\n")));
}
