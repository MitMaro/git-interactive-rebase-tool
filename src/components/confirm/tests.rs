use rstest::rstest;

use super::*;
use crate::assert_rendered_output;

#[test]
fn render() {
	let mut module = Confirm::new("Prompt message", &[String::from("y"), String::from("Z")], &[
		String::from("n"),
		String::from("X"),
	]);
	assert_rendered_output!(
		module.get_view_data(),
		"{TITLE}",
		"{BODY}",
		"{Normal}Prompt message (y,Z/n,X)? "
	);
}

#[test]
fn handle_input_yes() {
	let mut module = Confirm::new("Prompt message", &[], &[]);
	assert!(module.handle_input(Input::Yes).unwrap());
}

#[test]
fn handle_input_no() {
	let mut module = Confirm::new("Prompt message", &[], &[]);
	assert!(!module.handle_input(Input::No).unwrap());
}

#[rstest(
	input,
	case::other(Input::Character('x')),
	case::resize(Input::Resize),
	case::scroll_left(Input::ScrollLeft),
	case::scroll_right(Input::ScrollRight),
	case::scroll_down(Input::ScrollDown),
	case::scroll_up(Input::ScrollUp),
	case::scroll_jump_down(Input::ScrollJumpDown),
	case::scroll_jump_up(Input::ScrollJumpUp)
)]
fn input_standard(input: Input) {
	let mut module = Confirm::new("Prompt message", &[], &[]);
	assert!(module.handle_input(input).is_none());
}
