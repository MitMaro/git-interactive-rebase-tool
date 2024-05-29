use std::{env, process};

use chrono::{TimeZone, Utc};
use rustc_version::{version_meta, Channel};

fn main() {
	println!("cargo::rustc-check-cfg=cfg(allow_unknown_lints)");
	println!("cargo::rustc-check-cfg=cfg(include_nightly_lints)");
	// allow unknown lints in nightly builds
	if let Ok(meta) = version_meta() {
		if meta.channel == Channel::Nightly {
			println!("cargo:rustc-cfg=allow_unknown_lints");
			println!("cargo:rustc-cfg=include_nightly_lints");
		}
	}

	// Make the current git hash available to the build
	if let Some(rev) = git_revision_hash() {
		println!("cargo:rustc-env=GIRT_BUILD_GIT_HASH={rev}");
	}

	// Use provided SOURCE_DATE_EPOCH to make builds reproducible
	let build_date = match env::var("SOURCE_DATE_EPOCH") {
		Ok(val) => Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap(),
		Err(_) => Utc::now(),
	};
	println!("cargo:rustc-env=GIRT_BUILD_DATE={}", build_date.format("%Y-%m-%d"));
}

fn git_revision_hash() -> Option<String> {
	let result = process::Command::new("git")
		.args(["rev-parse", "--short=10", "HEAD"])
		.output();
	result.ok().and_then(|output| {
		let v = String::from(String::from_utf8_lossy(&output.stdout).trim());
		if v.is_empty() { None } else { Some(v) }
	})
}
