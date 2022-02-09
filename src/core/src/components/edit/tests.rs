use view::assert_rendered_output;

use super::*;

fn handle_events(module: &mut Edit, events: &[Event]) {
	for event in events {
		module.handle_event(*event);
	}
}

#[test]
fn with_label() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.set_label("Label: ");
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Dimmed}Label: {Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

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
fn move_cursor_end() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_event(Event::from(KeyCode::Right));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_1_left() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_event(Event::from(KeyCode::Left));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}fooba{Normal,Underline}r",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_2_from_start() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 2]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}foob{Normal,Underline}a{Normal}r",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_1_from_start() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 5]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}f{Normal,Underline}o{Normal}obar",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_to_start() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 6]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}f{Normal}oobar",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_to_home() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_event(Event::from(KeyCode::Home));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}f{Normal}oobar",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_to_end() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::End),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_on_empty_content() {
	let mut module = Edit::new();
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Right),
		Event::from(KeyCode::End),
		Event::from(KeyCode::Home),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_attempt_past_start() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 10]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}f{Normal}oobar",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_attempt_past_end() {
	let mut module = Edit::new();
	module.set_content("foobar");
	handle_events(&mut module, &[Event::from(KeyCode::Right); 10]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn multiple_width_unicode_single_width() {
	let mut module = Edit::new();
	module.set_content("a🗳b");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 2]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}a{Normal,Underline}🗳{Normal}b",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn multiple_width_unicode_emoji() {
	let mut module = Edit::new();
	module.set_content("a😀b");
	handle_events(&mut module, &[Event::from(KeyCode::Left); 2]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}a{Normal,Underline}😀{Normal}b",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn add_character_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.handle_event(Event::from('x'));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abcdx{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn add_character_one_from_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[Event::from(KeyCode::Left), Event::from('x')]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abcx{Normal,Underline}d",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn add_character_one_from_start() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from('x'),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}ax{Normal,Underline}b{Normal}cd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn add_character_at_start() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from('x'),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}x{Normal,Underline}a{Normal}bcd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn add_character_uppercase() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.handle_event(Event::Key(KeyEvent {
		code: input::KeyCode::Char('X'),
		modifiers: input::KeyModifiers::SHIFT,
	}));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abcdX{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn backspace_at_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.handle_event(Event::from(KeyCode::Backspace));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abc{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn backspace_one_from_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Backspace),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}ab{Normal,Underline}d",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn backspace_one_from_start() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Backspace),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}b{Normal}cd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn backspace_at_start() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Backspace),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}a{Normal}bcd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn delete_at_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.handle_event(Event::from(KeyCode::Delete));
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abcd{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn delete_last_character() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[Event::from(KeyCode::Left), Event::from(KeyCode::Delete)]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}abc{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn delete_second_character() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Delete),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal}a{Normal,Underline}c{Normal}d",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn delete_first_character() {
	let mut module = Edit::new();
	module.set_content("abcd");
	handle_events(&mut module, &[
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Left),
		Event::from(KeyCode::Delete),
	]);
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		"{Normal,Underline}b{Normal}cd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn ignore_other_input() {
	let mut module = Edit::new();
	module.handle_event(Event::from(KeyCode::Null));
}

#[test]
fn set_get_content() {
	let mut module = Edit::new();
	module.set_content("abcd");
	assert_eq!(module.cursor_position, 4);
	assert_eq!(module.get_content(), "abcd");
}

#[test]
fn clear_content() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.clear();
	assert_eq!(module.cursor_position, 0);
	assert_eq!(module.get_content(), "");
}
