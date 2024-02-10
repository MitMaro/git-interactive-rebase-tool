#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Operation {
	Load,
	Modify,
	SwapUp,
	SwapDown,
	Add,
	Remove,
}
