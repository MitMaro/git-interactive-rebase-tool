use std::cell::RefCell;

use bitflags::bitflags;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use xi_unicode::EmojiExt as _;

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

pub(crate) struct SegmentPartial {
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

	pub(crate) fn get_content(&self) -> &str {
		self.content.as_str()
	}

	pub(crate) const fn get_length(&self) -> usize {
		self.length
	}
}

bitflags! {
	/// Options for the `LineSegment` formatting
	#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
	pub(crate) struct LineSegmentOptions: u8 {
		/// None
		const NONE = 0b0000_0000;
		/// Dimmed
		const DIMMED = 0b0000_0001;
		/// Dimmed
		const REVERSED = 0b0000_0010;
		/// Dimmed
		const UNDERLINED = 0b0000_0100;
	}
}

impl LineSegmentOptions {
	pub(crate) fn conditional(condition: bool, options: Self) -> Self {
		if condition { options } else { LineSegmentOptions::NONE }
	}
}

/// Represents a segment in a larger line.
#[derive(Clone, Debug)]
pub(crate) struct LineSegment {
	color: DisplayColor,
	options: LineSegmentOptions,
	text: String,
	length: usize,
}

impl LineSegment {
	/// Create a new instance with just the content.
	#[must_use]
	pub(crate) fn new(text: &str) -> Self {
		Self::new_with_color_and_style(text, DisplayColor::Normal, LineSegmentOptions::NONE)
	}

	/// Create a new instance with just the content.
	#[must_use]
	pub(crate) fn new_copy_style(text: &str, segment: &Self) -> Self {
		Self::new_with_color_and_style(text, segment.color, segment.options)
	}

	/// Create a new instance with added color.
	#[must_use]
	pub(crate) fn new_with_color(text: &str, color: DisplayColor) -> Self {
		Self::new_with_color_and_style(text, color, LineSegmentOptions::NONE)
	}

	/// Create a new instance with added color and style.
	#[must_use]
	pub(crate) fn new_with_color_and_style(text: &str, color: DisplayColor, options: LineSegmentOptions) -> Self {
		Self {
			text: String::from(text),
			options,
			color,
			length: unicode_column_width(text),
		}
	}

	pub(crate) fn get_content(&self) -> &str {
		self.text.as_str()
	}

	pub(crate) const fn get_color(&self) -> DisplayColor {
		self.color
	}

	pub(crate) const fn is_dimmed(&self) -> bool {
		self.options.contains(LineSegmentOptions::DIMMED)
	}

	pub(crate) const fn is_underlined(&self) -> bool {
		self.options.contains(LineSegmentOptions::UNDERLINED)
	}

	pub(crate) const fn is_reversed(&self) -> bool {
		self.options.contains(LineSegmentOptions::REVERSED)
	}

	pub(crate) const fn get_length(&self) -> usize {
		self.length
	}

	pub(crate) fn get_partial_segment(&self, left: usize, max_width: usize) -> SegmentPartial {
		let segment_length = unicode_column_width(self.text.as_str());

		// segment is hidden to the left of the line/scroll
		if segment_length <= left {
			SegmentPartial::new("", 0)
		}
		else {
			let graphemes = UnicodeSegmentation::graphemes(self.text.as_str(), true);

			let skip_length = RefCell::new(0);
			let graphemes_itr = graphemes.skip_while(|v| {
				let len = grapheme_column_width(v);
				let value = *skip_length.borrow();
				if value + len > left {
					false
				}
				else {
					_ = skip_length.replace(value + len);
					true
				}
			});

			if segment_length - *skip_length.borrow() >= max_width {
				let take_length = RefCell::new(0);
				let partial_line = graphemes_itr
					.take_while(|v| {
						let len = grapheme_column_width(v);
						let value = *take_length.borrow();
						if value + len > max_width {
							false
						}
						else {
							_ = take_length.replace(value + len);
							true
						}
					})
					.collect::<String>();
				SegmentPartial::new(partial_line.as_str(), take_length.into_inner())
			}
			else {
				let partial_line = graphemes_itr.collect::<String>();
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
			LineSegmentOptions::all(),
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
			LineSegmentOptions::NONE,
		);

		assert_eq!(line_segment.get_color(), DisplayColor::IndicatorColor);
		assert!(!line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
		assert_eq!(line_segment.get_length(), 52);
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_dimmed() {
		let line_segment = LineSegment::new_with_color_and_style(
			"Test String",
			DisplayColor::IndicatorColor,
			LineSegmentOptions::DIMMED,
		);

		assert!(line_segment.is_dimmed());
		assert!(!line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_underlined() {
		let line_segment = LineSegment::new_with_color_and_style(
			"Test String",
			DisplayColor::IndicatorColor,
			LineSegmentOptions::UNDERLINED,
		);

		assert!(!line_segment.is_dimmed());
		assert!(line_segment.is_underlined());
		assert!(!line_segment.is_reversed());
	}

	#[test]
	fn line_segment_case_new_with_color_and_style_all_styles_reversed() {
		let line_segment = LineSegment::new_with_color_and_style(
			"Test String",
			DisplayColor::IndicatorColor,
			LineSegmentOptions::REVERSED,
		);

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
