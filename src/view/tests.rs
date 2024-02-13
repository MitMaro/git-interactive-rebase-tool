use super::*;
use crate::{config::Theme, display::Size, test_helpers::mocks};

fn assert_render(width: usize, height: usize, view_data: &ViewData, expected: &[&str]) {
	let theme = Theme::new();
	let mut crossterm = mocks::CrossTerm::new();
	let readonly_tui = crossterm.clone();
	crossterm.set_size(Size::new(width, height));
	let display = Display::new(crossterm, &theme);
	let mut view = View::new(display, "~", "?");

	let mut render_slice = RenderSlice::new();
	render_slice.record_resize(width, height);
	render_slice.sync_view_data(view_data);
	view.render(&render_slice).unwrap();
	assert_eq!(readonly_tui.get_output().join(""), format!("{}\n", expected.join("\n")));
}

#[test]
fn render_empty() {
	assert_render(20, 10, &ViewData::new(|_| {}), &["~"; 10]);
}

#[test]
fn render_title_full_width() {
	let mut expected = vec!["Git Interactive Rebase Tool        "];
	expected.extend(vec!["~"; 9]);
	assert_render(
		35,
		10,
		&ViewData::new(|updater| updater.set_show_title(true)),
		&expected,
	);
}

#[test]
fn render_title_short_title() {
	let mut expected = vec!["Git Rebase                "];
	expected.extend(vec!["~"; 9]);
	assert_render(
		26,
		10,
		&ViewData::new(|updater| updater.set_show_title(true)),
		&expected,
	);
}

#[test]
fn render_title_full_width_with_help() {
	let mut expected = vec!["Git Interactive Rebase Tool Help: ?"];
	expected.extend(vec!["~"; 9]);
	assert_render(
		35,
		10,
		&ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		}),
		&expected,
	);
}

#[test]
fn render_title_full_width_with_help_enabled_but_not_enough_length() {
	let mut expected = vec!["Git Interactive Rebase Tool       "];
	expected.extend(vec!["~"; 9]);
	assert_render(
		34,
		10,
		&ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		}),
		&expected,
	);
}

#[test]
fn render_leading_lines() {
	let mut expected = vec!["This is a leading line"];
	expected.extend(vec!["~"; 9]);
	assert_render(
		30,
		10,
		&ViewData::new(|updater| {
			updater.push_leading_line(ViewLine::from("This is a leading line"));
		}),
		&expected,
	);
}

#[test]
fn render_normal_lines() {
	let mut expected = vec!["This is a line"];
	expected.extend(vec!["~"; 9]);
	assert_render(
		30,
		10,
		&ViewData::new(|updater| {
			updater.push_line(ViewLine::from("This is a line"));
		}),
		&expected,
	);
}

#[test]
fn render_tailing_lines() {
	let mut expected = vec!["~"; 9];
	expected.push("This is a trailing line");
	assert_render(
		30,
		10,
		&ViewData::new(|updater| {
			updater.push_trailing_line(ViewLine::from("This is a trailing line"));
		}),
		&expected,
	);
}

#[test]
fn render_all_lines() {
	let mut expected = vec!["This is a leading line", "This is a line"];
	expected.extend(vec!["~"; 7]);
	expected.push("This is a trailing line");
	assert_render(
		30,
		10,
		&ViewData::new(|updater| {
			updater.push_leading_line(ViewLine::from("This is a leading line"));
			updater.push_line(ViewLine::from("This is a line"));
			updater.push_trailing_line(ViewLine::from("This is a trailing line"));
		}),
		&expected,
	);
}

#[test]
fn render_with_full_screen_data() {
	assert_render(
		30,
		6,
		&ViewData::new(|updater| {
			updater.push_leading_line(ViewLine::from("This is a leading line"));
			updater.push_line(ViewLine::from("This is line 1"));
			updater.push_line(ViewLine::from("This is line 2"));
			updater.push_line(ViewLine::from("This is line 3"));
			updater.push_line(ViewLine::from("This is line 4"));
			updater.push_trailing_line(ViewLine::from("This is a trailing line"));
		}),
		&[
			"This is a leading line",
			"This is line 1",
			"This is line 2",
			"This is line 3",
			"This is line 4",
			"This is a trailing line",
		],
	);
}

#[test]
fn render_with_scroll_bar() {
	assert_render(
		30,
		6,
		&ViewData::new(|updater| {
			updater.push_leading_line(ViewLine::from("This is a leading line"));
			updater.push_line(ViewLine::from("This is line 1"));
			updater.push_line(ViewLine::from("This is line 2"));
			updater.push_line(ViewLine::from("This is line 3"));
			updater.push_line(ViewLine::from("This is line 4"));
			updater.push_line(ViewLine::from("This is line 5"));
			updater.push_trailing_line(ViewLine::from("This is a trailing line"));
		}),
		&[
			"This is a leading line",
			"This is line 1█",
			"This is line 2 ",
			"This is line 3 ",
			"This is line 4 ",
			"This is a trailing line",
		],
	);
}
