use display::DisplayColor;

use super::LineSegment;

/// Represents a line in the view.
#[derive(Debug)]
pub struct ViewLine {
	pinned_segments: usize,
	segments: Vec<LineSegment>,
	selected: bool,
	padding: Option<LineSegment>,
}

impl ViewLine {
	/// Create a new instance that contains no content.
	#[must_use]
	#[inline]
	pub fn new_empty_line() -> Self {
		Self::new_with_pinned_segments(vec![], 1)
	}

	/// Create a new instance with all segments pinned.
	#[must_use]
	#[inline]
	pub fn new_pinned(segments: Vec<LineSegment>) -> Self {
		let segments_length = segments.len();
		Self::new_with_pinned_segments(segments, segments_length)
	}

	/// Create a new instance with a number of pinned leading segments.
	#[must_use]
	#[inline]
	pub fn new_with_pinned_segments(segments: Vec<LineSegment>, pinned_segments: usize) -> Self {
		Self {
			selected: false,
			segments,
			pinned_segments,
			padding: None,
		}
	}

	/// Set that this line is selected.
	#[must_use]
	#[inline]
	pub const fn set_selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	/// Set a padding character.
	#[must_use]
	#[inline]
	pub fn set_padding(mut self, c: char) -> Self {
		self.padding = Some(LineSegment::new(String::from(c).as_str()));
		self
	}

	/// Set the padding character with a related color and style.
	#[must_use]
	#[inline]
	pub fn set_padding_with_color_and_style(
		mut self,
		c: char,
		color: DisplayColor,
		dim: bool,
		underline: bool,
		reverse: bool,
	) -> Self {
		self.padding = Some(LineSegment::new_with_color_and_style(
			String::from(c).as_str(),
			color,
			dim,
			underline,
			reverse,
		));
		self
	}

	/// Get the number of pinned line segments.
	#[must_use]
	#[inline]
	pub const fn get_number_of_pinned_segment(&self) -> usize {
		self.pinned_segments
	}

	/// Get the view line segments.
	#[must_use]
	#[inline]
	pub const fn get_segments(&self) -> &Vec<LineSegment> {
		&self.segments
	}

	/// Is the line selected.
	#[must_use]
	#[inline]
	pub const fn get_selected(&self) -> bool {
		self.selected
	}

	pub(crate) const fn get_padding(&self) -> &Option<LineSegment> {
		&self.padding
	}
}

impl From<&str> for ViewLine {
	#[inline]
	fn from(line: &str) -> Self {
		Self::from(LineSegment::new(line))
	}
}

impl From<String> for ViewLine {
	#[inline]
	fn from(line: String) -> Self {
		Self::from(LineSegment::new(line.as_str()))
	}
}

impl From<LineSegment> for ViewLine {
	#[inline]
	fn from(line_segment: LineSegment) -> Self {
		Self::from(vec![line_segment])
	}
}

impl From<Vec<LineSegment>> for ViewLine {
	#[inline]
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
		assert!(!view_line.get_selected());
	}

	#[test]
	fn from_string() {
		let view_line = ViewLine::from(String::from("foo"));

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 1);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert!(!view_line.get_selected());
	}

	#[test]
	fn from_line_segment() {
		let view_line = ViewLine::from(LineSegment::new("foo"));

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 1);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert!(!view_line.get_selected());
	}

	#[test]
	fn from_list_line_segment() {
		let view_line = ViewLine::from(vec![LineSegment::new("foo"), LineSegment::new("bar")]);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert_eq!(view_line.get_segments().first().unwrap().get_content(), "foo");
		assert_eq!(view_line.get_segments().last().unwrap().get_content(), "bar");
		assert!(!view_line.get_selected());
	}

	#[test]
	fn new_selected() {
		let view_line = ViewLine::from(vec![LineSegment::new("foo"), LineSegment::new("bar")]).set_selected(true);

		assert_eq!(view_line.get_number_of_pinned_segment(), 0);
		assert_eq!(view_line.get_segments().len(), 2);
		assert!(view_line.get_selected());
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
		assert!(!view_line.get_selected());
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
		assert!(!view_line.get_selected());
	}

	#[test]
	fn set_padding_with_color_and_style() {
		let view_line =
			ViewLine::from("foo").set_padding_with_color_and_style(' ', DisplayColor::IndicatorColor, true, true, true);

		let padding = view_line.get_padding().as_ref().unwrap();
		assert_eq!(padding.get_content(), " ");
		assert_eq!(padding.get_color(), DisplayColor::IndicatorColor);
		assert!(padding.is_dimmed());
		assert!(padding.is_underlined());
		assert!(padding.is_reversed());
	}

	#[test]
	fn set_padding() {
		let view_line = ViewLine::from("foo").set_padding('@');

		assert_eq!(view_line.get_padding().as_ref().unwrap().get_content(), "@");
	}
}
