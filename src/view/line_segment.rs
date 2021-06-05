use std::cell::RefCell;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use xi_unicode::EmojiExt;

use crate::display::DisplayColor;

fn unicode_column_width(s: &str) -> usize {
	s.graphemes(true).map(grapheme_column_width).sum()
}

fn grapheme_column_width(s: &str) -> usize {
	for c in s.chars() {
		if c.is_emoji_modifier_base() || c.is_emoji_modifier() {
			return 2;
		}
	}
	UnicodeWidthStr::width(s)
}

pub struct SegmentPartial {
	content: String,
	length: usize,
}

impl SegmentPartial {
	fn new(content: &str, length: usize) -> Self {
		Self {
			content: String::from(content),
			length,
		}
	}

	pub(super) fn get_content(&self) -> &str {
		self.content.as_str()
	}

	pub(super) const fn get_length(&self) -> usize {
		self.length
	}
}

#[derive(Clone, Debug)]
pub struct LineSegment {
	color: DisplayColor,
	dim: bool,
	reverse: bool,
	text: String,
	length: usize,
	underline: bool,
}

impl LineSegment {
	pub(crate) fn new(text: &str) -> Self {
		Self::new_with_color_and_style(text, DisplayColor::Normal, false, false, false)
	}

	pub(crate) fn new_with_color(text: &str, color: DisplayColor) -> Self {
		Self::new_with_color_and_style(text, color, false, false, false)
	}

	pub(crate) fn new_with_color_and_style(
		text: &str,
		color: DisplayColor,
		dim: bool,
		underline: bool,
		reverse: bool,
	) -> Self {
		Self {
			text: String::from(text),
			color,
			reverse,
			dim,
			length: unicode_column_width(text),
			underline,
		}
	}

	pub(super) fn get_content(&self) -> &str {
		self.text.as_str()
	}

	pub(super) const fn get_color(&self) -> DisplayColor {
		self.color
	}

	pub(super) const fn is_dimmed(&self) -> bool {
		self.dim
	}

	pub(super) const fn is_underlined(&self) -> bool {
		self.underline
	}

	pub(super) const fn is_reversed(&self) -> bool {
		self.reverse
	}

	pub(super) const fn get_length(&self) -> usize {
		self.length
	}

	pub(super) fn get_partial_segment(&self, left: usize, max_width: usize) -> SegmentPartial {
		let segment_length = unicode_column_width(self.text.as_str());

		// segment is hidden to the left of the line/scroll
		if segment_length <= left {
			SegmentPartial::new("", 0)
		}
		else {
			let graphemes = UnicodeSegmentation::graphemes(self.text.as_str(), true);

			let skip_length = RefCell::new(0);
			let graphemes = graphemes.skip_while(|v| {
				let len = grapheme_column_width(*v);
				let value = *skip_length.borrow();
				if value + len > left {
					false
				}
				else {
					skip_length.replace(value + len);
					true
				}
			});

			if segment_length - *skip_length.borrow() >= max_width {
				let take_length = RefCell::new(0);
				let partial_line = graphemes
					.take_while(|v| {
						let len = grapheme_column_width(v);
						let value = *take_length.borrow();
						if value + len > max_width {
							false
						}
						else {
							take_length.replace(value + len);
							true
						}
					})
					.collect::<String>();
				SegmentPartial::new(partial_line.as_str(), take_length.into_inner())
			}
			else {
				let partial_line = graphemes.collect::<String>();
				SegmentPartial::new(partial_line.as_str(), segment_length - skip_length.into_inner())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn line_segment_case_new() {
		let line_segment = LineSegment::new("D'fhuascail Íosa, Úrmhac na hÓighe Beannaithe, pór Éava agus Ádhaimh");

		assert_eq!(line_segment.get_color(), DisplayColor::Normal);
		assert!(!line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
		assert_eq!(line_segment.get_length(), 68);
	}

	#[test]
	fn line_segment_case_new_with_color() {
		let line_segment = LineSegment::new_with_color("Árvíztűrő tükörfúrógép", DisplayColor::IndicatorColor);

		assert_eq!(line_segment.get_color(), DisplayColor::IndicatorColor);
		assert!(!line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
		assert_eq!(line_segment.get_length(), 22);
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_enabled() {
		let line_segment = LineSegment::new_with_color_and_style(
			"Sævör grét áðan því úlpan var ónýt",
			DisplayColor::IndicatorColor,
			true,
			true,
			true,
		);

		assert_eq!(line_segment.get_color(), DisplayColor::IndicatorColor);
		assert!(line_segment.is_dimmed());
		assert!(line_segment.is_underlined());
		assert!(line_segment.is_reversed());
		assert_eq!(line_segment.get_length(), 34);
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_disabled() {
		let line_segment = LineSegment::new_with_color_and_style(
			"? דג סקרן שט בים מאוכזב ולפתע מצא לו חברה איך הקליטה",
			DisplayColor::IndicatorColor,
			false,
			false,
			false,
		);

		assert_eq!(line_segment.get_color(), DisplayColor::IndicatorColor);
		assert!(!line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
		assert_eq!(line_segment.get_length(), 52);
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_dimmed() {
		let line_segment =
			LineSegment::new_with_color_and_style("Test String", DisplayColor::IndicatorColor, true, false, false);

		assert!(line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_underlined() {
		let line_segment =
			LineSegment::new_with_color_and_style("Test String", DisplayColor::IndicatorColor, false, true, false);

		assert!(!line_segment.is_dimmed());
		assert!(line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_reversed() {
		let line_segment =
			LineSegment::new_with_color_and_style("Test String", DisplayColor::IndicatorColor, false, false, true);

		assert!(!line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(line_segment.is_reversed());
	}

	#[test]
	fn line_segment_case_get_content() {
		let line_segment = LineSegment::new("1234567890");

		assert_eq!(line_segment.get_content(), "1234567890");
	}

	#[test]
	fn line_segment_case_get_partial_segment_full_segment_exact_fit() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 10);

		assert_eq!(partial.get_content(), "1234567890");
		assert_eq!(partial.get_length(), 10);
	}

	#[test]
	fn line_segment_case_get_partial_segment_full_segment_width_one_over() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 11);

		assert_eq!(partial.get_content(), "1234567890");
		assert_eq!(partial.get_length(), 10);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_one_less() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 9);

		assert_eq!(partial.get_content(), "123456789");
		assert_eq!(partial.get_length(), 9);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_left_one() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(1, 9);

		assert_eq!(partial.get_content(), "234567890");
		assert_eq!(partial.get_length(), 9);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_left_middle() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(5, 5);

		assert_eq!(partial.get_content(), "67890");
		assert_eq!(partial.get_length(), 5);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_left_middle_and_extra_max_width() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(5, 6);

		assert_eq!(partial.get_content(), "67890");
		assert_eq!(partial.get_length(), 5);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_one_left_in_segment() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(9, 1);

		assert_eq!(partial.get_content(), "0");
		assert_eq!(partial.get_length(), 1);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_width_one_left_in_segment_with_extra_max_width() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(9, 2);

		assert_eq!(partial.get_content(), "0");
		assert_eq!(partial.get_length(), 1);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_one_max_width() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 1);

		assert_eq!(partial.get_content(), "1");
		assert_eq!(partial.get_length(), 1);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_two_max_width() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 2);

		assert_eq!(partial.get_content(), "12");
		assert_eq!(partial.get_length(), 2);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_no_max_width() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(0, 0);

		assert_eq!(partial.get_content(), "");
		assert_eq!(partial.get_length(), 0);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_left_at_segment_length() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(10, 1);

		assert_eq!(partial.get_content(), "");
		assert_eq!(partial.get_length(), 0);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_left_after_segment_length() {
		let line_segment = LineSegment::new("1234567890");

		let partial = line_segment.get_partial_segment(11, 1);

		assert_eq!(partial.get_content(), "");
		assert_eq!(partial.get_length(), 0);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_left_with_emoji() {
		let line_segment = LineSegment::new("1😊2😊3😊4567890");

		let partial = line_segment.get_partial_segment(0, 10);

		assert_eq!(partial.get_content(), "1😊2😊3😊4");
		assert_eq!(partial.get_length(), 10);
	}

	#[test]
	fn line_segment_case_get_partial_segment_partial_segment_left_with_emoji_split_at_length() {
		let line_segment = LineSegment::new("123456789😊");

		let partial = line_segment.get_partial_segment(0, 10);

		assert_eq!(partial.get_content(), "123456789");
		assert_eq!(partial.get_length(), 9);
	}
}
