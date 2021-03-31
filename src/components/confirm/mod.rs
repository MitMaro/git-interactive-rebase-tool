#[cfg(test)]
mod tests;

use crate::{
	input::Input,
	process::util::handle_view_data_scroll,
	view::{view_data::ViewData, view_line::ViewLine},
};

pub struct Confirm {
	view_data: ViewData,
}

impl Confirm {
	pub fn new(prompt: &str, confirm_yes: &[String], confirm_no: &[String]) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		view_data.push_line(ViewLine::from(format!(
			"{} ({}/{})? ",
			prompt,
			confirm_yes.join(","),
			confirm_no.join(",")
		)));
		Self { view_data }
	}

	pub fn get_view_data(&mut self) -> &mut ViewData {
		&mut self.view_data
	}

	pub fn handle_input(&mut self, input: Input) -> Option<bool> {
		if handle_view_data_scroll(input, &mut self.view_data).is_none() {
			match input {
				Input::Yes => Some(true),
				Input::No => Some(false),
				_ => None,
			}
		}
		else {
			None
		}
	}
}
