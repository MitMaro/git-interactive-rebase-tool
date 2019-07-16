use crate::input::Input;
use crate::window::Window;
use crate::Config;
use pancurses::Input as PancursesInput;

pub struct InputHandler<'i> {
	config: &'i Config,
	confirm_yes_input: char,
	window: &'i Window<'i>,
}

impl<'i> InputHandler<'i> {
	pub fn new(window: &'i Window, config: &'i Config) -> Self {
		let confirm_yes_input = config.input_confirm_yes.to_lowercase().chars().next().unwrap_or('y');
		Self {
			config,
			confirm_yes_input,
			window,
		}
	}

	pub fn get_input(&self) -> Input {
		// ignore None's, since they are not really valid input
		let c = loop {
			let c = self.window.getch();
			if c.is_some() {
				break c.unwrap();
			}
		};

		let input = match c {
			PancursesInput::Character(c) => c.to_string(),
			PancursesInput::KeyDown => String::from("Down"),
			PancursesInput::KeyUp => String::from("Up"),
			PancursesInput::KeyPPage => String::from("PageUp"),
			PancursesInput::KeyNPage => String::from("PageDown"),
			PancursesInput::KeyResize => String::from("Resize"),
			_ => String::from("Other"),
		};

		match input.as_str() {
			i if i == self.config.input_abort.as_str() => Input::Abort,
			i if i == self.config.input_action_break.as_str() => Input::ActionBreak,
			i if i == self.config.input_action_drop.as_str() => Input::ActionDrop,
			i if i == self.config.input_action_drop.as_str() => Input::Help,
			i if i == self.config.input_action_edit.as_str() => Input::ActionEdit,
			i if i == self.config.input_action_fixup.as_str() => Input::ActionFixup,
			i if i == self.config.input_action_pick.as_str() => Input::ActionPick,
			i if i == self.config.input_action_reword.as_str() => Input::ActionReword,
			i if i == self.config.input_action_squash.as_str() => Input::ActionSquash,
			i if i == self.config.input_edit.as_str() => Input::Edit,
			i if i == self.config.input_force_abort.as_str() => Input::ForceAbort,
			i if i == self.config.input_force_rebase.as_str() => Input::ForceRebase,
			i if i == self.config.input_move_down.as_str() => Input::MoveCursorDown,
			i if i == self.config.input_move_selection_down.as_str() => Input::SwapSelectedDown,
			i if i == self.config.input_move_selection_up.as_str() => Input::SwapSelectedUp,
			i if i == self.config.input_move_up.as_str() => Input::MoveCursorUp,
			i if i == self.config.input_open_in_external_editor.as_str() => Input::OpenInEditor,
			i if i == self.config.input_rebase.as_str() => Input::Rebase,
			i if i == self.config.input_show_commit.as_str() => Input::ShowCommit,
			i if i == self.config.input_toggle_visual_mode.as_str() => Input::ToggleVisualMode,
			"PageUp" => Input::MoveCursorPageUp,
			"PageDown" => Input::MoveCursorPageDown,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	pub fn get_confirm(&self) -> Input {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) => {
				if c.to_lowercase().next().unwrap() == self.confirm_yes_input {
					Input::Yes
				}
				else {
					Input::No
				}
			},
			Some(PancursesInput::KeyResize) => Input::Resize,
			_ => Input::No,
		}
	}

	pub fn get_character(&self) -> Input {
		loop {
			let c = loop {
				let c = self.window.getch();
				if c.is_some() {
					break c.unwrap();
				}
			};

			match c {
				PancursesInput::Character(c) if c == '\n' => break Input::Enter,
				PancursesInput::Character(c) => break Input::Character(c),
				PancursesInput::KeyEnter => break Input::Enter,
				PancursesInput::KeyBackspace => break Input::Backspace,
				PancursesInput::KeyDC => break Input::Delete,
				PancursesInput::KeyRight => break Input::MoveCursorRight,
				PancursesInput::KeyLeft => break Input::MoveCursorLeft,
				PancursesInput::KeyResize => break Input::Resize,
				_ => {},
			};
		}
	}
}
