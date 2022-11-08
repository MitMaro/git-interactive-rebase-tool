//! Utilities for writing tests that interact with todo file
use std::{
	cell::RefCell,
	fmt::{Debug, Formatter},
	path::Path,
};

use tempfile::{Builder, NamedTempFile};

use crate::{Line, TodoFile};

/// Context for `with_todo_file`
pub struct TodoFileTestContext {
	todo_file: TodoFile,
	git_todo_file: RefCell<NamedTempFile>,
}

impl Debug for TodoFileTestContext {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TodoFileTestContext")
			.field("todo_file", &self.todo_file)
			.field("filepath", &self.todo_file.filepath)
			.finish()
	}
}

impl TodoFileTestContext {
	/// Return the path of the todo file
	#[inline]
	pub fn path(&self) -> String {
		String::from(self.git_todo_file.borrow().path().to_str().unwrap_or_default())
	}

	/// Get the todo file instance
	#[inline]
	pub const fn todo_file(&self) -> &TodoFile {
		&self.todo_file
	}

	/// Get the todo file instance as mutable
	#[inline]
	pub fn todo_file_mut(&mut self) -> &mut TodoFile {
		&mut self.todo_file
	}

	/// Get the todo file instance
	#[inline]
	pub fn to_owned(self) -> (NamedTempFile, TodoFile) {
		(self.git_todo_file.into_inner(), self.todo_file)
	}

	/// Delete the path behind the todo file
	///
	/// # Panics
	/// Will panic if the file cannot be deleted for any reason
	#[inline]
	pub fn delete_file(&self) {
		self.git_todo_file
			.replace(Builder::new().tempfile().unwrap())
			.close()
			.unwrap();
	}

	/// Set the path behind ot todo file as readonly
	///
	/// # Panics
	/// Will panic if the file permissions cannot be changed for any reason
	#[inline]
	pub fn set_file_readonly(&self) {
		let git_todo_file = self.git_todo_file.borrow_mut();
		let todo_file = git_todo_file.as_file();
		let mut permissions = todo_file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		todo_file.set_permissions(permissions).unwrap();
	}
}

/// Provide a `TodoFileTestContext` instance containing a `Todo` for use in tests.
///
/// # Panics
/// Will panic if a temporary file cannot be created
#[inline]
pub fn with_todo_file<C>(lines: &[&str], callback: C)
where C: FnOnce(TodoFileTestContext) {
	let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("..")
		.join("..")
		.join("test")
		.join("fixtures")
		.join("simple");
	let git_todo_file = Builder::new()
		.prefix("git-rebase-todo-scratch")
		.suffix("")
		.tempfile_in(git_repo_dir.as_path())
		.unwrap();

	let mut todo_file = TodoFile::new(git_todo_file.path().to_str().unwrap(), 1, "#");
	todo_file.set_lines(lines.iter().map(|l| Line::new(l).unwrap()).collect());
	callback(TodoFileTestContext {
		git_todo_file: RefCell::new(git_todo_file),
		todo_file,
	});
}
