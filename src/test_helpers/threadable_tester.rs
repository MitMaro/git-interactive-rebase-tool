//! Utilities for writing tests that interact with the runtime.
use std::{
	borrow::BorrowMut,
	mem,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	thread::{sleep, spawn},
	time::Duration,
};

use crossbeam_channel::{bounded, Receiver, Sender};
use parking_lot::Mutex;

use crate::runtime::{Installer, Status, ThreadStatuses};

const WAIT_TIME: Duration = Duration::from_millis(100);

/// A tester utility for `Threadable`.
#[derive(Clone, Debug)]
pub(crate) struct ThreadableTester {
	receiver: Receiver<(String, Status)>,
	sender: Sender<(String, Status)>,
	statuses: Arc<Mutex<Vec<Status>>>,
	ended: Arc<AtomicBool>,
}

impl ThreadableTester {
	/// Create a new instance of the test utility.
	#[must_use]
	pub(crate) fn new() -> Self {
		let (sender, receiver) = bounded(0);

		Self {
			receiver,
			sender,
			statuses: Arc::new(Mutex::new(vec![Status::New])),
			ended: Arc::new(AtomicBool::new(true)),
		}
	}

	/// Take the current `Status` changes.
	#[must_use]
	pub(crate) fn take_statuses(&self) -> Vec<Status> {
		mem::take(self.statuses.lock().borrow_mut())
	}

	/// Start a `Threadable` running the thread specified by the name, to completion in a separate thread.
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn start_threadable<Threadable: crate::runtime::Threadable>(
		&self,
		theadable: &Threadable,
		thread_name: &str,
	) {
		self.ended.store(false, Ordering::Release);
		let installer = Installer::new(ThreadStatuses::new(), self.sender.clone());
		theadable.install(&installer);
		let mut ops = installer.into_ops();
		let op = ops.remove(thread_name).expect("Expected to find thead");

		let statuses = Arc::clone(&self.statuses);
		let receiver = self.receiver.clone();

		let _status_thread_id = spawn(move || {
			for (_, status) in &receiver {
				let mut statuses_lock = statuses.lock();
				let last_status = statuses_lock.last().unwrap();
				if !matches!(*last_status, Status::Error(_)) && last_status != &status {
					statuses_lock.push(status);
				}
			}
		});
		let _op_id = spawn(op);
		self.ended.store(true, Ordering::Release);
	}

	/// Wait for a particular status to be reached.
	///
	/// # Panics
	///
	/// Will panic if the wait takes too long and times out.
	pub(crate) fn wait_for_status(&self, status: &Status) {
		let mut attempt = 0;

		loop {
			let statuses_lock = self.statuses.lock();
			let current_status = statuses_lock.last().unwrap();

			if current_status == status {
				break;
			}
			assert!(
				attempt <= 100,
				"Timeout waited for status change to '{status:?}' on thread.\n Status is: {current_status:?}",
			);

			sleep(WAIT_TIME);
			attempt += 1;
		}
	}

	/// Wait for an error status to be reached.
	///
	/// # Panics
	///
	/// Will panic if the wait takes too long and times out.
	pub(crate) fn wait_for_error_status(&self) {
		let mut attempt = 0;

		loop {
			let statuses_lock = self.statuses.lock();
			let current_status = statuses_lock.last().unwrap();

			if matches!(current_status, &Status::Error(_)) {
				break;
			}
			assert!(
				attempt <= 100,
				"Timeout waited for status change to 'Status::Error(_)' on thread.\n Status is: {current_status:?}"
			);

			sleep(WAIT_TIME);
			attempt += 1;
		}
	}

	/// Wait for the thread started in `start_threadable` to finish.
	///
	/// # Panics
	///
	/// Will panic if the wait takes too long and times out.
	pub(crate) fn wait_for_finished(&self) {
		let mut attempt = 0;

		loop {
			if self.ended.load(Ordering::Acquire) {
				break;
			}

			sleep(WAIT_TIME);
			attempt += 1;
			assert!(attempt <= 100, "Timeout waited for thread to finish");
		}
	}
}
