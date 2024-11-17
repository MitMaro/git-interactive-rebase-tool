use std::{thread::sleep, time::Duration};

use claims::assert_some_eq;

use super::*;
use crate::{
	action_line,
	assert_rendered_output,
	assert_results,
	input::KeyCode,
	modules::list::search::LineMatch,
	process::Artifact,
	search::Interrupter,
	test_helpers::{assertions::AnyArtifact, create_test_keybindings, testers::ModuleTestContext},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Action<'action> {
	Start(&'action str),
	Search,
	Finish,
	Next,
	Previous,
	Cancel,
	Event(Event),
}

struct TestContext {
	list: List,
	module_test_context: ModuleTestContext,
	results: Vec<Results>,
	key_bindings: KeyBindings,
}

impl TestContext {
	fn build_view_data(&mut self) -> &ViewData {
		self.module_test_context.build_view_data(&mut self.list)
	}

	fn handle_event(&mut self, event: Event) {
		self.results.push(self.list.handle_event(
			self.list.read_event(event, &self.key_bindings),
			&self.module_test_context.view_context.state,
		));
	}
}

fn search_test<C>(actions: &[Action<'_>], lines: &[&str], callback: C)
where C: FnOnce(TestContext) {
	testers::module(lines, &[], |mut context| {
		let list = create_list(&create_config(), context.take_todo_file());
		let mut search_context = TestContext {
			list,
			module_test_context: context,
			results: vec![],
			key_bindings: create_test_keybindings(),
		};

		let mut finish_pushed = false;

		for action in actions {
			match *action {
				Action::Start(term) => {
					finish_pushed = false;
					search_context.handle_event(Event::from(StandardEvent::SearchStart));
					for c in term.chars() {
						search_context.handle_event(Event::from(c));
					}
				},
				Action::Search => {
					if let Some(term) = search_context.list.search_bar.search_value() {
						_ = search_context
							.list
							.search
							.search(Interrupter::new(Duration::from_secs(5)), term);
						search_context.handle_event(Event::from(StandardEvent::SearchUpdate));
					}
				},
				Action::Finish => {
					finish_pushed = true;
					search_context.handle_event(Event::from(StandardEvent::SearchFinish));
				},
				Action::Next => {
					if !finish_pushed {
						search_context.handle_event(Event::from(StandardEvent::SearchFinish));
					}
					search_context.handle_event(Event::from(StandardEvent::SearchNext));
				},
				Action::Previous => {
					if !finish_pushed {
						search_context.handle_event(Event::from(StandardEvent::SearchFinish));
					}
					search_context.handle_event(Event::from(StandardEvent::SearchPrevious));
				},
				Action::Cancel => {
					search_context.handle_event(Event::from(KeyCode::Esc));
				},
				Action::Event(event) => {
					search_context.handle_event(event);
				},
			}
		}

		callback(search_context);
	});
}

#[test]
fn render_start() {
	search_test(&[Action::Start("")], &["pick aaaaaaaa comment"], |mut test_context| {
		assert_rendered_output!(
			Style test_context.build_view_data(),
			"{TITLE}{HELP}",
			"{BODY}",
			"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa comment{Pad( )}",
			"{TRAILING}",
			"{Normal}/{Normal,Underline}"
		);
	});
}

#[test]
fn render_match_hash() {
	search_test(
		&[Action::Start("aaa"), Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				Style test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {IndicatorColor}aaaaaaaa{Normal} comment{Pad( )}",
				"{TRAILING}",
				"{Normal}/aaa{Normal,Underline}"
			);
		},
	);
}

#[test]
fn render_match_content_start() {
	search_test(
		&[Action::Start("com"), Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				Style test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa {IndicatorColor}com{Normal}ment{Pad( )}",
				"{TRAILING}",
				"{Normal}/com{Normal,Underline}"
			);
		},
	);
}

#[test]
fn render_match_content_middle() {
	search_test(
		&[Action::Start("omm"), Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				Style test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa c{IndicatorColor}omm{Normal}ent{Pad( )}",
				"{TRAILING}",
				"{Normal}/omm{Normal,Underline}"
			);
		},
	);
}

#[test]
fn render_match_content_end() {
	search_test(
		&[Action::Start("ent"), Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				Style test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa comm{IndicatorColor}ent{Normal}{Pad( )}",
				"{TRAILING}",
				"{Normal}/ent{Normal,Underline}"
			);
		},
	);
}

#[test]
fn render_match_content_full() {
	search_test(
		&[Action::Start("comment"), Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				Style test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa {IndicatorColor}comment{Normal}{Pad( )}",
				"{TRAILING}",
				"{Normal}/comment{Normal,Underline}"
			);
		},
	);
}

#[test]
fn render_match_finish_with_search_active() {
	search_test(
		&[Action::Start("aaa"), Action::Search, Action::Finish],
		&[
			"pick aaaaaaaa comment",
			"pick aaaaaaab comment",
			"pick aaaaaaac comment",
		],
		|mut test_context| {
			assert_rendered_output!(
				test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				action_line!(Selected Pick "aaaaaaaa", "comment"),
				action_line!(Pick "aaaaaaab", "comment"),
				action_line!(Pick "aaaaaaac", "comment"),
				"{TRAILING}",
				"[aaa]: 1/3 Searching [(-)]"
			);
		},
	);
}

#[test]
fn render_match_finish_with_search_complete() {
	search_test(
		&[Action::Start("aaa"), Action::Search, Action::Finish, Action::Search],
		&[
			"pick aaaaaaaa comment",
			"pick aaaaaaab comment",
			"pick aaaaaaac comment",
		],
		|mut test_context| {
			assert_rendered_output!(
				test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				action_line!(Selected Pick "aaaaaaaa", "comment"),
				action_line!(Pick "aaaaaaab", "comment"),
				action_line!(Pick "aaaaaaac", "comment"),
				"{TRAILING}",
				"[aaa]: 1/3"
			);
		},
	);
}

#[test]
fn render_no_results() {
	search_test(
		&[Action::Start("xxx"), Action::Search, Action::Finish, Action::Search],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_rendered_output!(
				test_context.build_view_data(),
				"{TITLE}{HELP}",
				"{BODY}",
				action_line!(Selected Pick "aaaaaaaa", "comment"),
				"{TRAILING}",
				"[xxx]: No Results"
			);
		},
	);
}

#[test]
fn search_indicator_refresh_on_update() {
	search_test(&[Action::Start("")], &["pick aaaaaaaa comment"], |mut test_context| {
		let cur_indicator = test_context.list.spin_indicator.indicator();
		sleep(Duration::from_millis(200)); // indicator only changes every 100 ms
		test_context.list.search_update();
		assert_ne!(test_context.list.spin_indicator.indicator(), cur_indicator);
	});
}

#[test]
fn start_edit() {
	search_test(&[Action::Start("")], &["pick aaaaaaaa comment"], |test_context| {
		assert!(test_context.list.search_bar.is_active());
	});
}

#[test]
fn term_entry() {
	search_test(
		&[Action::Start("aaa")],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_results!(
				test_context.results.pop().unwrap(),
				AnyArtifact,
				Artifact::SearchTerm(String::from("aaa"))
			);
		},
	);
}

#[test]
fn term_entry_delete_last_character() {
	search_test(
		&[Action::Start("a"), Action::Event(Event::from(KeyCode::Backspace))],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_results!(test_context.results.pop().unwrap(), AnyArtifact, Artifact::SearchCancel);
		},
	);
}

#[test]
fn start_search_with_empty_term() {
	search_test(
		&[Action::Start(""), Action::Search, Action::Finish],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_results!(test_context.results.pop().unwrap(), AnyArtifact, Artifact::SearchCancel);
			assert!(!test_context.list.search_bar.is_active());
		},
	);
}

#[test]
fn start_search_with_term() {
	search_test(
		&[Action::Start("aaa"), Action::Search, Action::Finish],
		&["pick aaaaaaaa comment"],
		|mut test_context| {
			assert_results!(
				test_context.results.pop().unwrap(),
				AnyArtifact,
				Artifact::SearchTerm(String::from("aaa"))
			);
			assert_some_eq!(test_context.list.search.current_match(), LineMatch::new(0, true, false));
		},
	);
}

#[test]
fn next() {
	search_test(
		&[Action::Start("aaa"), Action::Search, Action::Finish, Action::Next],
		&[
			"pick aaaaaaaa comment1",
			"pick aaaaaaaa comment2",
			"pick aaaaaaaa comment3",
		],
		|mut test_context| {
			assert_results!(
				test_context.results.pop().unwrap(),
				AnyArtifact,
				Artifact::SearchTerm(String::from("aaa"))
			);
			assert_some_eq!(test_context.list.search.current_match(), LineMatch::new(1, true, false));
		},
	);
}

#[test]
fn previous() {
	search_test(
		&[Action::Start("aaa"), Action::Search, Action::Finish, Action::Previous],
		&[
			"pick aaaaaaaa comment1",
			"pick aaaaaaaa comment2",
			"pick aaaaaaaa comment3",
		],
		|mut test_context| {
			assert_results!(
				test_context.results.pop().unwrap(),
				AnyArtifact,
				Artifact::SearchTerm(String::from("aaa"))
			);
			assert_some_eq!(test_context.list.search.current_match(), LineMatch::new(2, true, false));
		},
	);
}

#[test]
fn cancel() {
	search_test(
		&[Action::Start("aaa"), Action::Cancel],
		&["pick aaaaaaaa comment1"],
		|mut test_context| {
			assert_results!(test_context.results.pop().unwrap(), AnyArtifact, Artifact::SearchCancel);
		},
	);
}

#[test]
fn ignored_event() {
	search_test(
		&[Action::Start("a"), Action::Event(Event::from(KeyCode::Null))],
		&["pick aaaaaaaa comment1"],
		|mut test_context| {
			assert!(test_context.results.pop().unwrap().artifacts.is_empty());
		},
	);
}
