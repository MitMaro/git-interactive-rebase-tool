use super::*;
use crate::assert_rendered_output;

#[test]
fn with_description() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.set_description("Description");
	module.handle_input(Input::Right);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{LEADING}",
		"{IndicatorColor}Description",
		"",
		"{BODY}",
		"{Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn with_label() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.set_label("Label: ");
	module.handle_input(Input::Right);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{BODY}",
		"{Normal,Dimmed}Label: {Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn with_label_and_description() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.set_description("Description");
	module.set_label("Label: ");
	module.handle_input(Input::Right);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{LEADING}",
		"{IndicatorColor}Description",
		"",
		"{BODY}",
		"{Normal,Dimmed}Label: {Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_end() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.handle_input(Input::Right);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Left);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Left);
	module.handle_input(Input::Left);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..5 {
		module.handle_input(Input::Left);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..6 {
		module.handle_input(Input::Left);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Home);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..3 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::End);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{BODY}",
		"{Normal}foobar{Normal,Underline} ",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn move_cursor_on_empty_content() {
	let mut module = Edit::new();
	module.handle_input(Input::Left);
	module.handle_input(Input::Right);
	module.handle_input(Input::End);
	module.handle_input(Input::Home);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..10 {
		module.handle_input(Input::Left);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..10 {
		module.handle_input(Input::Right);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..2 {
		module.handle_input(Input::Left);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..2 {
		module.handle_input(Input::Left);
	}
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Character('x'));
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Left);
	module.handle_input(Input::Character('x'));
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..3 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Character('x'));
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..4 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Character('x'));
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{BODY}",
		"{Normal}x{Normal,Underline}a{Normal}bcd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn backspace_at_end() {
	let mut module = Edit::new();
	module.set_content("abcd");
	module.handle_input(Input::Backspace);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Left);
	module.handle_input(Input::Backspace);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..3 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Backspace);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..4 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Backspace);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Delete);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	module.handle_input(Input::Left);
	module.handle_input(Input::Delete);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..3 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Delete);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
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
	for _ in 0..4 {
		module.handle_input(Input::Left);
	}
	module.handle_input(Input::Delete);
	let view_data = &mut ViewData::new();
	view_data.set_view_size(500, 30);
	module.update_view_data(view_data);
	assert_rendered_output!(
		view_data,
		"{BODY}",
		"{Normal,Underline}b{Normal}cd",
		"{TRAILING}",
		"{IndicatorColor}Enter to finish"
	);
}

#[test]
fn ignore_other_input() {
	let mut module = Edit::new();
	assert!(!module.handle_input(Input::Other));
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
