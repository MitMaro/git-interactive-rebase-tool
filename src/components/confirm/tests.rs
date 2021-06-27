use input::testutil::with_event_handler;
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
fn handle_event_yes_uppercase() {
	with_event_handler(&[Event::from('Y')], |context| {
		let module = Confirm::new("Prompt message", &[], &[]);
		let (confirmed, event) = module.handle_event(&context.event_handler);
		assert_eq!(event, Event::from(MetaEvent::Yes));
		assert_eq!(confirmed, Confirmed::Yes);
	});
}

#[test]
fn handle_event_yes_lowercase() {
	with_event_handler(&[Event::from('y')], |context| {
		let module = Confirm::new("Prompt message", &[], &[]);
		let (confirmed, event) = module.handle_event(&context.event_handler);
		assert_eq!(event, Event::from(MetaEvent::Yes));
		assert_eq!(confirmed, Confirmed::Yes);
	});
}

#[test]
fn handle_event_no() {
	with_event_handler(&[Event::from('n')], |context| {
		let module = Confirm::new("Prompt message", &[], &[]);
		let (confirmed, event) = module.handle_event(&context.event_handler);
		assert_eq!(event, Event::from(MetaEvent::No));
		assert_eq!(confirmed, Confirmed::No);
	});
}

#[rstest(
	event,
	case::resize(Event::Resize(100, 100)),
	case::scroll_left(Event::from(MetaEvent::ScrollLeft)),
	case::scroll_right(Event::from(MetaEvent::ScrollRight)),
	case::scroll_down(Event::from(MetaEvent::ScrollDown)),
	case::scroll_up(Event::from(MetaEvent::ScrollUp)),
	case::scroll_jump_down(Event::from(MetaEvent::ScrollJumpDown)),
	case::scroll_jump_up(Event::from(MetaEvent::ScrollJumpUp))
)]
fn input_standard(event: Event) {
	with_event_handler(&[event], |context| {
		let module = Confirm::new("Prompt message", &[], &[]);
		let (confirmed, evt) = module.handle_event(&context.event_handler);
		assert_eq!(evt, event);
		assert_eq!(confirmed, Confirmed::Other);
	});
}
