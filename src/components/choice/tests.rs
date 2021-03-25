use rstest::rstest;

use super::*;
use crate::assert_rendered_output;

#[derive(Clone, Debug, PartialEq)]
pub enum TestAction {
	A,
	B,
	C,
}

fn create_choices() -> Vec<(TestAction, char, String)> {
	vec![
		(TestAction::A, 'a', String::from("Description A")),
		(TestAction::B, 'b', String::from("Description B")),
		(TestAction::C, 'c', String::from("Description C")),
	]
}

#[test]
fn render_options_no_prompt() {
	let mut module = Choice::new(create_choices());
	assert_rendered_output!(
		module.get_view_data(100, 100),
		"{TITLE}",
		"{BODY}",
		"{Normal}a) Description A",
		"{Normal}b) Description B",
		"{Normal}c) Description C",
		"",
		"{IndicatorColor}Please choose an option."
	);
}

#[test]
fn render_options_prompt() {
	let mut module = Choice::new(create_choices());
	module.set_prompt(vec![ViewLine::from("Prompt")]);
	assert_rendered_output!(
		module.get_view_data(100, 100),
		"{TITLE}",
		"{LEADING}",
		"{Normal}Prompt",
		"",
		"{BODY}",
		"{Normal}a) Description A",
		"{Normal}b) Description B",
		"{Normal}c) Description C",
		"",
		"{IndicatorColor}Please choose an option."
	);
}

#[test]
fn invalid_selection_character() {
	let mut module = Choice::new(create_choices());
	assert!(module.handle_input(Input::Character('z')).is_none());
	assert_rendered_output!(
		module.get_view_data(100, 100),
		"{TITLE}",
		"{BODY}",
		"{Normal}a) Description A",
		"{Normal}b) Description B",
		"{Normal}c) Description C",
		"",
		"{IndicatorColor}Invalid option selected. Please choose an option."
	);
}

#[test]
fn invalid_selection_other() {
	let mut module = Choice::new(create_choices());
	assert!(module.handle_input(Input::Other).is_none());
	assert_rendered_output!(
		module.get_view_data(100, 100),
		"{TITLE}",
		"{BODY}",
		"{Normal}a) Description A",
		"{Normal}b) Description B",
		"{Normal}c) Description C",
		"",
		"{IndicatorColor}Invalid option selected. Please choose an option."
	);
}

#[test]
fn valid_selection() {
	let mut module = Choice::new(create_choices());
	assert_eq!(module.handle_input(Input::Character('b')).unwrap(), &TestAction::B);
	assert_rendered_output!(
		module.get_view_data(100, 100),
		"{TITLE}",
		"{BODY}",
		"{Normal}a) Description A",
		"{Normal}b) Description B",
		"{Normal}c) Description C",
		"",
		"{IndicatorColor}Please choose an option."
	);
}

#[rstest(
	input,
	case::resize(Input::Resize),
	case::scroll_left(Input::ScrollLeft),
	case::scroll_right(Input::ScrollRight),
	case::scroll_down(Input::ScrollDown),
	case::scroll_up(Input::ScrollUp),
	case::scroll_jump_down(Input::ScrollJumpDown),
	case::scroll_jump_up(Input::ScrollJumpUp)
)]
fn input_standard(input: Input) {
	let mut module = Choice::new(create_choices());
	module.handle_input(input);
	assert!(!module.invalid_selection);
}
