use std::{
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
};

use crossbeam_channel::Sender;

use crate::runtime::{Notifier, Status, ThreadStatuses};

/// A thread installer that is passed to a `Threadable` when installing the threads into the `Runtime`
pub(crate) struct Installer {
	sender: Sender<(String, Status)>,
	thread_statuses: ThreadStatuses,
	ops: RefCell<HashMap<String, Box<dyn FnOnce() + Send>>>,
}

impl Installer {
	pub(crate) fn new(thread_statuses: ThreadStatuses, sender: Sender<(String, Status)>) -> Self {
		Self {
			sender,
			thread_statuses,
			ops: RefCell::new(HashMap::new()),
		}
	}

	pub(crate) fn into_ops(self) -> HashMap<String, Box<dyn FnOnce() + Send>> {
		self.ops.take()
	}

	/// Spawn a new thread with a name. The installer function callback will be called with a `Notifier` and is
	/// returns the thread function.
	pub(crate) fn spawn<InstallFn, ThreadFn>(&self, name: &str, install: InstallFn)
	where
		InstallFn: FnOnce(Notifier) -> ThreadFn,
		ThreadFn: FnOnce() + Send + 'static,
	{
		self.thread_statuses.register_thread(name, Status::New);
		let sender = self.sender.clone();
		let notifier = Notifier::new(name, sender);
		let _previous = self
			.ops
			.borrow_mut()
			.insert(String::from(name), Box::new(install(notifier)));
	}
}

impl Debug for Installer {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Installer")
			.field("sender", &self.sender)
			.field("thread_statuses", &self.thread_statuses)
			.finish_non_exhaustive()
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	};

	use crossbeam_channel::unbounded;

	use super::*;
	use crate::runtime::Threadable;

	struct Thread {
		called: Arc<AtomicBool>,
	}

	impl Thread {
		fn new() -> Self {
			Self {
				called: Arc::new(AtomicBool::new(false)),
			}
		}
	}

	impl Threadable for Thread {
		fn install(&self, installer: &Installer) {
			let called = Arc::clone(&self.called);
			installer.spawn("name", |_| {
				move || {
					called.store(true, Ordering::Relaxed);
				}
			});
		}
	}

	#[test]
	fn test() {
		let (sender, _receiver) = unbounded();
		let installer = Installer::new(ThreadStatuses::new(), sender);

		let thread = Thread::new();
		thread.install(&installer);

		let mut ops = installer.into_ops();
		let func = ops.remove("name").unwrap();
		func();

		assert!(thread.called.load(Ordering::Acquire));
	}

	#[test]
	fn debug() {
		let (sender, _receiver) = unbounded();
		let installer = Installer::new(ThreadStatuses::new(), sender);
		assert_eq!(
			format!("{installer:?}"),
			"Installer { sender: Sender { .. }, thread_statuses: ThreadStatuses { statuses: Mutex { data: {} } }, .. }"
		);
	}
}
