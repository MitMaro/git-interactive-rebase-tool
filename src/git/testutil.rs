#![cfg(not(tarpaulin_include))]

//! Utilities for writing tests that interact with Git.
mod build_commit;
mod build_commit_diff;
mod build_file_status;
mod build_reference;
mod create_commit;
mod with_temp_repository;

use std::path::PathBuf;

use git2::Oid;

pub(crate) use self::{
	build_commit::CommitBuilder,
	build_commit_diff::CommitDiffBuilder,
	build_file_status::FileStatusBuilder,
	build_reference::ReferenceBuilder,
	create_commit::{add_path_to_index, create_commit, remove_path_from_index, CreateCommitOptions},
	with_temp_repository::{with_temp_bare_repository, with_temp_repository},
};
use crate::git::Repository;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

/// Get the the path to the repository.
#[must_use]
pub(crate) fn repo_path(repo: &Repository) -> PathBuf {
	repo.repo_path()
}

/// Get the Oid of provided head reference name.
///
/// # Panics
/// If the head id cannot be queried.
#[must_use]
pub(crate) fn head_id(repo: &Repository, head_name: &str) -> Oid {
	repo.head_id(head_name).unwrap()
}

/// Get the Commit Oid from a reference name.
///
/// # Panics
/// If the head id cannot be queried.
#[must_use]
pub(crate) fn commit_id_from_ref(repo: &Repository, reference: &str) -> Oid {
	repo.commit_id_from_ref(reference).unwrap()
}
