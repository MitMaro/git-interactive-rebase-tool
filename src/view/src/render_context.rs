const MINIMUM_WINDOW_HEIGHT: usize = 5; // title + pad top + line + pad bottom + help
const MINIMUM_COMPACT_WINDOW_WIDTH: usize = 20; // ">s ccc mmmmmmmmmmmmm".len()
const MINIMUM_FULL_WINDOW_WIDTH: usize = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

/// Represents data associated with rendering content.
#[derive(Debug, Copy, Clone)]
pub struct RenderContext {
	height: usize,
	width: usize,
}

impl RenderContext {
	/// Create a new instance with a width and height.
	#[must_use]
	#[inline]
	pub const fn new(width: u16, height: u16) -> Self {
		Self {
			height: height as usize,
			width: width as usize,
		}
	}

	/// Update the recorded width and height.
	#[inline]
	pub fn update(&mut self, width: u16, height: u16) {
		self.width = width as usize;
		self.height = height as usize;
	}

	/// Get the width of the terminal window.
	#[must_use]
	#[inline]
	pub const fn width(&self) -> usize {
		self.width
	}

	/// Get the height of the terminal window.
	#[must_use]
	#[inline]
	pub const fn height(&self) -> usize {
		self.height
	}

	/// Is the terminal window width at least the minimal supported width.
	#[must_use]
	#[inline]
	pub const fn is_minimum_view_width(&self) -> bool {
		self.width > MINIMUM_COMPACT_WINDOW_WIDTH
	}

	/// Is the terminal window height at least the minimal supported height.
	#[must_use]
	#[inline]
	pub const fn is_minimum_view_height(&self) -> bool {
		self.height > MINIMUM_WINDOW_HEIGHT
	}

	/// Is the terminal window large enough to render lines using their full width.
	#[must_use]
	#[inline]
	pub const fn is_full_width(&self) -> bool {
		self.width >= MINIMUM_FULL_WINDOW_WIDTH
	}

	/// Is the terminal window too small to render content.
	#[must_use]
	#[inline]
	pub const fn is_window_too_small(&self) -> bool {
		!self.is_minimum_view_width() || !self.is_minimum_view_height()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn update() {
		let mut context = RenderContext { width: 10, height: 20 };
		context.update(100, 200);
		assert_eq!(context.width(), 100);
		assert_eq!(context.height(), 200);
	}

	#[test]
	fn is_window_too_small_width_too_small() {
		let context = RenderContext {
			width: MINIMUM_COMPACT_WINDOW_WIDTH,
			height: MINIMUM_WINDOW_HEIGHT + 1,
		};
		assert!(context.is_window_too_small());
	}

	#[test]
	fn is_window_too_small_height_too_small() {
		let context = RenderContext {
			width: MINIMUM_COMPACT_WINDOW_WIDTH + 1,
			height: MINIMUM_WINDOW_HEIGHT,
		};
		assert!(context.is_window_too_small());
	}

	#[test]
	fn is_window_too_small_height_and_width_too_small() {
		let context = RenderContext {
			width: MINIMUM_COMPACT_WINDOW_WIDTH,
			height: MINIMUM_WINDOW_HEIGHT,
		};
		assert!(context.is_window_too_small());
	}

	#[test]
	fn is_window_too_small_width_and_height_large() {
		let context = RenderContext {
			width: MINIMUM_COMPACT_WINDOW_WIDTH + 1,
			height: MINIMUM_WINDOW_HEIGHT + 1,
		};
		assert!(!context.is_window_too_small());
	}
}
