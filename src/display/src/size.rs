/// Represents a terminal window size.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
	width: usize,
	height: usize,
}

impl Size {
	/// Create a new instance with a width and height.
	#[inline]
	#[must_use]
	pub const fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	/// Get the width.
	#[inline]
	#[must_use]
	pub const fn width(&self) -> usize {
		self.width
	}

	/// Get the height.
	#[inline]
	#[must_use]
	pub const fn height(&self) -> usize {
		self.height
	}
}
