#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum ViewAction {
	Stop,
	Refresh,
	Render,
	Start,
	End,
}
