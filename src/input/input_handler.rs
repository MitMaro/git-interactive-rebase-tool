use crate::config::key_bindings::KeyBindings;
use crate::display::Display;
use crate::input::utils::curses_input_to_string;
use crate::input::Input;
use pancurses::Input as PancursesInput;

#[derive(Debug, PartialEq)]
pub(crate) enum InputMode {
	Confirm,
	Default,
	List,
	Raw,
	ShowCommit,
}

pub(crate) struct InputHandler<'i> {
	key_bindings: &'i KeyBindings,
	display: &'i Display<'i>,
}

impl<'i> InputHandler<'i> {
	pub(crate) fn new(display: &'i Display<'_>, key_bindings: &'i KeyBindings) -> Self {
		Self { key_bindings, display }
	}

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		let c = self.get_next_input();

		let input = curses_input_to_string(c);

		match mode {
			InputMode::ShowCommit => self.get_show_commit_input(input.as_str()),
			InputMode::Raw => self.get_character(input.as_str()),
			InputMode::List => self.get_list_input(input.as_str()),
			InputMode::Confirm => self.get_confirm(input.as_str()),
			InputMode::Default => self.get_default_input(input.as_str()),
		}
	}

	fn get_default_input(self: &Self, input: &str) -> Input {
		match input {
			i if i == self.key_bindings.move_up.as_str() => Input::MoveCursorUp,
			i if i == self.key_bindings.move_down.as_str() => Input::MoveCursorDown,
			i if i == self.key_bindings.move_left.as_str() => Input::MoveCursorLeft,
			i if i == self.key_bindings.move_right.as_str() => Input::MoveCursorRight,
			i if i == self.key_bindings.move_up_step.as_str() => Input::MoveCursorPageUp,
			i if i == self.key_bindings.move_down_step.as_str() => Input::MoveCursorPageDown,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	fn get_show_commit_input(self: &Self, input: &str) -> Input {
		match input {
			i if i == self.key_bindings.help.as_str() => Input::Help,
			i if i == self.key_bindings.move_up.as_str() => Input::MoveCursorUp,
			i if i == self.key_bindings.move_down.as_str() => Input::MoveCursorDown,
			i if i == self.key_bindings.move_left.as_str() => Input::MoveCursorLeft,
			i if i == self.key_bindings.move_right.as_str() => Input::MoveCursorRight,
			i if i == self.key_bindings.move_up_step.as_str() => Input::MoveCursorPageUp,
			i if i == self.key_bindings.move_down_step.as_str() => Input::MoveCursorPageDown,
			i if i == self.key_bindings.show_diff.as_str() => Input::ShowDiff,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	#[allow(clippy::cognitive_complexity)]
	fn get_list_input(self: &Self, input: &str) -> Input {
		match input {
			i if i == self.key_bindings.abort.as_str() => Input::Abort,
			i if i == self.key_bindings.rebase.as_str() => Input::Rebase,
			i if i == self.key_bindings.force_abort.as_str() => Input::ForceAbort,
			i if i == self.key_bindings.force_rebase.as_str() => Input::ForceRebase,
			i if i == self.key_bindings.open_in_external_editor.as_str() => Input::OpenInEditor,
			i if i == self.key_bindings.show_commit.as_str() => Input::ShowCommit,
			i if i == self.key_bindings.edit.as_str() => Input::Edit,
			i if i == self.key_bindings.help.as_str() => Input::Help,
			i if i == self.key_bindings.toggle_visual_mode.as_str() => Input::ToggleVisualMode,
			i if i == self.key_bindings.action_break.as_str() => Input::ActionBreak,
			i if i == self.key_bindings.action_drop.as_str() => Input::ActionDrop,
			i if i == self.key_bindings.action_edit.as_str() => Input::ActionEdit,
			i if i == self.key_bindings.action_fixup.as_str() => Input::ActionFixup,
			i if i == self.key_bindings.action_pick.as_str() => Input::ActionPick,
			i if i == self.key_bindings.action_reword.as_str() => Input::ActionReword,
			i if i == self.key_bindings.action_squash.as_str() => Input::ActionSquash,
			i if i == self.key_bindings.move_up.as_str() => Input::MoveCursorUp,
			i if i == self.key_bindings.move_down.as_str() => Input::MoveCursorDown,
			i if i == self.key_bindings.move_left.as_str() => Input::MoveCursorLeft,
			i if i == self.key_bindings.move_right.as_str() => Input::MoveCursorRight,
			i if i == self.key_bindings.move_up_step.as_str() => Input::MoveCursorPageUp,
			i if i == self.key_bindings.move_down_step.as_str() => Input::MoveCursorPageDown,
			i if i == self.key_bindings.move_selection_down.as_str() => Input::SwapSelectedDown,
			i if i == self.key_bindings.move_selection_up.as_str() => Input::SwapSelectedUp,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	fn get_confirm(&self, input: &str) -> Input {
		let input = input.to_lowercase();
		match input.as_str() {
			i if i == self.key_bindings.confirm_yes.to_lowercase() => Input::Yes,
			"resize" => Input::Resize,
			_ => Input::No,
		}
	}

	fn get_character(&self, input: &str) -> Input {
		match input {
			c if c == "\n" => Input::Enter,
			c if c == "Enter" => Input::Enter,
			c if c == "Backspace" => Input::Backspace,
			c if c == "Delete" => Input::Delete,
			c if c == "Right" => Input::MoveCursorRight,
			c if c == "Left" => Input::MoveCursorLeft,
			c if c == "Resize" => Input::Resize,
			c if c == "Other" => Input::Other,
			c => Input::Character(c.chars().next().unwrap()),
		}
	}

	fn get_next_input(&self) -> PancursesInput {
		loop {
			let c = self.display.getch();
			// technically this will never be None with delay mode
			if let Some(input) = c {
				break input;
			}
		}
	}
}
