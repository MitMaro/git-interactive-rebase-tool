use anyhow::Result;

use crate::installer::Installer;

/// An interface for a entity that has threads managed by the `Runtime`.
pub trait Threadable: Send {
	/// Method that installs the threads that the `Threadable` is responsible for.
	fn install(&self, installer: &Installer);

	/// Called when threads are requested to pause.
	///
	/// # Errors
	/// Returns an error is that thread cannot be paused for any reason.
	fn pause(&self) -> Result<()>;

	/// Called when threads are requested to resume.
	///
	/// # Errors
	/// Returns an error is that thread cannot be resumed for any reason.
	fn resume(&self) -> Result<()>;

	/// Called when threads are requested to finish.
	///
	/// # Errors
	/// Returns an error is that thread cannot be ended for any reason.
	fn end(&self) -> Result<()>;
}
