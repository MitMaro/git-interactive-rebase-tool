use std::{env, process};

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit, clippy::print_stderr)]
#[cfg(not(tarpaulin_include))]
fn main() {
	let exit = girt::run(env::args_os().skip(1).collect());
	if let Some(message) = exit.get_message().as_ref() {
		eprintln!("{message}");
	}
	process::exit(exit.get_status().to_code());
}
