#[derive(Debug)]
pub enum RenderAction {
	ScrollDown,
	ScrollUp,
	ScrollRight,
	ScrollLeft,
	PageUp,
	PageDown,
	Resize(usize, usize),
}
