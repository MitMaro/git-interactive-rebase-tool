mod confirmed;
#[cfg(test)]
mod tests;

pub(crate) use confirmed::Confirmed;
use input::{Event, EventHandler, InputOptions, KeyCode, KeyEvent, MetaEvent};
use lazy_static::lazy_static;
use view::{ViewData, ViewLine};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(true);
}

pub(crate) struct Confirm {
	view_data: ViewData,
}

impl Confirm {
	pub(crate) fn new(prompt: &str, confirm_yes: &[String], confirm_no: &[String]) -> Self {
		let view_data = ViewData::new(|updater| {
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

	#[allow(clippy::unused_self)]
	pub(crate) fn handle_event(&self, event_handler: &EventHandler) -> (Confirmed, Event) {
		let event = event_handler.read_event(&INPUT_OPTIONS, |event, key_bindings| {
			if let Event::Key(key) = event {
				if let KeyCode::Char(c) = key.code {
					let event_lower = Event::Key(KeyEvent::new(KeyCode::Char(c.to_ascii_lowercase()), key.modifiers));
					let event_upper = Event::Key(KeyEvent::new(KeyCode::Char(c.to_ascii_uppercase()), key.modifiers));

					return if key_bindings.confirm_yes.contains(&event_lower)
						|| key_bindings.confirm_yes.contains(&event_upper)
					{
						Event::from(MetaEvent::Yes)
					}
					else {
						Event::from(MetaEvent::No)
					};
				}
			}
			event
		});
		let confirmed = if let Event::Meta(meta_event) = event {
			match meta_event {
				MetaEvent::Yes => Confirmed::Yes,
				MetaEvent::No => Confirmed::No,
				_ => Confirmed::Other,
			}
		}
		else {
			Confirmed::Other
		};
		(confirmed, event)
	}
}
