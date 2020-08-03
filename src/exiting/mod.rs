use crate::git_interactive::GitInteractive;
use crate::process::process_module::ProcessModule;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub struct Exiting {
	view_data: ViewData,
}

impl ProcessModule for Exiting {
	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}
}

impl Exiting {
	pub(crate) fn new() -> Self {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::new(vec![LineSegment::new("Exiting...")]));
		Self { view_data }
	}
}
