#![cfg(not(tarpaulin_include))]

//! Utilities for writing tests that interact with Git.
mod build_commit;
mod build_commit_diff;
mod build_file_status;
mod build_reference;
mod create_commit;
mod with_temp_repository;

use git2::Oid;

pub use self::{
	build_commit::CommitBuilder,
	build_commit_diff::CommitDiffBuilder,
	build_file_status::FileStatusBuilder,
	build_reference::ReferenceBuilder,
	create_commit::{create_commit, CreateCommitOptions},
	with_temp_repository::{with_temp_bare_repository, with_temp_repository},
};
use crate::Repository;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

/// Get the Oid of provided head reference name.
#[inline]
#[must_use]
pub fn head_id(repo: &Repository, head_name: &str) -> Oid {
	repo.git2_repository()
		.revparse_single(format!("refs/heads/{}", head_name).as_str())
		.expect("main does not exist")
		.id()
}

/// Get the reference to the main head.
#[inline]
#[must_use]
pub fn get_main_reference(repo: &Repository) -> git2::Reference<'_> {
	repo.git2_repository()
		.find_reference("refs/heads/main")
		.expect("main does not exist")
}
