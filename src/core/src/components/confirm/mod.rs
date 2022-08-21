mod confirmed;
#[cfg(test)]
mod tests;

use captur::capture;
pub(crate) use confirmed::Confirmed;
use input::{InputOptions, KeyCode, KeyEvent, KeyModifiers};
use lazy_static::lazy_static;
use view::{ViewData, ViewLine};

use crate::events::{Event, KeyBindings, MetaEvent};

lazy_static! {
	pub static ref INPUT_OPTIONS: InputOptions = InputOptions::RESIZE | InputOptions::MOVEMENT;
}

pub(crate) struct Confirm {
	view_data: ViewData,
}

impl Confirm {
	pub(crate) fn new(prompt: &str, confirm_yes: &[String], confirm_no: &[String]) -> Self {
		let view_data = ViewData::new(|updater| {
			capture!(confirm_yes, confirm_no);
			updater.set_show_title(true);
			updater.set_retain_scroll_position(false);
			updater.push_line(ViewLine::from(format!(
				"{} ({}/{})? ",
				prompt,
				confirm_yes.join(","),
				confirm_no.join(",")
			)));
		});
		Self { view_data }
	}

	pub(crate) fn get_view_data(&mut self) -> &ViewData {
		&self.view_data
	}

	pub(crate) fn read_event(event: Event, key_bindings: &KeyBindings) -> Event {
		if let Event::Key(key) = event {
			if let KeyCode::Char(c) = key.code {
				let mut event_lower_modifiers = key.modifiers;
				event_lower_modifiers.remove(KeyModifiers::SHIFT);
				let event_lower = Event::Key(KeyEvent::new(
					KeyCode::Char(c.to_ascii_lowercase()),
					event_lower_modifiers,
				));
				let event_upper = Event::Key(KeyEvent::new(KeyCode::Char(c.to_ascii_uppercase()), key.modifiers));

				return if key_bindings.custom.confirm_yes.contains(&event_lower)
					|| key_bindings.custom.confirm_yes.contains(&event_upper)
				{
					Event::from(MetaEvent::Yes)
				}
				else {
					Event::from(MetaEvent::No)
				};
			}
		}
		event
	}

	#[allow(clippy::unused_self)]
	pub(crate) const fn handle_event(&self, event: Event) -> Confirmed {
		if let Event::MetaEvent(meta_event) = event {
			match meta_event {
				MetaEvent::Yes => Confirmed::Yes,
				MetaEvent::No => Confirmed::No,
				_ => Confirmed::Other,
			}
		}
		else {
			Confirmed::Other
		}
	}
}
