use view::assert_rendered_output;

use super::*;

#[test]
fn with_before_and_after_build() {
	let mut module = Edit::new();
	module.set_content("foobar");
	let view_data = module.build_view_data(
		|updater| {
			updater.push_line(ViewLine::from("Before"));
		},
		|updater| {
			updater.push_line(ViewLine::from("After"));
		},
	);
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}Before",
		"{Normal}foobar{Normal,Underline} ",
		"{Normal}After",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn edit_event() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_event(Event::from(KeyCode::Left));
	let view_data = module.get_view_data();

	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}fooba{Normal,Underline}r",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn finish_event() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_event(Event::from(KeyCode::Enter));
	assert!(module.is_finished());
}

#[test]
fn set_get_content() {
	let mut module = Edit::new();
	module.set_content("abcd");
	assert_eq!(module.get_content(), "abcd");
}

#[test]
fn reset() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.reset();
	assert_eq!(module.get_content(), "");
	assert!(!module.is_finished());
}
