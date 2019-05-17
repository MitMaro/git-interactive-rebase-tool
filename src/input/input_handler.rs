use crate::input::Input;
use crate::window::Window;
use pancurses::Input as PancursesInput;

pub struct InputHandler<'i> {
	window: &'i Window<'i>,
}

impl<'i> InputHandler<'i> {
	pub fn new(window: &'i Window) -> Self {
		Self { window }
	}

	pub fn get_input(&self) -> Input {
		// ignore None's, since they are not really valid input
		let c = loop {
			let c = self.window.getch();
			if c.is_some() {
				break c.unwrap();
			}
		};

		match c {
			PancursesInput::Character(c) if c == '?' => Input::Help,
			PancursesInput::Character(c) if c == 'c' => Input::ShowCommit,
			PancursesInput::Character(c) if c == 'q' => Input::Abort,
			PancursesInput::Character(c) if c == 'Q' => Input::ForceAbort,
			PancursesInput::Character(c) if c == 'w' => Input::Rebase,
			PancursesInput::Character(c) if c == 'W' => Input::ForceRebase,
			PancursesInput::Character(c) if c == 'p' => Input::ActionPick,
			PancursesInput::Character(c) if c == 'b' => Input::ActionBreak,
			PancursesInput::Character(c) if c == 'r' => Input::ActionReword,
			PancursesInput::Character(c) if c == 'e' => Input::ActionEdit,
			PancursesInput::Character(c) if c == 's' => Input::ActionSquash,
			PancursesInput::Character(c) if c == 'f' => Input::ActionFixup,
			PancursesInput::Character(c) if c == 'd' => Input::ActionDrop,
			PancursesInput::Character(c) if c == 'E' => Input::Edit,
			PancursesInput::Character(c) if c == 'v' => Input::ToggleVisualMode,
			PancursesInput::Character(c) if c == 'j' => Input::SwapSelectedDown,
			PancursesInput::Character(c) if c == 'k' => Input::SwapSelectedUp,
			PancursesInput::KeyDown => Input::MoveCursorDown,
			PancursesInput::KeyUp => Input::MoveCursorUp,
			PancursesInput::KeyPPage => Input::MoveCursorPageUp,
			PancursesInput::KeyNPage => Input::MoveCursorPageDown,
			PancursesInput::KeyResize => Input::Resize,
			PancursesInput::Character(c) if c == '!' => Input::OpenInEditor,
			_ => Input::Other,
		}
	}

	pub fn get_confirm(&self) -> Input {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) if c == 'y' || c == 'Y' => Input::Yes,
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
