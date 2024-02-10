mod action;
mod state;
#[cfg(test)]
mod tests;

pub(crate) use self::action::Action as SearchBarAction;
use crate::{
	components::{
		search_bar::state::State,
		shared::{EditAction, EditableLine},
	},
	events::Event,
	input::{InputOptions, KeyCode, KeyEvent, KeyModifiers, StandardEvent},
	view::{LineSegment, ViewLine},
};

const DEFAULT_LABEL: &str = "/";
const INPUT_OPTIONS_EDITING: InputOptions = InputOptions::RESIZE;
const INPUT_OPTIONS_SEARCHING: InputOptions = InputOptions::RESIZE
	.union(InputOptions::SEARCH)
	.union(InputOptions::HELP);

pub(crate) struct SearchBar {
	editable_line: EditableLine,
	state: State,
}

impl SearchBar {
	pub(crate) fn new() -> Self {
		let mut editable_line = EditableLine::new();
		editable_line.set_label(LineSegment::new(DEFAULT_LABEL));
		Self {
			editable_line,
			state: State::Deactivated,
		}
	}

	pub(crate) fn start_search(&mut self, initial_value: Option<&str>) {
		if let Some(value) = initial_value {
			self.editable_line.set_content(value);
		}
		self.editable_line.set_read_only(false);
		self.state = State::Editing;
	}

	pub(crate) fn reset(&mut self) {
		self.state = State::Deactivated;
	}

	pub(crate) const fn input_options(&self) -> Option<&InputOptions> {
		match self.state {
			State::Deactivated => None,
			State::Editing => Some(&INPUT_OPTIONS_EDITING),
			State::Searching => Some(&INPUT_OPTIONS_SEARCHING),
		}
	}

	pub(crate) const fn read_event(&self, event: Event) -> Option<Event> {
		match self.state {
			State::Deactivated | State::Searching => None,
			State::Editing => Some(event),
		}
	}

	pub(crate) fn handle_event(&mut self, event: Event) -> SearchBarAction {
		if !self.state.is_active() {
			return SearchBarAction::None;
		}
		match event {
			Event::Standard(StandardEvent::SearchNext) => {
				SearchBarAction::Next(String::from(self.editable_line.get_content()))
			},
			Event::Standard(StandardEvent::SearchPrevious) => {
				SearchBarAction::Previous(String::from(self.editable_line.get_content()))
			},
			Event::Standard(StandardEvent::SearchFinish)
			| Event::Key(KeyEvent {
				code: KeyCode::Enter,
				modifiers: KeyModifiers::NONE,
			}) => {
				self.editable_line.set_read_only(true);
				self.state = State::Searching;
				SearchBarAction::Start(String::from(self.editable_line.get_content()))
			},
			Event::Standard(StandardEvent::SearchStart) => {
				self.state = State::Deactivated;
				SearchBarAction::Cancel
			},
			Event::Key(KeyEvent {
				code: KeyCode::Esc,
				modifiers: KeyModifiers::NONE,
			}) => {
				self.reset();
				SearchBarAction::Cancel
			},
			_ => {
				if self.state == State::Editing && self.editable_line.handle_event(event) == EditAction::ContentUpdate {
					SearchBarAction::Update(String::from(self.editable_line.get_content()))
				}
				else {
					SearchBarAction::None
				}
			},
		}
	}

	pub(crate) fn search_value(&self) -> Option<&str> {
		match self.state {
			State::Deactivated => None,
			State::Editing | State::Searching => {
				let content = self.editable_line.get_content();
				if content.is_empty() { None } else { Some(content) }
			},
		}
	}

	pub(crate) const fn is_active(&self) -> bool {
		self.state.is_active()
	}

	pub(crate) fn is_editing(&self) -> bool {
		self.state == State::Editing
	}

	pub(crate) fn is_searching(&self) -> bool {
		self.state == State::Searching
	}

	pub(crate) fn build_view_line(&self) -> ViewLine {
		ViewLine::from(self.editable_line.line_segments())
	}
}
