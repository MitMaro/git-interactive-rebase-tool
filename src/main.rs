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
fn run(os_args: Vec<OsString>) -> Exit {
	match Args::try_from(os_args) {
		Err(err) => err,
		Ok(args) => {
			match *args.mode() {
				Mode::Help => help::run(),
				Mode::Version => version::run(),
				Mode::License => license::run(),
				Mode::Editor => editor::run(&args),
			}
		},
	}
}

#[expect(clippy::print_stderr, reason = "Required to print error message.")]
#[cfg(not(tarpaulin_include))]
fn main() -> impl Termination {
	let exit = run(env::args_os().skip(1).collect());
	if let Some(message) = exit.get_message() {
		eprintln!("{message}");
	}
	*exit.get_status()
}
