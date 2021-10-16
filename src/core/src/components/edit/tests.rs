use input::testutil::with_event_handler;
use view::assert_rendered_output;

use super::*;

#[test]
fn with_description() {
	let mut module = Edit::new();
	module.set_content("foobar");
	module.set_description("Description");
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
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
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
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
	let view_data = module.get_view_data();
	assert_rendered_output!(
		view_data,
		"{TITLE}",
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
	with_event_handler(&[Event::from(KeyCode::Right)], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}foobar{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_1_left() {
	with_event_handler(&[Event::from(KeyCode::Left)], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}fooba{Normal,Underline}r",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_2_from_start() {
	with_event_handler(&[Event::from(KeyCode::Left); 2], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}foob{Normal,Underline}a{Normal}r",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_1_from_start() {
	with_event_handler(&[Event::from(KeyCode::Left); 5], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}f{Normal,Underline}o{Normal}obar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_to_start() {
	with_event_handler(&[Event::from(KeyCode::Left); 6], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_to_home() {
	with_event_handler(&[Event::from(KeyCode::Home)], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_to_end() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::End),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("foobar");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}foobar{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn move_cursor_on_empty_content() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Right),
			Event::from(KeyCode::End),
			Event::from(KeyCode::Home),
		],
		|context| {
			let mut module = Edit::new();
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn move_cursor_attempt_past_start() {
	with_event_handler(&[Event::from(KeyCode::Left); 10], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn move_cursor_attempt_past_end() {
	with_event_handler(&[Event::from(KeyCode::Right); 10], |context| {
		let mut module = Edit::new();
		module.set_content("foobar");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}foobar{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn multiple_width_unicode_single_width() {
	with_event_handler(&[Event::from(KeyCode::Left); 2], |context| {
		let mut module = Edit::new();
		module.set_content("a🗳b");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}a{Normal,Underline}🗳{Normal}b",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn multiple_width_unicode_emoji() {
	with_event_handler(&[Event::from(KeyCode::Left); 2], |context| {
		let mut module = Edit::new();
		module.set_content("a😀b");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}a{Normal,Underline}😀{Normal}b",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn add_character_end() {
	with_event_handler(&[Event::from('x')], |context| {
		let mut module = Edit::new();
		module.set_content("abcd");
		let _ = module.handle_event(&context.event_handler);
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}abcdx{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn add_character_one_from_end() {
	with_event_handler(&[Event::from(KeyCode::Left), Event::from('x')], |context| {
		let mut module = Edit::new();
		module.set_content("abcd");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}abcx{Normal,Underline}d",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn add_character_one_from_start() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from('x'),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}ax{Normal,Underline}b{Normal}cd",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn add_character_at_start() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from('x'),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}x{Normal,Underline}a{Normal}bcd",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn add_character_uppercase() {
	with_event_handler(
		&[Event::Key(KeyEvent {
			code: input::KeyCode::Char('X'),
			modifiers: input::KeyModifiers::SHIFT,
		})],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = module.handle_event(&context.event_handler);
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}abcdX{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn backspace_at_end() {
	with_event_handler(&[Event::from(KeyCode::Backspace)], |context| {
		let mut module = Edit::new();
		module.set_content("abcd");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}abc{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn backspace_one_from_end() {
	with_event_handler(
		&[Event::from(KeyCode::Left), Event::from(KeyCode::Backspace)],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}ab{Normal,Underline}d",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn backspace_one_from_start() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Backspace),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal,Underline}b{Normal}cd",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn backspace_at_start() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Backspace),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal,Underline}a{Normal}bcd",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn delete_at_end() {
	with_event_handler(&[Event::from(KeyCode::Delete)], |context| {
		let mut module = Edit::new();
		module.set_content("abcd");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}abcd{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn delete_last_character() {
	with_event_handler(&[Event::from(KeyCode::Left), Event::from(KeyCode::Delete)], |context| {
		let mut module = Edit::new();
		module.set_content("abcd");
		let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
		let view_data = module.get_view_data();
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			"{Normal}abc{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn delete_second_character() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Delete),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}a{Normal,Underline}c{Normal}d",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn delete_first_character() {
	with_event_handler(
		&[
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Left),
			Event::from(KeyCode::Delete),
		],
		|context| {
			let mut module = Edit::new();
			module.set_content("abcd");
			let _ = context.for_each_event(|event_handler| module.handle_event(event_handler));
			let view_data = module.get_view_data();
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal,Underline}b{Normal}cd",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
		},
	);
}

#[test]
fn ignore_other_input() {
	with_event_handler(&[Event::from(KeyCode::Null)], |context| {
		let mut module = Edit::new();
		assert_eq!(module.handle_event(&context.event_handler), Event::from(KeyCode::Null));
	});
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
