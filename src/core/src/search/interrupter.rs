use std::{
	ops::Add,
	time::{Duration, Instant},
};

pub(crate) struct Interrupter {
	finish: Instant,
}

impl Interrupter {
	pub(crate) fn new(duration: Duration) -> Self {
		Self {
			finish: Instant::now().add(duration),
		}
	}

	pub(crate) fn should_continue(&self) -> bool {
		Instant::now() < self.finish
	}
}

#[cfg(test)]
mod test {
	use std::{ops::Sub, time::Duration};

	use super::*;
	use crate::search::Interrupter;

	#[test]
	fn should_continue_before_finish() {
		let interrupter = Interrupter::new(Duration::from_secs(60));
		assert!(interrupter.should_continue());
	}

	#[test]
	fn should_continue_after_finish() {
		let interrupter = Interrupter {
			finish: Instant::now().sub(Duration::from_secs(60)),
		};
		assert!(!interrupter.should_continue());
	}
}
