use crate::view::line_segment::LineSegment;

pub(crate) struct ViewLine {
	pinned_segments: usize,
	segments: Vec<LineSegment>,
	selected: bool,
}

impl ViewLine {
	pub(crate) fn new(segments: Vec<LineSegment>) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments: 0,
		}
	}

	pub(crate) fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments,
		}
	}

	pub(crate) fn get_length(&self) -> usize {
		let mut length = 0;
		for s in self.segments.iter() {
			length += s.get_length();
		}
		length
	}

	pub(crate) fn set_selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	pub(crate) fn get_number_of_pinned_segment(&self) -> usize {
		self.pinned_segments
	}

	pub(crate) fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}

	pub(crate) fn get_selected(&self) -> bool {
		self.selected
	}
}
