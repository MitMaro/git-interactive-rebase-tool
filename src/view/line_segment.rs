use crate::window::{Window, WindowColor};
use unicode_segmentation::UnicodeSegmentation;

pub struct LineSegment {
	color: WindowColor,
	dim: bool,
	reverse: bool,
	text: String,
	underline: bool,
}

impl LineSegment {
	pub fn new(text: &str) -> Self {
		Self {
			text: String::from(text),
			color: WindowColor::Foreground,
			reverse: false,
			dim: false,
			underline: false,
		}
	}

	pub fn new_with_color(text: &str, color: WindowColor) -> Self {
		Self {
			text: String::from(text),
			color,
			reverse: false,
			dim: false,
			underline: false,
		}
	}

	pub fn new_with_color_and_style(text: &str, color: WindowColor, dim: bool, underline: bool, reverse: bool) -> Self {
		Self {
			text: String::from(text),
			color,
			reverse,
			dim,
			underline,
		}
	}

	pub fn draw(&self, max_width: usize, window: &Window) -> usize {
		window.color(self.color);
		window.set_style(self.dim, self.underline, self.reverse);
		let segment_length = UnicodeSegmentation::graphemes(self.text.as_str(), true).count();

		if segment_length >= max_width {
			let graphemes = UnicodeSegmentation::graphemes(self.text.as_str(), true);
			let partial_line = graphemes.take(max_width).collect::<String>();
			window.draw_str(partial_line.as_str());
			max_width
		}
		else {
			window.draw_str(self.text.as_str());
			segment_length
		}
	}
}
