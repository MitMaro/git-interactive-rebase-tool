#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
	width: usize,
	height: usize,
}

impl Size {
	#[inline]
	#[must_use]
	pub const fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	#[inline]
	#[must_use]
	pub const fn width(&self) -> usize {
		self.width
	}

	#[inline]
	#[must_use]
	pub const fn height(&self) -> usize {
		self.height
	}
}
