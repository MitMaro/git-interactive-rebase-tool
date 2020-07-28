use crate::display::display_color::DisplayColor;
use crate::view::line_segment::LineSegment;
use crate::view::scroll_position::ScrollPosition;
use crate::view::view_line::ViewLine;

pub struct ViewData {
	scroll_position: ScrollPosition,
	height: usize,
	width: usize,
	empty_lines: Vec<ViewLine>,
	leading_lines: Vec<ViewLine>,
	leading_lines_cache: Option<Vec<ViewLine>>,
	lines: Vec<ViewLine>,
	lines_cache: Option<Vec<ViewLine>>,
	trailing_lines: Vec<ViewLine>,
	trailing_lines_cache: Option<Vec<ViewLine>>,
	show_title: bool,
	show_help: bool,
	max_leading_line_length: usize,
	max_line_length: usize,
	max_trailing_line_length: usize,
}

impl ViewData {
	pub(crate) const fn new() -> Self {
		Self {
			scroll_position: ScrollPosition::new(),
			height: 0,
			width: 0,
			empty_lines: vec![],
			leading_lines: vec![],
			leading_lines_cache: None,
			lines: vec![],
			lines_cache: None,
			trailing_lines: vec![],
			trailing_lines_cache: None,
			show_title: false,
			show_help: false,
			max_leading_line_length: 0,
			max_line_length: 0,
			max_trailing_line_length: 0,
		}
	}

	pub(crate) fn new_error(error: &str) -> Self {
		let mut inst = Self::new();
		inst.set_show_title(true);
		inst.push_line(ViewLine::new(vec![LineSegment::new(error)]));
		inst.push_trailing_line(ViewLine::new(vec![LineSegment::new_with_color(
			"Press any key to continue",
			DisplayColor::IndicatorColor,
		)]));
		inst.rebuild();
		inst
	}

	pub(crate) fn reset(&mut self) {
		self.clear();
		self.scroll_position.reset();
	}

	pub(crate) fn clear(&mut self) {
		self.leading_lines.clear();
		self.leading_lines_cache = None;
		self.lines.clear();
		self.lines_cache = None;
		self.trailing_lines.clear();
		self.trailing_lines_cache = None;
	}

	pub(crate) fn scroll_up(&mut self) {
		self.lines_cache = None;
		self.scroll_position.scroll_up();
		self.rebuild();
	}

	pub(crate) fn scroll_down(&mut self) {
		self.lines_cache = None;
		self.scroll_position.scroll_down();
		self.rebuild();
	}

	pub(crate) fn page_up(&mut self) {
		self.lines_cache = None;
		self.scroll_position.page_up();
		self.rebuild();
	}

	pub(crate) fn page_down(&mut self) {
		self.lines_cache = None;
		self.scroll_position.page_down();
		self.rebuild();
	}

	pub(crate) fn scroll_left(&mut self) {
		self.leading_lines_cache = None;
		self.lines_cache = None;
		self.trailing_lines_cache = None;
		self.scroll_position.scroll_left();
		self.rebuild();
	}

	pub(crate) fn scroll_right(&mut self) {
		self.leading_lines_cache = None;
		self.lines_cache = None;
		self.trailing_lines_cache = None;
		self.scroll_position.scroll_right();
		self.rebuild();
	}

	pub(crate) fn ensure_line_visible(&mut self, new_cursor_position: usize) {
		let previous_top = self.scroll_position.get_top_position();
		self.scroll_position.ensure_line_visible(new_cursor_position);

		if previous_top != self.scroll_position.get_top_position() {
			self.leading_lines_cache = None;
			self.lines_cache = None;
			self.trailing_lines_cache = None;
			self.rebuild();
		}
	}

	pub(crate) fn set_view_size(&mut self, view_width: usize, view_height: usize) {
		if self.height != view_height || self.width != view_width {
			self.height = view_height;
			self.width = view_width;
			self.leading_lines_cache = None;
			self.lines_cache = None;
			self.trailing_lines_cache = None;

			self.scroll_position.view_resize(
				if self.height == 0 || self.leading_lines.len() + self.trailing_lines.len() >= self.height {
					0
				}
				else {
					self.height
						- self.leading_lines.len()
						- self.trailing_lines.len()
						- if self.show_title { 1 } else { 0 }
				},
				if self.width == 0 {
					0
				}
				else {
					self.width - if self.should_show_scroll_bar() { 1 } else { 0 }
				},
			);

			self.rebuild();
		}
	}

	pub(crate) fn set_show_title(&mut self, show: bool) {
		self.show_title = show;
	}

	pub(crate) fn set_show_help(&mut self, show: bool) {
		self.show_help = show;
	}

	pub(crate) fn push_leading_line(&mut self, view_line: ViewLine) {
		self.leading_lines_cache = None;
		self.lines_cache = None;
		self.trailing_lines_cache = None;
		self.leading_lines.push(view_line);
	}

	pub(crate) fn push_line(&mut self, view_line: ViewLine) {
		self.lines_cache = None;
		self.lines.push(view_line);
	}

	pub(crate) fn push_trailing_line(&mut self, view_line: ViewLine) {
		self.lines_cache = None;
		self.trailing_lines_cache = None;
		self.trailing_lines.push(view_line);
	}

	pub(crate) fn should_show_scroll_bar(&self) -> bool {
		if self.lines.is_empty() {
			return false;
		}

		// all other lines take precedence over regular lines
		let padding_height = if self.show_title { 1 } else { 0 } + self.leading_lines.len() + self.trailing_lines.len();
		if padding_height < self.height {
			self.lines.len() > (self.height - padding_height)
		}
		else {
			false
		}
	}

	pub(crate) fn get_scroll_index(&self) -> usize {
		if self.lines.is_empty() || self.scroll_position.get_top_position() == 0 {
			return 0;
		}

		let padding_height = if self.show_title { 1 } else { 0 } + self.leading_lines.len() + self.trailing_lines.len();
		let view_height = if padding_height < self.height {
			self.height - padding_height
		}
		else {
			0
		};

		if view_height <= 1 || view_height > self.lines.len() {
			return 0;
		}

		// view_height >= 2
		// lines.len() > 2

		// if at bottom of list
		if self.scroll_position.get_top_position() >= self.lines.len() - view_height {
			return view_height - 1;
		}

		if view_height <= 2 {
			return 0;
		}

		// 0 input range, if first and last item are pinned, so scroll center
		if self.lines.len() - view_height <= 2 {
			return (0.5 * view_height as f64).round() as usize;
		}

		// linear range map from scroll range to view range. This only maps the range between the
		// first and last items, since those items are always returned as 0 or view_height
		let value = self.scroll_position.get_top_position() as f64;
		let input_start = 1.0;
		let input_end = (self.lines.len() - view_height) as f64 - 1.0;
		let output_start = 1.0;
		let output_end = view_height as f64 - 2.0;
		let input_range = input_end - input_start;
		let output_range = output_end - output_start;
		let slope = output_range / input_range;
		slope.mul_add(value - input_start, output_start).round() as usize
	}

	pub(super) const fn show_title(&self) -> bool {
		self.show_title
	}

	pub(super) const fn show_help(&self) -> bool {
		self.show_help
	}

	pub(crate) fn is_empty(&self) -> bool {
		self.lines.is_empty() && self.leading_lines.is_empty() && self.trailing_lines.is_empty()
	}

	pub(super) fn get_leading_lines(&self) -> &Vec<ViewLine> {
		match &self.leading_lines_cache {
			Some(lines) => lines,
			None => &self.empty_lines,
		}
	}

	pub(super) fn get_lines(&self) -> &Vec<ViewLine> {
		match &self.lines_cache {
			Some(lines) => lines,
			None => &self.empty_lines,
		}
	}

	pub(super) fn get_trailing_lines(&self) -> &Vec<ViewLine> {
		match &self.trailing_lines_cache {
			Some(lines) => lines,
			None => &self.empty_lines,
		}
	}

	pub(crate) fn rebuild(&mut self) {
		if self.leading_lines_cache.is_none() {
			self.leading_lines_cache = Some(match self.leading_lines.len() {
				0 => {
					self.max_leading_line_length =
						Self::calculate_max_line_length(&self.leading_lines, 0, self.leading_lines.len());
					self.build_lines(&self.leading_lines, 0, self.leading_lines.len(), false)
				},
				_ => {
					// trailing lines have precedence over leading lines, title always has precedence
					let padding_height = if self.show_title { 1 } else { 0 } + self.trailing_lines.len();
					let available_height = if padding_height < self.height {
						self.height - padding_height
					}
					else {
						0
					};
					let end = if self.leading_lines.len() < available_height {
						self.leading_lines.len()
					}
					else {
						available_height
					};

					self.max_leading_line_length = Self::calculate_max_line_length(&self.leading_lines, 0, end);
					self.build_lines(&self.leading_lines, 0, end, false)
				},
			});
		}

		if self.trailing_lines_cache.is_none() {
			self.trailing_lines_cache = Some(match self.trailing_lines.len() {
				0 => {
					self.max_trailing_line_length =
						Self::calculate_max_line_length(&self.trailing_lines, 0, self.trailing_lines.len());
					self.build_lines(&self.trailing_lines, 0, self.trailing_lines.len(), false)
				},
				_ => {
					// title always has precedence
					let padding_height = if self.show_title { 1 } else { 0 };
					let available_height = if padding_height < self.height {
						self.height - padding_height
					}
					else {
						0
					};

					let end = if self.trailing_lines.len() < available_height {
						self.trailing_lines.len()
					}
					else {
						available_height
					};

					self.max_trailing_line_length = Self::calculate_max_line_length(&self.trailing_lines, 0, end);
					self.build_lines(&self.trailing_lines, 0, end, false)
				},
			});
		}

		if self.lines_cache.is_none() {
			self.lines_cache = Some(match self.lines.len() {
				0 => {
					self.max_line_length = Self::calculate_max_line_length(&self.lines, 0, self.lines.len());
					self.build_lines(&self.lines, 0, self.lines.len(), self.should_show_scroll_bar())
				},
				_ => {
					// all other lines take precedence over regular lines
					let padding_height =
						if self.show_title { 1 } else { 0 } + self.leading_lines.len() + self.trailing_lines.len();
					let available_height = if padding_height < self.height {
						self.height - padding_height
					}
					else {
						0
					};

					let start = if self.lines.len() <= available_height {
						0
					}
					else if self.scroll_position.get_top_position() + available_height > self.lines.len() {
						self.lines.len() - available_height
					}
					else {
						self.scroll_position.get_top_position()
					};

					let end = if self.lines.len() < available_height {
						self.lines.len()
					}
					else {
						available_height
					};

					self.max_line_length = Self::calculate_max_line_length(&self.lines, start, end);

					self.scroll_position
						.set_line_maximums(self.get_max_line_length(), self.lines.len());

					self.build_lines(&self.lines, start, end, self.should_show_scroll_bar())
				},
			});
		}
		else {
			self.scroll_position
				.set_line_maximums(self.get_max_line_length(), self.lines.len());
		}
	}

	fn calculate_max_line_length(view_lines: &[ViewLine], start: usize, length: usize) -> usize {
		view_lines
			.iter()
			.skip(start)
			.take(length)
			.fold(0, |longest, line| -> usize {
				if line.get_segments().len() <= line.get_number_of_pinned_segment() {
					longest
				}
				else {
					let sum = line.get_segments().iter().fold(0, |s, l| s + l.get_length());

					if sum > longest {
						sum
					}
					else {
						longest
					}
				}
			})
	}

	fn get_max_line_length(&self) -> usize {
		self.max_line_length
			.max(self.max_leading_line_length)
			.max(self.max_trailing_line_length)
	}

	fn build_lines(&self, view_lines: &[ViewLine], start: usize, end: usize, scroll_bar: bool) -> Vec<ViewLine> {
		let window_width = if scroll_bar { self.width - 1 } else { self.width };
		let left = self.scroll_position.get_left_position();

		view_lines
			.iter()
			.skip(start)
			.take(end)
			.map(|line| -> ViewLine {
				let mut start = 0;
				let mut left_start = 0;
				let mut segments = vec![];
				for (i, segment) in line.get_segments().iter().enumerate() {
					// set left on first non-pinned segment
					if i == line.get_number_of_pinned_segment() {
						left_start = left;
					}

					let partial = segment.get_partial_segment(left_start, window_width - start);

					if partial.get_length() > 0 {
						segments.push(LineSegment::new_with_color_and_style(
							partial.get_content(),
							segment.get_color(),
							segment.is_dimmed(),
							segment.is_underlined(),
							segment.is_reversed(),
						));

						start += partial.get_length();
						if start >= window_width {
							break;
						}
						left_start = 0;
					}
					else {
						left_start -= segment.get_length();
					}
				}

				if start < window_width {
					let padding = line.padding_character().repeat(window_width - start);

					segments.push(LineSegment::new_with_color_and_style(
						padding.as_str(),
						line.get_padding_color(),
						line.is_padding_dimmed(),
						line.is_padding_underlined(),
						line.is_padding_reversed(),
					));
				}

				ViewLine::new_with_pinned_segments(segments, line.get_number_of_pinned_segment())
					.set_selected(line.get_selected())
			})
			.collect::<Vec<ViewLine>>()
	}
}

#[cfg(test)]
mod tests {
	use crate::view::line_segment::LineSegment;
	use crate::view::view_data::ViewData;
	use crate::view::view_line::ViewLine;

	fn create_mock_view_line() -> ViewLine {
		ViewLine::new(vec![LineSegment::new("Mocked Line")])
	}

	fn create_mocked_view_data() -> ViewData {
		let mut view_data = ViewData::new();
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_leading_line(create_mock_view_line());

		view_data.push_line(create_mock_view_line());
		view_data.push_line(create_mock_view_line());
		view_data.push_line(create_mock_view_line());
		view_data.push_line(create_mock_view_line());

		view_data.push_trailing_line(create_mock_view_line());
		view_data.push_trailing_line(create_mock_view_line());
		view_data.rebuild();

		view_data
	}

	fn create_mocked_scroll_vertical_view_data() -> ViewData {
		let mut view_data = ViewData::new();
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_leading_line(create_mock_view_line());

		view_data.push_line(ViewLine::new(vec![LineSegment::new("a")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("b")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("c")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("d")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("1")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("2")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("3")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("4")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("5")]));

		view_data.push_trailing_line(create_mock_view_line());
		view_data.push_trailing_line(create_mock_view_line());
		view_data.set_view_size(100, 10);

		view_data
	}

	fn create_mocked_scroll_horizontal_view_data() -> ViewData {
		let mut view_data = ViewData::new();
		view_data.push_line(ViewLine::new(vec![LineSegment::new("aaaaa")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("aaaaaaaaaa")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("aaaaaaaaaaaaaaa")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("aaaaaaaaaa")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("aaaaa")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("bbbbb")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("ccccc")]));
		view_data.push_line(ViewLine::new(vec![LineSegment::new("ddddd")]));
		view_data.set_view_size(7, 20);

		view_data
	}

	fn create_mocked_scroll_index_data(number_of_items: usize, height: usize, scroll_position: usize) -> ViewData {
		let mut view_data = ViewData::new();

		for _ in 0..number_of_items {
			view_data.push_line(create_mock_view_line());
		}

		view_data.set_view_size(10, height);

		for _ in 0..scroll_position {
			view_data.scroll_down();
		}

		view_data
	}

	fn get_segment_content_for_view_line(view_lines: &[ViewLine], line_index: usize, segment_index: usize) -> String {
		String::from(
			view_lines
				.get(line_index)
				.unwrap()
				.get_segments()
				.get(segment_index)
				.unwrap()
				.get_content(),
		)
	}

	#[test]
	fn view_data_case_with_show_help() {
		let mut view_data = ViewData::new();
		view_data.set_show_help(true);
		assert_eq!(view_data.show_help(), true);
	}

	#[test]
	fn view_data_case_with_show_title() {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		assert_eq!(view_data.show_title(), true);
	}

	#[test]
	fn view_data_case_new_error() {
		let mut view_data = ViewData::new_error("My Error");
		view_data.set_view_size(100, 100);
		assert_eq!(view_data.show_title(), true);
		assert_eq!(view_data.show_help(), false);
		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 1);
		assert_eq!(view_data.get_trailing_lines().len(), 1);
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_lines(), 0, 0),
			"My Error"
		);
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_trailing_lines(), 0, 0),
			"Press any key to continue"
		);
	}

	#[test]
	fn view_data_case_clear() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 3);
		view_data.scroll_position.scroll_down();
		view_data.clear();

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
		assert_eq!(view_data.scroll_position.get_top_position(), 1);
	}

	#[test]
	fn view_data_case_reset() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 3);
		view_data.scroll_position.scroll_down();
		view_data.reset();

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
		assert_eq!(view_data.scroll_position.get_top_position(), 0);
	}

	#[test]
	fn view_data_case_with_no_lines() {
		// default case with more than enough view height for all lines with title
		let mut view_data = ViewData::new();
		view_data.set_view_size(100, 10);
		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
	}

	#[test]
	fn view_data_case_with_no_leading_lines() {
		let mut view_data = ViewData::new();
		view_data.set_view_size(100, 10);
		view_data.push_line(create_mock_view_line());
		view_data.push_trailing_line(create_mock_view_line());
		view_data.rebuild();

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 1);
		assert_eq!(view_data.get_trailing_lines().len(), 1);
	}

	#[test]
	fn view_data_case_with_no_general_lines() {
		let mut view_data = ViewData::new();
		view_data.set_view_size(100, 10);
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_trailing_line(create_mock_view_line());
		view_data.rebuild();

		assert_eq!(view_data.get_leading_lines().len(), 1);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 1);
	}

	#[test]
	fn view_data_case_with_no_trailing_lines() {
		let mut view_data = ViewData::new();
		view_data.set_view_size(100, 10);
		view_data.push_leading_line(create_mock_view_line());
		view_data.push_line(create_mock_view_line());
		view_data.rebuild();

		assert_eq!(view_data.get_leading_lines().len(), 1);
		assert_eq!(view_data.get_lines().len(), 1);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
	}

	#[test]
	fn view_data_case_with_more_than_enough_view_height_for_all_lines_with_title() {
		let mut view_data = create_mocked_view_data();
		view_data.set_show_title(true);
		view_data.set_view_size(100, 12);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 4);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_more_than_enough_view_height_for_all_lines_without_title() {
		let mut view_data = create_mocked_view_data();
		view_data.set_show_title(false);
		view_data.set_view_size(100, 12);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 4);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_just_enough_height_for_all_lines_with_title() {
		let mut view_data = create_mocked_view_data();
		view_data.set_show_title(true);
		view_data.set_view_size(100, 10);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 4);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_just_enough_height_for_all_lines_without_title() {
		let mut view_data = create_mocked_view_data();
		view_data.set_show_title(false);
		view_data.set_view_size(100, 9);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 4);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_removal_of_single_general_line() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 8);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_removal_of_all_but_one_general_line() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 6);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 1);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_removal_of_all_general_lines() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 5);

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_no_general_lines_remove_one_leading_line() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 4);

		assert_eq!(view_data.get_leading_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_no_general_lines_remove_all_but_one_leading_line() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 3);

		assert_eq!(view_data.get_leading_lines().len(), 1);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_no_general_lines_no_leading_lines() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 2);

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_no_general_lines_no_leading_lines_remove_one_trailing_line() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 1);

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 1);
	}

	#[test]
	fn view_data_case_rebuild_no_change() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 100);
		view_data.rebuild();
		view_data.rebuild();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_lines().len(), 4);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
	}

	#[test]
	fn view_data_case_with_no_general_lines_no_leading_lines_no_trailing_lines() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 0);

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
	}

	#[test]
	fn view_data_case_with_no_general_lines_no_leading_lines_no_trailing_lines_with_title() {
		let mut view_data = create_mocked_view_data();
		view_data.set_view_size(100, 0);
		view_data.set_show_title(true);

		assert_eq!(view_data.get_leading_lines().len(), 0);
		assert_eq!(view_data.get_lines().len(), 0);
		assert_eq!(view_data.get_trailing_lines().len(), 0);
	}

	#[test]
	fn view_data_case_scroll_down_one_line() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		view_data.scroll_down();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "2");
	}

	#[test]
	fn view_data_case_scroll_down_two_lines() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		view_data.scroll_down();
		view_data.scroll_down();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "3");
	}

	#[test]
	fn view_data_case_scroll_down_bottom() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		for _ in 0..4 {
			view_data.scroll_down();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "5");
	}

	#[test]
	fn view_data_case_scroll_down_one_past_bottom() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		for _ in 0..5 {
			view_data.scroll_down();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "5");
	}

	#[test]
	fn view_data_case_scroll_down_well_past_bottom() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		for _ in 0..20 {
			view_data.scroll_down();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "5");
	}

	#[test]
	fn view_data_case_scroll_up_one_line() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.scroll_up();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "4");
	}

	#[test]
	fn view_data_case_scroll_up_two_lines() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.scroll_up();
		view_data.scroll_up();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "3");
	}

	#[test]
	fn view_data_case_scroll_up_top() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		for _ in 0..4 {
			view_data.scroll_up();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "1");
	}

	#[test]
	fn view_data_case_scroll_up_one_past_top() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		for _ in 0..5 {
			view_data.scroll_up();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "1");
	}

	#[test]
	fn view_data_case_scroll_up_well_past_top() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		for _ in 0..20 {
			view_data.scroll_up();
		}

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "1");
	}

	#[test]
	fn view_data_case_page_down_once() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		view_data.page_down();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "3");
	}

	#[test]
	fn view_data_case_page_down_past_bottom() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		view_data.page_down();
		view_data.page_down();
		view_data.page_down();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "5");
	}

	#[test]
	fn view_data_case_page_up_once() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.page_up();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "3");
	}

	#[test]
	fn view_data_case_page_up_past_top() {
		let mut view_data = create_mocked_scroll_vertical_view_data();

		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.page_up();
		view_data.page_up();
		view_data.page_up();

		assert_eq!(view_data.get_leading_lines().len(), 3);
		assert_eq!(view_data.get_trailing_lines().len(), 2);
		assert_eq!(view_data.get_lines().len(), 5);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 4, 0), "1");
	}

	#[test]
	fn view_data_case_scroll_left_one_from_start() {
		let mut view_data = create_mocked_scroll_horizontal_view_data();
		view_data.scroll_left();

		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 0), "aaaaa");
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 1), "  ");
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_lines(), 1, 0),
			"aaaaaaa"
		);
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_lines(), 2, 0),
			"aaaaaaa"
		);
	}

	#[test]
	fn view_data_case_scroll_right_one_from_start() {
		let mut view_data = create_mocked_scroll_horizontal_view_data();
		view_data.scroll_right();

		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 0), "aaaa");
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 1), "   ");
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_lines(), 1, 0),
			"aaaaaaa"
		);
		assert_eq!(
			get_segment_content_for_view_line(view_data.get_lines(), 2, 0),
			"aaaaaaa"
		);
	}

	#[test]
	fn view_data_case_scroll_right_to_end() {
		let mut view_data = create_mocked_scroll_horizontal_view_data();
		for _ in 0..8 {
			view_data.scroll_right();
		}

		let lines = view_data.get_lines();
		assert_eq!(get_segment_content_for_view_line(lines, 0, 0), "       ");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 0), "aa");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 1), "     ");
		assert_eq!(get_segment_content_for_view_line(lines, 2, 0), "aaaaaaa");
	}

	#[test]
	fn view_data_case_scroll_right_past_end() {
		let mut view_data = create_mocked_scroll_horizontal_view_data();
		for _ in 0..20 {
			view_data.scroll_right();
		}

		let lines = view_data.get_lines();
		assert_eq!(get_segment_content_for_view_line(lines, 0, 0), "       ");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 0), "aa");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 1), "     ");
		assert_eq!(get_segment_content_for_view_line(lines, 2, 0), "aaaaaaa");
	}

	#[test]
	fn view_data_case_scroll_down_trigger_shorter_width() {
		let mut view_data = create_mocked_scroll_horizontal_view_data();
		view_data.set_view_size(7, 3);
		for _ in 0..10 {
			view_data.scroll_right();
		}
		for _ in 0..3 {
			view_data.scroll_down();
		}

		let lines = view_data.get_lines();
		assert_eq!(get_segment_content_for_view_line(lines, 0, 0), "aaaaaa");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 0), "a");
		assert_eq!(get_segment_content_for_view_line(lines, 1, 1), "     ");
	}

	#[test]
	fn view_data_case_calculate_max_line_length_max_first() {
		let view_lines = [
			ViewLine::new(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("012345")]),
		];
		assert_eq!(ViewData::calculate_max_line_length(&view_lines, 0, 1), 16);
	}

	#[test]
	fn view_data_case_calculate_max_line_length_max_last() {
		let view_lines = [
			ViewLine::new(vec![LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
		];
		assert_eq!(ViewData::calculate_max_line_length(&view_lines, 0, 2), 16);
	}

	#[test]
	fn view_data_case_calculate_max_line_length_with_slice() {
		let view_lines = [
			ViewLine::new(vec![LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("0123456789"), LineSegment::new("01234567")]),
		];
		assert_eq!(ViewData::calculate_max_line_length(&view_lines, 1, 2), 16);
	}

	#[test]
	fn view_data_case_calculate_max_line_length_ignore_pinned() {
		let view_lines = [
			ViewLine::new(vec![LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("012345")]),
			ViewLine::new(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
			ViewLine::new_pinned(vec![LineSegment::new("0123456789"), LineSegment::new("01234567")]),
		];
		assert_eq!(ViewData::calculate_max_line_length(&view_lines, 0, 4), 16);
	}

	#[test]
	fn view_data_case_view_resize_zero() {
		let mut view_data = create_mocked_scroll_vertical_view_data();
		view_data.set_view_size(0, 0);
		assert_eq!(view_data.height, 0);
		assert_eq!(view_data.width, 0);
	}

	#[test]
	fn view_data_case_view_resize_and_top_greater_than_length() {
		// I don't think this path is triggerable, since a view resize will reset the top position - Tim
		let mut view_data = create_mocked_scroll_vertical_view_data();
		view_data.set_show_title(false);
		view_data.scroll_down();
		view_data.scroll_down();
		view_data.scroll_down();
		view_data.scroll_down();
		view_data.lines_cache = None;
		view_data.height = 13;
		view_data.rebuild();
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 0), "b");
	}

	#[test]
	fn view_data_case_ensure_line_visible_with_scroll_change() {
		let mut view_data = create_mocked_scroll_vertical_view_data();
		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.ensure_line_visible(1);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 0), "b");
	}

	#[test]
	fn view_data_case_ensure_line_visible_without_scroll_change() {
		let mut view_data = create_mocked_scroll_vertical_view_data();
		// set scroll position to bottom
		for _ in 0..5 {
			view_data.scroll_down();
		}

		view_data.ensure_line_visible(4);
		assert_eq!(get_segment_content_for_view_line(view_data.get_lines(), 0, 0), "1");
	}

	#[test]
	fn view_data_case_get_scroll_index_top_position() {
		let view_data = create_mocked_scroll_index_data(100, 100, 0);
		assert_eq!(view_data.get_scroll_index(), 0);
	}

	#[test]
	fn view_data_case_get_scroll_index_empty_lines() {
		let view_data = ViewData::new();
		assert_eq!(view_data.get_scroll_index(), 0);
	}

	#[test]
	fn view_data_case_get_scroll_index_end_position() {
		let view_data = create_mocked_scroll_index_data(100, 10, 90);
		assert_eq!(view_data.get_scroll_index(), 9);
	}

	#[test]
	fn view_data_case_get_scroll_index_position_one_down() {
		let view_data = create_mocked_scroll_index_data(100, 10, 1);
		assert_eq!(view_data.get_scroll_index(), 1);
	}

	#[test]
	fn view_data_case_get_scroll_index_position_low_input_range_1() {
		let view_data = create_mocked_scroll_index_data(10, 8, 1);
		assert_eq!(view_data.get_scroll_index(), 4);
	}

	#[test]
	fn view_data_case_get_scroll_index_item_count_smaller_than_height() {
		let view_data = create_mocked_scroll_index_data(10, 11, 1);
		assert_eq!(view_data.get_scroll_index(), 0);
	}

	#[test]
	fn view_data_case_get_scroll_index_view_height_too_small() {
		let view_data = create_mocked_scroll_index_data(10, 2, 5);
		assert_eq!(view_data.get_scroll_index(), 0);
	}

	#[test]
	fn view_data_case_get_scroll_index_position_extreme_lows() {
		assert_eq!(create_mocked_scroll_index_data(0, 0, 0).get_scroll_index(), 0);
		assert_eq!(create_mocked_scroll_index_data(2, 1, 1).get_scroll_index(), 0);
		assert_eq!(create_mocked_scroll_index_data(2, 0, 1).get_scroll_index(), 0);
	}
}
