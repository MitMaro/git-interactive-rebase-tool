#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use crate::{
	components::shared::EditableLine,
	display::DisplayColor,
	input::{Event, InputOptions, KeyCode, KeyEvent, KeyModifiers},
	view::{LineSegment, LineSegmentOptions, ViewData, ViewDataUpdater, ViewLine},
};

pub(crate) static INPUT_OPTIONS: LazyLock<InputOptions> = LazyLock::new(|| InputOptions::RESIZE);

const FINISH_EVENT: Event = Event::Key(KeyEvent {
	code: KeyCode::Enter,
	modifiers: KeyModifiers::NONE,
});

pub(crate) struct Edit {
	editable_line: EditableLine,
	finished: bool,
	view_data: ViewData,
}

impl Edit {
	pub(crate) fn new() -> Self {
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
		});
		Self {
			editable_line: EditableLine::new(),
			finished: false,
			view_data,
		}
	}

	pub(crate) fn build_view_data<F, G>(&mut self, before_build: F, after_build: G) -> &ViewData
	where
		F: FnOnce(&mut ViewDataUpdater<'_>),
		G: FnOnce(&mut ViewDataUpdater<'_>),
	{
		self.view_data.update_view_data(|updater| {
			updater.clear();
			before_build(updater);
			updater.push_line(ViewLine::from(self.editable_line.line_segments()));
			updater.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
				"Enter to finish",
				DisplayColor::IndicatorColor,
			)]));
			updater.ensure_column_visible(self.editable_line.cursor_position());
			updater.ensure_line_visible(0);
			after_build(updater);
		});
		&self.view_data
	}

	pub(crate) fn get_view_data(&mut self) -> &ViewData {
		self.build_view_data(|_| {}, |_| {})
	}

	pub(crate) fn handle_event(&mut self, event: Event) {
		if event == FINISH_EVENT {
			self.finished = true;
		}
		else {
			_ = self.editable_line.handle_event(event);
		}
	}

	pub(crate) fn set_label(&mut self, label: &str) {
		self.editable_line.set_label(LineSegment::new_with_color_and_style(
			label,
			DisplayColor::Normal,
			LineSegmentOptions::DIMMED,
		));
	}

	pub(crate) fn set_content(&mut self, content: &str) {
		self.editable_line.set_content(content);
	}

	pub(crate) fn reset(&mut self) {
		self.editable_line.clear();
		self.editable_line.set_read_only(false);
		self.finished = false;
	}

	pub(crate) fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	pub(crate) const fn is_finished(&self) -> bool {
		self.finished
	}

	pub(crate) fn get_content(&self) -> &str {
		self.editable_line.get_content()
	}
}
