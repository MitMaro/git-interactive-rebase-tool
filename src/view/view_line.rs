use crate::view::LineSegment;

pub struct ViewLine {
	pinned_segments: usize,
	segments: Vec<LineSegment>,
	selected: bool,
}

impl ViewLine {
	pub fn new(segments: Vec<LineSegment>) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments: 0,
		}
	}

	pub fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments,
		}
	}

	pub fn set_selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	pub fn get_number_of_pinned_segment(&self) -> usize {
		self.pinned_segments
	}

	pub fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}

	pub fn get_selected(&self) -> bool {
		self.selected
	}
}
