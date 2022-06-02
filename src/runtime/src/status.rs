use anyhow::Error;

/// The threads status.
#[derive(Debug)]
#[allow(variant_size_differences)]
#[allow(clippy::exhaustive_enums)]
pub enum Status {
	/// Thread is new, and hasn't yet started. This is the initial status of all threads.
	New,
	/// The thread is busy processing.
	Busy,
	/// The thread is waiting for more work to complete.
	Waiting,
	/// The thread is finished. This is a final state.
	Ended,
	/// The thread has requested all threads pause.
	RequestPause,
	/// The thread has requested all threads resume.
	RequestResume,
	/// The thread has requested all threads end.
	RequestEnd,
	/// The thread has errored with provided `Error`. This is a final state.
	Error(Error),
}

impl PartialEq for Status {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(&Self::New, &Self::New)
			| (&Self::Busy, &Self::Busy)
			| (&Self::Waiting, &Self::Waiting)
			| (&Self::Ended, &Self::Ended)
			| (&Self::RequestPause, &Self::RequestPause)
			| (&Self::RequestResume, &Self::RequestResume)
			| (&Self::RequestEnd, &Self::RequestEnd)
			| (&Self::Error(_), &Self::Error(_)) => true,
			(
				&(Self::New
				| Self::Busy
				| Self::Waiting
				| Self::Ended
				| Self::RequestPause
				| Self::RequestResume
				| Self::RequestEnd
				| Self::Error(_)),
				_,
			) => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;

	use super::*;

	#[test]
	fn partial_eq_matches() {
		assert_eq!(Status::New, Status::New);
		assert_eq!(Status::Busy, Status::Busy);
		assert_eq!(Status::Waiting, Status::Waiting);
		assert_eq!(Status::Ended, Status::Ended);
		assert_eq!(Status::Error(anyhow!("Error1")), Status::Error(anyhow!("Error2")));
		assert_eq!(Status::RequestPause, Status::RequestPause);
		assert_eq!(Status::RequestResume, Status::RequestResume);
		assert_eq!(Status::RequestEnd, Status::RequestEnd);
	}

	#[test]
	fn partial_eq_mismatches() {
		assert_ne!(Status::New, Status::Busy);
		assert_ne!(Status::Busy, Status::New);
		assert_ne!(Status::Waiting, Status::Busy);
		assert_ne!(Status::Ended, Status::Busy);
		assert_ne!(Status::Error(anyhow!("Error1")), Status::Busy);
		assert_ne!(Status::RequestPause, Status::Busy);
		assert_ne!(Status::RequestResume, Status::Busy);
		assert_ne!(Status::RequestEnd, Status::Busy);
	}
}
