use crate::display::display_color::DisplayColor;
use crate::view::line_segment::LineSegment;

pub(crate) struct ViewLine {
	pinned_segments: usize,
	segments: Vec<LineSegment>,
	selected: bool,
	padding_color: DisplayColor,
	padding_dim: bool,
	padding_reverse: bool,
	padding_underline: bool,
	padding_character: String,
}

impl ViewLine {
	pub(crate) fn new_empty_line() -> Self {
		Self::new_with_pinned_segments(vec![], 1)
	}

	pub(crate) fn new(segments: Vec<LineSegment>) -> Self {
		Self::new_with_pinned_segments(segments, 0)
	}

	pub(crate) fn new_pinned(segments: Vec<LineSegment>) -> Self {
		let segments_length = segments.len();
		Self::new_with_pinned_segments(segments, segments_length)
	}

	pub(crate) fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments,
			padding_color: DisplayColor::Normal,
			padding_dim: false,
			padding_reverse: false,
			padding_underline: false,
			padding_character: String::from(" "),
		}
	}

	pub(crate) fn set_selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	pub(crate) fn set_padding_character(mut self, character: &str) -> Self {
		self.padding_character = String::from(character);
		self
	}

	pub(crate) fn set_padding_color_and_style(
		mut self,
		color: DisplayColor,
		dim: bool,
		underline: bool,
		reverse: bool,
	) -> Self
	{
		self.padding_color = color;
		self.padding_dim = dim;
		self.padding_underline = underline;
		self.padding_reverse = reverse;
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

	pub(super) fn get_padding_color(&self) -> DisplayColor {
		self.padding_color
	}

	pub(super) fn is_padding_dimmed(&self) -> bool {
		self.padding_dim
	}

	pub(super) fn is_padding_underlined(&self) -> bool {
		self.padding_underline
	}

	pub(super) fn is_padding_reversed(&self) -> bool {
		self.padding_reverse
	}

	pub(super) fn padding_character(&self) -> &str {
		self.padding_character.as_str()
	}
}

#[cfg(test)]
mod tests {
	use crate::display::display_color::DisplayColor;
	use crate::view::line_segment::LineSegment;
	use crate::view::view_line::ViewLine;

	#[test]
	fn view_line_new() {
		let view_line = ViewLine::new(vec![LineSegment::new("foo"), LineSegment::new("bar")]);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_selected(), false);
	}

	#[test]
	fn view_line_new_selected() {
		let view_line = ViewLine::new(vec![LineSegment::new("foo"), LineSegment::new("bar")]).set_selected(true);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_selected(), true);
	}

	#[test]
	fn view_line_new_pinned() {
		let view_line = ViewLine::new_pinned(vec![
			LineSegment::new("foo"),
			LineSegment::new("bar"),
			LineSegment::new("baz"),
			LineSegment::new("foobar"),
		]);

		assert_eq!(view_line.get_number_of_pinned_segment(), 4);
		assert_eq!(view_line.get_segments().len(), 4);
		assert_eq!(view_line.get_selected(), false);
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
	}

	#[test]
	fn view_line_set_padding_color_and_style() {
		let view_line = ViewLine::new(vec![LineSegment::new("foo")]);
		let view_line = view_line.set_padding_color_and_style(DisplayColor::IndicatorColor, true, true, true);

		assert_eq!(view_line.get_padding_color(), DisplayColor::IndicatorColor);
		assert_eq!(view_line.is_padding_dimmed(), true);
		assert_eq!(view_line.is_padding_underlined(), true);
		assert_eq!(view_line.is_padding_reversed(), true);
	}
}
