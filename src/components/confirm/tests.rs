use rstest::rstest;

use super::*;
use crate::{
	assert_rendered_output,
	input::StandardEvent,
	test_helpers::assertions::assert_rendered_output::AssertRenderOptions,
};

#[test]
fn render() {
	let mut module = Confirm::new("Prompt message", &[String::from("y"), String::from("Z")], &[
		String::from("n"),
		String::from("X"),
	]);
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE | AssertRenderOptions::BODY_ONLY,
		module.get_view_data(),
		"Prompt message (y,Z/n,X)? "
	);
}

#[test]
fn read_event_yes_uppercase() {
	assert_eq!(
		Confirm::read_event(Event::from('Y'), &KeyBindings::default()),
		Event::from(StandardEvent::Yes)
	);
}

#[test]
fn read_event_yes_lowercase() {
	assert_eq!(
		Confirm::read_event(Event::from('y'), &KeyBindings::default()),
		Event::from(StandardEvent::Yes)
	);
}

#[test]
fn read_event_no_lowercase() {
	assert_eq!(
		Confirm::read_event(Event::from('n'), &KeyBindings::default()),
		Event::from(StandardEvent::No)
	);
}

#[test]
fn read_event_no_uppercase() {
	assert_eq!(
		Confirm::read_event(Event::from('N'), &KeyBindings::default()),
		Event::from(StandardEvent::No)
	);
}

#[test]
fn read_event_not_key_event() {
	assert_eq!(
		Confirm::read_event(Event::None, &KeyBindings::default()),
		Event::None
	);
}

#[test]
fn read_event_not_char_event() {
	assert_eq!(
		Confirm::read_event(Event::from(KeyCode::Backspace), &KeyBindings::default()),
		Event::from(KeyCode::Backspace)
	);
}

#[test]
fn handle_event_yes() {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(Event::from(StandardEvent::Yes));
	assert_eq!(confirmed, Confirmed::Yes);
}

#[test]
fn handle_event_no() {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(Event::from(StandardEvent::No));
	assert_eq!(confirmed, Confirmed::No);
}

#[rstest]
#[case::resize(Event::Resize(100, 100))]
#[case::scroll_left(Event::from(StandardEvent::ScrollLeft))]
#[case::scroll_right(Event::from(StandardEvent::ScrollRight))]
#[case::scroll_down(Event::from(StandardEvent::ScrollDown))]
#[case::scroll_up(Event::from(StandardEvent::ScrollUp))]
#[case::scroll_jump_down(Event::from(StandardEvent::ScrollJumpDown))]
#[case::scroll_jump_up(Event::from(StandardEvent::ScrollJumpUp))]
fn input_standard(#[case] event: Event) {
	let module = Confirm::new("Prompt message", &[], &[]);
	let confirmed = module.handle_event(event);
	assert_eq!(confirmed, Confirmed::Other);
}
