#[derive(Debug, PartialEq)]
pub(crate) enum Action {
	Start(String),
	Next(String),
	Previous(String),
	Cancel,
	None,
}
