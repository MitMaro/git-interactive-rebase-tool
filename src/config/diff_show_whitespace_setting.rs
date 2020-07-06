#[derive(Clone, PartialEq, Debug)]
pub(crate) enum DiffShowWhitespaceSetting {
	None,
	Trailing,
	Leading,
	Both,
}
