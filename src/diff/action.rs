use std::fmt::{Debug, Formatter};

pub(crate) enum Action {
	End,
	Load(String),
}

impl Debug for Action {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::End => write!(f, "End"),
			Self::Load(ref hash) => write!(f, "Load({hash})"),
		}
	}
}
// #[cfg(test)]
// mod tests {
// 	use rstest::rstest;
//
// 	use super::*;
// 	use crate::search::{Interrupter, SearchResult};
//
// 	struct TestSearchable;
//
// 	impl Searchable for TestSearchable {
// 		fn reset(&mut self) {}
//
// 		fn search(&mut self, _: Interrupter, _: &str) -> SearchResult {
// 			SearchResult::None
// 		}
// 	}
//
// 	#[rstest]
// 	#[case::cancel(Action::Cancel, "Cancel")]
// 	#[case::cont(Action::Continue, "Continue")]
// 	#[case::end(Action::End, "End")]
// 	#[case::set_searchable(Action::SetSearchable(Box::new(TestSearchable {})), "SetSearchable(_)")]
// 	#[case::start(Action::Start(String::from("foo")), "Start(foo)")]
// 	fn debug(#[case] action: Action, #[case] expected: &str) {
// 		assert_eq!(format!("{action:?}"), expected);
// 	}
// }
