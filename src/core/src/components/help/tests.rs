use input::{KeyModifiers, MouseEvent, MouseEventKind, StandardEvent};
use rstest::rstest;

use super::*;
use crate::{assert_rendered_output, view::testutil::with_view_state};

fn handle_event(help: &mut Help, event: Event) {
	let evt = help.read_event(event).unwrap_or(event);
	with_view_state(|context| help.handle_event(evt, &context.state));
}

#[test]
fn empty() {
	let mut module = Help::new_from_keybindings(&[]);
	assert_rendered_output!(
		Style module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Pad( )}",
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
		Style module.get_view_data(),
		"{TITLE}",
		"{LEADING}",
		"{Normal,Underline} Key Action{Pad( )}",
		"{BODY}",
		"{IndicatorColor} a{Normal,Dimmed}|{Normal}Description A",
		"{IndicatorColor} b{Normal,Dimmed}|{Normal}Description B",
		"{TRAILING}",
		"{IndicatorColor}Press any key to close"
	);
}

#[rstest]
#[case::resize(Event::Resize(100, 100))]
#[case::scroll_left(Event::from(StandardEvent::ScrollLeft))]
#[case::scroll_right(Event::from(StandardEvent::ScrollRight))]
#[case::scroll_down(Event::from(StandardEvent::ScrollDown))]
#[case::scroll_up(Event::from(StandardEvent::ScrollUp))]
#[case::scroll_jump_down(Event::from(StandardEvent::ScrollJumpDown))]
#[case::scroll_jump_up(Event::from(StandardEvent::ScrollJumpUp))]
#[case::mouse_event(Event::Mouse(MouseEvent {
	kind: MouseEventKind::ScrollUp,
	column: 0,
	row: 0,
	modifiers: KeyModifiers::empty(),
}))]
fn handle_standard_events(#[case] event: Event) {
	let mut module = Help::new_from_keybindings(&[]);
	module.set_active();
	handle_event(&mut module, event);
	assert!(module.is_active());
}

#[test]
fn handle_other_key_event() {
	let mut module = Help::new_from_keybindings(&[]);
	module.set_active();
	handle_event(&mut module, Event::from('a'));
	assert!(!module.is_active());
}
