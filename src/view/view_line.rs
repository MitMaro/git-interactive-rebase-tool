use crate::view::LineSegment;

pub struct ViewLine {
	segments: Vec<LineSegment>,
	pinned_segments: usize,
}

impl ViewLine {
	pub fn new(segments: Vec<LineSegment>) -> Self {
		Self {
			segments,
			pinned_segments: 0,
		}
	}

	pub fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		Self {
			segments,
			pinned_segments,
		}
	}

	pub fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}

	pub fn get_number_of_pinned_segment(&self) -> usize {
		self.pinned_segments
	}
}
