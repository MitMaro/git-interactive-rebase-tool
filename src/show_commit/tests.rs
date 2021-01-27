use super::*;
use crate::assert_process_result;
use crate::assert_rendered_output;
use crate::display::size::Size;
use crate::process::testutil::{process_module_test, TestContext, ViewState};
use crate::show_commit::delta::Delta;
use crate::show_commit::diff_line::DiffLine;
use crate::show_commit::file_stat::FileStat;
use crate::show_commit::origin::Origin;
use crate::show_commit::status::Status;
use crate::show_commit::user::User;
use anyhow::anyhow;
use chrono::Local;

fn create_minimal_commit() -> Commit {
	Commit {
		author: User::new(None, None),
		body: None,
		committer: User::new(None, None),
		date: Local::now(),
		file_stats: vec![],
		hash: String::from("0123456789abcdef0123456789abcdef"),
		number_files_changed: 0,
		insertions: 0,
		deletions: 0,
	}
}

#[test]
#[serial_test::serial]
fn load_commit_during_activate() {
	process_module_test(
		&["pick 18d82dcc4c36cade807d7cf79700b6bbad8080b9 comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert!(module.commit.is_some());
		},
	);
}

#[test]
#[serial_test::serial]
fn cached_commit_in_activate() {
	process_module_test(
		&["pick 18d82dcc4c36cade807d7cf79700b6bbad8080b9 comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.activate(&mut module, State::List));
		},
	);
}

#[test]
#[serial_test::serial]
fn activate_error() {
	process_module_test(
		&["pick aaaaaaaaaa comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(
				test_context.activate(&mut module, State::List),
				state = State::List,
				error = anyhow!(
					"Error loading commit: aaaaaaaaaa: revspec 'aaaaaaaaaa' not found; class=Reference (4); \
					 code=NotFound (-3)"
				)
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_minimal_commit() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_minimal_commit_compact() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(33, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{Normal}01234567",
				"{BODY}",
				format!("{{IndicatorColor}}D: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_author() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.author = User::new(Some("John Doe"), Some("john.doe@example.com"));
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"{IndicatorColor}Author: {Normal}John Doe <john.doe@example.com>",
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_author_compact() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(33, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.author = User::new(Some("John Doe"), Some("john.doe@example.com"));
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{Normal}01234567",
				"{BODY}",
				format!("{{IndicatorColor}}D: {{Normal}}{}", commit_date).as_str(),
				"{IndicatorColor}A: {Normal}John Doe <john.doe@example.com",
				"",
				"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_committer() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.committer = User::new(Some("John Doe"), Some("john.doe@example.com"));
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"{IndicatorColor}Committer: {Normal}John Doe <john.doe@example.com>",
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_committer_compact() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(33, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.committer = User::new(Some("John Doe"), Some("john.doe@example.com"));
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{Normal}01234567",
				"{BODY}",
				format!("{{IndicatorColor}}D: {{Normal}}{}", commit_date).as_str(),
				"{IndicatorColor}C: {Normal}John Doe <john.doe@example.com",
				"",
				"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_commit_body() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.body = Some(String::from("Commit title\n\nCommit body"));
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"{Normal}Commit title",
				"",
				"{Normal}Commit body",
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_file_stats() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.file_stats = vec![
				FileStat::new("file.1a", "file.1b", Status::Renamed),
				FileStat::new("file.2a", "file.2a", Status::Added),
				FileStat::new("file.3a", "file.3a", Status::Deleted),
				FileStat::new("file.4a", "file.4b", Status::Copied),
				FileStat::new("file.5a", "file.5a", Status::Modified),
				FileStat::new("file.6a", "file.6a", Status::Typechange),
				FileStat::new("file.7a", "file.7b", Status::Other),
			];
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{DiffChangeColor} renamed: {DiffRemoveColor}file.1b{Normal} → {DiffAddColor}file.1a",
				"{DiffAddColor}   added: {DiffAddColor}file.2a",
				"{DiffRemoveColor} deleted: {DiffRemoveColor}file.3a",
				"{DiffAddColor}  copied: {Normal}file.4b{Normal} → {DiffAddColor}file.4a",
				"{DiffChangeColor}modified: {DiffChangeColor}file.5a",
				"{DiffChangeColor} changed: {DiffChangeColor}file.6a",
				"{Normal} unknown: {Normal}file.7a"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_with_file_stats_compact() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(33, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.file_stats = vec![
				FileStat::new("file.1a", "file.1b", Status::Renamed),
				FileStat::new("file.2a", "file.2a", Status::Added),
				FileStat::new("file.3a", "file.3a", Status::Deleted),
				FileStat::new("file.4a", "file.4b", Status::Copied),
				FileStat::new("file.5a", "file.5a", Status::Modified),
				FileStat::new("file.6a", "file.6a", Status::Typechange),
				FileStat::new("file.7a", "file.7b", Status::Other),
			];
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{Normal}01234567",
				"{BODY}",
				format!("{{IndicatorColor}}D: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0",
				"{DiffChangeColor}R {DiffRemoveColor}file.1b{Normal}→{DiffAddColor}file.1a",
				"{DiffAddColor}A {DiffAddColor}file.2a",
				"{DiffRemoveColor}D {DiffRemoveColor}file.3a",
				"{DiffAddColor}C {Normal}file.4b{Normal}→{DiffAddColor}file.4a",
				"{DiffChangeColor}M {DiffChangeColor}file.5a",
				"{DiffChangeColor}T {DiffChangeColor}file.6a",
				"{Normal}X {Normal}file.7a"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_single_file_changed() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.number_files_changed = 1;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}1{Normal} file{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_more_than_one_file_changed() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.number_files_changed = 2;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}2{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_single_insertion() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.insertions = 1;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}1{Normal} insertion{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_more_than_one_insertion() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.insertions = 2;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}2{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_single_deletion() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.deletions = 1;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}1{Normal} deletion"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_overview_more_than_one_deletion() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let commit_date = commit.get_date().format("%c %z").to_string();
			commit.deletions = 2;
			module.commit = Some(commit);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{BODY}",
				format!("{{IndicatorColor}}Date: {{Normal}}{}", commit_date).as_str(),
				"",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}2{Normal} deletions"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_minimal_commit() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			module.commit = Some(create_minimal_commit());
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_minimal_commit_compact() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(33, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			module.commit = Some(create_minimal_commit());
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{Normal}01234567",
				"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0",
				"{BODY}",
				"{Normal}{Pad ―,99}"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_basic_file_stats() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			commit.file_stats = vec![
				FileStat::new("file.1a", "file.1b", Status::Renamed),
				FileStat::new("file.2a", "file.2a", Status::Added),
				FileStat::new("file.3a", "file.3a", Status::Deleted),
				FileStat::new("file.4a", "file.4b", Status::Copied),
				FileStat::new("file.5a", "file.5a", Status::Modified),
				FileStat::new("file.6a", "file.6a", Status::Typechange),
				FileStat::new("file.7a", "file.7b", Status::Other),
			];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor} renamed: {DiffRemoveColor}file.1b{Normal} → {DiffAddColor}file.1a",
				"{Normal}{Pad ―,150}",
				"{DiffAddColor}   added: {DiffAddColor}file.2a",
				"{Normal}{Pad ―,150}",
				"{DiffRemoveColor} deleted: {DiffRemoveColor}file.3a",
				"{Normal}{Pad ―,150}",
				"{DiffAddColor}  copied: {Normal}file.4b{Normal} → {DiffAddColor}file.4a",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.5a",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor} changed: {DiffChangeColor}file.6a",
				"{Normal}{Pad ―,150}",
				"{Normal} unknown: {Normal}file.7a"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_end_new_line_missing() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
			delta.add_line(DiffLine::new(Origin::Addition, "", None, Some(15), true));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line",
				"{Normal}       {DiffContextColor}\\ No newline at end of file"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_add_line() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_delete_line() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_context_add_remove_lines() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Context, "context 1", Some(13), Some(13), false));
			delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));
			delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
			delta.add_line(DiffLine::new(Origin::Context, "context 2", Some(15), Some(15), false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}13{Normal} {Normal}13{Normal}| {DiffContextColor}context 1",
				"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line",
				"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line",
				"{Normal}15{Normal} {Normal}15{Normal}| {DiffContextColor}context 2"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_add_line_with_show_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_delete_line_with_show_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_context_add_remove_lines_with_show_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
			delta.add_line(DiffLine::new(Origin::Context, "context 1", Some(13), Some(13), false));
			delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));
			delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
			delta.add_line(DiffLine::new(Origin::Context, "context 2", Some(15), Some(15), false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}13{Normal} {Normal}13{Normal}| {DiffContextColor}context 1",
				"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line",
				"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line",
				"{Normal}15{Normal} {Normal}15{Normal}| {DiffContextColor}context 2"
			);
		},
	);
}

fn generate_white_space_delta() -> Delta {
	let mut delta = Delta::new("@@ -1,7 +1,7 @@ context", 1, 1, 7, 7);
	// leading tabs
	delta.add_line(generate_diff_line_context("\t\tsp tabs\t\tcontent", 1));
	// trailing tabs
	delta.add_line(generate_diff_line_context("sp tabs\t\tcontent\t\t", 2));
	// leading and trailing tabs
	delta.add_line(generate_diff_line_context("\t\tsp tabs\t\tcontent\t\t", 3));
	// leading spaces
	delta.add_line(generate_diff_line_context("    sp tabs\t\tcontent", 4));
	// trailing spaces
	delta.add_line(generate_diff_line_context("sp tabs\t\tcontent    ", 5));
	// leading and trailing spaces
	delta.add_line(generate_diff_line_context("    sp tabs\t\tcontent    ", 6));
	// mix of spaces and tabs
	delta.add_line(generate_diff_line_context(" \t\t sp tabs\t\tcontent\t  \t", 7));
	delta
}

fn generate_diff_line_context(content: &str, line_num: u32) -> DiffLine {
	DiffLine::new(Origin::Context, content, Some(line_num), Some(line_num), false)
}

#[test]
#[serial_test::serial]
fn render_diff_show_both_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
			config.diff_tab_symbol = String::from("#");
			config.diff_space_symbol = String::from("%");
			config.diff_tab_width = 2;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			file_stat.add_delta(generate_white_space_delta());
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -1,7 +1,7 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}1{Normal} {Normal}1{Normal}| {DiffWhitespaceColor}# # {DiffContextColor}sp tabs    content",
				"{Normal}2{Normal} {Normal}2{Normal}| {DiffContextColor}sp tabs    content{DiffWhitespaceColor}# # ",
				"{Normal}3{Normal} {Normal}3{Normal}| {DiffWhitespaceColor}# # {DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}# # ",
				"{Normal}4{Normal} {Normal}4{Normal}| {DiffWhitespaceColor}%%%%{DiffContextColor}sp tabs    content",
				"{Normal}5{Normal} {Normal}5{Normal}| {DiffContextColor}sp tabs    content{DiffWhitespaceColor}%%%%",
				"{Normal}6{Normal} {Normal}6{Normal}| {DiffWhitespaceColor}%%%%{DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}%%%%",
				"{Normal}7{Normal} {Normal}7{Normal}| {DiffWhitespaceColor}%# # %{DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}# %%# "
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_show_leading_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Leading;
			config.diff_tab_symbol = String::from("#");
			config.diff_space_symbol = String::from("%");
			config.diff_tab_width = 2;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			file_stat.add_delta(generate_white_space_delta());
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -1,7 +1,7 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}1{Normal} {Normal}1{Normal}| {DiffWhitespaceColor}# # {DiffContextColor}sp tabs    content",
				"{Normal}2{Normal} {Normal}2{Normal}| {DiffContextColor}sp tabs    content{DiffWhitespaceColor}    ",
				"{Normal}3{Normal} {Normal}3{Normal}| {DiffWhitespaceColor}# # {DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}    ",
				"{Normal}4{Normal} {Normal}4{Normal}| {DiffWhitespaceColor}%%%%{DiffContextColor}sp tabs    content",
				"{Normal}5{Normal} {Normal}5{Normal}| {DiffContextColor}sp tabs    content{DiffWhitespaceColor}    ",
				"{Normal}6{Normal} {Normal}6{Normal}| {DiffWhitespaceColor}%%%%{DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}    ",
				"{Normal}7{Normal} {Normal}7{Normal}| {DiffWhitespaceColor}%# # %{DiffContextColor}sp tabs    \
				 content{DiffWhitespaceColor}      "
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_show_no_whitespace() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
			config.diff_tab_symbol = String::from("#");
			config.diff_space_symbol = String::from("%");
			config.diff_tab_width = 2;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			file_stat.add_delta(generate_white_space_delta());
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -1,7 +1,7 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal}1{Normal} {Normal}1{Normal}| {DiffContextColor}    sp tabs    content",
				"{Normal}2{Normal} {Normal}2{Normal}| {DiffContextColor}sp tabs    content    ",
				"{Normal}3{Normal} {Normal}3{Normal}| {DiffContextColor}    sp tabs    content    ",
				"{Normal}4{Normal} {Normal}4{Normal}| {DiffContextColor}    sp tabs    content",
				"{Normal}5{Normal} {Normal}5{Normal}| {DiffContextColor}sp tabs    content    ",
				"{Normal}6{Normal} {Normal}6{Normal}| {DiffContextColor}    sp tabs    content    ",
				"{Normal}7{Normal} {Normal}7{Normal}| {DiffContextColor}      sp tabs    content      "
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn render_diff_show_whitespace_all_spaces() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef comment1"],
		ViewState {
			size: Size::new(50, 100),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
			config.diff_tab_symbol = String::from("#");
			config.diff_space_symbol = String::from("%");
			config.diff_tab_width = 2;
			let mut module = ShowCommit::new(&config);
			let mut commit = create_minimal_commit();
			let mut file_stat = FileStat::new("file.txt", "file.txt", Status::Modified);
			let mut delta = Delta::new("@@ -1,7 +1,7 @@ context", 1, 1, 7, 7);
			delta.add_line(DiffLine::new(Origin::Addition, "    ", None, Some(1), false));
			file_stat.add_delta(delta);
			commit.file_stats = vec![file_stat];
			module.commit = Some(commit);
			module.state = ShowCommitState::Diff;
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{LEADING}",
				"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
				"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
				 {DiffRemoveColor}0{Normal} deletions",
				"{BODY}",
				"{Normal}{Pad ―,150}",
				"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
				"",
				"{Normal,Dimmed}@@{DiffContextColor} -1,7 +1,7 {Normal,Dimmed}@@{DiffContextColor} context",
				"{Normal,Dimmed}{Pad ┈,150}",
				"{Normal} {Normal} {Normal}1{Normal}| {DiffWhitespaceColor}%%%%"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_toggle_diff_to_overview() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::ShowDiff],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			module.view_data.push_line(ViewLine::from("foo"));
			module.state = ShowCommitState::Diff;
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ShowDiff);
			assert!(module.view_data.is_empty());
			assert_eq!(module.state, ShowCommitState::Overview);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_toggle_overview_to_diff() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::ShowDiff],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			module.view_data.push_line(ViewLine::from("foo"));
			module.state = ShowCommitState::Overview;
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ShowDiff);
			assert!(module.view_data.is_empty());
			assert_eq!(module.state, ShowCommitState::Diff);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_resize() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::Resize],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_help() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::Help],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Help);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_other_key_from_diff() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::Character('a')],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			module.state = ShowCommitState::Diff;
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
			assert_eq!(module.state, ShowCommitState::Overview);
		},
	);
}

#[test]
#[serial_test::serial]
fn handle_input_other_key_from_overview() {
	process_module_test(
		&["pick 0123456789abcdef0123456789abcdef c1"],
		ViewState::default(),
		&[Input::Character('a')],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			module.state = ShowCommitState::Overview;
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Other,
				state = State::List
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn scroll_events() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::ScrollLeft,
			Input::ScrollRight,
			Input::ScrollDown,
			Input::ScrollUp,
			Input::ScrollJumpDown,
			Input::ScrollJumpUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = ShowCommit::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollLeft);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollRight);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollDown);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollUp);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpDown);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpUp);
		},
	);
}
