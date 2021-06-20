#[derive(Clone, Copy, PartialEq, Debug)]
#[non_exhaustive]
pub enum DiffShowWhitespaceSetting {
	None,
	Trailing,
	Leading,
	Both,
}
