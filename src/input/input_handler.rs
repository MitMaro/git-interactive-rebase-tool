use crate::config::key_bindings::KeyBindings;
use crate::display::curses::Input as CursesInput;
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
}

impl<'i> InputHandler<'i> {
	pub(crate) const fn new(key_bindings: &'i KeyBindings) -> Self {
		Self { key_bindings }
	}

	pub(crate) fn get_input(&self, mode: InputMode, event: CursesInput) -> Input {
		let input = match event {
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
			CursesInput::KeyExit => String::from("Exit"),
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
			InputMode::Default => Self::get_default_input(input.as_str()),
			InputMode::List => self.get_list_input(input.as_str()),
			InputMode::Raw => Self::get_raw_input(input.as_str()),
			InputMode::ShowCommit => self.get_show_commit_input(input.as_str()),
		}
	}

	fn get_standard_inputs(input: &str) -> Option<Input> {
		Some(match input {
			"Up" => Input::ScrollUp,
			"Down" => Input::ScrollDown,
			"Left" => Input::ScrollLeft,
			"Right" => Input::ScrollRight,
			"PageUp" => Input::ScrollJumpUp,
			"PageDown" => Input::ScrollJumpDown,
			"Home" => Input::ScrollTop,
			"End" => Input::ScrollBottom,
			"Exit" => Input::Exit,
			"Resize" => Input::Resize,
			_ => return None,
		})
	}

	fn get_confirm(&self, input: &str) -> Input {
		Self::get_standard_inputs(input).unwrap_or_else(|| {
			match input {
				c if c.to_lowercase() == self.key_bindings.confirm_yes.to_lowercase() => Input::Yes,
				_ => Input::No,
			}
		})
	}

	fn get_default_input(input: &str) -> Input {
		Self::get_standard_inputs(input).unwrap_or_else(|| Self::get_raw_input(input))
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
			"Exit" => Input::Exit,
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
			c if c == "Exit" => Input::Exit,
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
		Self::get_standard_inputs(input).unwrap_or_else(|| {
			match input {
				i if i == self.key_bindings.help.as_str() => Input::Help,
				i if i == self.key_bindings.show_diff.as_str() => Input::ShowDiff,
				_ => Input::Other,
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::Config;
	use rstest::rstest;
	use std::env::set_var;
	use std::path::Path;

	fn input_handler_test<C>(callback: C)
	where C: for<'p> FnOnce(&'p InputHandler<'_>) {
		let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap()
			.to_string();

		set_var("GIT_DIR", git_repo_dir.as_str());
		let config = Config::new().unwrap();
		let input_handler = InputHandler::new(&config.key_bindings);
		callback(&input_handler);
	}

	#[rstest(
		input,
		expected,
		case::yes_lower(CursesInput::Character('y'), Input::Yes),
		case::yes_upper(CursesInput::Character('Y'), Input::Yes),
		case::no_n_lower(CursesInput::Character('n'), Input::No),
		case::no_n_upper(CursesInput::Character('N'), Input::No),
		case::no_other(CursesInput::KeyEOL, Input::No),
		case::standard_resize(CursesInput::KeyResize, Input::Resize),
		case::standard_move_up(CursesInput::KeyUp, Input::ScrollUp),
		case::standard_move_down(CursesInput::KeyDown, Input::ScrollDown),
		case::standard_move_left(CursesInput::KeyLeft, Input::ScrollLeft),
		case::standard_move_right(CursesInput::KeyRight, Input::ScrollRight),
		case::standard_move_jump_up(CursesInput::KeyPPage, Input::ScrollJumpUp),
		case::standard_move_jump_down(CursesInput::KeyNPage, Input::ScrollJumpDown),
		case::exit(CursesInput::KeyExit, Input::Exit)
	)]
	#[serial_test::serial]
	fn confirm_mode(input: CursesInput, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Confirm, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::character(CursesInput::Character('a'), Input::Character('a')),
		case::tab_character(CursesInput::Character('\t'), Input::Tab),
		case::backspace_key(CursesInput::KeyBackspace, Input::Backspace),
		case::backspace_character(CursesInput::Character('\u{7f}'), Input::Backspace),
		case::enter(CursesInput::KeyEnter, Input::Enter),
		case::newline(CursesInput::Character('\n'), Input::Enter),
		case::other(CursesInput::KeyEOL, Input::Other),
		case::standard_resize(CursesInput::KeyResize, Input::Resize),
		case::standard_move_up(CursesInput::KeyUp, Input::ScrollUp),
		case::standard_move_down(CursesInput::KeyDown, Input::ScrollDown),
		case::standard_move_left(CursesInput::KeyLeft, Input::ScrollLeft),
		case::standard_move_right(CursesInput::KeyRight, Input::ScrollRight),
		case::standard_move_jump_up(CursesInput::KeyPPage, Input::ScrollJumpUp),
		case::standard_move_jump_down(CursesInput::KeyNPage, Input::ScrollJumpDown),
		case::exit(CursesInput::KeyExit, Input::Exit)
	)]
	#[serial_test::serial]
	fn default_mode(input: CursesInput, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Default, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::abort(CursesInput::Character('q'), Input::Abort),
		case::rebase(CursesInput::Character('w'), Input::Rebase),
		case::force_abort(CursesInput::Character('Q'), Input::ForceAbort),
		case::force_rebase(CursesInput::Character('W'), Input::ForceRebase),
		case::open_in_external_editor(CursesInput::Character('!'), Input::OpenInEditor),
		case::show_commit(CursesInput::Character('c'), Input::ShowCommit),
		case::edit(CursesInput::Character('E'), Input::Edit),
		case::help(CursesInput::Character('?'), Input::Help),
		case::toggle_visual_mode(CursesInput::Character('v'), Input::ToggleVisualMode),
		case::action_break(CursesInput::Character('b'), Input::ActionBreak),
		case::action_drop(CursesInput::Character('d'), Input::ActionDrop),
		case::action_edit(CursesInput::Character('e'), Input::ActionEdit),
		case::action_fixup(CursesInput::Character('f'), Input::ActionFixup),
		case::action_pick(CursesInput::Character('p'), Input::ActionPick),
		case::action_reword(CursesInput::Character('r'), Input::ActionReword),
		case::action_squash(CursesInput::Character('s'), Input::ActionSquash),
		case::move_up(CursesInput::KeyUp, Input::MoveCursorUp),
		case::move_down(CursesInput::KeyDown, Input::MoveCursorDown),
		case::move_left(CursesInput::KeyLeft, Input::MoveCursorLeft),
		case::move_right(CursesInput::KeyRight, Input::MoveCursorRight),
		case::move_page_up(CursesInput::KeyPPage, Input::MoveCursorPageUp),
		case::move_page_down(CursesInput::KeyNPage, Input::MoveCursorPageDown),
		case::swap_selected_down(CursesInput::Character('j'), Input::SwapSelectedDown),
		case::swap_selected_up(CursesInput::Character('k'), Input::SwapSelectedUp),
		case::resize(CursesInput::KeyResize, Input::Resize),
		case::other(CursesInput::Character('z'), Input::Other),
		case::exit(CursesInput::KeyExit, Input::Exit)
	)]
	#[serial_test::serial]
	fn list_mode(input: CursesInput, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::List, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::tab_character(CursesInput::Character('\t'), Input::Tab),
		case::newline(CursesInput::Character('\n'), Input::Enter),
		case::backspace_character(CursesInput::Character('\u{7f}'), Input::Backspace),
		case::character(CursesInput::Character('a'), Input::Character('a')),
		case::backspace_key(CursesInput::KeyBackspace, Input::Backspace),
		case::btab_key(CursesInput::KeyBTab, Input::ShiftTab),
		case::dc_key(CursesInput::KeyDC, Input::Delete),
		case::down_key(CursesInput::KeyDown, Input::Down),
		case::end_key(CursesInput::KeyEnd, Input::End),
		case::enter_key(CursesInput::KeyEnter, Input::Enter),
		case::f0_key(CursesInput::KeyF0, Input::F0),
		case::f1_key(CursesInput::KeyF1, Input::F1),
		case::f2_key(CursesInput::KeyF2, Input::F2),
		case::f3_key(CursesInput::KeyF3, Input::F3),
		case::f4_key(CursesInput::KeyF4, Input::F4),
		case::f5_key(CursesInput::KeyF5, Input::F5),
		case::f6_key(CursesInput::KeyF6, Input::F6),
		case::f7_key(CursesInput::KeyF7, Input::F7),
		case::f8_key(CursesInput::KeyF8, Input::F8),
		case::f9_key(CursesInput::KeyF9, Input::F9),
		case::f10_key(CursesInput::KeyF10, Input::F10),
		case::f11_key(CursesInput::KeyF11, Input::F11),
		case::f12_key(CursesInput::KeyF12, Input::F12),
		case::f13_key(CursesInput::KeyF13, Input::F13),
		case::f14_key(CursesInput::KeyF14, Input::F14),
		case::f15_key(CursesInput::KeyF15, Input::F15),
		case::home_key(CursesInput::KeyHome, Input::Home),
		case::ic_key(CursesInput::KeyIC, Input::Insert),
		case::left_key(CursesInput::KeyLeft, Input::Left),
		case::npage_key(CursesInput::KeyNPage, Input::PageDown),
		case::ppage_key(CursesInput::KeyPPage, Input::PageUp),
		case::resize_key(CursesInput::KeyResize, Input::Resize),
		case::right_key(CursesInput::KeyRight, Input::Right),
		case::sdc_key(CursesInput::KeySDC, Input::ShiftDelete),
		case::send_key(CursesInput::KeySEnd, Input::ShiftEnd),
		case::sf_key(CursesInput::KeySF, Input::ShiftDown),
		case::shome_key(CursesInput::KeySHome, Input::ShiftHome),
		case::sleft_key(CursesInput::KeySLeft, Input::ShiftLeft),
		case::snext_key(CursesInput::KeySNext, Input::ShiftPageDown),
		case::sprevious_key(CursesInput::KeySPrevious, Input::ShiftPageUp),
		case::sr_key(CursesInput::KeySR, Input::ShiftUp),
		case::sright_key(CursesInput::KeySRight, Input::ShiftRight),
		case::up_key(CursesInput::KeyUp, Input::Up),
		case::print_key(CursesInput::KeyPrint, Input::Print),
		case::sprint_key(CursesInput::KeySPrint, Input::ShiftPrint),
		case::a1_key(CursesInput::KeyA1, Input::KeypadUpperLeft),
		case::a3_key(CursesInput::KeyA3, Input::KeypadUpperRight),
		case::b2_key(CursesInput::KeyB2, Input::KeypadCenter),
		case::c1_key(CursesInput::KeyC1, Input::KeypadLowerLeft),
		case::c3_key(CursesInput::KeyC3, Input::KeypadLowerRight),
		case::exit(CursesInput::KeyExit, Input::Exit)
	)]
	#[serial_test::serial]
	fn raw_mode(input: CursesInput, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Raw, input), expected);
		});
	}

	#[rstest(
		input => [
			CursesInput::Unknown(0),
			CursesInput::KeyDL,
			CursesInput::KeyIL,
			CursesInput::KeyClear,
			CursesInput::KeyCodeYes,
			CursesInput::KeyBreak,
			CursesInput::KeyEIC,
			CursesInput::KeyEOS,
			CursesInput::KeyEOL,
			CursesInput::KeySTab,
			CursesInput::KeyCTab,
			CursesInput::KeyCATab,
			CursesInput::KeySReset,
			CursesInput::KeyReset,
			CursesInput::KeyLL,
			CursesInput::KeyAbort,
			CursesInput::KeySHelp,
			CursesInput::KeyLHelp,
			CursesInput::KeyBeg,
			CursesInput::KeyCancel,
			CursesInput::KeyClose,
			CursesInput::KeyCommand,
			CursesInput::KeyCopy,
			CursesInput::KeyCreate,
			CursesInput::KeyFind,
			CursesInput::KeyHelp,
			CursesInput::KeyMark,
			CursesInput::KeyMessage,
			CursesInput::KeyMove,
			CursesInput::KeyNext,
			CursesInput::KeyOpen,
			CursesInput::KeyOptions,
			CursesInput::KeyPrevious,
			CursesInput::KeyRedo,
			CursesInput::KeyReference,
			CursesInput::KeyRefresh,
			CursesInput::KeyReplace,
			CursesInput::KeyRestart,
			CursesInput::KeyResume,
			CursesInput::KeySave,
			CursesInput::KeySBeg,
			CursesInput::KeySCancel,
			CursesInput::KeySCommand,
			CursesInput::KeySCopy,
			CursesInput::KeySCreate,
			CursesInput::KeySDL,
			CursesInput::KeySelect,
			CursesInput::KeySEOL,
			CursesInput::KeySExit,
			CursesInput::KeySFind,
			CursesInput::KeySIC,
			CursesInput::KeySMessage,
			CursesInput::KeySMove,
			CursesInput::KeySOptions,
			CursesInput::KeySRedo,
			CursesInput::KeySReplace,
			CursesInput::KeySResume,
			CursesInput::KeySSave,
			CursesInput::KeySSuspend,
			CursesInput::KeySUndo,
			CursesInput::KeySuspend,
			CursesInput::KeyUndo,
			CursesInput::KeyEvent,
			CursesInput::KeyMouse,
		],
	)]
	#[serial_test::serial]
	fn raw_mode_unsupported(input: CursesInput) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Raw, input), Input::Other);
		});
	}

	#[rstest(
		input,
		expected,
		case::help(CursesInput::Character('?'), Input::Help),
		case::newline(CursesInput::Character('d'), Input::ShowDiff),
		case::other(CursesInput::KeyEOL, Input::Other),
		case::standard_resize(CursesInput::KeyResize, Input::Resize),
		case::standard_move_up(CursesInput::KeyUp, Input::ScrollUp),
		case::standard_move_down(CursesInput::KeyDown, Input::ScrollDown),
		case::standard_move_left(CursesInput::KeyLeft, Input::ScrollLeft),
		case::standard_move_right(CursesInput::KeyRight, Input::ScrollRight),
		case::standard_move_jump_up(CursesInput::KeyPPage, Input::ScrollJumpUp),
		case::standard_move_jump_down(CursesInput::KeyNPage, Input::ScrollJumpDown),
		case::exit(CursesInput::KeyExit, Input::Exit)
	)]
	#[serial_test::serial]
	fn confirm_input_mode(input: CursesInput, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::ShowCommit, input), expected);
		});
	}
}
