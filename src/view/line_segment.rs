use crate::display::{Display, DisplayColor};
use unicode_segmentation::UnicodeSegmentation;

pub struct LineSegment {
	color: DisplayColor,
	dim: bool,
	reverse: bool,
	text: String,
	length: usize,
	underline: bool,
}

impl LineSegment {
	pub fn new(text: &str) -> Self {
		Self {
			text: String::from(text),
			color: DisplayColor::Normal,
			reverse: false,
			dim: false,
			length: UnicodeSegmentation::graphemes(text, true).count(),
			underline: false,
		}
	}

	pub fn new_with_color(text: &str, color: DisplayColor) -> Self {
		Self {
			text: String::from(text),
			color,
			reverse: false,
			dim: false,
			length: UnicodeSegmentation::graphemes(text, true).count(),
			underline: false,
		}
	}

	pub fn new_with_color_and_style(
		text: &str,
		color: DisplayColor,
		dim: bool,
		underline: bool,
		reverse: bool,
	) -> Self
	{
		Self {
			text: String::from(text),
			color,
			reverse,
			dim,
			length: UnicodeSegmentation::graphemes(text, true).count(),
			underline,
		}
	}

	pub fn get_length(&self) -> usize {
		self.length
	}

	pub fn draw(&self, left: usize, max_width: usize, selected: bool, display: &Display) -> (usize, usize) {
		display.color(self.color, selected);
		display.set_style(self.dim, self.underline, self.reverse);
		let segment_length = UnicodeSegmentation::graphemes(self.text.as_str(), true).count();

		if segment_length <= left {
			(0, segment_length)
		}
		else if segment_length - left >= max_width {
			let graphemes = UnicodeSegmentation::graphemes(self.text.as_str(), true);
			let partial_line = graphemes.skip(left).take(max_width).collect::<String>();
			display.draw_str(partial_line.as_str());
			(max_width, segment_length)
		}
		else {
			let graphemes = UnicodeSegmentation::graphemes(self.text.as_str(), true);
			let partial_line = graphemes.skip(left).collect::<String>();
			display.draw_str(partial_line.as_str());
			(segment_length - left, segment_length)
		}
	}
}
