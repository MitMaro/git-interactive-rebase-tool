use uuid::Uuid;

/// Tracks the changing state of the rebase file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
	id: Uuid,
	counter: u32,
}

impl Version {
	/// Create an empty version, useful as an initial version for caching
	#[inline]
	#[must_use]
	pub const fn empty() -> Self {
		Self {
			id: Uuid::nil(),
			counter: 0,
		}
	}

	pub(crate) fn new() -> Self {
		Self {
			id: Uuid::new_v4(),
			counter: 0,
		}
	}

	pub(crate) fn reset(&mut self) {
		self.id = Uuid::new_v4();
		self.counter = 0;
	}

	pub(crate) fn increment(&mut self) {
		self.counter = self.counter.wrapping_add(1);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let version = Version::empty();
		assert_eq!(version.id.as_u128(), 0);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn reset() {
		let mut version = Version::new();
		version.counter = 42;
		let prev_id = version.id;
		version.reset();
		assert_ne!(version.id, prev_id);
		assert_eq!(version.counter, 0);
	}

	#[test]
	fn increment() {
		let mut version = Version::new();
		let prev_id = version.id;
		version.increment();
		assert_eq!(version.id, prev_id);
		assert_eq!(version.counter, 1);
	}

	#[test]
	fn increment_wrap() {
		let mut version = Version::new();
		version.counter = u32::MAX;
		version.increment();
		assert_eq!(version.counter, 0);
	}
}
