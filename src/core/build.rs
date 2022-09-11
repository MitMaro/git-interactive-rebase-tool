use std::process;

use chrono::Utc;
use rustc_version::{version_meta, Channel};

fn main() {
	// allow unknown lints in nightly builds
	if let Ok(meta) = version_meta() {
		if meta.channel == Channel::Nightly {
			println!("cargo:rustc-cfg=allow_unknown_lints");
			println!("cargo:rustc-cfg=include_nightly_lints");
		}
	}

	// Make the current git hash available to the build
	if let Some(rev) = git_revision_hash() {
		println!("cargo:rustc-env=GIRT_BUILD_GIT_HASH={}", rev);
	}
	println!("cargo:rustc-env=GIRT_BUILD_DATE={}", Utc::now().format("%Y-%m-%d"));
}

fn git_revision_hash() -> Option<String> {
	let result = process::Command::new("git")
		.args(["rev-parse", "--short=10", "HEAD"])
		.output();
	result.ok().and_then(|output| {
		let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
		if v.is_empty() { None } else { Some(v) }
	})
}
