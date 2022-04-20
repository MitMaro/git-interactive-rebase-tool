#[derive(Debug)]
pub(crate) enum RenderAction {
	ScrollDown,
	ScrollUp,
	ScrollRight,
	ScrollLeft,
	ScrollTop,
	ScrollBottom,
	PageUp,
	PageDown,
	Resize(usize, usize),
}
