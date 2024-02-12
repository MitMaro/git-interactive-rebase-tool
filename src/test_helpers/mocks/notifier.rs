use crate::runtime::{Status, ThreadStatuses};

/// A mocked version of the `Notifier`, that will interact directly with a `ThreadStatuses` without the use of a thread
/// or the `Runtime`.
#[derive(Debug)]
pub(crate) struct Notifier<'notifier> {
	threadable_statuses: &'notifier ThreadStatuses,
}

impl<'notifier> Notifier<'notifier> {
	/// Create a new instance of a `MockNotifier`.
	#[must_use]
	pub(crate) const fn new(threadable_statuses: &'notifier ThreadStatuses) -> Self {
		Self { threadable_statuses }
	}

	/// Register a thread by name and status. This does not create a thread.
	pub(crate) fn register_thread(&mut self, thread_name: &str, status: Status) {
		self.threadable_statuses.register_thread(thread_name, status);
	}
}
