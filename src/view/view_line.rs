use crate::display::display_color::DisplayColor;
use crate::view::line_segment::LineSegment;

#[derive(Debug)]
pub struct ViewLine {
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

	pub(crate) const fn set_selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	pub(crate) fn set_padding_character(mut self, character: &str) -> Self {
		self.padding_character = String::from(character);
		self
	}

	pub(crate) const fn set_padding_color_and_style(
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

	pub(crate) const fn get_number_of_pinned_segment(&self) -> usize {
		self.pinned_segments
	}

	pub(crate) const fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}

	pub(crate) const fn get_selected(&self) -> bool {
		self.selected
	}

	pub(super) const fn get_padding_color(&self) -> DisplayColor {
		self.padding_color
	}

	pub(super) const fn is_padding_dimmed(&self) -> bool {
		self.padding_dim
	}

	pub(super) const fn is_padding_underlined(&self) -> bool {
		self.padding_underline
	}

	pub(super) const fn is_padding_reversed(&self) -> bool {
		self.padding_reverse
	}

	pub(super) fn padding_character(&self) -> &str {
		self.padding_character.as_str()
	}
}

impl<'a> From<&'a str> for ViewLine {
	fn from(line: &'a str) -> Self {
		Self::from(LineSegment::new(line))
	}
}

impl<'a> From<String> for ViewLine {
	fn from(line: String) -> Self {
		Self::from(LineSegment::new(line.as_str()))
	}
}

impl<'a> From<LineSegment> for ViewLine {
	fn from(line_segment: LineSegment) -> Self {
		Self::from(vec![line_segment])
	}
}

impl<'a> From<Vec<LineSegment>> for ViewLine {
	fn from(line_segment: Vec<LineSegment>) -> Self {
		Self::new_with_pinned_segments(line_segment, 0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_str() {
		let view_line = ViewLine::from("foo");

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 1);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert_eq!(view_line.get_selected(), false);
	}

	#[test]
	fn from_string() {
		let view_line = ViewLine::from(String::from("foo"));

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 1);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert_eq!(view_line.get_selected(), false);
	}

	#[test]
	fn from_line_segment() {
		let view_line = ViewLine::from(LineSegment::new("foo"));

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 1);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert_eq!(view_line.get_selected(), false);
	}

	#[test]
	fn from_list_line_segment() {
		let view_line = ViewLine::from(vec![LineSegment::new("foo"), LineSegment::new("bar")]);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert_eq!(view_line.get_segments().last().unwrap().get_content(), "bar");
		assert_eq!(view_line.get_selected(), false);
	}

	#[test]
	fn new_selected() {
		let view_line = ViewLine::from(vec![LineSegment::new("foo"), LineSegment::new("bar")]).set_selected(true);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_selected(), true);
	}

	#[test]
	fn new_pinned() {
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
	fn new_with_pinned_segments() {
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
	fn set_padding_color_and_style() {
		let view_line =
			ViewLine::from("foo").set_padding_color_and_style(DisplayColor::IndicatorColor, true, true, true);

		assert_eq!(view_line.get_padding_color(), DisplayColor::IndicatorColor);
		assert_eq!(view_line.is_padding_dimmed(), true);
		assert_eq!(view_line.is_padding_underlined(), true);
		assert_eq!(view_line.is_padding_reversed(), true);
	}

	#[test]
	fn set_padding_character() {
		let view_line = ViewLine::from("foo").set_padding_character("@");

		assert_eq!(view_line.padding_character(), "@");
	}
}
