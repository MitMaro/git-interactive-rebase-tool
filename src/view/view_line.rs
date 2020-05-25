use crate::view::line_segment::LineSegment;

pub(crate) struct ViewLine {
	pinned_segments: usize,
	segments: Vec<LineSegment>,
	selected: bool,
	length: usize,
}

impl ViewLine {
	pub(crate) fn new(segments: Vec<LineSegment>) -> Self {
		Self::new_with_pinned_segments(segments, 0)
	}

	pub(crate) fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		let length = segments.iter().fold(0, |len, seg| len + seg.get_length());

		Self {
			selected: false,
			segments,
			pinned_segments,
			length,
		}
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

	pub(crate) fn get_length(&self) -> usize {
		self.length
	}
}

#[cfg(test)]
mod tests {
	use crate::view::line_segment::LineSegment;
	use crate::view::view_line::ViewLine;

	#[test]
	fn view_line_new() {
		let view_line = ViewLine::new(vec![LineSegment::new("foo"), LineSegment::new("bar")]);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_selected(), false);
		assert_eq!(view_line.get_length(), 6);
	}

	#[test]
	fn view_line_new_selected() {
		let view_line = ViewLine::new(vec![LineSegment::new("foo"), LineSegment::new("bar")]).set_selected(true);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_selected(), true);
		assert_eq!(view_line.get_length(), 6);
	}

	#[test]
	fn view_line_new_with_pinned_segments() {
		let view_line = ViewLine::new_with_pinned_segments(
			vec![
				LineSegment::new("foo"),
				LineSegment::new("bar"),
				LineSegment::new("baz"),
				LineSegment::new("foobar"),
			],
			2,
		);

		assert_eq!(view_line.get_number_of_pinned_segment(), 2);
		assert_eq!(view_line.get_segments().len(), 4);
		assert_eq!(view_line.get_selected(), false);
		assert_eq!(view_line.get_length(), 15);
	}
}
