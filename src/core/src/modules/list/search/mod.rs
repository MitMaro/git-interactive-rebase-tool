mod line_match;
#[allow(clippy::module_inception)]
mod search;
mod state;

pub(crate) use self::{line_match::LineMatch, search::Search, state::State};
