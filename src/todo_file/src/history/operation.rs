#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Operation {
	Modify,
	SwapUp,
	SwapDown,
	Add,
	Remove,
}
