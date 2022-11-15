use claim::{assert_none, assert_some_eq};
use view::{assert_rendered_output, ViewData};

use super::*;

fn create_view_data(search_bar: &SearchBar) -> ViewData {
	let view_line = search_bar.build_view_line();
	ViewData::new(|updater| updater.push_line(view_line))
}

#[test]
fn new() {
	let search_bar = SearchBar::new();
	assert_eq!(search_bar.state, State::Deactivated);
	assert_rendered_output!(&create_view_data(&search_bar), "{BODY}", "{Normal}/{Normal,Underline}");
}

#[test]
fn start_search_without_initial_value() {
	let mut search_bar = SearchBar::new();
	search_bar.editable_line.set_read_only(true);
	search_bar.start_search(None);
	assert_eq!(search_bar.state, State::Editing);
	assert_rendered_output!(&create_view_data(&search_bar), "{BODY}", "{Normal}/{Normal,Underline}");
}

#[test]
fn start_search_with_initial_value() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	assert_rendered_output!(
		&create_view_data(&search_bar),
		"{BODY}",
		"{Normal}/foo{Normal,Underline}"
	);
}

#[test]
fn start_search_with_previous_value_and_without_initial_value() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	search_bar.start_search(None);
	assert_rendered_output!(
		&create_view_data(&search_bar),
		"{BODY}",
		"{Normal}/foo{Normal,Underline}"
	);
}

#[test]
fn reset() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(None);
	search_bar.reset();
	assert_eq!(search_bar.state, State::Deactivated);
}

#[test]
fn input_options_deactivated() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Deactivated;
	assert_none!(search_bar.input_options());
}

#[test]
fn input_options_editing() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	assert_some_eq!(search_bar.input_options(), &INPUT_OPTIONS_EDITING);
}

#[test]
fn input_options_searching() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Searching;
	assert_some_eq!(search_bar.input_options(), &INPUT_OPTIONS_SEARCHING);
}

#[test]
fn read_event_deactivated() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Deactivated;
	assert_none!(search_bar.read_event(Event::from('a')));
}

#[test]
fn read_event_searching() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Searching;
	assert_none!(search_bar.read_event(Event::from('a')));
}

#[test]
fn read_event_editing() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	assert_some_eq!(search_bar.read_event(Event::from('a')), Event::from('a'));
}

#[test]
fn handle_event_inactive() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Deactivated;
	assert_eq!(search_bar.handle_event(Event::from('a')), SearchBarAction::None);
}

#[test]
fn handle_event_search_next() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from(StandardEvent::SearchNext);
	assert_eq!(
		search_bar.handle_event(event),
		SearchBarAction::Next(String::from("foo"))
	);
}

#[test]
fn handle_event_search_previous() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from(StandardEvent::SearchPrevious);
	assert_eq!(
		search_bar.handle_event(event),
		SearchBarAction::Previous(String::from("foo"))
	);
}

#[test]
fn handle_event_search_finish() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from(StandardEvent::SearchFinish);
	assert_eq!(
		search_bar.handle_event(event),
		SearchBarAction::Start(String::from("foo"))
	);
	assert_eq!(search_bar.state, State::Searching);
	assert_rendered_output!(&create_view_data(&search_bar), "{BODY}", "{Normal}foo");
}

#[test]
fn handle_event_search_finish_with_enter() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from(KeyCode::Enter);
	assert_eq!(
		search_bar.handle_event(event),
		SearchBarAction::Start(String::from("foo"))
	);
}

#[test]
fn handle_event_search_start_active() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from(StandardEvent::SearchStart);
	assert_eq!(search_bar.handle_event(event), SearchBarAction::Cancel);
	assert_eq!(search_bar.state, State::Deactivated);
}

#[test]
fn handle_event_other_active() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	let event = Event::from('a');
	assert_eq!(search_bar.handle_event(event), SearchBarAction::None);
	assert_rendered_output!(
		&create_view_data(&search_bar),
		"{BODY}",
		"{Normal}/fooa{Normal,Underline}"
	);
}

#[test]
fn handle_event_other_inactive() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	search_bar.state = State::Deactivated;
	let event = Event::from('a');
	assert_eq!(search_bar.handle_event(event), SearchBarAction::None);
	assert_rendered_output!(
		&create_view_data(&search_bar),
		"{BODY}",
		"{Normal}/foo{Normal,Underline}"
	);
}

#[test]
fn search_value_deactivated() {
	let mut search_bar = SearchBar::new();
	search_bar.start_search(Some("foo"));
	search_bar.state = State::Deactivated;
	assert_none!(search_bar.search_value());
}

#[test]
fn search_value_editing() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	search_bar.start_search(Some("foo"));
	assert_some_eq!(search_bar.search_value(), "foo");
}

#[test]
fn search_value_searching() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Searching;
	search_bar.start_search(Some("foo"));
	assert_some_eq!(search_bar.search_value(), "foo");
}

#[test]
fn is_active() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	assert!(search_bar.is_active());
	search_bar.state = State::Deactivated;
	assert!(!search_bar.is_active());
	search_bar.state = State::Searching;
	assert!(search_bar.is_active());
}

#[test]
fn is_editing() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	assert!(search_bar.is_editing());
	search_bar.state = State::Deactivated;
	assert!(!search_bar.is_editing());
	search_bar.state = State::Searching;
	assert!(!search_bar.is_editing());
}

#[test]
fn is_searching() {
	let mut search_bar = SearchBar::new();
	search_bar.state = State::Editing;
	assert!(!search_bar.is_searching());
	search_bar.state = State::Deactivated;
	assert!(!search_bar.is_searching());
	search_bar.state = State::Searching;
	assert!(search_bar.is_searching());
}

#[test]
fn build_view_line() {
	assert_rendered_output!(
		&create_view_data(&SearchBar::new()),
		"{BODY}",
		"{Normal}/{Normal,Underline}"
	);
}
