use crate::view::LineSegment;

pub struct ViewLine {
	segments: Vec<LineSegment>,
}

impl ViewLine {
	pub fn new(segments: Vec<LineSegment>) -> Self {
		Self { segments }
	}

	pub fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}
}
