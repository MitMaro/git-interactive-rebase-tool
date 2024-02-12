//! Git Interactive Rebase Tool - Git Module
//!
//! # Description
//! This module is used to handle working with external Git systems.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance, they should only be used in test code.

mod commit;
mod commit_diff;
mod commit_diff_loader;
mod commit_diff_loader_options;
mod delta;
mod diff_line;
mod errors;
mod file_mode;
mod file_status;
mod file_status_builder;
mod origin;
mod reference;
mod reference_kind;
mod repository;
mod status;
mod user;

pub(crate) use git2::{Config, ErrorCode};

pub(crate) use self::{
	commit::Commit,
	commit_diff::CommitDiff,
	commit_diff_loader::CommitDiffLoader,
	commit_diff_loader_options::CommitDiffLoaderOptions,
	delta::Delta,
	diff_line::DiffLine,
	errors::{GitError, RepositoryLoadKind},
	file_mode::FileMode,
	file_status::FileStatus,
	file_status_builder::FileStatusBuilder,
	origin::Origin,
	reference::Reference,
	reference_kind::ReferenceKind,
	repository::Repository,
	status::Status,
	user::User,
};
