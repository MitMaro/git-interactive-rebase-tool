#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DisplayColor {
	ActionBreak,
	ActionDrop,
	ActionEdit,
	ActionExec,
	ActionFixup,
	ActionPick,
	ActionReword,
	ActionSquash,
	DiffAddColor,
	DiffChangeColor,
	DiffRemoveColor,
	DiffContextColor,
	DiffWhitespaceColor,
	IndicatorColor,
	Normal,
}
