use crate::config::key_bindings::KeyBindings;
use crate::display::curses::Input as CursesInput;
use crate::display::Display;
use crate::input::Input;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputMode {
	Confirm,
	Default,
	List,
	Raw,
	ShowCommit,
}

pub struct InputHandler<'i> {
	key_bindings: &'i KeyBindings,
	display: &'i Display<'i>,
}

impl<'i> InputHandler<'i> {
	pub(crate) const fn new(display: &'i Display<'_>, key_bindings: &'i KeyBindings) -> Self {
		Self { key_bindings, display }
	}

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		let input = match self.get_next_input() {
			CursesInput::Character(c) if c == '\t' => String::from("Tab"),
			CursesInput::Character(c) if c == '\n' => String::from("Enter"),
			CursesInput::Character(c) if c == '\u{7f}' => String::from("Backspace"),
			CursesInput::Character(c) => c.to_string(),
			CursesInput::KeyBackspace => String::from("Backspace"),
			CursesInput::KeyBTab => String::from("ShiftTab"),
			CursesInput::KeyDC => String::from("Delete"),
			CursesInput::KeyDown => String::from("Down"),
			CursesInput::KeyEnd => String::from("End"),
			CursesInput::KeyEnter => String::from("Enter"),
			CursesInput::KeyF0 => String::from("F0"),
			CursesInput::KeyF1 => String::from("F1"),
			CursesInput::KeyF2 => String::from("F2"),
			CursesInput::KeyF3 => String::from("F3"),
			CursesInput::KeyF4 => String::from("F4"),
			CursesInput::KeyF5 => String::from("F5"),
			CursesInput::KeyF6 => String::from("F6"),
			CursesInput::KeyF7 => String::from("F7"),
			CursesInput::KeyF8 => String::from("F8"),
			CursesInput::KeyF9 => String::from("F9"),
			CursesInput::KeyF10 => String::from("F10"),
			CursesInput::KeyF11 => String::from("F11"),
			CursesInput::KeyF12 => String::from("F12"),
			CursesInput::KeyF13 => String::from("F13"),
			CursesInput::KeyF14 => String::from("F14"),
			CursesInput::KeyF15 => String::from("F15"),
			CursesInput::KeyHome => String::from("Home"),
			CursesInput::KeyIC => String::from("Insert"),
			CursesInput::KeyLeft => String::from("Left"),
			CursesInput::KeyNPage => String::from("PageDown"),
			CursesInput::KeyPPage => String::from("PageUp"),
			CursesInput::KeyResize => String::from("Resize"),
			CursesInput::KeyRight => String::from("Right"),
			CursesInput::KeySDC => String::from("ShiftDelete"),
			CursesInput::KeySEnd => String::from("ShiftEnd"),
			CursesInput::KeySF => String::from("ShiftDown"),
			CursesInput::KeySHome => String::from("ShiftHome"),
			CursesInput::KeySLeft => String::from("ShiftLeft"),
			CursesInput::KeySNext => String::from("ShiftPageDown"),
			CursesInput::KeySPrevious => String::from("ShiftPageUp"),
			CursesInput::KeySR => String::from("ShiftUp"),
			CursesInput::KeySRight => String::from("ShiftRight"),
			CursesInput::KeyUp => String::from("Up"),
			CursesInput::KeyPrint => String::from("Print"),
			CursesInput::KeySPrint => String::from("ShiftPrint"),
			CursesInput::KeyA1 => String::from("KeypadUpperLeft"),
			CursesInput::KeyA3 => String::from("KeypadUpperRight"),
			CursesInput::KeyB2 => String::from("KeypadCenter"),
			CursesInput::KeyC1 => String::from("KeypadLowerLeft"),
			CursesInput::KeyC3 => String::from("KeypadLowerRight"),
			CursesInput::Unknown(_)
			| CursesInput::KeyDL
			| CursesInput::KeyIL
			| CursesInput::KeyClear
			| CursesInput::KeyCodeYes
			| CursesInput::KeyBreak
			| CursesInput::KeyEIC
			| CursesInput::KeyEOS
			| CursesInput::KeyEOL
			| CursesInput::KeySTab
			| CursesInput::KeyCTab
			| CursesInput::KeyCATab
			| CursesInput::KeySReset
			| CursesInput::KeyReset
			| CursesInput::KeyLL
			| CursesInput::KeyAbort
			| CursesInput::KeySHelp
			| CursesInput::KeyLHelp
			| CursesInput::KeyBeg
			| CursesInput::KeyCancel
			| CursesInput::KeyClose
			| CursesInput::KeyCommand
			| CursesInput::KeyCopy
			| CursesInput::KeyCreate
			| CursesInput::KeyExit
			| CursesInput::KeyFind
			| CursesInput::KeyHelp
			| CursesInput::KeyMark
			| CursesInput::KeyMessage
			| CursesInput::KeyMove
			| CursesInput::KeyNext
			| CursesInput::KeyOpen
			| CursesInput::KeyOptions
			| CursesInput::KeyPrevious
			| CursesInput::KeyRedo
			| CursesInput::KeyReference
			| CursesInput::KeyRefresh
			| CursesInput::KeyReplace
			| CursesInput::KeyRestart
			| CursesInput::KeyResume
			| CursesInput::KeySave
			| CursesInput::KeySBeg
			| CursesInput::KeySCancel
			| CursesInput::KeySCommand
			| CursesInput::KeySCopy
			| CursesInput::KeySCreate
			| CursesInput::KeySDL
			| CursesInput::KeySelect
			| CursesInput::KeySEOL
			| CursesInput::KeySExit
			| CursesInput::KeySFind
			| CursesInput::KeySIC
			| CursesInput::KeySMessage
			| CursesInput::KeySMove
			| CursesInput::KeySOptions
			| CursesInput::KeySRedo
			| CursesInput::KeySReplace
			| CursesInput::KeySResume
			| CursesInput::KeySSave
			| CursesInput::KeySSuspend
			| CursesInput::KeySUndo
			| CursesInput::KeySuspend
			| CursesInput::KeyUndo
			| CursesInput::KeyEvent
			| CursesInput::KeyMouse => String::from("Other"),
		};

		match mode {
			InputMode::Confirm => self.get_confirm(input.as_str()),
			InputMode::Default => self.get_default_input(input.as_str()),
			InputMode::List => self.get_list_input(input.as_str()),
			InputMode::Raw => Self::get_raw_input(input.as_str()),
			InputMode::ShowCommit => self.get_show_commit_input(input.as_str()),
		}
	}

	fn get_standard_inputs(&self, input: &str) -> Option<Input> {
		Some(match input {
			i if i == self.key_bindings.move_up.as_str() => Input::ScrollUp,
			i if i == self.key_bindings.move_down.as_str() => Input::ScrollDown,
			i if i == self.key_bindings.move_left.as_str() => Input::ScrollLeft,
			i if i == self.key_bindings.move_right.as_str() => Input::ScrollRight,
			i if i == self.key_bindings.move_up_step.as_str() => Input::ScrollJumpUp,
			i if i == self.key_bindings.move_down_step.as_str() => Input::ScrollJumpDown,
			"Up" => Input::ScrollUp,
			"Down" => Input::ScrollDown,
			"Left" => Input::ScrollLeft,
			"Right" => Input::ScrollRight,
			"PageUp" => Input::ScrollJumpUp,
			"PageDown" => Input::ScrollJumpDown,
			"Home" => Input::ScrollTop,
			"End" => Input::ScrollBottom,
			"resize" => Input::Resize,
			_ => return None,
		})
	}

	fn get_confirm(&self, input: &str) -> Input {
		self.get_standard_inputs(input).unwrap_or_else(|| {
			match input {
				"Resize" => Input::Resize,
				c if c.to_lowercase() == self.key_bindings.confirm_yes.to_lowercase() => Input::Yes,
				_ => Input::No,
			}
		})
	}

	fn get_default_input(&self, input: &str) -> Input {
		self.get_standard_inputs(input).unwrap_or_else(|| {
			if input == "Resize" {
				Input::Resize
			}
			else {
				Input::Other
			}
		})
	}

	#[allow(clippy::cognitive_complexity)]
	fn get_list_input(&self, input: &str) -> Input {
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
			"Left" => Input::ScrollLeft,
			"Right" => Input::ScrollRight,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	#[allow(clippy::cognitive_complexity)]
	fn get_raw_input(input: &str) -> Input {
		match input {
			c if c == "Tab" => Input::Tab,
			c if c == "Backspace" => Input::Backspace,
			c if c == "ShiftTab" => Input::ShiftTab,
			c if c == "Delete" => Input::Delete,
			c if c == "Down" => Input::Down,
			c if c == "End" => Input::End,
			c if c == "Enter" => Input::Enter,
			c if c == "F0" => Input::F0,
			c if c == "F1" => Input::F1,
			c if c == "F2" => Input::F2,
			c if c == "F3" => Input::F3,
			c if c == "F4" => Input::F4,
			c if c == "F5" => Input::F5,
			c if c == "F6" => Input::F6,
			c if c == "F7" => Input::F7,
			c if c == "F8" => Input::F8,
			c if c == "F9" => Input::F9,
			c if c == "F10" => Input::F10,
			c if c == "F11" => Input::F11,
			c if c == "F12" => Input::F12,
			c if c == "F13" => Input::F13,
			c if c == "F14" => Input::F14,
			c if c == "F15" => Input::F15,
			c if c == "Home" => Input::Home,
			c if c == "Insert" => Input::Insert,
			c if c == "Left" => Input::Left,
			c if c == "PageDown" => Input::PageDown,
			c if c == "PageUp" => Input::PageUp,
			c if c == "Resize" => Input::Resize,
			c if c == "Right" => Input::Right,
			c if c == "ShiftDelete" => Input::ShiftDelete,
			c if c == "ShiftEnd" => Input::ShiftEnd,
			c if c == "ShiftDown" => Input::ShiftDown,
			c if c == "ShiftHome" => Input::ShiftHome,
			c if c == "ShiftLeft" => Input::ShiftLeft,
			c if c == "ShiftPageDown" => Input::ShiftPageDown,
			c if c == "ShiftPageUp" => Input::ShiftPageUp,
			c if c == "ShiftUp" => Input::ShiftUp,
			c if c == "ShiftRight" => Input::ShiftRight,
			c if c == "Up" => Input::Up,
			c if c == "Print" => Input::Print,
			c if c == "ShiftPrint" => Input::ShiftPrint,
			c if c == "KeypadUpperLeft" => Input::KeypadUpperLeft,
			c if c == "KeypadUpperRight" => Input::KeypadUpperRight,
			c if c == "KeypadCenter" => Input::KeypadCenter,
			c if c == "KeypadLowerLeft" => Input::KeypadLowerLeft,
			c if c == "KeypadLowerRight" => Input::KeypadLowerRight,
			c if c == "Other" => Input::Other,
			c => Input::Character(c.chars().next().unwrap()),
		}
	}

	fn get_show_commit_input(&self, input: &str) -> Input {
		self.get_standard_inputs(input).unwrap_or_else(|| {
			match input {
				i if i == self.key_bindings.help.as_str() => Input::Help,
				i if i == self.key_bindings.show_diff.as_str() => Input::ShowDiff,
				"Resize" => Input::Resize,
				_ => Input::Other,
			}
		})
	}

	fn get_next_input(&self) -> CursesInput {
		loop {
			let c = self.display.getch();
			// technically this will never be None with delay mode
			if let Some(input) = c {
				break input;
			}
		}
	}
}
