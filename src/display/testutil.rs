//! Utilities for writing tests that interact with the display.
mod mockable_tui;
mod mockcrossterm;
mod state;

pub(crate) use self::{
	mockable_tui::{create_unexpected_error, MockableTui},
	mockcrossterm::CrossTerm,
	state::State,
};
