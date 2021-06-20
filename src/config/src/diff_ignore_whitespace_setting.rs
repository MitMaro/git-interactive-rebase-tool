#[derive(Clone, Copy, PartialEq, Debug)]
#[non_exhaustive]
pub enum DiffIgnoreWhitespaceSetting {
	None,
	All,
	Change,
}
