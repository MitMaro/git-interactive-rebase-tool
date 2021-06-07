// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration
#![deny(
	warnings,
	nonstandard_style,
	unused,
	future_incompatible,
	rust_2018_idioms,
	unsafe_code
)]
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::as_conversions)]
#![allow(clippy::blocks_in_if_conditions)] // sometimes rustfmt makes blocks out of simple statements
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::expect_used)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::panic)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::wildcard_enum_match_arm)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreachable)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::redundant_closure_for_method_calls)] // too many false positives
#![allow(clippy::default_numeric_fallback)]

mod components;
mod confirm_abort;
mod confirm_rebase;
mod core;
mod error;
mod external_editor;
mod insert;
mod list;
mod module;
mod process;
mod show_commit;
mod window_size_error;

use std::env::args_os;

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit, clippy::print_stderr)]
fn main() {
	let exit = core::run(args_os().skip(1).collect());
	if let Some(message) = exit.get_message().as_ref() {
		eprintln!("{}", message);
	}
	std::process::exit(exit.get_status().to_code());
}
