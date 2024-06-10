use std::env::{remove_var, set_var, var};

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
	static ref ENV_CHANGE_LOCK: Mutex<()> = Mutex::new(());
}

#[derive(Debug, Clone)]
pub(crate) enum EnvVarAction<'var> {
	Remove(&'var str),
	Set(&'var str, String),
}

// This wraps the unsafe modification on environment variables in a lock, that ensures that only one test thread is
// trying to modify environment variables at a time. This does not provide any guarantee that a value was not changed
// outside the scope of tests using this function. So in that regard, this could be considered unsafe, however within
// the confines of the tests for this project, this is safe enough.
//
// The wrapper will attempt to restore all values back to their previous value, cleaning up any changes made.
#[allow(unsafe_code, unused_unsafe)] // unused unsafe until Rust 2024
pub(crate) fn with_env_var<C>(actions: &[EnvVarAction<'_>], callback: C)
where C: FnOnce() {
	let lock = ENV_CHANGE_LOCK.lock();
	let mut undo_actions = vec![];

	for action in actions {
		let name = match action {
			EnvVarAction::Set(name, _) | EnvVarAction::Remove(name) => *name,
		};
		if let Ok(v) = var(name) {
			undo_actions.push(EnvVarAction::Set(name, v));
		}
		else {
			undo_actions.push(EnvVarAction::Remove(name));
		}
		match action {
			EnvVarAction::Remove(name) => unsafe {
				remove_var(*name);
			},
			EnvVarAction::Set(name, value) => unsafe {
				set_var(*name, value.as_str());
			},
		}
	}
	callback();
	for action in undo_actions {
		match action {
			EnvVarAction::Remove(name) => unsafe {
				remove_var(name);
			},
			EnvVarAction::Set(name, value) => unsafe {
				set_var(name, value);
			},
		}
	}
	drop(lock);
}
