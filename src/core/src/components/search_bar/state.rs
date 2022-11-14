#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum State {
	Deactivated,
	Editing,
	Searching,
}

impl State {
	pub(crate) const fn is_active(self) -> bool {
		match self {
			Self::Deactivated => false,
			Self::Editing | Self::Searching => true,
		}
	}
}
