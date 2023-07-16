use display::DisplayColor;
use input::{KeyCode, KeyEvent, KeyModifiers};
use unicode_segmentation::UnicodeSegmentation;
use view::LineSegment;

use crate::events::Event;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EditAction {
	CursorMove,
	ContentUpdate,
	None,
}

pub(crate) struct EditableLine {
	content: String,
	cursor_position: usize,
	label: Option<LineSegment>,
	read_only: bool,
}

impl EditableLine {
	pub(crate) const fn new() -> Self {
		Self {
			content: String::new(),
			cursor_position: 0,
			label: None,
			read_only: false,
		}
	}

	pub(crate) fn set_label(&mut self, label: LineSegment) {
		self.label = Some(label);
	}

	pub(crate) fn set_content(&mut self, content: &str) {
		self.content = String::from(content);
		self.cursor_position = UnicodeSegmentation::graphemes(content, true).count();
	}

	pub(crate) fn set_read_only(&mut self, read_only: bool) {
		self.read_only = read_only;
	}

	pub(crate) fn clear(&mut self) {
		self.content.clear();
		self.cursor_position = 0;
	}

	pub(crate) fn get_content(&self) -> &str {
		self.content.as_str()
	}

	pub(crate) const fn cursor_position(&self) -> usize {
		self.cursor_position
	}

	pub(crate) fn line_segments(&self) -> Vec<LineSegment> {
		if self.read_only {
			return vec![LineSegment::new(self.get_content())];
		}

		let line = self.content.as_str();
		let pointer = self.cursor_position;

		let graphemes = UnicodeSegmentation::graphemes(line, true);

		let start = graphemes.clone().take(pointer).collect::<String>();
		let indicator = graphemes.clone().skip(pointer).take(1).collect::<String>();
		let end = graphemes.skip(pointer + 1).collect::<String>();

		let mut segments = vec![];
		if let Some(label) = self.label.as_ref() {
			segments.push(label.clone());
		}
		if !start.is_empty() {
			segments.push(LineSegment::new(start.as_str()));
		}
		segments.push(
			if indicator.is_empty() {
				LineSegment::new_with_color_and_style(" ", DisplayColor::Normal, false, true, false)
			}
			else {
				LineSegment::new_with_color_and_style(indicator.as_str(), DisplayColor::Normal, false, true, false)
			},
		);
		if !end.is_empty() {
			segments.push(LineSegment::new(end.as_str()));
		}

		segments
	}

	pub(crate) fn handle_event(&mut self, event: Event) -> EditAction {
		if self.read_only {
			return EditAction::None;
		}
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Backspace,
				modifiers: KeyModifiers::NONE,
			}) => {
				if self.cursor_position == 0 {
					EditAction::None
				}
				else {
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position - 1)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position)
						.collect::<String>();
					self.content = format!("{start}{end}");
					self.cursor_position -= 1;
					EditAction::ContentUpdate
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Delete,
				modifiers: KeyModifiers::NONE,
			}) => {
				let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if self.cursor_position == length {
					EditAction::None
				}
				else {
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position + 1)
						.collect::<String>();
					self.content = format!("{start}{end}");
					EditAction::ContentUpdate
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => {
				if self.cursor_position == 0 {
					EditAction::None
				}
				else {
					self.cursor_position = 0;
					EditAction::CursorMove
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => {
				let new_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if new_position == self.cursor_position {
					EditAction::None
				}
				else {
					self.cursor_position = new_position;
					EditAction::CursorMove
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => {
				let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if self.cursor_position < length {
					self.cursor_position += 1;
					EditAction::CursorMove
				}
				else {
					EditAction::None
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => {
				if self.cursor_position == 0 {
					EditAction::None
				}
				else {
					self.cursor_position -= 1;
					EditAction::CursorMove
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				modifiers: KeyModifiers::NONE,
			}) => {
				let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.take(self.cursor_position)
					.collect::<String>();
				let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.skip(self.cursor_position)
					.collect::<String>();
				self.content = format!("{start}{c}{end}");
				self.cursor_position += 1;
				EditAction::ContentUpdate
			},
			_ => EditAction::None,
		}
	}
}

#[cfg(test)]
mod tests {
	use view::{assert_rendered_output, ViewData, ViewLine};

	use super::*;

	macro_rules! view_data_from_editable_line {
		($editable_line:expr) => {{
			let segments = $editable_line.line_segments();
			&ViewData::new(|updater| updater.push_line(ViewLine::from(segments)))
		}};
	}

	fn handle_events(module: &mut EditableLine, events: &[Event]) {
		for event in events {
			_ = module.handle_event(*event);
		}
	}

	#[test]
	fn with_label() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		editable_line.set_label(LineSegment::new("Label: "));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}Label: foobar{Normal,Underline} "
		);
	}

	#[test]
	fn move_cursor_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		_ = editable_line.handle_event(Event::from(KeyCode::Right));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}foobar{Normal,Underline} "
		);
	}

	#[test]
	fn move_cursor_1_left() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		_ = editable_line.handle_event(Event::from(KeyCode::Left));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}fooba{Normal,Underline}r"
		);
	}

	#[test]
	fn move_cursor_2_from_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 2]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}foob{Normal,Underline}a{Normal}r"
		);
	}

	#[test]
	fn move_cursor_1_from_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 5]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}f{Normal,Underline}o{Normal}obar"
		);
	}

	#[test]
	fn move_cursor_to_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 6]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar"
		);
	}

	#[test]
	fn move_cursor_to_home() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		_ = editable_line.handle_event(Event::from(KeyCode::Home));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar"
		);
	}

	#[test]
	fn move_cursor_right() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Right),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}foob{Normal,Underline}a{Normal}r"
		);
	}

	#[test]
	fn move_cursor_to_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::End),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}foobar{Normal,Underline} "
		);
	}

	#[test]
	fn move_cursor_on_empty_content() {
		let mut editable_line = EditableLine::new();
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Right),
			Event::from(KeyCode::End),
			Event::from(KeyCode::Home),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline} "
		);
	}

	#[test]
	fn move_cursor_attempt_past_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 10]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar"
		);
	}

	#[test]
	fn move_cursor_attempt_past_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("foobar");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Right); 10]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}foobar{Normal,Underline} "
		);
	}

	#[test]
	fn multiple_width_unicode_single_width() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("a🗳b");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 2]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}a{Normal,Underline}🗳{Normal}b"
		);
	}

	#[test]
	fn multiple_width_unicode_emoji() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("a😀b");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left); 2]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}a{Normal,Underline}😀{Normal}b"
		);
	}

	#[test]
	fn add_character_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		_ = editable_line.handle_event(Event::from('x'));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abcdx{Normal,Underline} "
		);
	}

	#[test]
	fn add_character_one_from_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[Event::from(KeyCode::Left), Event::from('x')]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abcx{Normal,Underline}d"
		);
	}

	#[test]
	fn add_character_one_from_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from('x'),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}ax{Normal,Underline}b{Normal}cd"
		);
	}

	#[test]
	fn add_character_at_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from('x'),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}x{Normal,Underline}a{Normal}bcd"
		);
	}

	#[test]
	fn add_character_uppercase() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		_ = editable_line.handle_event(Event::from(KeyCode::Char('X')));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abcdX{Normal,Underline} "
		);
	}

	#[test]
	fn backspace_at_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		_ = editable_line.handle_event(Event::from(KeyCode::Backspace));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abc{Normal,Underline} "
		);
	}

	#[test]
	fn backspace_one_from_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Backspace),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}ab{Normal,Underline}d"
		);
	}

	#[test]
	fn backspace_one_from_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Backspace),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}b{Normal}cd"
		);
	}

	#[test]
	fn backspace_at_start() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Backspace),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}a{Normal}bcd"
		);
	}

	#[test]
	fn delete_at_end() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		_ = editable_line.handle_event(Event::from(KeyCode::Delete));
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abcd{Normal,Underline} "
		);
	}

	#[test]
	fn delete_last_character() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Delete),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}abc{Normal,Underline} "
		);
	}

	#[test]
	fn delete_second_character() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Delete),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal}a{Normal,Underline}c{Normal}d"
		);
	}

	#[test]
	fn delete_first_character() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		handle_events(&mut editable_line, &[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Delete),
		]);
		assert_rendered_output!(
			Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
			view_data_from_editable_line!(&editable_line),
			"{BODY}",
			"{Normal,Underline}b{Normal}cd"
		);
	}

	#[test]
	fn ignore_other_input() {
		let mut editable_line = EditableLine::new();
		_ = editable_line.handle_event(Event::from(KeyCode::Null));
	}

	#[test]
	fn ignore_input_on_readonly() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.set_read_only(true);
		_ = editable_line.handle_event(Event::from(KeyCode::Home));
		assert_eq!(editable_line.cursor_position(), 4);
	}

	#[test]
	fn set_get_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(editable_line.cursor_position(), 4);
		assert_eq!(editable_line.get_content(), "abcd");
	}

	#[test]
	fn set_readonly() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.set_read_only(true);
		assert_eq!(editable_line.line_segments().len(), 1);
	}

	#[test]
	fn clear_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.clear();
		assert_eq!(editable_line.cursor_position(), 0);
		assert_eq!(editable_line.get_content(), "");
	}

	#[test]
	fn handle_event_edit_action_backspace_with_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Backspace)),
			EditAction::ContentUpdate
		);
	}

	#[test]
	fn handle_event_edit_action_backspace_without_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Backspace)),
			EditAction::None
		);
	}

	#[test]
	fn handle_event_edit_action_delete_with_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.cursor_position = 0;
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Delete)),
			EditAction::ContentUpdate
		);
	}

	#[test]
	fn handle_event_edit_action_delete_without_content() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Delete)),
			EditAction::None
		);
	}

	#[test]
	fn handle_event_edit_action_home_with_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Home)),
			EditAction::CursorMove
		);
	}

	#[test]
	fn handle_event_edit_action_home_without_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.cursor_position = 0;
		assert_eq!(editable_line.handle_event(Event::from(KeyCode::Home)), EditAction::None);
	}

	#[test]
	fn handle_event_edit_action_end_with_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.cursor_position = 0;
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::End)),
			EditAction::CursorMove
		);
	}

	#[test]
	fn handle_event_edit_action_end_without_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(editable_line.handle_event(Event::from(KeyCode::End)), EditAction::None);
	}

	#[test]
	fn handle_event_edit_action_right_with_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.cursor_position = 0;
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Right)),
			EditAction::CursorMove
		);
	}

	#[test]
	fn handle_event_edit_action_right_without_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Right)),
			EditAction::None
		);
	}

	#[test]
	fn handle_event_edit_action_left_with_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Left)),
			EditAction::CursorMove
		);
	}

	#[test]
	fn handle_event_edit_action_left_without_cursor_change() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		editable_line.cursor_position = 0;
		assert_eq!(editable_line.handle_event(Event::from(KeyCode::Left)), EditAction::None);
	}

	#[test]
	fn handle_event_edit_action_new_character() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(
			editable_line.handle_event(Event::from(KeyCode::Char('a'))),
			EditAction::ContentUpdate
		);
	}

	#[test]
	fn handle_event_edit_action_other() {
		let mut editable_line = EditableLine::new();
		editable_line.set_content("abcd");
		assert_eq!(editable_line.handle_event(Event::from(KeyCode::Esc)), EditAction::None);
	}
}
