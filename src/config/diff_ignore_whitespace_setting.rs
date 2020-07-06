#[derive(Clone, PartialEq, Debug)]
pub(crate) enum DiffIgnoreWhitespaceSetting {
	None,
	All,
	Change,
}
