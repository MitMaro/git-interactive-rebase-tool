use crate::action::Action;
use crate::commit::Commit;
use crate::constants::{
	HEIGHT_ERROR_MESSAGE,
	LIST_FOOTER_COMPACT,
	LIST_FOOTER_COMPACT_WIDTH,
	LIST_FOOTER_FULL,
	LIST_FOOTER_FULL_WIDTH,
	MINIMUM_COMPACT_WINDOW_WIDTH,
	MINIMUM_FULL_WINDOW_WIDTH,
	MINIMUM_WINDOW_HEIGHT,
	MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH,
	SHORT_ERROR_MESSAGE,
	SHORT_ERROR_MESSAGE_WIDTH,
	TITLE,
	TITLE_HELP_INDICATOR,
	TITLE_HELP_INDICATOR_LENGTH,
	TITLE_LENGTH,
	TITLE_SHORT,
	TITLE_SHORT_LENGTH,
	TO_FILE_INDICATOR,
	TO_FILE_INDICATOR_SHORT,
	VISUAL_MODE_FOOTER_COMPACT,
	VISUAL_MODE_FOOTER_COMPACT_WIDTH,
	VISUAL_MODE_FOOTER_FULL,
	VISUAL_MODE_FOOTER_FULL_WIDTH,
};
use crate::line::Line;
use crate::scroll::{get_scroll_position, ScrollPosition};
use crate::view::{LineSegment, ViewLine};
use crate::window::Window;
use crate::window::WindowColor;
use git2::Delta;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct View<'v> {
	commit_top: ScrollPosition,
	help_top: ScrollPosition,
	main_top: ScrollPosition,
	window: &'v Window<'v>,
}

impl<'v> View<'v> {
	pub fn new(window: &'v Window) -> Self {
		Self {
			commit_top: ScrollPosition::new(3, 6, 3),
			help_top: ScrollPosition::new(3, 6, 3),
			main_top: ScrollPosition::new(2, 1, 1),
			window,
		}
	}

	pub fn check_window_size(&self) -> bool {
		let (window_width, window_height) = self.window.get_window_size();
		!(window_width <= MINIMUM_COMPACT_WINDOW_WIDTH || window_height <= MINIMUM_WINDOW_HEIGHT)
	}

	pub fn draw_error(&self, message: &str) {
		self.draw_title(false);
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, false, false);
		self.window.draw_str(message);
		self.window.draw_str("\n");
		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Press any key to continue");
	}

	pub fn clear(&self) {
		self.window.clear();
	}

	pub fn refresh(&self) {
		self.window.refresh();
	}

	pub fn draw_window_size_error(&self) {
		let (window_width, window_height) = self.window.get_window_size();

		self.window.color(WindowColor::Foreground);
		if window_width <= MINIMUM_COMPACT_WINDOW_WIDTH {
			if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				self.window.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				self.window.draw_str("Size!\n");
			}
			return;
		}

		if window_height <= MINIMUM_WINDOW_HEIGHT {
			if window_width >= MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH {
				self.window.draw_str(HEIGHT_ERROR_MESSAGE);
			}
			else if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				self.window.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				self.window.draw_str("Size!\n");
			}
		}
	}

	pub fn draw_view_lines(&self, lines: Vec<ViewLine>, top: usize, height: usize) {
		let number_of_lines = lines.len();

		let scroll_indicator_index = get_scroll_position(top, number_of_lines, height);
		let show_scroll_bar = height < number_of_lines;

		let mut index: usize = 0;
		for line in lines.iter().skip(top).take(height) {
			self.draw_view_line(line, show_scroll_bar);
			if show_scroll_bar {
				self.window.color(WindowColor::Foreground);
				self.window.set_style(scroll_indicator_index != index, false, true);
				self.window.draw_str(" ");
			}
			index += 1;
		}

		if height > lines.len() {
			self.draw_vertical_spacer((height - index) as i32);
		}
	}

	pub fn draw_view_line(&self, line: &ViewLine, scrollbar: bool) {
		let (window_width, _) = self.window.get_window_size();
		let window_width = if scrollbar { window_width - 1 } else { window_width } as usize;

		let mut start = 0;
		for segment in &line.segments {
			start += segment.draw(window_width - start, &self.window);
			if start >= window_width {
				break;
			}
		}

		if start < window_width {
			let padding = " ".repeat(window_width - start);
			self.window.draw_str(padding.as_str());
		}
	}

	fn draw_title(&self, show_help: bool) {
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, true, true);
		let (window_width, _) = self.window.get_window_size();

		if window_width >= TITLE_LENGTH {
			self.window.draw_str(TITLE);
			// only draw help if there is room
			if window_width > TITLE_LENGTH + TITLE_HELP_INDICATOR_LENGTH {
				if (window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) > 0 {
					let padding = " ".repeat((window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) as usize);
					self.window.draw_str(padding.as_str());
				}
				if show_help {
					self.window.draw_str(TITLE_HELP_INDICATOR);
				}
				else {
					let padding = " ".repeat(TITLE_HELP_INDICATOR_LENGTH as usize);
					self.window.draw_str(padding.as_str());
				}
			}
			else if (window_width - TITLE_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_LENGTH) as usize);
				self.window.draw_str(padding.as_str());
			}
		}
		else {
			self.window.draw_str(TITLE_SHORT);
			if (window_width - TITLE_SHORT_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_SHORT_LENGTH) as usize);
				self.window.draw_str(padding.as_str());
			}
		}
	}

	fn draw_vertical_spacer(&self, repeat: i32) {
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, false, false);
		for _x in 0..repeat {
			self.window.draw_vertical_space_character();
		}
	}

	fn draw_visual_mode_footer(&self) {
		let (window_width, _) = self.window.get_window_size();
		self.window.color(WindowColor::Foreground);
		self.window.set_style(true, false, false);
		if window_width >= VISUAL_MODE_FOOTER_FULL_WIDTH {
			self.window.draw_str(VISUAL_MODE_FOOTER_FULL);
		}
		else if window_width >= VISUAL_MODE_FOOTER_COMPACT_WIDTH {
			self.window.draw_str(VISUAL_MODE_FOOTER_COMPACT);
		}
		else {
			self.window.draw_str("(Visual) Help: ?");
		}
		self.window.set_style(false, false, false);
	}

	fn draw_list_footer(&self) {
		let (window_width, _) = self.window.get_window_size();
		self.window.color(WindowColor::Foreground);
		self.window.set_style(true, false, false);
		if window_width >= LIST_FOOTER_FULL_WIDTH {
			self.window.draw_str(LIST_FOOTER_FULL);
		}
		else if window_width >= LIST_FOOTER_COMPACT_WIDTH {
			self.window.draw_str(LIST_FOOTER_COMPACT);
		}
		else {
			self.window.draw_str("Help: ?");
		}
		self.window.set_style(false, false, false);
	}

	pub fn update_main_top(&mut self, number_of_lines: usize, selected_index: usize) {
		let (_, window_height) = self.window.get_window_size();
		self.main_top
			.ensure_cursor_visible(selected_index, window_height as usize, number_of_lines);
	}

	#[allow(clippy::nonminimal_bool)]
	pub fn draw_main(&self, lines: &[Line], selected_index: usize, visual_index_start: Option<usize>) {
		let (_, window_height) = self.window.get_window_size();
		let view_height = window_height as usize - 2;

		let mut view_lines: Vec<ViewLine> = vec![];

		for (index, line) in lines.iter().enumerate() {
			let is_cursor_line = match visual_index_start {
				Some(visual_index) => {
					(visual_index <= selected_index && index >= visual_index && index <= selected_index)
						|| (visual_index > selected_index && index >= selected_index && index <= visual_index)
				},
				None => false,
			};
			view_lines.push(ViewLine {
				segments: self.get_todo_line_segments(line, selected_index == index, is_cursor_line),
			});
		}

		self.window.clear();
		self.draw_title(true);

		self.draw_view_lines(view_lines, self.main_top.get_position(), view_height);

		// TODO need something else here
		if visual_index_start.is_some() {
			self.draw_visual_mode_footer();
		}
		else {
			self.draw_list_footer();
		}
	}

	fn get_action_color(&self, action: Action) -> WindowColor {
		match action {
			Action::Break => WindowColor::ActionBreak,
			Action::Drop => WindowColor::ActionDrop,
			Action::Edit => WindowColor::ActionEdit,
			Action::Exec => WindowColor::ActionExec,
			Action::Fixup => WindowColor::ActionFixup,
			Action::Pick => WindowColor::ActionPick,
			Action::Reword => WindowColor::ActionReword,
			Action::Squash => WindowColor::ActionSquash,
		}
	}

	pub fn get_todo_line_segments(&self, line: &Line, is_cursor_line: bool, selected: bool) -> Vec<LineSegment> {
		let (window_width, _) = self.window.get_window_size();

		let mut segments: Vec<LineSegment> = vec![];

		let action = line.get_action();

		self.window.set_style(false, false, false);
		if window_width >= MINIMUM_FULL_WINDOW_WIDTH {
			segments.push(LineSegment::new_with_color_and_style(
				if is_cursor_line || selected { " > " } else { "   " },
				WindowColor::Foreground,
				!is_cursor_line && selected,
				false,
				false,
			));

			segments.push(LineSegment::new_with_color(
				format!("{:6} ", action.as_string()).as_str(),
				self.get_action_color(*action),
			));

			segments.push(LineSegment::new(
				if *action == Action::Exec {
					line.get_command().clone()
				}
				else if *action == Action::Break {
					String::from("         ")
				}
				else {
					let max_index = cmp::min(line.get_hash().len(), 8);
					format!("{:8} ", line.get_hash()[0..max_index].to_string())
				}
				.as_str(),
			));
		}
		else {
			segments.push(LineSegment::new_with_color_and_style(
				if is_cursor_line || selected { ">" } else { " " },
				WindowColor::Foreground,
				!is_cursor_line && selected,
				false,
				false,
			));

			segments.push(LineSegment::new_with_color(
				format!("{:1} ", line.get_action().to_abbreviation()).as_str(),
				self.get_action_color(*action),
			));

			segments.push(LineSegment::new(
				if *action == Action::Exec {
					line.get_command().clone()
				}
				else if *action == Action::Break {
					String::from("    ")
				}
				else {
					let max_index = cmp::min(line.get_hash().len(), 3);
					format!("{:3} ", line.get_hash()[0..max_index].to_string())
				}
				.as_str(),
			));
		}
		if *action != Action::Exec && *action != Action::Break {
			segments.push(LineSegment::new(line.get_comment().as_str()));
		}
		segments
	}

	pub fn update_commit_top(&mut self, scroll_up: bool, reset: bool, lines_length: usize) {
		let (_, window_height) = self.window.get_window_size();
		if reset {
			self.commit_top.reset();
		}
		else if scroll_up {
			self.commit_top.scroll_up(window_height as usize, lines_length);
		}
		else {
			self.commit_top.scroll_down(window_height as usize, lines_length);
		}
	}

	pub fn update_help_top(&self, scroll_up: bool, reset: bool, help_lines: &[(&str, &str)]) {
		let (_, window_height) = self.window.get_window_size();
		if reset {
			self.help_top.reset();
		}
		else if scroll_up {
			self.help_top.scroll_up(window_height as usize, help_lines.len());
		}
		else {
			self.help_top.scroll_down(window_height as usize, help_lines.len());
		}
	}

	pub fn draw_help(&self, help_lines: &[(&str, &str)]) {
		let (window_width, window_height) = self.window.get_window_size();
		let view_height = window_height as usize - 3;

		let mut view_lines: Vec<ViewLine> = vec![];

		for line in help_lines {
			view_lines.push(ViewLine {
				segments: vec![
					LineSegment::new_with_color(format!(" {:4} ", line.0).as_str(), WindowColor::IndicatorColor),
					LineSegment::new(line.1),
				],
			})
		}

		self.window.set_style(false, false, false);
		self.window.clear();
		self.draw_title(false);

		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, true, false);
		self.window.draw_str(" Key   Action");
		if window_width as usize > 13 {
			let padding = " ".repeat(window_width as usize - 13);
			self.window.draw_str(padding.as_str());
		}

		self.draw_view_lines(view_lines, self.help_top.get_position(), view_height);

		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Any key to close");
	}

	pub fn draw_prompt(&self, message: &str) {
		self.draw_title(false);
		self.window.set_style(false, false, false);
		self.window.draw_str(&format!("\n{} ", message));
	}

	pub fn draw_confirm(&self, message: &str) {
		self.draw_prompt(&format!("{} (y/n)? ", message));
	}

	pub fn draw_exiting(&self) {
		self.window.draw_str("Exiting...")
	}

	fn get_file_stat_long(&self, status: Delta) -> String {
		match status {
			Delta::Added => format!("{:>8}: ", "added"),
			Delta::Copied => format!("{:>8}: ", "copied"),
			Delta::Deleted => format!("{:>8}: ", "deleted"),
			Delta::Modified => format!("{:>8}: ", "modified"),
			Delta::Renamed => format!("{:>8}: ", "renamed"),
			Delta::Typechange => format!("{:>8}: ", "changed"),

			// these should never happen in a rebase
			Delta::Conflicted => format!("{:>8}: ", "unknown"),
			Delta::Ignored => format!("{:>8}: ", "unknown"),
			Delta::Unmodified => format!("{:>8}: ", "unknown"),
			Delta::Unreadable => format!("{:>8}: ", "unknown"),
			Delta::Untracked => format!("{:>8}: ", "unknown"),
		}
	}

	fn get_file_stat_abbreviated(&self, status: Delta) -> String {
		match status {
			Delta::Added => String::from("A "),
			Delta::Copied => String::from("C "),
			Delta::Deleted => String::from("D "),
			Delta::Modified => String::from("M "),
			Delta::Renamed => String::from("R "),
			Delta::Typechange => String::from("T "),

			// these should never happen in a rebase
			Delta::Conflicted => String::from("X "),
			Delta::Ignored => String::from("X "),
			Delta::Unmodified => String::from("X "),
			Delta::Unreadable => String::from("X "),
			Delta::Untracked => String::from("X "),
		}
	}

	fn get_file_stat_color(&self, status: Delta) -> WindowColor {
		match status {
			Delta::Added => WindowColor::DiffAddColor,
			Delta::Copied => WindowColor::DiffAddColor,
			Delta::Deleted => WindowColor::DiffRemoveColor,
			Delta::Modified => WindowColor::DiffChangeColor,
			Delta::Renamed => WindowColor::DiffChangeColor,
			Delta::Typechange => WindowColor::DiffChangeColor,

			// these should never happen in a rebase
			Delta::Conflicted => WindowColor::Foreground,
			Delta::Ignored => WindowColor::Foreground,
			Delta::Unmodified => WindowColor::Foreground,
			Delta::Unreadable => WindowColor::Foreground,
			Delta::Untracked => WindowColor::Foreground,
		}
	}

	fn get_stat_item_segments(&self, status: Delta, to_name: &str, from_name: &str) -> Vec<LineSegment> {
		let (window_width, _) = self.window.get_window_size();

		let status_name = if window_width >= MINIMUM_FULL_WINDOW_WIDTH {
			self.get_file_stat_long(status)
		}
		else {
			self.get_file_stat_abbreviated(status)
		};

		let color = self.get_file_stat_color(status);

		let to_file_indicator = if window_width >= MINIMUM_FULL_WINDOW_WIDTH {
			TO_FILE_INDICATOR
		}
		else {
			TO_FILE_INDICATOR_SHORT
		};

		match status {
			Delta::Copied => {
				vec![
					LineSegment::new_with_color(status_name.clone().as_str(), color),
					LineSegment::new_with_color(to_name, WindowColor::Foreground),
					LineSegment::new(to_file_indicator),
					LineSegment::new_with_color(from_name, WindowColor::DiffAddColor),
				]
			},
			Delta::Renamed => {
				vec![
					LineSegment::new_with_color(status_name.as_str(), color),
					LineSegment::new_with_color(to_name, WindowColor::DiffRemoveColor),
					LineSegment::new(to_file_indicator),
					LineSegment::new_with_color(from_name, WindowColor::DiffAddColor),
				]
			},
			_ => {
				vec![
					LineSegment::new_with_color(status_name.as_str(), color),
					LineSegment::new_with_color(from_name, color),
				]
			},
		}
	}

	pub fn draw_show_commit(&self, commit_data: &Option<Commit>) {
		let (window_width, window_height) = self.window.get_window_size();
		let view_height = window_height as usize - 2;

		let is_full_width = window_width >= MINIMUM_FULL_WINDOW_WIDTH;

		self.window.clear();
		self.draw_title(false);

		let commit = match commit_data {
			None => {
				self.draw_error("Not commit data to show");
				return;
			},
			Some(c) => c,
		};

		let full_hash = commit.get_hash();
		let author = commit.get_author();
		let committer = commit.get_committer();
		let date = commit.get_date();
		let body = commit.get_body();
		let file_stats = commit.get_file_stats();

		let mut lines: Vec<ViewLine> = vec![];

		lines.push(ViewLine {
			segments: vec![LineSegment::new_with_color(
				if is_full_width {
					format!("Commit: {}", full_hash)
				}
				else {
					let max_index = cmp::min(full_hash.len(), 8);
					format!("{:8} ", full_hash[0..max_index].to_string())
				}
				.as_str(),
				WindowColor::IndicatorColor,
			)],
		});

		lines.push(ViewLine {
			segments: vec![LineSegment::new(
				if is_full_width {
					format!("Date: {}", date.format("%c %z"))
				}
				else {
					format!("{}", date.format("%c %z"))
				}
				.as_str(),
			)],
		});

		if let Some(a) = author.to_string() {
			lines.push(ViewLine {
				segments: vec![LineSegment::new(
					if is_full_width {
						format!("Author: {}", a)
					}
					else {
						format!("A: {}", a)
					}
					.as_str(),
				)],
			});
		}

		if let Some(c) = committer.to_string() {
			lines.push(ViewLine {
				segments: vec![LineSegment::new(
					if is_full_width {
						format!("Committer: {}", c)
					}
					else {
						format!("C: {}", c)
					}
					.as_str(),
				)],
			})
		};

		match body {
			Some(b) => {
				for line in b.lines() {
					lines.push(ViewLine {
						segments: vec![LineSegment::new(line)],
					});
				}
			},
			None => {},
		};

		lines.push(ViewLine {
			segments: vec![LineSegment::new("")],
		});

		match file_stats {
			Some(stats) => {
				for stat in stats {
					lines.push(ViewLine {
						segments: self.get_stat_item_segments(
							*stat.get_status(),
							stat.get_to_name().as_str(),
							stat.get_from_name().as_str(),
						),
					})
				}
			},
			None => {},
		}

		self.draw_view_lines(lines, self.commit_top.get_position(), view_height);

		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Any key to close");
	}

	pub fn draw_edit(&self, line: &str, pointer: usize) {
		self.draw_title(false);
		self.window.set_style(false, true, false);
		self.window.color(WindowColor::Foreground);

		// this could probably be made way more efficient
		let graphemes = UnicodeSegmentation::graphemes(line, true);
		let segment_length = graphemes.clone().count();
		for (counter, c) in graphemes.enumerate() {
			if counter == pointer {
				self.window.set_style(false, true, false);
				self.window.draw_str(c);
				self.window.set_style(false, false, false);
			}
			else {
				self.window.draw_str(c);
			}
		}
		if pointer >= segment_length {
			self.window.set_style(false, true, false);
			self.window.draw_str(" ");
			self.window.set_style(false, false, false);
		}

		self.window.draw_str("\n\n");
		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Enter to finish");
	}
}
