use crate::commit::Commit;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::DisplayColor;
use crate::show_commit::util::get_stat_item_segments;
use crate::view::{LineSegment, ViewLine};
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Data {
	height: usize,
	width: usize,
	lines: Vec<ViewLine>,
	line_lengths: Vec<usize>,
	max_line_length: usize,
}

impl Data {
	pub fn new() -> Self {
		Self {
			height: 0,
			width: 0,
			lines: Vec::new(),
			line_lengths: Vec::new(),
			max_line_length: 0,
		}
	}

	pub fn reset(&mut self) {
		self.height = 0;
		self.width = 0;
		self.lines.clear();
		self.line_lengths.clear();
		self.max_line_length = 0;
	}

	pub fn update(&mut self, commit: &Commit, window_width: usize, window_height: usize) {
		if window_width != self.width || window_height != self.height {
			self.reset();

			self.height = window_height;
			self.width = window_width;

			let is_full_width = window_width >= MINIMUM_FULL_WINDOW_WIDTH;

			let full_hash = commit.get_hash();
			let author = commit.get_author();
			let committer = commit.get_committer();
			let date = commit.get_date();
			let body = commit.get_body();
			let file_stats = commit.get_file_stats();

			let hash_line = if is_full_width {
				format!("Commit: {}", full_hash)
			}
			else {
				let max_index = cmp::min(full_hash.len(), 8);
				format!("{:8} ", full_hash[0..max_index].to_string())
			};

			self.lines.push(ViewLine::new(vec![LineSegment::new_with_color(
				hash_line.as_str(),
				DisplayColor::IndicatorColor,
			)]));
			self.line_lengths.push(hash_line.len());

			let date_line = if is_full_width {
				format!("Date: {}", date.format("%c %z"))
			}
			else {
				format!("{}", date.format("%c %z"))
			};

			self.lines
				.push(ViewLine::new(vec![LineSegment::new(date_line.as_str())]));
			self.line_lengths.push(date_line.len());

			if let Some(a) = author.to_string() {
				let author_line = if is_full_width {
					format!("Author: {}", a)
				}
				else {
					format!("A: {}", a)
				};
				self.lines
					.push(ViewLine::new(vec![LineSegment::new(author_line.as_str())]));
				self.line_lengths
					.push(UnicodeSegmentation::graphemes(author_line.as_str(), true).count());
			}

			if let Some(c) = committer.to_string() {
				let committer_line = if is_full_width {
					format!("Committer: {}", c)
				}
				else {
					format!("C: {}", c)
				};
				self.lines
					.push(ViewLine::new(vec![LineSegment::new(committer_line.as_str())]));
				self.line_lengths
					.push(UnicodeSegmentation::graphemes(committer_line.as_str(), true).count());
			}

			match body {
				Some(b) => {
					for line in b.lines() {
						self.lines.push(ViewLine::new(vec![LineSegment::new(line)]));
						self.line_lengths
							.push(UnicodeSegmentation::graphemes(line, true).count());
					}
				},
				None => {},
			}

			self.lines.push(ViewLine::new(vec![LineSegment::new("")]));
			self.line_lengths.push(0);

			match file_stats {
				Some(stats) => {
					for stat in stats {
						let stat_to_name = stat.get_to_name();
						let stat_from_name = stat.get_from_name();
						let stat_view_line = ViewLine::new(get_stat_item_segments(
							*stat.get_status(),
							stat_to_name.as_str(),
							stat_from_name.as_str(),
							is_full_width,
						));
						self.line_lengths.push(stat_view_line.get_length());
						self.lines.push(stat_view_line);
					}
				},
				None => {},
			}
		}
	}

	pub fn get_lines(&self) -> &Vec<ViewLine> {
		&self.lines
	}

	pub fn get_max_line_length(&self, start: usize, end: usize) -> usize {
		let mut max_length = 0;
		for len in self.line_lengths[start..=end.min(self.line_lengths.len() - 1)].iter() {
			if *len > max_length {
				max_length = *len;
			}
		}
		max_length
	}
}
