use crate::display::{CrossTerm, Event, KeyCode, KeyModifiers, MouseEventKind};

pub struct EventHandler {}

impl EventHandler {
	pub fn poll_event() -> String {
		loop {
			if let Ok(input) = CrossTerm::read_event() {
				let normalized_input = Self::normalize_input(input);
				// this is a hack to work around unhandled mouse events, input handling needs to be changed
				// to properly handle dynamic inputs like mouse events
				// TODO remove hack
				if normalized_input != "Ignore" {
					return normalized_input;
				}
			}
			else {
				return String::from("Other");
			}
		}
	}

	fn modifiers_to_string(modifiers: KeyModifiers, code: Option<KeyCode>) -> String {
		let mut result = vec![];

		if modifiers.contains(KeyModifiers::SHIFT) {
			if let Some(KeyCode::Char(k)) = code {
				if k == '\t' || k == '\n' || k == '\u{7f}' {
					result.push(String::from("Shift"));
				}
			}
			else {
				result.push(String::from("Shift"));
			}
		}
		if modifiers.contains(KeyModifiers::CONTROL) {
			result.push(String::from("Control"));
		}
		if modifiers.contains(KeyModifiers::ALT) {
			result.push(String::from("Alt"));
		}
		result.join("")
	}

	pub(crate) fn normalize_input(event: Event) -> String {
		match event {
			Event::Key(event) => {
				let code = format!(
					"{}{}",
					Self::modifiers_to_string(event.modifiers, Some(event.code)),
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
						KeyCode::Char(c) => String::from(c),
					}
				);

				match code.as_str() {
					"Controlc" => String::from("Kill"),
					"Controld" => String::from("Exit"),
					_ => code,
				}
			},
			Event::Mouse(event) => {
				let kind = match event.kind {
					MouseEventKind::ScrollDown => String::from("Down"),
					MouseEventKind::ScrollUp => String::from("Up"),
					_ => return String::from("Ignore"), // early return to ignore modifiers
				};
				format!("{}{}", kind, Self::modifiers_to_string(event.modifiers, None),)
			},
			Event::Resize(..) => String::from("Resize"),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{create_key_event, create_mouse_event};

	#[test]
	#[serial_test::serial]
	fn read_event_success() {
		CrossTerm::set_inputs(vec![create_key_event!('z')]);
		assert_eq!(EventHandler::poll_event(), "z");
	}

	#[test]
	#[serial_test::serial]
	fn read_event_fail() {
		CrossTerm::set_inputs(vec![]);
		assert_eq!(EventHandler::poll_event(), "Other");
	}

	#[test]
	#[serial_test::serial]
	fn read_event_ignore_hack() {
		CrossTerm::set_inputs(vec![create_mouse_event!(MouseEventKind::Moved), create_key_event!('z')]);
		assert_eq!(EventHandler::poll_event(), "z");
	}

	#[test]
	fn modifiers_to_string_no_modifiers() {
		assert_eq!(EventHandler::modifiers_to_string(KeyModifiers::NONE, None), "");
	}

	#[test]
	fn modifiers_to_string_alt() {
		assert_eq!(EventHandler::modifiers_to_string(KeyModifiers::ALT, None), "Alt");
	}

	#[test]
	fn modifiers_to_string_control() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::CONTROL, None),
			"Control"
		);
	}

	#[test]
	fn modifiers_to_string_shift() {
		assert_eq!(EventHandler::modifiers_to_string(KeyModifiers::SHIFT, None), "Shift");
	}

	#[test]
	fn modifiers_to_string_combined() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::all(), None),
			"ShiftControlAlt"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_char() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('A'))),
			""
		);
	}

	#[test]
	fn modifiers_to_string_with_code_char_tab() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\t'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_newline() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\n'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_backspace() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Char('\u{7f}'))),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_other() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::SHIFT, Some(KeyCode::Enter)),
			"Shift"
		);
	}

	#[test]
	fn modifiers_to_string_with_code_alphabetic_combined() {
		assert_eq!(
			EventHandler::modifiers_to_string(KeyModifiers::all(), Some(KeyCode::Char('A'))),
			"ControlAlt"
		);
	}

	#[rstest(
		event,
		expected,
		case::key_event_abort(create_key_event!(code KeyCode::Backspace), "Backspace"),
		case::key_event_abort(create_key_event!(code KeyCode::BackTab), "BackTab"),
		case::key_event_backspace(create_key_event!(code KeyCode::Backspace), "Backspace"),
		case::key_event_back_tab(create_key_event!(code KeyCode::BackTab), "BackTab"),
		case::key_event_delete(create_key_event!(code KeyCode::Delete), "Delete"),
		case::key_event_down(create_key_event!(code KeyCode::Down), "Down"),
		case::key_event_end(create_key_event!(code KeyCode::End), "End"),
		case::key_event_enter(create_key_event!(code KeyCode::Enter), "Enter"),
		case::key_event_esc(create_key_event!(code KeyCode::Esc), "Esc"),
		case::key_event_f0(create_key_event!(code KeyCode::F(0)),"F0"),
		case::key_event_f255(create_key_event!(code KeyCode::F(255)),"F255"),
		case::key_event_home(create_key_event!(code KeyCode::Home), "Home"),
		case::key_event_insert(create_key_event!(code KeyCode::Insert), "Insert"),
		case::key_event_left(create_key_event!(code KeyCode::Left), "Left"),
		case::key_event_null(create_key_event!(code KeyCode::Null), "Other"),
		case::key_event_page_down(create_key_event!(code KeyCode::PageDown), "PageDown"),
		case::key_event_page_up(create_key_event!(code KeyCode::PageUp), "PageUp"),
		case::key_event_right(create_key_event!(code KeyCode::Right), "Right"),
		case::key_event_tab(create_key_event!(code KeyCode::Tab), "Tab"),
		case::key_event_up(create_key_event!(code KeyCode::Up), "Up"),
		case::key_event_tab_character(create_key_event!('\t'), "Tab"),
		case::key_event_enter_character(create_key_event!('\n'), "Enter"),
		case::key_event_backspace_character(create_key_event!('\u{7f}'), "Backspace"),
		case::key_event_character(create_key_event!('x'), "x"),
		case::key_event_modifiers(create_key_event!('x', "Control"), "Controlx"),
		case::key_event_kill(create_key_event!('c', "Control"), "Kill"),
		case::key_event_exit(create_key_event!('d', "Control"), "Exit"),
		case::mouse_event_scroll_down(create_mouse_event!(MouseEventKind::ScrollDown), "Down"),
		case::mouse_event_scroll_up(create_mouse_event!(MouseEventKind::ScrollUp), "Up"),
		case::mouse_event_other(create_mouse_event!(MouseEventKind::Moved, "Control"), "Ignore"),
		case::mouse_event_other(create_mouse_event!(MouseEventKind::Moved), "Ignore"),
		case::resize(Event::Resize(0, 0), "Resize")
	)]
	fn normalize_input(event: Event, expected: &str) {
		assert_eq!(EventHandler::normalize_input(event), String::from(expected));
	}
}
