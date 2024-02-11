use crate::runtime::installer::Installer;

/// An interface for a entity that has threads managed by the `Runtime`.
pub(crate) trait Threadable: Send {
	/// Method that installs the threads that the `Threadable` is responsible for.
	fn install(&self, installer: &Installer);

	/// Called when threads are requested to pause.
	///
	/// # Errors
	/// Returns an error is that thread cannot be paused for any reason.
	#[inline]
	fn pause(&self) {}

	/// Called when threads are requested to resume.
	///
	/// # Errors
	/// Returns an error is that thread cannot be resumed for any reason.
	#[inline]
	fn resume(&self) {}

	/// Called when threads are requested to finish.
	///
	/// # Errors
	/// Returns an error is that thread cannot be ended for any reason.
	#[inline]
	fn end(&self) {}
}
