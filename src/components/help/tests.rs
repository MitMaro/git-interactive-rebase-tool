use rstest::rstest;

use super::*;
use crate::assert_rendered_output;

#[test]
#[serial_test::serial]
fn empty() {
	let mut module = Help::new_from_keybindings(&[]);
	assert_rendered_output!(
		module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Normal,Underline}{Pad  }",
		"{TRAILING}",
		"{IndicatorColor}Press any key to close"
	);
}

#[test]
#[serial_test::serial]
fn from_key_bindings() {
	let mut module = Help::new_from_keybindings(&[
		(vec![String::from("a")], String::from("Description A")),
		(vec![String::from("b")], String::from("Description B")),
	]);
	assert_rendered_output!(
		module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Normal,Underline}{Pad  }",
		"{BODY}",
		"{IndicatorColor} a{Normal,Dimmed}|{Normal}Description A",
		"{IndicatorColor} b{Normal,Dimmed}|{Normal}Description B",
		"{TRAILING}",
		"{IndicatorColor}Press any key to close"
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
fn input_continue_active(input: Input) {
	let mut module = Help::new_from_keybindings(&[]);
	module.set_active();
	module.handle_input(input);
	assert!(module.is_active());
}

#[test]
fn input_other() {
	let mut module = Help::new_from_keybindings(&[]);
	module.set_active();
	module.handle_input(Input::Character('a'));
	assert!(!module.is_active());
}
