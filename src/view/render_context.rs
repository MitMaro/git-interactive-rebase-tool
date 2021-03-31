const MINIMUM_WINDOW_HEIGHT: usize = 5; // title + pad top + line + pad bottom + help
const MINIMUM_COMPACT_WINDOW_WIDTH: usize = 20; // ">s ccc mmmmmmmmmmmmm".len()
const MINIMUM_FULL_WINDOW_WIDTH: usize = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

#[derive(Debug)]
pub struct RenderContext {
	height: usize,
	width: usize,
}

impl RenderContext {
	pub const fn new(width: usize, height: usize) -> Self {
		Self { height, width }
	}

	pub const fn width(&self) -> usize {
		self.width
	}

	pub const fn height(&self) -> usize {
		self.height
	}

	pub const fn is_minimum_view_width(&self) -> bool {
		self.width > MINIMUM_COMPACT_WINDOW_WIDTH
	}

	pub const fn is_minimum_view_height(&self) -> bool {
		self.height > MINIMUM_WINDOW_HEIGHT
	}

	pub const fn is_full_width(&self) -> bool {
		self.width >= MINIMUM_FULL_WINDOW_WIDTH
	}

	pub const fn is_window_too_small(&self) -> bool {
		!self.is_minimum_view_width() || !self.is_minimum_view_height()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
