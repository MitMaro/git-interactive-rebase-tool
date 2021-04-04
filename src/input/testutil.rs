use crate::{
	config::key_bindings::KeyBindings,
	create_key_event,
	display::{CrossTerm, Event, KeyCode, KeyEvent, KeyModifiers},
	input::Input,
};

fn map_str_to_event(input: &str) -> Event {
	match input {
		"Backspace" => create_key_event!(code KeyCode::Backspace),
		"Enter" => create_key_event!(code KeyCode::Enter),
		"Delete" => create_key_event!(code KeyCode::Delete),
		"End" => create_key_event!(code KeyCode::End),
		"Home" => create_key_event!(code KeyCode::Home),
		"Other" => create_key_event!(code KeyCode::Null),
		"Left" => create_key_event!(code KeyCode::Left),
		"PageUp" | "ScrollJumpUp" => create_key_event!(code KeyCode::PageUp),
		"PageDown" | "ScrollJumpDown" => create_key_event!(code KeyCode::PageDown),
		"Up" | "ScrollUp" => create_key_event!(code KeyCode::Up),
		"Right" | "ScrollRight" => create_key_event!(code KeyCode::Right),
		"Down" | "ScrollDown" => create_key_event!(code KeyCode::Down),
		"Controlz" => create_key_event!('z', "Control"),
		"Controly" => create_key_event!('y', "Control"),
		"Exit" => create_key_event!('d', "Control"),
		"Resize" => Event::Resize(0, 0),
		_ => {
			if input.len() > 1 {
				panic!("Unexpected input: {}", input);
			}
			Event::Key(KeyEvent::new(
				KeyCode::Char(input.chars().next().unwrap()),
				KeyModifiers::NONE,
			))
		},
	}
}

fn map_input_to_event(key_bindings: &KeyBindings, input: Input) -> Event {
	match input {
		Input::Abort => map_str_to_event(key_bindings.abort.first().unwrap().as_str()),
		Input::ActionBreak => map_str_to_event(key_bindings.action_break.first().unwrap().as_str()),
		Input::ActionDrop => map_str_to_event(key_bindings.action_drop.first().unwrap().as_str()),
		Input::ActionEdit => map_str_to_event(key_bindings.action_edit.first().unwrap().as_str()),
		Input::ActionFixup => map_str_to_event(key_bindings.action_fixup.first().unwrap().as_str()),
		Input::ActionPick => map_str_to_event(key_bindings.action_pick.first().unwrap().as_str()),
		Input::ActionReword => map_str_to_event(key_bindings.action_reword.first().unwrap().as_str()),
		Input::ActionSquash => map_str_to_event(key_bindings.action_squash.first().unwrap().as_str()),
		Input::Backspace => map_str_to_event("Backspace"),
		Input::Character(c) => map_str_to_event(String::from(c).as_str()),
		Input::Delete => map_str_to_event("Delete"),
		Input::Down | Input::ScrollDown => map_str_to_event("Down"),
		Input::Edit => map_str_to_event(key_bindings.edit.first().unwrap().as_str()),
		Input::End | Input::ScrollBottom => map_str_to_event("End"),
		Input::Enter => map_str_to_event("Enter"),
		Input::Exit => map_str_to_event("Exit"),
		Input::ForceAbort => map_str_to_event(key_bindings.force_abort.first().unwrap().as_str()),
		Input::ForceRebase => map_str_to_event(key_bindings.force_rebase.first().unwrap().as_str()),
		Input::Help => map_str_to_event(key_bindings.help.first().unwrap().as_str()),
		Input::Home | Input::ScrollTop => map_str_to_event("Home"),
		Input::InsertLine => map_str_to_event(key_bindings.insert_line.first().unwrap().as_str()),
		Input::Left | Input::ScrollLeft => map_str_to_event("Left"),
		Input::MoveCursorDown => map_str_to_event(key_bindings.move_down.first().unwrap().as_str()),
		Input::MoveCursorEnd => map_str_to_event(key_bindings.move_end.first().unwrap().as_str()),
		Input::MoveCursorHome => map_str_to_event(key_bindings.move_home.first().unwrap().as_str()),
		Input::MoveCursorLeft => map_str_to_event(key_bindings.move_left.first().unwrap().as_str()),
		Input::MoveCursorPageDown => map_str_to_event(key_bindings.move_down_step.first().unwrap().as_str()),
		Input::MoveCursorPageUp => map_str_to_event(key_bindings.move_up_step.first().unwrap().as_str()),
		Input::MoveCursorRight => map_str_to_event(key_bindings.move_right.first().unwrap().as_str()),
		Input::MoveCursorUp => map_str_to_event(key_bindings.move_up.first().unwrap().as_str()),
		Input::No => map_str_to_event(key_bindings.confirm_no.first().unwrap().as_str()),
		Input::OpenInEditor => map_str_to_event(key_bindings.open_in_external_editor.first().unwrap().as_str()),
		Input::Other => map_str_to_event("Other"),
		Input::PageDown | Input::ScrollJumpDown => map_str_to_event("PageDown"),
		Input::PageUp | Input::ScrollJumpUp => map_str_to_event("PageUp"),
		Input::Rebase => map_str_to_event(key_bindings.rebase.first().unwrap().as_str()),
		Input::Redo => map_str_to_event(key_bindings.redo.first().unwrap().as_str()),
		Input::Resize => map_str_to_event("Resize"),
		Input::Right | Input::ScrollRight => map_str_to_event("Right"),
		Input::ShowCommit => map_str_to_event(key_bindings.show_commit.first().unwrap().as_str()),
		Input::ShowDiff => map_str_to_event(key_bindings.show_diff.first().unwrap().as_str()),
		Input::SwapSelectedDown => map_str_to_event(key_bindings.move_selection_down.first().unwrap().as_str()),
		Input::SwapSelectedUp => map_str_to_event(key_bindings.move_selection_up.first().unwrap().as_str()),
		Input::ToggleVisualMode => map_str_to_event(key_bindings.toggle_visual_mode.first().unwrap().as_str()),
		Input::Undo => map_str_to_event(key_bindings.undo.first().unwrap().as_str()),
		Input::Up | Input::ScrollUp => map_str_to_event("Up"),
		Input::Yes => map_str_to_event(key_bindings.confirm_yes.first().unwrap().as_str()),
		_ => {
			panic!("Unsupported input: {:?}", input);
		},
	}
}

pub fn setup_mocked_inputs(inputs: &[Input], key_bindings: &KeyBindings) {
	CrossTerm::set_inputs(
		inputs
			.iter()
			.map(|input| map_input_to_event(key_bindings, *input))
			.collect(),
	);
}
