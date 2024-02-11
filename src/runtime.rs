//! Git Interactive Rebase Tool - Runtime
//!
//! # Description
//! This module is used to handle the application lifecycles and management of threads.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod errors;
mod installer;
mod notifier;
#[allow(clippy::module_inception)]
mod runtime;
mod status;
#[cfg(test)]
pub(crate) mod testutils;
mod thread_statuses;
mod threadable;

pub(crate) use self::{
	errors::RuntimeError,
	installer::Installer,
	notifier::Notifier,
	runtime::Runtime,
	status::Status,
	thread_statuses::ThreadStatuses,
	threadable::Threadable,
};
