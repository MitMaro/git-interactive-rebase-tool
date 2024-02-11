#[derive(Debug, PartialEq)]
pub(crate) enum Action {
	Update(String),
	Start(String),
	Next(String),
	Previous(String),
	Cancel,
	None,
}
