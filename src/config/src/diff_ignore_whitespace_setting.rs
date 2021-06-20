#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum DiffIgnoreWhitespaceSetting {
	None,
	All,
	Change,
}
