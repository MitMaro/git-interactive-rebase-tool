use std::time::{Duration, Instant};

const INDICATOR_CHARACTERS: [&str; 4] = ["-", "\\", "|", "/"];
const ANIMATE_SPEED: Duration = Duration::from_millis(100);

pub(crate) struct SpinIndicator {
	index: u8,
	last_refreshed_at: Instant,
}

impl SpinIndicator {
	pub(crate) fn new() -> Self {
		Self {
			index: 0,
			last_refreshed_at: Instant::now(),
		}
	}

	pub(crate) fn refresh(&mut self) {
		if self.last_refreshed_at.elapsed() >= ANIMATE_SPEED {
			self.last_refreshed_at = Instant::now();
			self.index = if self.index == 3 { 0 } else { self.index + 1 }
		}
	}

	pub(crate) fn indicator(&self) -> String {
		format!("({})", INDICATOR_CHARACTERS[self.index as usize])
	}
}
#[cfg(test)]
mod tests {
	use std::ops::{Add, Sub};

	const SAFE_TEST_DURATION: Duration = Duration::from_secs(60);

	use super::*;

	#[test]
	fn does_not_advance_if_duration_has_not_elapsed() {
		let mut indicator = SpinIndicator::new();
		// this test is unlikely to finish before this elapsed time
		indicator.last_refreshed_at = Instant::now().add(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(-)");
		indicator.refresh();
		assert_eq!(indicator.indicator(), "(-)");
	}

	#[test]
	fn does_not_advance_if_duration_has_elapsed() {
		let mut indicator = SpinIndicator::new();
		indicator.last_refreshed_at = Instant::now().sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(-)");
		indicator.refresh();
		assert_eq!(indicator.indicator(), "(\\)");
	}

	const INDICATOR_CHARACTERS: [&str; 4] = ["-", "\\", "|", "/"];
	#[test]
	fn full_animation() {
		let mut indicator = SpinIndicator::new();
		indicator.last_refreshed_at = Instant::now().sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(-)");
		indicator.refresh();
		indicator.last_refreshed_at = indicator.last_refreshed_at.sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(\\)");
		indicator.refresh();
		indicator.last_refreshed_at = indicator.last_refreshed_at.sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(|)");
		indicator.refresh();
		indicator.last_refreshed_at = indicator.last_refreshed_at.sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(/)");
		indicator.refresh();
		indicator.last_refreshed_at = indicator.last_refreshed_at.sub(SAFE_TEST_DURATION);
		assert_eq!(indicator.indicator(), "(-)");
	}
}
