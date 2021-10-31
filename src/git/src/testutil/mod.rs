#![cfg(not(tarpaulin_include))]

//! Utilities for writing tests that interact with Git.
mod build_reference;
mod with_temp_repository;

use git2::Oid;

pub use self::{
	build_reference::ReferenceBuilder,
	with_temp_repository::{with_temp_bare_repository, with_temp_repository},
};
use crate::Repository;

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
