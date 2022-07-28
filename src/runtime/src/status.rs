use crate::RuntimeError;

/// The threads status.
#[derive(Debug, PartialEq)]
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
	/// The thread has errored with provided `RuntimeError`. This is a final state.
	Error(RuntimeError),
}
