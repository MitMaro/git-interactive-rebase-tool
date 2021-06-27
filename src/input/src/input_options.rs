#[derive(Copy, Clone, Debug)]
pub struct InputOptions {
	pub(super) help: bool,
	pub(super) movement: bool,
	pub(super) resize: bool,
	pub(super) undo_redo: bool,
}

impl InputOptions {
	#[inline]
	#[must_use]
	pub const fn new() -> Self {
		Self {
			help: false,
			movement: false,
			resize: true,
			undo_redo: false,
		}
	}

	#[inline]
	#[must_use]
	pub const fn help(mut self, val: bool) -> Self {
		self.help = val;
		self
	}

	#[inline]
	#[must_use]
	pub const fn movement(mut self, val: bool) -> Self {
		self.movement = val;
		self
	}

	#[inline]
	#[must_use]
	pub const fn resize(mut self, val: bool) -> Self {
		self.resize = val;
		self
	}

	#[inline]
	#[must_use]
	pub const fn undo_redo(mut self, val: bool) -> Self {
		self.undo_redo = val;
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_default() {
		let options = InputOptions::new();
		assert!(!options.help);
		assert!(!options.movement);
		assert!(options.resize);
		assert!(!options.undo_redo);
	}

	#[test]
	fn help() {
		let options = InputOptions::new().help(true);
		assert!(options.help);
	}

	#[test]
	fn movement() {
		let options = InputOptions::new().movement(true);
		assert!(options.movement);
	}

	#[test]
	fn resize() {
		let options = InputOptions::new().resize(false);
		assert!(!options.resize);
	}

	#[test]
	fn undo_redo() {
		let options = InputOptions::new().undo_redo(true);
		assert!(options.undo_redo);
	}
}
