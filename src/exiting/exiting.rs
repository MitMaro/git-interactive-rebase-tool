use crate::git_interactive::GitInteractive;
use crate::process::ProcessModule;
use crate::view::View;

pub struct Exiting {}

impl ProcessModule for Exiting {
	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		view.draw_str("Exiting...")
	}
}

impl Exiting {
	pub fn new() -> Self {
		Self {}
	}
}
