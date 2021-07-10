use input::MetaEvent;
use rstest::rstest;
use view::{assert_rendered_output, testutil::with_view_sender};

use super::*;

#[test]
fn empty() {
	let mut module = Help::new_from_keybindings(&[]);
	assert_rendered_output!(
		module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Normal,Underline}{Pad( )}",
		"{TRAILING}",
		"{IndicatorColor}Press any key to close"
	);
}

#[test]
fn from_key_bindings() {
	let mut module = Help::new_from_keybindings(&[
		(vec![String::from("a")], String::from("Description A")),
		(vec![String::from("b")], String::from("Description B")),
	]);
	assert_rendered_output!(
		module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Normal,Underline}{Pad( )}",
		"{BODY}",
		"{IndicatorColor} a{Normal,Dimmed}|{Normal}Description A",
		"{IndicatorColor} b{Normal,Dimmed}|{Normal}Description B",
		"{TRAILING}",
		"{IndicatorColor}Press any key to close"
	);
}

#[rstest]
#[case::resize(Event::Resize(100, 100))]
#[case::scroll_left(Event::from(MetaEvent::ScrollLeft))]
#[case::scroll_right(Event::from(MetaEvent::ScrollRight))]
#[case::scroll_down(Event::from(MetaEvent::ScrollDown))]
#[case::scroll_up(Event::from(MetaEvent::ScrollUp))]
#[case::scroll_jump_down(Event::from(MetaEvent::ScrollJumpDown))]
#[case::scroll_jump_up(Event::from(MetaEvent::ScrollJumpUp))]
fn input_continue_active(#[case] event: Event) {
	with_view_sender(|context| {
		let mut module = Help::new_from_keybindings(&[]);
		module.set_active();
		let _ = module.handle_event(event, &context.sender);
		assert!(module.is_active());
	});
}

#[test]
fn input_other() {
	with_view_sender(|context| {
		let mut module = Help::new_from_keybindings(&[]);
		module.set_active();
		let _ = module.handle_event(Event::from('a'), &context.sender);
		assert!(!module.is_active());
	});
}
