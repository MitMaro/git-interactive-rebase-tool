// LINT-REPLACE-START
// This section is autogenerated, do not modify directly
// enable all rustc's built-in lints
// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![cfg_attr(allow_unknown_lints, allow(renamed_and_removed_lints))]
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	disjoint_capture_migration,
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
	or_patterns_back_compat,
	pointer_structural_match,
	semicolon_in_expressions_from_macros,
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
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::blanket_clippy_restriction_lints,
	clippy::implicit_return,
	clippy::missing_docs_in_private_items,
	clippy::redundant_pub_crate,
	clippy::tabs_in_doc_comments
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
// LINT-REPLACE-END
#![allow(
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::exhaustive_structs,
	clippy::indexing_slicing,
	clippy::integer_arithmetic,
	clippy::non_ascii_literal,
	clippy::wildcard_enum_match_arm
)]

//! Git Interactive Rebase Tool - Configuration Module
//!
//! # Description
//! This module is used to handle the loading of configuration from the Git config system.
//!
//! ```
//! use config::Config;
//! let config = Config::new();
//! ```
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.
mod color;
mod diff_ignore_whitespace_setting;
mod diff_show_whitespace_setting;
mod git_config;
mod key_bindings;
#[cfg(not(tarpaulin_include))]
pub mod testutil;
mod theme;
mod utils;

#[cfg(test)]
mod tests;

use anyhow::Result;

use self::utils::{
	get_bool,
	get_diff_ignore_whitespace,
	get_diff_show_whitespace,
	get_string,
	get_unsigned_integer,
	open_git_config,
};
pub use self::{
	color::Color,
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	git_config::GitConfig,
	key_bindings::KeyBindings,
	theme::Theme,
};

/// Represents the configuration options.
#[derive(Clone, Debug)]
pub struct Config {
	/// If to select the next line in the list after performing an action.
	pub auto_select_next: bool,
	/// How to handle whitespace when calculating diffs.
	pub diff_ignore_whitespace: DiffIgnoreWhitespaceSetting,
	/// How to show whitespace in diffs.
	pub diff_show_whitespace: DiffShowWhitespaceSetting,
	/// The symbol used to replace space characters.
	pub diff_space_symbol: String,
	/// The symbol used to replace tab characters.
	pub diff_tab_symbol: String,
	/// The display width of the tab character.
	pub diff_tab_width: u32,
	/// The maximum number of undo steps.
	pub undo_limit: u32,
	/// Configuration options loaded directly from Git.
	pub git: GitConfig,
	/// Key binding configuration.
	pub key_bindings: KeyBindings,
	/// Theme configuration.
	pub theme: Theme,
}

impl Config {
	/// Creates a new Config instance loading the Git Config using [`git2::Repository::open_from_env`].
	///
	/// # Errors
	///
	/// Will return an `Err` if there is a problem loading the configuration.
	#[inline]
	pub fn new() -> Result<Self> {
		let config = open_git_config().map_err(|e| e.context("Error loading git config"))?;
		Self::new_from_config(&config).map_err(|e| e.context("Error reading git config"))
	}

	fn new_from_config(git_config: &git2::Config) -> Result<Self> {
		Ok(Self {
			auto_select_next: get_bool(git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			diff_ignore_whitespace: get_diff_ignore_whitespace(git_config)?,
			diff_show_whitespace: get_diff_show_whitespace(git_config)?,
			diff_space_symbol: get_string(git_config, "interactive-rebase-tool.diffSpaceSymbol", "·")?,
			diff_tab_symbol: get_string(git_config, "interactive-rebase-tool.diffTabSymbol", "→")?,
			diff_tab_width: get_unsigned_integer(git_config, "interactive-rebase-tool.diffTabWidth", 4)?,
			undo_limit: get_unsigned_integer(git_config, "interactive-rebase-tool.undoLimit", 5000)?,
			git: GitConfig::new(git_config)?,
			key_bindings: KeyBindings::new(git_config)?,
			theme: Theme::new(git_config)?,
		})
	}
}
