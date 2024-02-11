/// Represents a terminal window size.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Size {
	width: usize,
	height: usize,
}

impl Size {
	/// Create a new instance with a width and height.
	#[inline]
	#[must_use]
	pub(crate) const fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	/// Get the width.
	#[inline]
	#[must_use]
	pub(crate) const fn width(&self) -> usize {
		self.width
	}

	/// Get the height.
	#[inline]
	#[must_use]
	pub(crate) const fn height(&self) -> usize {
		self.height
	}
}
