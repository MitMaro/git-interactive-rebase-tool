use crate::{config::key_bindings::KeyBindings, input::Input};

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

	pub(crate) fn get_input(&self, mode: InputMode, input: String) -> Input {
		match mode {
			InputMode::Confirm => self.get_confirm(input.as_str()),
			InputMode::Default => Self::get_default_input(input.as_str()),
			InputMode::List => self.get_list_input(input),
			InputMode::Raw => Self::get_raw_input(input.as_str()),
			InputMode::ShowCommit => self.get_show_commit_input(input),
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
				c if self.key_bindings.confirm_yes.contains(&c.to_lowercase()) => Input::Yes,
				_ => Input::No,
			}
		})
	}

	fn get_default_input(input: &str) -> Input {
		Self::get_standard_inputs(input).unwrap_or_else(|| Self::get_raw_input(input))
	}

	#[allow(clippy::cognitive_complexity)]
	fn get_list_input(&self, input: String) -> Input {
		match input {
			i if self.key_bindings.abort.contains(&i) => Input::Abort,
			i if self.key_bindings.action_break.contains(&i) => Input::ActionBreak,
			i if self.key_bindings.action_drop.contains(&i) => Input::ActionDrop,
			i if self.key_bindings.action_edit.contains(&i) => Input::ActionEdit,
			i if self.key_bindings.action_fixup.contains(&i) => Input::ActionFixup,
			i if self.key_bindings.action_pick.contains(&i) => Input::ActionPick,
			i if self.key_bindings.action_reword.contains(&i) => Input::ActionReword,
			i if self.key_bindings.action_squash.contains(&i) => Input::ActionSquash,
			i if self.key_bindings.edit.contains(&i) => Input::Edit,
			i if self.key_bindings.force_abort.contains(&i) => Input::ForceAbort,
			i if self.key_bindings.force_rebase.contains(&i) => Input::ForceRebase,
			i if self.key_bindings.help.contains(&i) => Input::Help,
			i if self.key_bindings.insert_line.contains(&i) => Input::InsertLine,
			i if self.key_bindings.move_down.contains(&i) => Input::MoveCursorDown,
			i if self.key_bindings.move_down_step.contains(&i) => Input::MoveCursorPageDown,
			i if self.key_bindings.move_down_step.contains(&i) => Input::MoveCursorPageDown,
			i if self.key_bindings.move_end.contains(&i) => Input::MoveCursorEnd,
			i if self.key_bindings.move_home.contains(&i) => Input::MoveCursorHome,
			i if self.key_bindings.move_left.contains(&i) => Input::MoveCursorLeft,
			i if self.key_bindings.move_right.contains(&i) => Input::MoveCursorRight,
			i if self.key_bindings.move_selection_down.contains(&i) => Input::SwapSelectedDown,
			i if self.key_bindings.move_selection_up.contains(&i) => Input::SwapSelectedUp,
			i if self.key_bindings.move_up.contains(&i) => Input::MoveCursorUp,
			i if self.key_bindings.move_up_step.contains(&i) => Input::MoveCursorPageUp,
			i if self.key_bindings.open_in_external_editor.contains(&i) => Input::OpenInEditor,
			i if self.key_bindings.rebase.contains(&i) => Input::Rebase,
			i if self.key_bindings.redo.contains(&i) => Input::Redo,
			i if self.key_bindings.remove_line.contains(&i) => Input::Delete,
			i if self.key_bindings.show_commit.contains(&i) => Input::ShowCommit,
			i if self.key_bindings.toggle_visual_mode.contains(&i) => Input::ToggleVisualMode,
			i if self.key_bindings.undo.contains(&i) => Input::Undo,
			i if i.as_str() == "Exit" => Input::Exit,
			i if i.as_str() == "Kill" => Input::Kill,
			i if i.as_str() == "Resize" => Input::Resize,
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

	fn get_show_commit_input(&self, input: String) -> Input {
		Self::get_standard_inputs(input.as_str()).unwrap_or_else(|| {
			match input {
				i if self.key_bindings.help.contains(&i) => Input::Help,
				i if self.key_bindings.show_diff.contains(&i) => Input::ShowDiff,
				_ => Input::Other,
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use std::{env::set_var, path::Path};

	use rstest::rstest;

	use super::*;
	use crate::config::Config;

	fn input_handler_test<G, C>(config_setup: G, callback: C)
	where
		G: for<'p> FnOnce(&'p mut Config),
		C: for<'p> FnOnce(&'p InputHandler<'_>),
	{
		let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap()
			.to_owned();

		set_var("GIT_DIR", git_repo_dir.as_str());
		let mut config = Config::new().unwrap();
		config_setup(&mut config);
		let input_handler = InputHandler::new(&config.key_bindings);
		callback(&input_handler);
	}

	#[rstest(
		input,
		expected,
		case::yes_lower("y", Input::Yes),
		case::yes_upper("Y", Input::Yes),
		case::no_n_lower("n", Input::No),
		case::no_n_upper("N", Input::No),
		case::no_other("x", Input::No),
		case::multiple_bindings("7", Input::Yes),
		case::standard_resize("Resize", Input::Resize),
		case::standard_move_up("Up", Input::ScrollUp),
		case::standard_move_down("Down", Input::ScrollDown),
		case::standard_move_left("Left", Input::ScrollLeft),
		case::standard_move_right("Right", Input::ScrollRight),
		case::standard_move_jump_up("PageUp", Input::ScrollJumpUp),
		case::standard_move_jump_down("PageDown", Input::ScrollJumpDown),
		case::standard_move_end("End", Input::ScrollBottom),
		case::standard_move_home("Home", Input::ScrollTop),
		case::standard_exit("Exit", Input::Exit),
		case::standard_kill("Kill", Input::Kill)
	)]
	#[serial_test::serial]
	fn confirm_mode(input: &str, expected: Input) {
		input_handler_test(
			|config| {
				config.key_bindings.confirm_yes = vec![String::from('y'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(
					input_handler.get_input(InputMode::Confirm, String::from(input)),
					expected
				);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::character("a", Input::Character('a')),
		case::backspace_key("Backspace", Input::Backspace),
		case::tab_character("BackTab", Input::BackTab),
		case::delete("Delete", Input::Delete),
		case::enter("Enter", Input::Enter),
		case::escape("Esc", Input::Escape),
		case::insert("Insert", Input::Insert),
		case::other_characters("AB", Input::Other),
		case::other("Other", Input::Other),
		case::tab_character("Tab", Input::Tab),
		case::standard_resize("Resize", Input::Resize),
		case::standard_move_up("Up", Input::ScrollUp),
		case::standard_move_down("Down", Input::ScrollDown),
		case::standard_move_left("Left", Input::ScrollLeft),
		case::standard_move_right("Right", Input::ScrollRight),
		case::standard_move_jump_up("PageUp", Input::ScrollJumpUp),
		case::standard_move_jump_down("PageDown", Input::ScrollJumpDown),
		case::standard_move_end("End", Input::ScrollBottom),
		case::standard_move_home("Home", Input::ScrollTop),
		case::standard_exit("Exit", Input::Exit),
		case::standard_kill("Kill", Input::Kill)
	)]
	#[serial_test::serial]
	fn default_mode(input: &str, expected: Input) {
		input_handler_test(
			|_| {},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(
					input_handler.get_input(InputMode::Default, String::from(input)),
					expected
				);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::abort("q", Input::Abort),
		case::action_break("b", Input::ActionBreak),
		case::action_drop("d", Input::ActionDrop),
		case::action_edit("e", Input::ActionEdit),
		case::action_fixup("f", Input::ActionFixup),
		case::action_pick("p", Input::ActionPick),
		case::action_reword("r", Input::ActionReword),
		case::action_squash("s", Input::ActionSquash),
		case::edit("E", Input::Edit),
		case::force_abort("Q", Input::ForceAbort),
		case::force_rebase("W", Input::ForceRebase),
		case::help("?", Input::Help),
		case::insert_line("I", Input::InsertLine),
		case::move_down("Down", Input::MoveCursorDown),
		case::move_end("End", Input::MoveCursorEnd),
		case::move_home("Home", Input::MoveCursorHome),
		case::move_left("Left", Input::MoveCursorLeft),
		case::move_page_down("PageDown", Input::MoveCursorPageDown),
		case::move_page_up("PageUp", Input::MoveCursorPageUp),
		case::move_right("Right", Input::MoveCursorRight),
		case::move_up("Up", Input::MoveCursorUp),
		case::open_in_external_editor("!", Input::OpenInEditor),
		case::rebase("w", Input::Rebase),
		case::redo("Controly", Input::Redo),
		case::remove_line("Delete", Input::Delete),
		case::show_commit("c", Input::ShowCommit),
		case::swap_selected_down("j", Input::SwapSelectedDown),
		case::swap_selected_up("k", Input::SwapSelectedUp),
		case::toggle_visual_mode("v", Input::ToggleVisualMode),
		case::undo("Controlz", Input::Undo),
		case::other("z", Input::Other),
		case::standard_exit("Exit", Input::Exit),
		case::standard_kill("Kill", Input::Kill),
		case::standard_resize("Resize", Input::Resize),
		case::multiple_bindings("7", Input::Abort)
	)]
	#[serial_test::serial]
	fn list_mode(input: &str, expected: Input) {
		input_handler_test(
			|config| {
				config.key_bindings.abort = vec![String::from('q'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::List, String::from(input)), expected);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::backspace_character("Backspace", Input::Backspace),
		case::backtab_key("BackTab", Input::BackTab),
		case::delete_key("Delete", Input::Delete),
		case::down_key("Down", Input::Down),
		case::end_key("End", Input::End),
		case::enter_key("Enter", Input::Enter),
		case::escape_key("Esc", Input::Escape),
		case::standard_exit("Exit", Input::Exit),
		case::home_key("Home", Input::Home),
		case::insert_key("Insert", Input::Insert),
		case::standard_kill("Kill", Input::Kill),
		case::left_key("Left", Input::Left),
		case::other("Other", Input::Other),
		case::page_down_key("PageDown", Input::PageDown),
		case::page_up_key("PageUp", Input::PageUp),
		case::standard_resize("Resize", Input::Resize),
		case::right_key("Right", Input::Right),
		case::tab_key("Tab", Input::Tab),
		case::up_key("Up", Input::Up),
		case::character("a", Input::Character('a')),
		case::unknown("F(1)", Input::Other)
	)]
	#[serial_test::serial]
	fn raw_mode(input: &str, expected: Input) {
		input_handler_test(
			|_| {},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::Raw, String::from(input)), expected);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::help("?", Input::Help),
		case::show_diff("d", Input::ShowDiff),
		case::other("Null", Input::Other),
		case::standard_resize("Resize", Input::Resize),
		case::standard_move_up("Up", Input::ScrollUp),
		case::standard_move_down("Down", Input::ScrollDown),
		case::standard_move_left("Left", Input::ScrollLeft),
		case::standard_move_right("Right", Input::ScrollRight),
		case::standard_move_jump_up("PageUp", Input::ScrollJumpUp),
		case::standard_move_jump_down("PageDown", Input::ScrollJumpDown),
		case::standard_exit("Exit", Input::Exit),
		case::standard_kill("Kill", Input::Kill),
		case::multiple_bindings("7", Input::ShowDiff)
	)]
	#[serial_test::serial]
	fn show_commit_mode(input: &str, expected: Input) {
		input_handler_test(
			|config| {
				config.key_bindings.show_diff = vec![String::from('d'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(
					input_handler.get_input(InputMode::ShowCommit, String::from(input)),
					expected
				);
			},
		);
	}
}
