use rustc_version::{version_meta, Channel};

fn main() {
	// allow unknown lints in nightly builds
	if let Ok(meta) = version_meta() {
		if meta.channel == Channel::Nightly {
			println!("cargo:rustc-cfg=allow_unknown_lints");
			println!("cargo:rustc-cfg=include_nightly_lints");
		}
	}
}
