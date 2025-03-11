/// An abstraction of colors to display.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DisplayColor {
	/// The color for the break action.
	ActionBreak,
	/// The color for the drop action.
	ActionDrop,
	/// The color for the edit action.
	ActionEdit,
	/// The color for the exec action.
	ActionExec,
	/// The color for the fixup action.
	ActionFixup,
	/// The color for the pick action.
	ActionPick,
	/// The color for the reword action.
	ActionReword,
	/// The color for the squash action.
	ActionSquash,
	/// The color for the label action.
	ActionLabel,
	/// The color for the reset action.
	ActionReset,
	/// The color for the merge action.
	ActionMerge,
	/// The color for the merge action.
	ActionUpdateRef,
	/// The color for added lines in a diff.
	DiffAddColor,
	/// The color for changed lines in a diff.
	DiffChangeColor,
	/// The color for removed lines in a diff.
	DiffRemoveColor,
	/// The color for context lines in a diff.
	DiffContextColor,
	/// The color for whitespace characters in a diff.
	DiffWhitespaceColor,
	/// The color for indicator text.
	IndicatorColor,
	/// The color for the standard text.
	Normal,
}
