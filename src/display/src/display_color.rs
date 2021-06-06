#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum DisplayColor {
	ActionBreak,
	ActionDrop,
	ActionEdit,
	ActionExec,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	ActionLabel,
	ActionReset,
	ActionMerge,
	DiffAddColor,
	DiffChangeColor,
	DiffRemoveColor,
	DiffContextColor,
	DiffWhitespaceColor,
	IndicatorColor,
	Normal,
}
