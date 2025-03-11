// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints, renamed_and_removed_lints))]
// allow some things in tests
#![cfg_attr(
	test,
	allow(
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
		clippy::unused_self,
		let_underscore_drop,
		missing_docs
	)
)]
// allowable upcoming nightly lints
#![cfg_attr(include_nightly_lints, allow(clippy::arc_with_non_send_sync))]

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

use std::{env, ffi::OsString};

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

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit, clippy::print_stderr)]
#[cfg(not(tarpaulin_include))]
fn main() {
	let exit = run(env::args_os().skip(1).collect());
	if let Some(message) = exit.get_message() {
		eprintln!("{message}");
	}
	std::process::exit(exit.get_status().to_code());
}
