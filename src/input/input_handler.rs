use crate::{
	config::key_bindings::KeyBindings,
	display::{Event, KeyCode, KeyModifiers, MouseEventKind},
	input::Input,
};

fn modifiers_to_string(modifiers: KeyModifiers, code: Option<KeyCode>) -> String {
	let mut result = vec![];

	if modifiers.contains(KeyModifiers::SHIFT) {
		if let Some(KeyCode::Char(k)) = code {
			if k == '\t' || k == '\n' || k == '\u{7f}' {
				result.push(String::from("Shift"))
			}
		}
		else {
			result.push(String::from("Shift"))
		}
	}
	if modifiers.contains(KeyModifiers::CONTROL) {
		result.push(String::from("Control"))
	}
	if modifiers.contains(KeyModifiers::ALT) {
		result.push(String::from("Alt"))
	}
	result.join("")
}

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

	pub(crate) fn get_input(&self, mode: InputMode, event: Event) -> Input {
		let input = match event {
			Event::Key(event) => {
				let code = format!(
					"{}{}",
					modifiers_to_string(event.modifiers, Some(event.code)),
					match event.code {
						KeyCode::Backspace => String::from("Backspace"),
						KeyCode::BackTab => String::from("BackTab"),
						KeyCode::Delete => String::from("Delete"),
						KeyCode::Down => String::from("Down"),
						KeyCode::End => String::from("End"),
						KeyCode::Enter => String::from("Enter"),
						KeyCode::Esc => String::from("Esc"),
						KeyCode::F(i) => format!("F{}", i),
						KeyCode::Home => String::from("Home"),
						KeyCode::Insert => String::from("Insert"),
						KeyCode::Left => String::from("Left"),
						KeyCode::Null => String::from("Other"),
						KeyCode::PageDown => String::from("PageDown"),
						KeyCode::PageUp => String::from("PageUp"),
						KeyCode::Right => String::from("Right"),
						KeyCode::Tab => String::from("Tab"),
						KeyCode::Up => String::from("Up"),
						KeyCode::Char(c) if c == '\t' => String::from("Tab"),
						KeyCode::Char(c) if c == '\n' => String::from("Enter"),
						KeyCode::Char(c) if c == '\u{7f}' => String::from("Backspace"),
						KeyCode::Char(c) => c.to_string(),
					}
				);

				match code.as_str() {
					"Controlc" => String::from("Kill"),
					"Controld" => String::from("Exit"),
					_ => code,
				}
			},
			Event::Mouse(event) => {
				format!("{}{}", modifiers_to_string(event.modifiers, None), match event.kind {
					MouseEventKind::ScrollDown => String::from("Down"),
					MouseEventKind::ScrollUp => String::from("Up"),
					_ => String::from("Ignore"),
				})
			},
			Event::Resize(..) => String::from("Resize"),
		};

		// this is a hack to work around unhandled mouse events, input handling needs to be changed
		// to properly handle dynamic inputs like mouse events
		// TODO remove hack
		if input == "Ignore" {
			return Input::Ignore;
		}

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
			"Kill" => Input::Kill,
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
			"Kill" => Input::Kill,
			"Resize" => Input::Resize,
			_ => Input::Other,
		}
	}

	#[allow(clippy::cognitive_complexity)]
	fn get_raw_input(input: &str) -> Input {
		match input {
			c if c == "Backspace" => Input::Backspace,
			c if c == "BackTab" => Input::BackTab,
			c if c == "Delete" => Input::Delete,
			c if c == "Down" => Input::Down,
			c if c == "End" => Input::End,
			c if c == "Enter" => Input::Enter,
			c if c == "Esc" => Input::Escape,
			c if c == "Exit" => Input::Exit,
			c if c == "Home" => Input::Home,
			c if c == "Insert" => Input::Insert,
			c if c == "Kill" => Input::Kill,
			c if c == "Left" => Input::Left,
			c if c == "Other" => Input::Other,
			c if c == "PageDown" => Input::PageDown,
			c if c == "PageUp" => Input::PageUp,
			c if c == "Resize" => Input::Resize,
			c if c == "Right" => Input::Right,
			c if c == "Tab" => Input::Tab,
			c if c == "Up" => Input::Up,
			c => {
				if c.chars().count() == 1 {
					Input::Character(c.chars().next().unwrap())
				}
				else {
					Input::Other
				}
			},
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
	use std::{env::set_var, path::Path};

	use crossterm::event::MouseEvent;
	use rstest::rstest;

	use super::*;
	use crate::{config::Config, create_key_event, create_mouse_event};

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

	#[test]
	fn modifiers_to_string_no_modifiers() {
		assert_eq!(modifiers_to_string(KeyModifiers::NONE, None), "");
	}

	#[test]
	fn modifiers_to_string_alt() {
		assert_eq!(modifiers_to_string(KeyModifiers::ALT, None), "Alt");
	}

	#[test]
	fn modifiers_to_string_control() {
		assert_eq!(modifiers_to_string(KeyModifiers::CONTROL, None), "Control");
	}

	#[test]
	fn modifiers_to_string_shift() {
		assert_eq!(modifiers_to_string(KeyModifiers::SHIFT, None), "Shift");
	}

	#[test]
	fn modifiers_to_string_combined() {
		assert_eq!(modifiers_to_string(KeyModifiers::all(), None), "ShiftControlAlt");
	}

	#[test]
	fn modifiers_to_string_with_code_char() {
		assert_eq!(modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('A'))), "");
	}

	#[test]
	fn modifiers_to_string_with_code_char_tab() {
		assert_eq!(
			modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\t'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_newline() {
		assert_eq!(
			modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\n'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_backspace() {
		assert_eq!(
			modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\u{7f}'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_other() {
		assert_eq!(modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Enter)), "Shift");
	}

	#[test]
	fn modifiers_to_string_with_code_alphabetic_combined() {
		assert_eq!(
			modifiers_to_string(KeyModifiers::all(), Some(KeyCode::Char('A'))),
			"ControlAlt"
		);
	}

	#[test]
	#[serial_test::serial]
	fn ignore_hack() {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(
				input_handler.get_input(
					InputMode::Confirm,
					Event::Mouse(MouseEvent {
						kind: MouseEventKind::Moved,
						column: 0,
						row: 0,
						modifiers: KeyModifiers::NONE
					})
				),
				Input::Ignore
			);
		});
	}
	#[rstest(
		input,
		expected,
		case::yes_lower(create_key_event!('y'), Input::Yes),
		case::yes_upper(create_key_event!('Y'), Input::Yes),
		case::no_n_lower(create_key_event!('n'), Input::No),
		case::no_n_upper(create_key_event!('N'), Input::No),
		case::no_other(create_key_event!(code KeyCode::Null), Input::No),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill),
		case::exit(create_key_event!('d', "Control"), Input::Exit)
	)]
	#[serial_test::serial]
	fn confirm_mode(input: Event, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Confirm, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::character(create_key_event!('a'), Input::Character('a')),
		case::tab_character(create_key_event!('\t'), Input::Tab),
		case::tab_key_code(create_key_event!(code KeyCode::Tab), Input::Tab),
		case::backspace_key(create_key_event!(code KeyCode::Backspace), Input::Backspace),
		case::backspace_character(create_key_event!('\u{7f}'), Input::Backspace),
		case::enter(create_key_event!(code KeyCode::Enter), Input::Enter),
		case::newline(create_key_event!('\n'), Input::Enter),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill),
		case::esc(create_key_event!(code KeyCode::Esc), Input::Escape),
		case::mouse_down(create_mouse_event!(MouseEventKind::ScrollDown), Input::ScrollDown),
		case::mouse_up(create_mouse_event!(MouseEventKind::ScrollUp), Input::ScrollUp)

	)]
	#[serial_test::serial]
	fn default_mode(input: Event, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Default, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::abort(create_key_event!('q'), Input::Abort),
		case::rebase(create_key_event!('w'), Input::Rebase),
		case::force_abort(create_key_event!('Q'), Input::ForceAbort),
		case::force_rebase(create_key_event!('W'), Input::ForceRebase),
		case::open_in_external_editor(create_key_event!('!'), Input::OpenInEditor),
		case::show_commit(create_key_event!('c'), Input::ShowCommit),
		case::edit(create_key_event!('E'), Input::Edit),
		case::help(create_key_event!('?'), Input::Help),
		case::toggle_visual_mode(create_key_event!('v'), Input::ToggleVisualMode),
		case::action_break(create_key_event!('b'), Input::ActionBreak),
		case::action_drop(create_key_event!('d'), Input::ActionDrop),
		case::action_edit(create_key_event!('e'), Input::ActionEdit),
		case::action_fixup(create_key_event!('f'), Input::ActionFixup),
		case::action_pick(create_key_event!('p'), Input::ActionPick),
		case::action_reword(create_key_event!('r'), Input::ActionReword),
		case::action_squash(create_key_event!('s'), Input::ActionSquash),
		case::move_up(create_key_event!(code KeyCode::Up), Input::MoveCursorUp),
		case::move_down(create_key_event!(code KeyCode::Down), Input::MoveCursorDown),
		case::move_left(create_key_event!(code KeyCode::Left), Input::MoveCursorLeft),
		case::move_right(create_key_event!(code KeyCode::Right), Input::MoveCursorRight),
		case::move_page_up(create_key_event!(code KeyCode::PageUp), Input::MoveCursorPageUp),
		case::move_page_down(create_key_event!(code KeyCode::PageDown), Input::MoveCursorPageDown),
		case::swap_selected_down(create_key_event!('j'), Input::SwapSelectedDown),
		case::swap_selected_up(create_key_event!('k'), Input::SwapSelectedUp),
		case::resize(Event::Resize(0, 0), Input::Resize),
		case::other(create_key_event!('z'), Input::Other),
		case::exit(create_key_event!('d', "Control"), Input::Exit),
		case::exit(create_key_event!('c', "Control"), Input::Kill),
	)]
	#[serial_test::serial]
	fn list_mode(input: Event, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::List, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::backspace_character(create_key_event!(code KeyCode::Backspace), Input::Backspace),
		case::backtab_key(create_key_event!(code KeyCode::BackTab), Input::BackTab),
		case::delete_key(create_key_event!(code KeyCode::Delete), Input::Delete),
		case::down_key(create_key_event!(code KeyCode::Down), Input::Down),
		case::end_key(create_key_event!(code KeyCode::End), Input::End),
		case::enter_key(create_key_event!(code KeyCode::Enter), Input::Enter),
		case::exit_key(create_key_event!('d', "Control"), Input::Exit),
		case::home_key(create_key_event!(code KeyCode::Home), Input::Home),
		case::insert_key(create_key_event!(code KeyCode::Insert), Input::Insert),
		case::kill_key(create_key_event!('c', "Control"), Input::Kill),
		case::left_key(create_key_event!(code KeyCode::Left), Input::Left),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::page_down_key(create_key_event!(code KeyCode::PageDown), Input::PageDown),
		case::page_up_key(create_key_event!(code KeyCode::PageUp), Input::PageUp),
		case::resize_key(Event::Resize(0, 0), Input::Resize),
		case::right_key(create_key_event!(code KeyCode::Right), Input::Right),
		case::tab_key(create_key_event!(code KeyCode::Tab), Input::Tab),
		case::up_key(create_key_event!(code KeyCode::Up), Input::Up),
		case::character(create_key_event!('a'), Input::Character('a')),
		case::unknown(create_key_event!(code KeyCode::F(1)), Input::Other)
	)]
	#[serial_test::serial]
	fn raw_mode(input: Event, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::Raw, input), expected);
		});
	}

	#[rstest(
		input,
		expected,
		case::help(create_key_event!('?'), Input::Help),
		case::show_diff(create_key_event!('d'), Input::ShowDiff),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill),
	)]
	#[serial_test::serial]
	fn show_commit_mode(input: Event, expected: Input) {
		input_handler_test(|input_handler: &InputHandler<'_>| {
			assert_eq!(input_handler.get_input(InputMode::ShowCommit, input), expected);
		});
	}
}
