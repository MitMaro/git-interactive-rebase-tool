use input::StandardEvent;
use rstest::rstest;
use view::{
	assert_rendered_output,
	testutil::{with_view_state, AssertRenderOptions},
};

use super::*;

#[derive(Clone, Debug, PartialEq)]
enum TestAction {
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
		Style module.get_view_data(),
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
		Style module.get_view_data(),
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
fn valid_selection() {
	with_view_state(|context| {
		let mut module = Choice::new(create_choices());
		let choice = module.handle_event(Event::from('b'), &context.state);
		assert_eq!(choice.unwrap(), &TestAction::B);
		assert_rendered_output!(
			Body module.get_view_data(),
			"a) Description A",
			"b) Description B",
			"c) Description C",
			"",
			"Please choose an option."
		);
	});
}

#[test]
fn invalid_selection_character() {
	with_view_state(|context| {
		let mut module = Choice::new(create_choices());
		let choice = module.handle_event(Event::from('z'), &context.state);
		assert!(choice.is_none());
		assert_rendered_output!(
			Body module.get_view_data(),
			"a) Description A",
			"b) Description B",
			"c) Description C",
			"",
			"Invalid option selected. Please choose an option."
		);
	});
}

#[rstest]
#[case::resize(Event::Resize(100, 100))]
#[case::scroll_left(Event::from(StandardEvent::ScrollLeft))]
#[case::scroll_right(Event::from(StandardEvent::ScrollRight))]
#[case::scroll_down(Event::from(StandardEvent::ScrollDown))]
#[case::scroll_up(Event::from(StandardEvent::ScrollUp))]
#[case::scroll_jump_down(Event::from(StandardEvent::ScrollJumpDown))]
#[case::scroll_jump_up(Event::from(StandardEvent::ScrollJumpUp))]
fn event_standard(#[case] event: Event) {
	with_view_state(|context| {
		let mut module = Choice::new(create_choices());
		_ = module.handle_event(event, &context.state);
		assert!(!module.invalid_selection);
	});
}
