use std::{clone::Clone, sync::Arc, thread};

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;

use crate::{Installer, RuntimeError, Status, ThreadStatuses, Threadable};

const RUNTIME_THREAD_NAME: &str = "runtime";

/// A system the manages the lifetime of threads. This includes ensuring errors are handled, threads are paused and
/// resumed on request and that once the main application is completed, all threads complete and end.
#[allow(missing_debug_implementations)]
pub struct Runtime<'runtime> {
	receiver: Receiver<(String, Status)>,
	sender: Sender<(String, Status)>,
	thread_statuses: ThreadStatuses,
	threadables: Arc<Mutex<Vec<&'runtime mut dyn Threadable>>>,
}

impl<'runtime> Runtime<'runtime> {
	/// Create a new instances of the `Runtime`.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		let (sender, receiver) = unbounded();

		let thread_statuses = ThreadStatuses::new();
		thread_statuses.register_thread(RUNTIME_THREAD_NAME, Status::Waiting);

		Self {
			receiver,
			sender,
			thread_statuses,
			threadables: Arc::new(Mutex::new(vec![])),
		}
	}

	/// Get a cloned copy of the `ThreadStatuses`.
	#[inline]
	#[must_use]
	pub fn statuses(&self) -> ThreadStatuses {
		self.thread_statuses.clone()
	}

	/// Register a new `Threadable`.
	#[inline]
	pub fn register(&self, threadable: &'runtime mut (dyn Threadable)) {
		self.threadables.lock().push(threadable);
	}

	/// Join the runtime thread, waiting for all threads to finish.
	///
	/// # Errors
	/// Returns and error if any of the threads registered to the runtime produce an error.
	#[inline]
	pub fn join(&self) -> Result<(), RuntimeError> {
		let installer = Installer::new(self.sender.clone());
		{
			let threadables = self.threadables.lock();
			for threadable in threadables.iter() {
				threadable.install(&installer);
			}
		}
		let mut handles = vec![];

		for (name, op) in installer.into_ops().drain() {
			handles.push(
				thread::Builder::new()
					.name(String::from(name.as_str()))
					.spawn(op)
					.map_err(|_err| RuntimeError::ThreadSpawnError(name))?,
			);
		}

		let mut result = Ok(());

		for (name, status) in &self.receiver {
			match status {
				Status::Error(err) => {
					// since we entered an error state, we attempt to shutdown the other threads, but
					// they could fail due to the error state, but keeping the shutdown error is less
					// important than the original error.
					let _result = self.shutdown();
					result = Err(err);
					break;
				},
				Status::RequestPause => {
					for threadable in self.threadables.lock().iter() {
						threadable.pause();
					}
				},
				Status::RequestResume => {
					for threadable in self.threadables.lock().iter() {
						threadable.resume();
					}
				},
				Status::RequestEnd => {
					self.thread_statuses.update_thread(RUNTIME_THREAD_NAME, Status::Ended);
					for threadable in self.threadables.lock().iter() {
						threadable.end();
					}
				},
				Status::New | Status::Busy | Status::Waiting | Status::Ended => {},
			}

			self.thread_statuses.update_thread(name.as_str(), status);

			if self.thread_statuses.all_ended() {
				result = self.shutdown();
				break;
			}
		}

		while let Some(handle) = handles.pop() {
			let _result = handle.join();
		}

		result
	}

	#[inline]
	fn shutdown(&self) -> Result<(), RuntimeError> {
		if self.thread_statuses.all_ended() {
			return Ok(());
		}

		for threadable in self.threadables.lock().iter() {
			threadable.end();
		}
		self.sender
			.send((String::from(RUNTIME_THREAD_NAME), Status::Ended))
			.map_err(|_err| RuntimeError::SendError)
	}
}

#[cfg(test)]
mod tests {
	use std::{
		sync::atomic::{AtomicBool, Ordering},
		thread::sleep,
		time::Duration,
	};

	use claim::assert_err;

	use super::*;

	#[test]
	fn run_thread_finish() {
		struct Thread;

		impl Thread {
			const fn new() -> Self {
				Self {}
			}
		}

		impl Threadable for Thread {
			fn install(&self, installer: &Installer) {
				installer.spawn("name", |notifier| {
					move || {
						notifier.end();
						notifier.request_end();
					}
				});
			}
		}

		let runtime = Runtime::new();
		let mut thread = Thread::new();
		runtime.register(&mut thread);
		runtime.join().unwrap();
		assert!(runtime.statuses().all_ended());
	}

	#[test]
	fn run_thread_error() {
		struct Thread1;

		impl Thread1 {
			const fn new() -> Self {
				Self {}
			}
		}

		impl Threadable for Thread1 {
			fn install(&self, installer: &Installer) {
				installer.spawn("name0", |notifier| {
					move || {
						notifier.error(RuntimeError::ThreadError(String::from("error")));
					}
				});
			}
		}

		struct Thread2 {
			ended: Arc<AtomicBool>,
		}

		impl Thread2 {
			fn new() -> Self {
				Self {
					ended: Arc::new(AtomicBool::new(false)),
				}
			}
		}

		impl Threadable for Thread2 {
			fn install(&self, installer: &Installer) {
				let ended = Arc::clone(&self.ended);
				installer.spawn("name1", |notifier| {
					move || {
						while !ended.load(Ordering::Acquire) {
							sleep(Duration::from_millis(10));
						}
						notifier.end();
					}
				});
			}

			fn end(&self) {
				self.ended.store(true, Ordering::Release);
			}
		}

		let runtime = Runtime::new();
		let mut thread1 = Thread1::new();
		let mut thread2 = Thread2::new();
		runtime.register(&mut thread1);
		runtime.register(&mut thread2);
		assert_err!(runtime.join());
	}

	#[test]
	fn run_thread_request_pause() {
		struct Thread1;

		impl Thread1 {
			const fn new() -> Self {
				Self {}
			}
		}

		impl Threadable for Thread1 {
			fn install(&self, installer: &Installer) {
				installer.spawn("name0", |notifier| {
					move || {
						notifier.request_pause();
						notifier.end();
					}
				});
			}
		}

		struct Thread2 {
			paused: Arc<AtomicBool>,
		}

		impl Thread2 {
			fn new() -> Self {
				Self {
					paused: Arc::new(AtomicBool::new(false)),
				}
			}
		}

		impl Threadable for Thread2 {
			fn install(&self, installer: &Installer) {
				let paused = Arc::clone(&self.paused);
				installer.spawn("name1", |notifier| {
					move || {
						while !paused.load(Ordering::Acquire) {
							sleep(Duration::from_millis(10));
						}
						notifier.end();
						notifier.request_end();
					}
				});
			}

			fn pause(&self) {
				self.paused.store(true, Ordering::Release);
			}
		}

		let runtime = Runtime::new();
		let mut thread1 = Thread1::new();
		let mut thread2 = Thread2::new();
		runtime.register(&mut thread1);
		runtime.register(&mut thread2);
		runtime.join().unwrap();
		assert!(thread2.paused.load(Ordering::Acquire));
	}

	#[test]
	fn run_thread_request_resume() {
		struct Thread1;

		impl Thread1 {
			const fn new() -> Self {
				Self {}
			}
		}

		impl Threadable for Thread1 {
			fn install(&self, installer: &Installer) {
				installer.spawn("name0", |notifier| {
					move || {
						notifier.request_resume();
						notifier.end();
					}
				});
			}
		}

		struct Thread2 {
			resumed: Arc<AtomicBool>,
		}

		impl Thread2 {
			fn new() -> Self {
				Self {
					resumed: Arc::new(AtomicBool::new(false)),
				}
			}
		}

		impl Threadable for Thread2 {
			fn install(&self, installer: &Installer) {
				let resumed = Arc::clone(&self.resumed);
				installer.spawn("name1", |notifier| {
					move || {
						while !resumed.load(Ordering::Acquire) {
							sleep(Duration::from_millis(10));
						}
						notifier.end();
						notifier.request_end();
					}
				});
			}

			fn resume(&self) {
				self.resumed.store(true, Ordering::Release);
			}
		}

		let runtime = Runtime::new();
		let mut thread1 = Thread1::new();
		let mut thread2 = Thread2::new();
		runtime.register(&mut thread1);
		runtime.register(&mut thread2);
		runtime.join().unwrap();
		assert!(thread2.resumed.load(Ordering::Acquire));
	}

	#[test]
	fn run_thread_request_end() {
		struct Thread1;

		impl Thread1 {
			const fn new() -> Self {
				Self {}
			}
		}

		impl Threadable for Thread1 {
			fn install(&self, installer: &Installer) {
				installer.spawn("name0", |notifier| {
					move || {
						notifier.request_end();
						notifier.end();
					}
				});
			}
		}

		struct Thread2 {
			ended: Arc<AtomicBool>,
		}

		impl Thread2 {
			fn new() -> Self {
				Self {
					ended: Arc::new(AtomicBool::new(false)),
				}
			}
		}

		impl Threadable for Thread2 {
			fn install(&self, installer: &Installer) {
				let ended = Arc::clone(&self.ended);
				installer.spawn("name1", |notifier| {
					move || {
						while !ended.load(Ordering::Acquire) {
							sleep(Duration::from_millis(10));
						}
						notifier.end();
					}
				});
			}

			fn end(&self) {
				self.ended.store(true, Ordering::Release);
			}
		}

		let runtime = Runtime::new();
		let mut thread1 = Thread1::new();
		let mut thread2 = Thread2::new();
		runtime.register(&mut thread1);
		runtime.register(&mut thread2);
		runtime.join().unwrap();
		assert!(thread2.ended.load(Ordering::Acquire));
	}
}
