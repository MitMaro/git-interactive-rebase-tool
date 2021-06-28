#[derive(Debug)]
pub(crate) enum RenderAction {
	ScrollDown,
	ScrollUp,
	ScrollRight,
	ScrollLeft,
	PageUp,
	PageDown,
	Resize(usize, usize),
}
