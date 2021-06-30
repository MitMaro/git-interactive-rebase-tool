#[derive(Debug, PartialEq)]
pub(crate) enum Operation {
	Modify,
	SwapUp,
	SwapDown,
	Add,
	Remove,
}
