/// Configuration option for how to show whitespace when displaying diffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum DiffShowWhitespaceSetting {
	/// Do not show whitespace characters.
	None,
	/// Show only trailing whitespace characters.
	Trailing,
	/// Show only leading whitespace characters.
	Leading,
	/// Show both leading and trailing whitespace characters.
	Both,
}

impl DiffShowWhitespaceSetting {
	pub(crate) fn parse(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
		"true" | "on" | "both" => Some(DiffShowWhitespaceSetting::Both),
		"trailing" => Some(DiffShowWhitespaceSetting::Trailing),
		"leading" => Some(DiffShowWhitespaceSetting::Leading),
		"false" | "off" | "none" => Some(DiffShowWhitespaceSetting::None),
		_ => None,
		}
	}
}