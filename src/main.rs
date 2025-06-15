#![cfg_attr(
	allow_unknown_lints,
	allow(
		unknown_lints,
		renamed_and_removed_lints,
		reason = "Nightly sometimes removes/renames lints."
	)
)]
#![cfg_attr(
	test,
	allow(
		clippy::allow_attributes_without_reason,
		clippy::arbitrary_source_item_ordering,
		clippy::as_conversions,
		clippy::cast_possible_truncation,
		clippy::cognitive_complexity,
		clippy::let_underscore_must_use,
		clippy::let_underscore_untyped,
		clippy::missing_const_for_fn,
		clippy::missing_errors_doc,
		clippy::multiple_inherent_impl,
		clippy::needless_pass_by_value,
		clippy::panic,
		clippy::shadow_reuse,
		clippy::shadow_unrelated,
		clippy::struct_field_names,
		clippy::undocumented_unsafe_blocks,
		clippy::unimplemented,
		clippy::unreachable,
		let_underscore_drop,
		missing_docs,
		unfulfilled_lint_expectations,
		reason = "Relaxed for tests"
	)
)]
// #![cfg_attr(
// 	include_nightly_lints,
// 	expect(.., reason = "Upcoming lints, only in nightly")
// )]

mod application;
mod arguments;
mod components;
mod config;
mod diff;
mod display;
mod editor;
mod exit;
mod git;
mod help;
mod input;
mod license;
mod module;
mod modules;
mod process;
mod runtime;
mod search;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;
mod todo_file;
mod util;
mod version;
mod view;

use std::{env, ffi::OsString, process::Termination};

use crate::{
	arguments::{Args, Mode},
	exit::Exit,
};

#[must_use]
fn run(os_args: Vec<OsString>, git_config_parameters: Vec<(String, String)>) -> Exit {
	match Args::from_os_strings(os_args) {
		Err(err) => err,
		Ok(args) => {
			match *args.mode() {
				Mode::Help => help::run(),
				Mode::Version => version::run(),
				Mode::License => license::run(),
				Mode::Editor => editor::run(args, git_config_parameters),
			}
		},
	}
}
/// returns a pair of name/value pairs passed as overrides to `git`.
///
/// input here is expected to come from `git -c`, and is in the form
/// of `-c <name>=<value>` pairs, for example `-c interactive-rebase-tool.diffTabWidth=4`.
/// separated by a single whitespace
///
/// notes:
/// 1. the key/value have both gone through git's shell quoting.
///    from `gix-quote`: "every single-quote `'` is escaped as `'\''`, 
///    every exclamation mark `!` is escaped as `'\!'`, and the entire string
///    is enclosed in single quotes."
/// 2. if input doesn't contain a '=', a `=` is appended between
///    `key` and `value`. `value`, will be an empty string.
/// 3. if input does contain a '=' but `value` is empty, `value`
///    will also be an empty string.
/// 4. an empty string will later be interpreted as `true`,
///    but we don't do this here.
pub(crate) fn parse_git_config_parameters(env_var: OsString) -> Vec<(String, String)> {
	// we expect valid UTF-8 from the shell/git, so we don't handle errors here. 
	let Some(env_var) = env_var.to_str() else {
		return Vec::new();
	};

	// naive implementation: assumes correctly-escaped strings, efficiency isn't a priority
	fn unescape(s: &str) -> String {
		let s = s.trim_matches('\'');
		let mut s = s.replace("\\'", "\'");
		s = s.replace("\\!", "!");
		s
	}

	env_var
		.split_ascii_whitespace()
		.filter_map(|pair| pair.split_once('='))
		.map(|(name, value)| (unescape(name), unescape(value)))
		.collect()
}

#[expect(clippy::print_stderr, reason = "Required to print error message.")]
#[cfg(not(tarpaulin_include))]
fn main() -> impl Termination {
	let args = env::args_os().skip(1).collect();
	let git_config_parameters = env::var_os("GIT_CONFIG_PARAMETERS")
		.map(parse_git_config_parameters)
		.unwrap_or_default();
	let exit = run(args, git_config_parameters);
	if let Some(message) = exit.get_message() {
		eprintln!("{message}");
	}
	*exit.get_status()
}
