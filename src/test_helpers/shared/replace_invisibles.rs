const VISIBLE_SPACE_REPLACEMENT: &str = "\u{b7}"; // "·"
const VISIBLE_TAB_REPLACEMENT: &str = "   \u{2192}"; // "   →"

/// Replace invisible characters with visible counterparts
#[must_use]
pub(crate) fn replace_invisibles(line: &str) -> String {
	line.replace(' ', VISIBLE_SPACE_REPLACEMENT)
		.replace('\t', VISIBLE_TAB_REPLACEMENT)
}
