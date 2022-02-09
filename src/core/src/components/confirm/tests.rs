use input::testutil::create_test_keybindings;
use rstest::rstest;
use view::assert_rendered_output;

use super::*;

#[test]
fn render() {
	let mut module = Confirm::new("Prompt message", &[String::from("y"), String::from("Z")], &[
		String::from("n"),
		String::from("X"),
	]);
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		module.get_view_data(),
		"{TITLE}",
		"{BODY}",
		"{Normal}Prompt message (y,Z/n,X)? "
	);
}

#[test]
fn read_event_yes_uppercase() {
	assert_eq!(
		Confirm::read_event(Event::from('Y'), &create_test_keybindings()),
		Event::from(MetaEvent::Yes)
	);
}

#[test]
fn read_event_yes_lowercase() {
	assert_eq!(
		Confirm::read_event(Event::from('y'), &create_test_keybindings()),
		Event::from(MetaEvent::Yes)
	);
}

#[test]
fn read_event_no_lowercase() {
	assert_eq!(
		Confirm::read_event(Event::from('n'), &create_test_keybindings()),
		Event::from(MetaEvent::No)
	);
}

#[test]
fn read_event_no_uppercase() {
	assert_eq!(
		Confirm::read_event(Event::from('N'), &create_test_keybindings()),
		Event::from(MetaEvent::No)
	);
}

#[test]
fn read_event_not_key_event() {
	assert_eq!(
		Confirm::read_event(Event::None, &create_test_keybindings()),
		Event::None
	);
}

#[test]
fn read_event_not_char_event() {
	assert_eq!(
		Confirm::read_event(Event::from(KeyCode::Backspace), &create_test_keybindings()),
		Event::from(KeyCode::Backspace)
	);
}

#[test]
fn handle_event_yes() {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(Event::from(MetaEvent::Yes));
	assert_eq!(confirmed, Confirmed::Yes);
}

#[test]
fn handle_event_no() {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(Event::from(MetaEvent::No));
	assert_eq!(confirmed, Confirmed::No);
}

#[rstest]
#[case::resize(Event::Resize(100, 100))]
#[case::scroll_left(Event::from(MetaEvent::ScrollLeft))]
#[case::scroll_right(Event::from(MetaEvent::ScrollRight))]
#[case::scroll_down(Event::from(MetaEvent::ScrollDown))]
#[case::scroll_up(Event::from(MetaEvent::ScrollUp))]
#[case::scroll_jump_down(Event::from(MetaEvent::ScrollJumpDown))]
#[case::scroll_jump_up(Event::from(MetaEvent::ScrollJumpUp))]
fn input_standard(#[case] event: Event) {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(event);
	assert_eq!(confirmed, Confirmed::Other);
}
