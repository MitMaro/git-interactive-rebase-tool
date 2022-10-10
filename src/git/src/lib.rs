// LINT-REPLACE-START
// This section is autogenerated, do not modify directly
// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![cfg_attr(allow_unknown_lints, allow(renamed_and_removed_lints))]
// enable all rustc's built-in lints
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	rust_2021_incompatible_closure_captures,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
	rust_2021_prelude_collisions,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unstable_features,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![cfg_attr(include_nightly_lints, deny(clippy::nursery))]
#![allow(
	clippy::arithmetic,
	clippy::blanket_clippy_restriction_lints,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::indexing_slicing,
	clippy::integer_arithmetic,
	clippy::map_err_ignore,
	clippy::missing_docs_in_private_items,
	clippy::mod_module_files,
	clippy::module_name_repetitions,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::option_if_let_else,
	clippy::pub_use,
	clippy::redundant_pub_crate,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
	clippy::tabs_in_doc_comments,
	clippy::too_many_lines,
	clippy::unwrap_used
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
// allow some things in tests
#![cfg_attr(
	test,
	allow(
		clippy::cognitive_complexity,
		clippy::let_underscore_drop,
		clippy::let_underscore_must_use,
		clippy::needless_pass_by_value,
		clippy::panic,
		clippy::shadow_reuse,
		clippy::shadow_unrelated,
		clippy::undocumented_unsafe_blocks,
		clippy::unimplemented,
		clippy::unreachable
	)
)]
// allowable upcoming nightly lints
#![cfg_attr(
	include_nightly_lints,
	allow(clippy::arithmetic_side_effects, clippy::bool_to_int_with_if)
)]
// LINT-REPLACE-END

//! Git Interactive Rebase Tool - Git Module
//!
//! # Description
//! This module is used to handle working with external Git systems.
//!
//! ```
//! use git::Repository;
//! let repository = Repository::open_from_env().unwrap();
//!
//! let config = repository.load_config().unwrap();
//! let pager = config.get_str("core.pager");
//! ```
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
pub mod errors;
mod file_mode;
mod file_status;
mod file_status_builder;
mod origin;
mod reference;
mod reference_kind;
mod repository;
mod status;
pub mod testutil;
mod user;

pub use git2::{Config, ErrorCode};

pub use crate::{
	commit::Commit,
	commit_diff::CommitDiff,
	commit_diff_loader_options::CommitDiffLoaderOptions,
	delta::Delta,
	diff_line::DiffLine,
	file_mode::FileMode,
	file_status::FileStatus,
	origin::Origin,
	reference::Reference,
	reference_kind::ReferenceKind,
	repository::Repository,
	status::Status,
	user::User,
};
