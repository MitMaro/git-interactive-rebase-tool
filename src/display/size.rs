#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
	width: usize,
	height: usize,
}

impl Size {
	pub const fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	pub const fn width(&self) -> usize {
		self.width
	}

	pub const fn height(&self) -> usize {
		self.height
	}
}
