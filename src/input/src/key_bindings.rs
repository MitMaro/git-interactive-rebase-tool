use super::{Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub struct KeyBindings {
	pub abort: Vec<Event>,
	pub action_break: Vec<Event>,
	pub action_drop: Vec<Event>,
	pub action_edit: Vec<Event>,
	pub action_fixup: Vec<Event>,
	pub action_pick: Vec<Event>,
	pub action_reword: Vec<Event>,
	pub action_squash: Vec<Event>,
	pub confirm_yes: Vec<Event>,
	pub edit: Vec<Event>,
	pub force_abort: Vec<Event>,
	pub force_rebase: Vec<Event>,
	pub help: Vec<Event>,
	pub insert_line: Vec<Event>,
	pub move_down: Vec<Event>,
	pub move_down_step: Vec<Event>,
	pub move_end: Vec<Event>,
	pub move_home: Vec<Event>,
	pub move_left: Vec<Event>,
	pub move_right: Vec<Event>,
	pub move_selection_down: Vec<Event>,
	pub move_selection_up: Vec<Event>,
	pub move_up: Vec<Event>,
	pub move_up_step: Vec<Event>,
	pub open_in_external_editor: Vec<Event>,
	pub rebase: Vec<Event>,
	pub redo: Vec<Event>,
	pub remove_line: Vec<Event>,
	pub show_commit: Vec<Event>,
	pub show_diff: Vec<Event>,
	pub toggle_visual_mode: Vec<Event>,
	pub undo: Vec<Event>,
}

fn map_keybindings(bindings: &[String]) -> Vec<Event> {
	bindings
		.iter()
		.map(|b| {
			let mut key = String::from(b);
			let mut modifiers = KeyModifiers::empty();
			if key.contains("Control") {
				key = key.replace("Control", "");
				modifiers.insert(KeyModifiers::CONTROL);
			}
			if key.contains("Alt") {
				key = key.replace("Alt", "");
				modifiers.insert(KeyModifiers::ALT);
			}
			if key.contains("Shift") {
				key = key.replace("Shift", "");
				modifiers.insert(KeyModifiers::SHIFT);
			}

			let code = match key.as_str() {
				"Backspace" => KeyCode::Backspace,
				"BackTab" => KeyCode::BackTab,
				"Delete" => KeyCode::Delete,
				"Down" => KeyCode::Down,
				"End" => KeyCode::End,
				"Enter" => KeyCode::Enter,
				"Esc" => KeyCode::Esc,
				"Home" => KeyCode::Home,
				"Insert" => KeyCode::Insert,
				"Left" => KeyCode::Left,
				"PageDown" => KeyCode::PageDown,
				"PageUp" => KeyCode::PageUp,
				"Right" => KeyCode::Right,
				"Tab" => KeyCode::Tab,
				"Up" => KeyCode::Up,
				// assume that this is an F key
				k if k.len() > 1 => {
					let key_number = k[1..].parse::<u8>().unwrap_or(1);
					KeyCode::F(key_number)
				},
				k => {
					let c = k.chars().next().unwrap();
					KeyCode::Char(c)
				},
			};
			Event::Key(KeyEvent::new(code, modifiers))
		})
		.collect()
}

impl KeyBindings {
	#[must_use]
	pub fn new(key_bindings: &config::KeyBindings) -> Self {
		Self {
			abort: map_keybindings(&key_bindings.abort),
			action_break: map_keybindings(&key_bindings.action_break),
			action_drop: map_keybindings(&key_bindings.action_drop),
			action_edit: map_keybindings(&key_bindings.action_edit),
			action_fixup: map_keybindings(&key_bindings.action_fixup),
			action_pick: map_keybindings(&key_bindings.action_pick),
			action_reword: map_keybindings(&key_bindings.action_reword),
			action_squash: map_keybindings(&key_bindings.action_squash),
			edit: map_keybindings(&key_bindings.edit),
			force_abort: map_keybindings(&key_bindings.force_abort),
			force_rebase: map_keybindings(&key_bindings.force_rebase),
			help: map_keybindings(&key_bindings.help),
			insert_line: map_keybindings(&key_bindings.insert_line),
			move_down: map_keybindings(&key_bindings.move_down),
			move_down_step: map_keybindings(&key_bindings.move_down_step),
			move_end: map_keybindings(&key_bindings.move_end),
			move_home: map_keybindings(&key_bindings.move_home),
			move_left: map_keybindings(&key_bindings.move_left),
			move_right: map_keybindings(&key_bindings.move_right),
			move_selection_down: map_keybindings(&key_bindings.move_selection_down),
			move_selection_up: map_keybindings(&key_bindings.move_selection_up),
			move_up: map_keybindings(&key_bindings.move_up),
			move_up_step: map_keybindings(&key_bindings.move_up_step),
			open_in_external_editor: map_keybindings(&key_bindings.open_in_external_editor),
			rebase: map_keybindings(&key_bindings.rebase),
			redo: map_keybindings(&key_bindings.redo),
			remove_line: map_keybindings(&key_bindings.remove_line),
			show_commit: map_keybindings(&key_bindings.show_commit),
			show_diff: map_keybindings(&key_bindings.show_diff),
			toggle_visual_mode: map_keybindings(&key_bindings.toggle_visual_mode),
			undo: map_keybindings(&key_bindings.undo),
			confirm_yes: map_keybindings(&key_bindings.confirm_yes),
		}
	}
}

#[cfg(test)]
mod tests {
	use config::testutil::create_config;
	use rstest::rstest;

	use super::*;

	#[test]
	fn new() {
		let _key_bindings = KeyBindings::new(&create_config().key_bindings);
	}

	#[test]
	fn map_keybindings_with_modifiers() {
		assert_eq!(map_keybindings(&[String::from("ControlAltShifta")]), vec![Event::Key(
			KeyEvent {
				code: KeyCode::Char('a'),
				modifiers: KeyModifiers::all()
			}
		)]);
	}

	#[rstest(
		binding,
		key_code,
		case::backspace("Backspace", KeyCode::Backspace),
		case::back_tab("BackTab", KeyCode::BackTab),
		case::delete("Delete", KeyCode::Delete),
		case::down("Down", KeyCode::Down),
		case::end("End", KeyCode::End),
		case::enter("Enter", KeyCode::Enter),
		case::esc("Esc", KeyCode::Esc),
		case::home("Home", KeyCode::Home),
		case::insert("Insert", KeyCode::Insert),
		case::left("Left", KeyCode::Left),
		case::page_down("PageDown", KeyCode::PageDown),
		case::page_up("PageUp", KeyCode::PageUp),
		case::right("Right", KeyCode::Right),
		case::tab("Tab", KeyCode::Tab),
		case::up("Up", KeyCode::Up),
		case::function_in_range("F10", KeyCode::F(10)),
		case::function_out_of_range("F10000", KeyCode::F(1)),
		case::char("a", KeyCode::Char('a'))
	)]
	fn map_keybindings_key_code(binding: &str, key_code: KeyCode) {
		assert_eq!(map_keybindings(&[String::from(binding)]), vec![Event::from(key_code)]);
	}
}
