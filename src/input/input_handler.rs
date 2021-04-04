use crate::{
	config::key_bindings::KeyBindings,
	input::{EventHandler, Input},
};

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

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		let input = EventHandler::poll_event();

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
					// Mostly for F(x) keys that are not supported in raw mode
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
	use crate::{
		config::Config,
		create_key_event,
		display::{CrossTerm, Event, KeyCode},
	};

	fn input_handler_test<G, C>(events: &[Event], config_setup: G, callback: C)
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
		CrossTerm::set_inputs(events.to_vec());
		let input_handler = InputHandler::new(&config.key_bindings);
		callback(&input_handler);
	}

	#[rstest(
		input,
		expected,
		case::yes_lower(create_key_event!('y'), Input::Yes),
		case::yes_upper(create_key_event!('Y'), Input::Yes),
		case::no_n_lower(create_key_event!('n'), Input::No),
		case::no_n_upper(create_key_event!('N'), Input::No),
		case::no_other(create_key_event!('x'), Input::No),
		case::multiple_bindings(create_key_event!('7'), Input::Yes),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_move_end(create_key_event!(code KeyCode::End), Input::ScrollBottom),
		case::standard_move_home(create_key_event!(code KeyCode::Home), Input::ScrollTop),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill)
	)]
	#[serial_test::serial]
	fn confirm_mode(input: Event, expected: Input) {
		input_handler_test(
			&[input],
			|config| {
				config.key_bindings.confirm_yes = vec![String::from('y'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::Confirm), expected);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::character(create_key_event!('a'), Input::Character('a')),
		case::backspace_key(create_key_event!(code KeyCode::Backspace), Input::Backspace),
		case::tab_character(create_key_event!(code KeyCode::BackTab), Input::BackTab),
		case::delete(create_key_event!(code KeyCode::Delete), Input::Delete),
		case::enter(create_key_event!(code KeyCode::Enter), Input::Enter),
		case::escape(create_key_event!(code KeyCode::Esc), Input::Escape),
		case::insert(create_key_event!(code KeyCode::Insert), Input::Insert),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::tab_character(create_key_event!(code KeyCode::Tab), Input::Tab),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_move_end(create_key_event!(code KeyCode::End), Input::ScrollBottom),
		case::standard_move_home(create_key_event!(code KeyCode::Home), Input::ScrollTop),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill)
	)]
	#[serial_test::serial]
	fn default_mode(input: Event, expected: Input) {
		input_handler_test(
			&[input],
			|_| {},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::Default), expected);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::abort(create_key_event!('q'), Input::Abort),
		case::action_break(create_key_event!('b'), Input::ActionBreak),
		case::action_drop(create_key_event!('d'), Input::ActionDrop),
		case::action_edit(create_key_event!('e'), Input::ActionEdit),
		case::action_fixup(create_key_event!('f'), Input::ActionFixup),
		case::action_pick(create_key_event!('p'), Input::ActionPick),
		case::action_reword(create_key_event!('r'), Input::ActionReword),
		case::action_squash(create_key_event!('s'), Input::ActionSquash),
		case::edit(create_key_event!('E'), Input::Edit),
		case::force_abort(create_key_event!('Q'), Input::ForceAbort),
		case::force_rebase(create_key_event!('W'), Input::ForceRebase),
		case::help(create_key_event!('?'), Input::Help),
		case::insert_line(create_key_event!('I'), Input::InsertLine),
		case::move_down(create_key_event!(code KeyCode::Down), Input::MoveCursorDown),
		case::move_end(create_key_event!(code KeyCode::End), Input::MoveCursorEnd),
		case::move_home(create_key_event!(code KeyCode::Home), Input::MoveCursorHome),
		case::move_left(create_key_event!(code KeyCode::Left), Input::MoveCursorLeft),
		case::move_page_down(create_key_event!(code KeyCode::PageDown), Input::MoveCursorPageDown),
		case::move_page_up(create_key_event!(code KeyCode::PageUp), Input::MoveCursorPageUp),
		case::move_right(create_key_event!(code KeyCode::Right), Input::MoveCursorRight),
		case::move_up(create_key_event!(code KeyCode::Up), Input::MoveCursorUp),
		case::open_in_external_editor(create_key_event!('!'), Input::OpenInEditor),
		case::rebase(create_key_event!('w'), Input::Rebase),
		case::redo(create_key_event!('y', "Control"), Input::Redo),
		case::remove_line(create_key_event!(code KeyCode::Delete), Input::Delete),
		case::show_commit(create_key_event!('c'), Input::ShowCommit),
		case::swap_selected_down(create_key_event!('j'), Input::SwapSelectedDown),
		case::swap_selected_up(create_key_event!('k'), Input::SwapSelectedUp),
		case::toggle_visual_mode(create_key_event!('v'), Input::ToggleVisualMode),
		case::undo(create_key_event!('z', "Control"), Input::Undo),
		case::other(create_key_event!('z'), Input::Other),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill),
		case::multiple_bindings(create_key_event!('7'), Input::Abort)
	)]
	#[serial_test::serial]
	fn list_mode(input: Event, expected: Input) {
		input_handler_test(
			&[input],
			|config| {
				config.key_bindings.abort = vec![String::from('q'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::List), expected);
			},
		);
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
		case::escape_key(create_key_event!(code KeyCode::Esc), Input::Escape),
		case::home_key(create_key_event!(code KeyCode::Home), Input::Home),
		case::insert_key(create_key_event!(code KeyCode::Insert), Input::Insert),
		case::left_key(create_key_event!(code KeyCode::Left), Input::Left),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::page_down_key(create_key_event!(code KeyCode::PageDown), Input::PageDown),
		case::page_up_key(create_key_event!(code KeyCode::PageUp), Input::PageUp),
		case::right_key(create_key_event!(code KeyCode::Right), Input::Right),
		case::tab_key(create_key_event!(code KeyCode::Tab), Input::Tab),
		case::up_key(create_key_event!(code KeyCode::Up), Input::Up),
		case::character(create_key_event!('a'), Input::Character('a')),
		case::unknown(create_key_event!(code KeyCode::F(1)), Input::Other),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill),
	)]
	#[serial_test::serial]
	fn raw_mode(input: Event, expected: Input) {
		input_handler_test(
			&[input],
			|_| {},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::Raw), expected);
			},
		);
	}

	#[rstest(
		input,
		expected,
		case::help(create_key_event!('?'), Input::Help),
		case::show_diff(create_key_event!('d'), Input::ShowDiff),
		case::other(create_key_event!(code KeyCode::Null), Input::Other),
		case::multiple_bindings(create_key_event!('7'), Input::ShowDiff),
		case::standard_resize(Event::Resize(0, 0), Input::Resize),
		case::standard_move_up(create_key_event!(code KeyCode::Up), Input::ScrollUp),
		case::standard_move_down(create_key_event!(code KeyCode::Down), Input::ScrollDown),
		case::standard_move_left(create_key_event!(code KeyCode::Left), Input::ScrollLeft),
		case::standard_move_right(create_key_event!(code KeyCode::Right), Input::ScrollRight),
		case::standard_move_jump_up(create_key_event!(code KeyCode::PageUp), Input::ScrollJumpUp),
		case::standard_move_jump_down(create_key_event!(code KeyCode::PageDown), Input::ScrollJumpDown),
		case::standard_move_end(create_key_event!(code KeyCode::End), Input::ScrollBottom),
		case::standard_move_home(create_key_event!(code KeyCode::Home), Input::ScrollTop),
		case::standard_exit(create_key_event!('d', "Control"), Input::Exit),
		case::standard_kill(create_key_event!('c', "Control"), Input::Kill)
	)]
	#[serial_test::serial]
	fn show_commit_mode(input: Event, expected: Input) {
		input_handler_test(
			&[input],
			|config| {
				config.key_bindings.show_diff = vec![String::from('d'), String::from('7')];
			},
			|input_handler: &InputHandler<'_>| {
				assert_eq!(input_handler.get_input(InputMode::ShowCommit), expected);
			},
		);
	}
}
