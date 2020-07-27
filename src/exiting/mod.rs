use crate::git_interactive::GitInteractive;
use crate::process::process_module::ProcessModule;
use crate::view::View;

pub(crate) struct Exiting {}

impl ProcessModule for Exiting {
	fn render(&self, view: &View<'_>, _git_interactive: &GitInteractive) {
		view.draw_str("Exiting...")
	}
}

impl Exiting {
	pub(crate) fn new() -> Self {
		Self {}
	}
}
