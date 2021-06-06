use display::DisplayColor;

use super::*;
use crate::view::testutil::{_assert_rendered_output, render_view_line};

fn assert_rendered(render_slice: &RenderSlice, expected: &[&str]) {
	let mut output = vec![];

	if render_slice.show_title() {
		if render_slice.show_help() {
			output.push(String::from("{TITLE}{HELP}"));
		}
		else {
			output.push(String::from("{TITLE}"));
		}
	}

	let lines = render_slice.get_lines();
	if lines.is_empty() {
		output.push(String::from("{EMPTY}"));
	}
	else {
		let leading_line_count = render_slice.get_leading_lines_count();
		let trailing_line_count = render_slice.get_trailing_lines_count();
		let lines_count = lines.len() - leading_line_count - trailing_line_count;
		let leading_lines = lines.iter().take(leading_line_count);
		let body_lines = lines.iter().skip(leading_line_count).take(lines_count);
		let trailing_lines = lines.iter().skip(leading_line_count + lines_count);

		if leading_line_count > 0 {
			output.push(String::from("{LEADING}"));
			for line in leading_lines {
				output.push(render_view_line(line));
			}
		}

		if lines_count > 0 {
			output.push(String::from("{BODY}"));
			for line in body_lines {
				output.push(render_view_line(line));
			}
		}

		if trailing_line_count > 0 {
			output.push(String::from("{TRAILING}"));
			for line in trailing_lines {
				output.push(render_view_line(line));
			}
		}
	}
	_assert_rendered_output(
		&output,
		&expected.iter().map(|s| String::from(*s)).collect::<Vec<String>>(),
	);
}

fn create_view_data(leading_lines: u16, body_lines: u16, trailing_lines: u16) -> ViewData {
	ViewData::new(|updater| {
		for index in 1..=leading_lines {
			updater.push_leading_line(ViewLine::from(format!("L({})", index)));
		}

		for index in 1..=body_lines {
			updater.push_line(ViewLine::from(format!("B({})", index)));
		}

		for index in 1..=trailing_lines {
			updater.push_trailing_line(ViewLine::from(format!("T({})", index)));
		}
	})
}

fn create_render_slice(width: usize, height: usize, view_data: &ViewData) -> RenderSlice {
	let mut render_slice = RenderSlice::new();
	render_slice.set_size(width, height);
	render_slice.sync_view_data(view_data);
	render_slice
}

fn set_scroll_position(render_slice: &mut RenderSlice, scroll_position: usize) {
	for _ in 0..scroll_position {
		render_slice.scroll_position.scroll_down();
	}
}

#[test]
fn scroll_up_action() {
	let view_data = create_view_data(2, 10, 2);
	let mut render_slice = create_render_slice(100, 8, &view_data);
	for _ in 0..6 {
		render_slice.scroll_position.scroll_down();
	}
	render_slice.record_scroll_up();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(6)",
		"{Normal}B(7)",
		"{Normal}B(8)",
		"{Normal}B(9)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn scroll_down_action() {
	let view_data = create_view_data(2, 10, 2);
	let mut render_slice = create_render_slice(100, 8, &view_data);
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn page_up_action() {
	let view_data = create_view_data(2, 10, 2);
	let mut render_slice = create_render_slice(100, 8, &view_data);
	for _ in 0..6 {
		render_slice.scroll_position.scroll_down();
	}
	render_slice.record_page_up();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(5)",
		"{Normal}B(6)",
		"{Normal}B(7)",
		"{Normal}B(8)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn page_down_action() {
	let view_data = create_view_data(2, 10, 2);
	let mut render_slice = create_render_slice(100, 8, &view_data);
	render_slice.record_page_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{Normal}B(6)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn scroll_left_action() {
	let view_data = ViewData::new(|updater| updater.push_line(ViewLine::from("1234567890")));
	let mut render_slice = create_render_slice(5, 1, &view_data);
	for _ in 0..4 {
		render_slice.scroll_position.scroll_right();
	}
	render_slice.record_scroll_left();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}45678"]);
}

#[test]
fn scroll_right_action() {
	let view_data = ViewData::new(|updater| updater.push_line(ViewLine::from("1234567890")));
	let mut render_slice = create_render_slice(5, 1, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}23456"]);
}

#[test]
fn resize_action() {
	let view_data = create_view_data(0, 3, 0);
	let mut render_slice = create_render_slice(1, 1, &view_data);
	render_slice.record_resize(100, 100);
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
	]);
}

#[test]
fn resize_action_zero_width() {
	let view_data = create_view_data(0, 3, 0);
	let mut render_slice = create_render_slice(1, 1, &view_data);
	render_slice.record_resize(0, 100);
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "", "", ""]);
}

#[test]
fn resize_action_zero_height() {
	let view_data = create_view_data(0, 3, 0);
	let mut render_slice = create_render_slice(1, 1, &view_data);
	render_slice.record_resize(100, 0);
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{EMPTY}"]);
}

#[test]
fn rebuild_with_cache() {
	let view_data = create_view_data(0, 1, 0);
	let mut render_slice = create_render_slice(100, 300, &view_data);
	let version = render_slice.get_version();
	render_slice.lines.push(ViewLine::new_empty_line());
	render_slice.sync_view_data(&view_data);
	assert_eq!(version, render_slice.version);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}B(1)", ""]);
}

#[test]
fn rebuild_with_cache_miss_new_view() {
	let view_data_1 = create_view_data(0, 1, 0);
	let mut view_data_2 = create_view_data(0, 1, 0);
	view_data_2.update_view_data(|updater| updater.push_line(ViewLine::from("View Data 2")));
	let mut render_slice = create_render_slice(100, 300, &view_data_1);
	let version = render_slice.get_version();
	render_slice.sync_view_data(&view_data_2);
	assert_ne!(version, render_slice.version);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}B(1)", "{Normal}View Data 2"]);
}

#[test]
fn rebuild_with_cache_miss_new_version() {
	let mut view_data = create_view_data(0, 1, 0);
	let mut render_slice = create_render_slice(100, 300, &view_data);
	let version = render_slice.get_version();
	view_data.update_view_data(|updater| updater.push_line(ViewLine::from("View Data 2")));
	render_slice.sync_view_data(&view_data);
	assert_ne!(version, render_slice.version);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}B(1)", "{Normal}View Data 2"]);
}

#[test]
fn swap_active_scroll_position() {
	let mut view_data_1 = create_view_data(0, 10, 0);
	view_data_1.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 1")));
	let mut view_data_2 = create_view_data(0, 10, 0);
	view_data_2.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 2")));
	let mut render_slice = create_render_slice(300, 6, &view_data_1);
	// first view data scroll one way
	render_slice.record_page_down();
	render_slice.sync_view_data(&view_data_1);

	// second view data scroll differently
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data_2);
	// assert scroll position of second view data
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}View Data 2",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
	]);
	// assert previous scroll position is retained
	render_slice.sync_view_data(&view_data_1);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}View Data 1",
		"{BODY}",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{Normal}B(6)",
		"{Normal}B(7)",
	]);
}

#[test]
fn swap_active_scroll_position_false_retain_scroll_position() {
	let mut view_data_1 = create_view_data(0, 10, 0);
	view_data_1.update_view_data(|updater| {
		updater.push_leading_line(ViewLine::from("View Data 1"));
		updater.set_retain_scroll_position(false);
	});
	let mut view_data_2 = create_view_data(0, 10, 0);
	view_data_2.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 2")));
	let mut render_slice = create_render_slice(300, 6, &view_data_1);
	// first view data scroll one way
	render_slice.record_page_down();
	render_slice.sync_view_data(&view_data_1);

	// second view data scroll differently
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data_2);

	// assert scroll position of first view has been reset
	render_slice.sync_view_data(&view_data_1);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}View Data 1",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
	]);
}

#[test]
fn swap_active_scroll_position_after_resize_larger() {
	let mut view_data_1 = create_view_data(0, 10, 0);
	view_data_1.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 1")));
	let mut view_data_2 = create_view_data(0, 10, 0);
	view_data_2.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 2")));
	let mut render_slice = create_render_slice(300, 6, &view_data_1);
	// first view data scroll one way
	render_slice.record_page_down();
	render_slice.sync_view_data(&view_data_1);

	// second view data scroll differently
	render_slice.record_scroll_down();
	render_slice.record_resize(250, 7);
	render_slice.sync_view_data(&view_data_2);

	// assert scroll position of second view data
	render_slice.sync_view_data(&view_data_1);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}View Data 1",
		"{BODY}",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{Normal}B(6)",
		"{Normal}B(7)",
		"{Normal}B(8)",
	]);
}

#[test]
fn swap_active_scroll_position_after_resize_smaller() {
	let mut view_data_1 = create_view_data(0, 10, 0);
	view_data_1.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 1")));
	let mut view_data_2 = create_view_data(0, 10, 0);
	view_data_2.update_view_data(|updater| updater.push_leading_line(ViewLine::from("View Data 2")));
	let mut render_slice = create_render_slice(300, 6, &view_data_1);
	// first view data scroll one way
	render_slice.record_page_down();
	render_slice.sync_view_data(&view_data_1);

	// second view data scroll differently
	render_slice.record_scroll_down();
	render_slice.record_resize(250, 5);
	render_slice.sync_view_data(&view_data_2);

	// assert scroll position of second view data
	render_slice.sync_view_data(&view_data_1);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}View Data 1",
		"{BODY}",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{Normal}B(6)",
	]);
}

#[test]
fn empty_view_data() {
	let render_slice = create_render_slice(100, 300, &ViewData::new(|_| {}));
	assert_rendered(&render_slice, &["{EMPTY}"]);
}

#[test]
fn empty_view_data_with_title_with_help() {
	let render_slice = create_render_slice(
		100,
		300,
		&ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		}),
	);
	assert_rendered(&render_slice, &["{TITLE}{HELP}", "{EMPTY}"]);
}

#[test]
fn empty_view_data_with_title_without_help() {
	let render_slice = create_render_slice(
		100,
		300,
		&ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(false);
		}),
	);
	assert_rendered(&render_slice, &["{TITLE}", "{EMPTY}"]);
}

#[test]
fn only_leading_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(1, 0, 0));
	assert_rendered(&render_slice, &["{LEADING}", "{Normal}L(1)"]);
}

#[test]
fn only_body_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(0, 1, 0));
	assert_rendered(&render_slice, &["{BODY}", "{Normal}B(1)"]);
}

#[test]
fn only_trailing_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(0, 0, 1));
	assert_rendered(&render_slice, &["{TRAILING}", "{Normal}T(1)"]);
}

#[test]
fn no_leading_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(0, 1, 1));
	assert_rendered(&render_slice, &["{BODY}", "{Normal}B(1)", "{TRAILING}", "{Normal}T(1)"]);
}

#[test]
fn no_body_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(1, 0, 1));
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{TRAILING}",
		"{Normal}T(1)",
	]);
}

#[test]
fn no_trailing_lines() {
	let render_slice = create_render_slice(100, 10, &create_view_data(1, 1, 0));
	assert_rendered(&render_slice, &["{LEADING}", "{Normal}L(1)", "{BODY}", "{Normal}B(1)"]);
}

#[test]
fn ensure_row_visible() {
	let mut view_data = create_view_data(2, 10, 2);
	view_data.update_view_data(|updater| updater.ensure_line_visible(4));
	let render_slice = create_render_slice(100, 8, &view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn ensure_column_visible() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("0123456789"));
		updater.ensure_column_visible(5);
	});
	let render_slice = create_render_slice(5, 1, &view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}12345"]);
}

#[test]
fn false_should_show_scroll_bar_zero_view_height_with_zero_padding() {
	let view_data = create_view_data(0, 10, 0);
	let render_slice = create_render_slice(100, 0, &view_data);
	assert!(!render_slice.should_show_scroll_bar());
}

#[test]
fn false_should_show_scroll_bar_zero_view_height_with_padding() {
	let view_data = create_view_data(2, 10, 2);
	let render_slice = create_render_slice(100, 4, &view_data);
	assert!(!render_slice.should_show_scroll_bar());
}

#[test]
fn false_should_show_scroll_bar_lots_of_view_height_with_zero_padding() {
	let view_data = create_view_data(0, 10, 0);
	let render_slice = create_render_slice(100, 10, &view_data);
	assert!(!render_slice.should_show_scroll_bar());
}

#[test]
fn false_should_show_scroll_bar_lots_of_view_height_with_padding() {
	let view_data = create_view_data(2, 10, 2);
	let render_slice = create_render_slice(100, 14, &view_data);
	assert!(!render_slice.should_show_scroll_bar());
}

#[test]
fn true_should_show_scroll_bar_with_zero_padding() {
	let view_data = create_view_data(0, 10, 0);
	let render_slice = create_render_slice(100, 9, &view_data);
	assert!(render_slice.should_show_scroll_bar());
}

#[test]
fn true_should_show_scroll_bar_lots_of_view_height_with_padding() {
	let view_data = create_view_data(2, 10, 2);
	let render_slice = create_render_slice(100, 13, &view_data);
	assert!(render_slice.should_show_scroll_bar());
}

#[test]
fn with_more_than_enough_view_height_for_all_lines() {
	let view_data = create_view_data(2, 3, 2);
	let render_slice = create_render_slice(100, 8, &view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_more_than_enough_view_height_for_all_lines_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 9, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_just_enough_height_for_all_lines() {
	let view_data = create_view_data(2, 3, 2);
	let render_slice = create_render_slice(100, 7, &view_data);
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_just_enough_height_for_all_lines_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 8, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_all_but_one_body_line() {
	let render_slice = create_render_slice(100, 6, &create_view_data(2, 3, 2));
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_all_but_one_body_line_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 7, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{Normal}B(2)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_one_body_line() {
	let render_slice = create_render_slice(100, 5, &create_view_data(2, 3, 2));
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_one_body_line_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 6, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{BODY}",
		"{Normal}B(1)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_no_height_for_body() {
	let render_slice = create_render_slice(100, 4, &create_view_data(2, 3, 2));
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_no_height_for_body_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 5, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{Normal}L(2)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_one_less_leading_line() {
	let render_slice = create_render_slice(100, 3, &create_view_data(2, 3, 2));
	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}L(1)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_height_for_one_less_leading_line_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 4, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{LEADING}",
		"{Normal}L(1)",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_less_height_than_body_lines_and_scrolled_down_one() {
	let view_data = create_view_data(0, 10, 0);
	let mut render_slice = create_render_slice(100, 4, &view_data);
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{BODY}",
		"{Normal}B(2)",
		"{Normal}B(3)",
		"{Normal}B(4)",
		"{Normal}B(5)",
	]);
}

#[test]
fn with_height_only_for_leading_lines() {
	let render_slice = create_render_slice(100, 2, &create_view_data(2, 3, 2));
	assert_rendered(&render_slice, &["{TRAILING}", "{Normal}T(1)", "{Normal}T(2)"]);
}

#[test]
fn with_height_only_for_leading_lines_with_title() {
	let mut view_data = create_view_data(2, 3, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 3, &view_data);
	assert_rendered(&render_slice, &[
		"{TITLE}",
		"{TRAILING}",
		"{Normal}T(1)",
		"{Normal}T(2)",
	]);
}

#[test]
fn with_zero_height() {
	let render_slice = create_render_slice(100, 0, &create_view_data(2, 2, 2));
	assert_rendered(&render_slice, &["{EMPTY}"]);
}

#[test]
fn with_height_for_title() {
	let mut view_data = create_view_data(2, 2, 2);
	view_data.update_view_data(|updater| updater.set_show_title(true));
	let render_slice = create_render_slice(100, 1, &view_data);
	assert_rendered(&render_slice, &["{TITLE}", "{EMPTY}"]);
}

#[test]
fn with_padding() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("Foo").set_padding_with_color_and_style(
			'*',
			DisplayColor::ActionPick,
			true,
			true,
			true,
		));
	});
	let render_slice = create_render_slice(8, 1, &view_data);
	assert_rendered(&render_slice, &[
		"{BODY}",
		"{Normal}Foo{ActionPick,Dimmed,Underline,Reversed}*****",
	]);
}

#[test]
fn with_padding_no_width() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("Foo").set_padding_with_color_and_style(
			'*',
			DisplayColor::ActionPick,
			true,
			true,
			true,
		));
	});
	let render_slice = create_render_slice(3, 1, &view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}Foo"]);
}

#[test]
fn with_zero_with_and_scrollbar() {
	let view_data = create_view_data(0, 10, 0);
	let render_slice = create_render_slice(0, 4, &view_data);
	assert_rendered(&render_slice, &["{BODY}", "", "", "", ""]);
}

#[test]
fn get_scroll_index_top_position() {
	let view_data = create_view_data(0, 100, 0);
	let render_slice = create_render_slice(100, 100, &view_data);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_empty_lines() {
	let view_data = ViewData::new(|_| {});
	let render_slice = create_render_slice(100, 100, &view_data);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_end_position() {
	let view_data = create_view_data(0, 100, 0);
	let mut render_slice = create_render_slice(100, 10, &view_data);
	set_scroll_position(&mut render_slice, 90);
	assert_eq!(render_slice.get_scroll_index(), 9);
}

#[test]
fn get_scroll_index_position_one_down() {
	let view_data = create_view_data(0, 100, 0);
	let mut render_slice = create_render_slice(100, 10, &view_data);
	set_scroll_position(&mut render_slice, 1);
	assert_eq!(render_slice.get_scroll_index(), 1);
}

#[test]
fn get_scroll_index_position_low_input_range_1() {
	let view_data = create_view_data(0, 10, 0);
	let mut render_slice = create_render_slice(100, 8, &view_data);
	set_scroll_position(&mut render_slice, 1);
	assert_eq!(render_slice.get_scroll_index(), 4);
}

#[test]
fn get_scroll_index_item_count_smaller_than_height() {
	let view_data = create_view_data(0, 10, 0);
	let mut render_slice = create_render_slice(100, 11, &view_data);
	set_scroll_position(&mut render_slice, 1);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_view_height_too_small() {
	let view_data = create_view_data(0, 10, 0);
	let mut render_slice = create_render_slice(100, 2, &view_data);
	set_scroll_position(&mut render_slice, 5);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_position_zero_line_zero_height() {
	let view_data = ViewData::new(|_| {});
	let render_slice = create_render_slice(100, 0, &view_data);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_two_lines_one_height_scrolled_down() {
	let view_data = create_view_data(0, 2, 0);
	let mut render_slice = create_render_slice(100, 1, &view_data);
	set_scroll_position(&mut render_slice, 1);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn get_scroll_index_two_lines_zero_height_scrolled_down() {
	let view_data = create_view_data(0, 2, 0);
	let mut render_slice = create_render_slice(100, 0, &view_data);
	set_scroll_position(&mut render_slice, 1);
	assert_eq!(render_slice.get_scroll_index(), 0);
}

#[test]
fn scroll_right_one_from_start_long_leading() {
	let view_data = ViewData::new(|updater| {
		updater.push_leading_line(ViewLine::from("L(123456789)"));
		updater.push_line(ViewLine::from("B(123)"));
		updater.push_trailing_line(ViewLine::from("T(123)"));
	});
	let mut render_slice = create_render_slice(6, 10, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}(12345",
		"{BODY}",
		"{Normal}(123)",
		"{TRAILING}",
		"{Normal}(123)",
	]);
}

#[test]
fn scroll_right_one_from_start_long_body() {
	let view_data = ViewData::new(|updater| {
		updater.push_leading_line(ViewLine::from("L(123)"));
		updater.push_line(ViewLine::from("B(123456789)"));
		updater.push_trailing_line(ViewLine::from("T(123)"));
	});
	let mut render_slice = create_render_slice(6, 10, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}(123)",
		"{BODY}",
		"{Normal}(12345",
		"{TRAILING}",
		"{Normal}(123)",
	]);
}

#[test]
fn scroll_right_one_from_start_long_trailing() {
	let view_data = ViewData::new(|updater| {
		updater.push_leading_line(ViewLine::from("L(123)"));
		updater.push_line(ViewLine::from("B(123)"));
		updater.push_trailing_line(ViewLine::from("T(123456789)"));
	});
	let mut render_slice = create_render_slice(6, 10, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}(123)",
		"{BODY}",
		"{Normal}(123)",
		"{TRAILING}",
		"{Normal}(12345",
	]);
}

#[test]
fn scroll_right_one_from_start_all_same_length() {
	let view_data = ViewData::new(|updater| {
		updater.push_leading_line(ViewLine::from("L(123456789)"));
		updater.push_line(ViewLine::from("B(123456789)"));
		updater.push_trailing_line(ViewLine::from("T(123456789)"));
	});
	let mut render_slice = create_render_slice(6, 10, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}(12345",
		"{BODY}",
		"{Normal}(12345",
		"{TRAILING}",
		"{Normal}(12345",
	]);
}

#[test]
fn scroll_one_right_with_scrollbar() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("B(123456789)"));
		updater.push_line(ViewLine::from("B(123456789)"));
	});
	let mut render_slice = create_render_slice(3, 1, &view_data);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &["{BODY}", "{Normal}(1"]);
}

#[test]
fn scroll_right_to_end_with_scrollbar() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("B(123456789)"));
		updater.push_line(ViewLine::from("B(123456789)"));
	});
	let mut render_slice = create_render_slice(3, 1, &view_data);
	for _ in 0..10 {
		render_slice.record_scroll_right();
	}
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &["{BODY}", "{Normal}9)"]);
}

#[test]
fn scroll_right_to_end_all_same_length() {
	let view_data = ViewData::new(|updater| {
		updater.push_leading_line(ViewLine::from("L(123456789)"));
		updater.push_line(ViewLine::from("B(123456789)"));
		updater.push_trailing_line(ViewLine::from("T(123456789)"));
	});
	let mut render_slice = create_render_slice(6, 10, &view_data);
	for _ in 0..10 {
		render_slice.record_scroll_right();
	}
	render_slice.sync_view_data(&view_data);

	assert_rendered(&render_slice, &[
		"{LEADING}",
		"{Normal}56789)",
		"{BODY}",
		"{Normal}56789)",
		"{TRAILING}",
		"{Normal}56789)",
	]);
}

#[test]
fn scroll_down_trigger_shorter_line_width_smaller_than_view_width() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("123456789"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
	});
	let mut render_slice = create_render_slice(5, 3, &view_data);
	for _ in 0..8 {
		render_slice.record_scroll_right();
	}
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{BODY}",
		"{Normal}1234",
		"{Normal}1234",
		"{Normal}1234",
	]);
}

#[test]
fn scroll_down_trigger_shorter_line_width_larger_than_view_width() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("123456789"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
	});
	let mut render_slice = create_render_slice(5, 3, &view_data);
	for _ in 0..8 {
		render_slice.record_scroll_right();
	}
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &[
		"{BODY}",
		"{Normal}3456",
		"{Normal}3456",
		"{Normal}3456",
	]);
}

#[test]
fn scroll_down_trigger_shorter_longest_line_now_visible() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("123456789"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("123456"));
	});
	let mut render_slice = create_render_slice(5, 3, &view_data);
	for _ in 0..8 {
		render_slice.record_scroll_right();
	}
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}34", "{Normal}34", "{Normal}3456"]);
}

#[test]
fn scroll_down_trigger_shorter_longest_line_previous_second() {
	let view_data = ViewData::new(|updater| {
		updater.push_line(ViewLine::from("123456789"));
		updater.push_line(ViewLine::from("123456"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
		updater.push_line(ViewLine::from("1234"));
	});
	let mut render_slice = create_render_slice(5, 3, &view_data);
	for _ in 0..8 {
		render_slice.record_scroll_right();
	}
	render_slice.record_scroll_down();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}3456", "{Normal}34", "{Normal}34"]);
}

#[test]
fn scroll_horizontal_with_segments() {
	let view_data = ViewData::new(|updater| {
		let segments = vec![
			LineSegment::new("1"),
			LineSegment::new("23"),
			LineSegment::new("456"),
			LineSegment::new("7890"),
			LineSegment::new("abcde"),
		];
		updater.push_line(ViewLine::from(segments));
	});
	let mut render_slice = create_render_slice(3, 10, &view_data);
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}1{Normal}23"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}23{Normal}4"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}3{Normal}45"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}456"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}56{Normal}7"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}6{Normal}78"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}789"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}890"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}90{Normal}a"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}0{Normal}ab"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}abc"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}bcd"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}cde"]);
	render_slice.record_scroll_right();
	render_slice.sync_view_data(&view_data);
	assert_rendered(&render_slice, &["{BODY}", "{Normal}cde"]);
}

#[test]
fn calculate_max_line_length_max_first() {
	let view_lines = [
		ViewLine::from(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
		ViewLine::from("012345"),
	];
	assert_eq!(RenderSlice::calculate_max_line_length(&view_lines, 0, 1), 16);
}

#[test]
fn calculate_max_line_length_max_last() {
	let view_lines = [
		ViewLine::from("012345"),
		ViewLine::from(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
	];
	assert_eq!(RenderSlice::calculate_max_line_length(&view_lines, 0, 2), 16);
}

#[test]
fn calculate_max_line_length_with_slice() {
	let view_lines = [
		ViewLine::from("012345"),
		ViewLine::from("012345"),
		ViewLine::from(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
		ViewLine::from(vec![LineSegment::new("0123456789"), LineSegment::new("01234567")]),
	];
	assert_eq!(RenderSlice::calculate_max_line_length(&view_lines, 1, 2), 16);
}

#[test]
fn calculate_max_line_length_ignore_pinned() {
	let view_lines = [
		ViewLine::from("012345"),
		ViewLine::from("012345"),
		ViewLine::from(vec![LineSegment::new("0123456789"), LineSegment::new("012345")]),
		ViewLine::new_pinned(vec![LineSegment::new("0123456789"), LineSegment::new("01234567")]),
	];
	assert_eq!(RenderSlice::calculate_max_line_length(&view_lines, 0, 4), 16);
}
