#[derive(Clone, Copy, Debug)]
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
	IndicatorColor,
	Normal,
}
