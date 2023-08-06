use anyhow::anyhow;
use git::{
	testutil::{head_id, with_temp_repository, CommitBuilder, CommitDiffBuilder, FileStatusBuilder},
	Delta,
	DiffLine,
	FileMode,
	Origin,
	Status,
	User,
};
use input::StandardEvent;
use rstest::rstest;
use view::{assert_rendered_output, render_line, testutil::AssertRenderOptions, ViewLine};

use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

fn create_show_commit(config: &Config, repository: Repository, todo_file: TodoFile) -> ShowCommit {
	ShowCommit::new(config, repository, Arc::new(Mutex::new(todo_file)))
}

fn render_options() -> AssertRenderOptions {
	AssertRenderOptions::INCLUDE_STYLE | AssertRenderOptions::BODY_ONLY
}

#[test]
fn load_commit_during_activate() {
	with_temp_repository(|repo| {
		let oid = head_id(&repo, "main");
		let line = format!("pick {oid} comment1");
		module_test(&[line.as_str()], &[], |mut test_context| {
			let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
			assert_results!(test_context.activate(&mut module, State::List));
			assert!(module.diff.is_some());
		});
	});
}

#[test]
fn cached_commit_in_activate() {
	with_temp_repository(|repo| {
		let oid = head_id(&repo, "main");
		let line = format!("pick {oid} comment1");
		module_test(&[line.as_str()], &[], |mut test_context| {
			let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
			// would be nice to be able to test that a second call to load_commit_diff did not happen here
			assert_results!(test_context.activate(&mut module, State::List));
			assert_results!(test_context.activate(&mut module, State::List));
		});
	});
}

#[test]
fn no_selected_line_in_activate() {
	with_temp_repository(|repo| {
		module_test(&[], &[], |mut test_context| {
			let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
			assert_results!(
				test_context.activate(&mut module, State::List),
				Artifact::Error(anyhow!("No valid commit to show"), Some(State::List))
			);
		});
	});
}

#[test]
fn activate_error() {
	with_temp_repository(|repo| {
		module_test(&["pick aaaaaaaaaa comment1"], &[], |mut test_context| {
			let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
			assert_results!(
				test_context.activate(&mut module, State::List),
				Artifact::Error(
					anyhow!(
						"Could not load commit: revspec 'aaaaaaaaaa' not found; class=Reference (4); code=NotFound \
						 (-3)"
					),
					Some(State::List)
				)
			);
		});
	});
}

#[test]
fn render_overview_minimal_commit() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let commit = CommitBuilder::new("0123456789abcdef0123456789abcdef").build();
				let commit_date = commit.committed_date().format("%c %z").to_string();
				let diff = CommitDiffBuilder::new(commit).build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
					"{BODY}",
					format!("{{IndicatorColor}}Date: {{Normal}}{commit_date}"),
					"{Normal}",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_overview_minimal_commit_compact() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				test_context.render_context.update(30, 300);
				let commit = CommitBuilder::new("0123456789abcdef0123456789abcdef").build();
				let commit_date = commit.committed_date().format("%c %z").to_string();
				let diff = CommitDiffBuilder::new(commit).build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{Normal}01234567",
					"{BODY}",
					format!("{{IndicatorColor}}D: {{Normal}}{commit_date}"),
					"{Normal}",
					"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_author() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.author(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}Author: {Normal}John Doe <john.doe@example.com>"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_author_compact() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				test_context.render_context.update(30, 300);
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.author(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}A: {Normal}John Doe <john.doe@example.com>"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_committer() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.committer(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}Committer: {Normal}John Doe <john.doe@example.com>"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_committer_compact() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				test_context.render_context.update(30, 300);
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.committer(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}C: {Normal}John Doe <john.doe@example.com>"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_commit_summary() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.summary("Commit title")
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{Normal}Commit title"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_commit_body() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.message("Commit body")
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{Normal}Commit body"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_commit_summary_and_body() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.summary("Commit title")
						.message("Commit body")
						.build(),
				)
				.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1;2,
					test_context.build_view_data(&mut module),
					"{Normal}Commit title",
					"{Normal}",
					"{Normal}Commit body"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_file_stats() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.1a")
							.destination_path("file.1b")
							.status(Status::Renamed)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.2a")
							.destination_path("file.2a")
							.status(Status::Added)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.3a")
							.destination_path("file.3a")
							.status(Status::Deleted)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.4a")
							.destination_path("file.4b")
							.status(Status::Copied)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.5a")
							.destination_path("file.5a")
							.status(Status::Modified)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.6a")
							.destination_path("file.6a")
							.destination_mode(FileMode::Executable)
							.status(Status::Typechange)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.7a")
							.destination_path("file.7a")
							.status(Status::Other)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{DiffChangeColor} renamed: {DiffRemoveColor}file.1b{Normal} → {DiffAddColor}file.1a",
					"{DiffAddColor}   added: file.2a",
					"{DiffRemoveColor} deleted: file.3a",
					"{DiffAddColor}  copied: {Normal}file.4b → {DiffAddColor}file.4a",
					"{DiffChangeColor}modified: file.5a",
					"{DiffChangeColor} changed: file.6a",
					"{Normal} unknown: file.7a"
				);
			},
		);
	});
}

#[test]
fn render_overview_with_file_stats_compact() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				test_context.render_context.update(30, 300);
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.1a")
							.destination_path("file.1b")
							.status(Status::Renamed)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.2a")
							.destination_path("file.2a")
							.status(Status::Added)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.3a")
							.destination_path("file.3a")
							.status(Status::Deleted)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.4a")
							.destination_path("file.4b")
							.status(Status::Copied)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.5a")
							.source_path("file.5a")
							.destination_path("file.5a")
							.status(Status::Modified)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.6a")
							.destination_path("file.6a")
							.destination_mode(FileMode::Executable)
							.status(Status::Typechange)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.7a")
							.destination_path("file.7a")
							.status(Status::Other)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0",
					"{DiffChangeColor}R {DiffRemoveColor}file.1b{Normal}→{DiffAddColor}file.1a",
					"{DiffAddColor}A file.2a",
					"{DiffRemoveColor}D file.3a",
					"{DiffAddColor}C {Normal}file.4b→{DiffAddColor}file.4a",
					"{DiffChangeColor}M file.5a",
					"{DiffChangeColor}T file.6a",
					"{Normal}X file.7a"
				);
			},
		);
	});
}

#[test]
fn render_overview_single_file_changed() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_files_changed(1)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 2,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}1{Normal} file with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_overview_more_than_one_file_changed() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_files_changed(2)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1,
					test_context.build_view_data(&mut module),
					"{Normal}",
					"{IndicatorColor}2{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_overview_single_insertion() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_insertions(1)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1,
					test_context.build_view_data(&mut module),
					"{Normal}",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}1{Normal} insertion and \
					 {DiffRemoveColor}0{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_overview_more_than_one_insertion() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_insertions(2)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1,
					test_context.build_view_data(&mut module),
					"{Normal}",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}2{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_overview_single_deletion() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_deletions(1)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1,
					test_context.build_view_data(&mut module),
					"{Normal}",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}1{Normal} deletion"
				);
			},
		);
	});
}

#[test]
fn render_overview_more_than_one_deletion() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_deletions(2)
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				assert_rendered_output!(
					Options render_options(),
					Skip 1,
					test_context.build_view_data(&mut module),
					"{Normal}",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}2{Normal} deletions"
				);
			},
		);
	});
}

#[test]
fn render_diff_minimal_commit() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let diff =
					CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build()).build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}"
				);
			},
		);
	});
}

#[test]
fn render_diff_minimal_commit_compact() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				test_context.render_context.update(30, 300);
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let diff =
					CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build()).build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{Normal}01234567",
					"{IndicatorColor}0{Normal} / {DiffAddColor}0{Normal} / {DiffRemoveColor}0",
					"{BODY}",
					"{Normal}{Pad(―)}"
				);
			},
		);
	});
}

#[test]
fn render_diff_basic_file_stats() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.1a")
							.destination_path("file.1b")
							.status(Status::Renamed)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.2a")
							.destination_path("file.2a")
							.status(Status::Added)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.3a")
							.destination_path("file.3a")
							.status(Status::Deleted)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.4a")
							.destination_path("file.4b")
							.status(Status::Copied)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.5a")
							.destination_path("file.5a")
							.status(Status::Modified)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.6a")
							.destination_path("file.6a")
							.destination_mode(FileMode::Executable)
							.status(Status::Typechange)
							.build(),
						FileStatusBuilder::new()
							.source_path("file.7a")
							.destination_path("file.7a")
							.status(Status::Other)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 3,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor} renamed: {DiffRemoveColor}file.1b{Normal} → {DiffAddColor}file.1a",
					"{Normal}{Pad(―)}",
					"{DiffAddColor}   added: file.2a",
					"{Normal}{Pad(―)}",
					"{DiffRemoveColor} deleted: file.3a",
					"{Normal}{Pad(―)}",
					"{DiffAddColor}  copied: {Normal}file.4b → {DiffAddColor}file.4a",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: file.5a",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor} changed: file.6a",
					"{Normal}{Pad(―)}",
					"{Normal} unknown: file.7a"
				);
			},
		);
	});
}

#[test]
fn render_diff_end_new_line_missing() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
				delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
				delta.add_line(DiffLine::new(Origin::Addition, "", None, Some(15), true));

				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(delta)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 3,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}   14| {DiffAddColor}new line",
					"{Normal}       {DiffContextColor}\\ No newline at end of file"
				);
			},
		);
	});
}

#[test]
fn render_diff_add_line() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
				delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));

				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(delta)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 3,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}   14| {DiffAddColor}new line"
				);
			},
		);
	});
}

#[test]
fn render_diff_delete_line() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
				delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));

				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(delta)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 3,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}14   | {DiffRemoveColor}old line"
				);
			},
		);
	});
}

#[test]
fn render_diff_context_add_remove_lines() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let mut delta = Delta::new("@@ -14,2 +13,3 @@ context", 14, 14, 0, 1);
				delta.add_line(DiffLine::new(Origin::Context, "context 1", Some(13), Some(13), false));
				delta.add_line(DiffLine::new(Origin::Deletion, "old line", Some(14), None, false));
				delta.add_line(DiffLine::new(Origin::Addition, "new line", None, Some(14), false));
				delta.add_line(DiffLine::new(Origin::Context, "context 2", Some(15), Some(15), false));

				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(delta)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 3,
					test_context.build_view_data(&mut module),
					"{IndicatorColor}0{Normal} files with {DiffAddColor}0{Normal} insertions and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}13 13| {DiffContextColor}context 1",
					"{Normal}14   | {DiffRemoveColor}old line",
					"{Normal}   14| {DiffAddColor}new line",
					"{Normal}15 15| {DiffContextColor}context 2"
				);
			},
		);
	});
}

fn generate_diff_line_context(content: &str, line_num: u32) -> DiffLine {
	DiffLine::new(Origin::Context, content, Some(line_num), Some(line_num), false)
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

#[test]
fn render_diff_show_both_whitespace() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
				config.diff_tab_symbol = String::from("#>");
				config.diff_space_symbol = String::from("%");
				config.diff_tab_width = 2;
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(generate_white_space_delta())
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 10,
					test_context.build_view_data(&mut module),
					render_line!(EndsWith "#>#>{DiffContextColor}sp tabs    content"),
					render_line!(EndsWith "sp tabs    content{DiffWhitespaceColor}#>#>"),
					render_line!(EndsWith "#>#>{DiffContextColor}sp tabs    content{DiffWhitespaceColor}#>#>"),
					render_line!(EndsWith "%%%%{DiffContextColor}sp tabs    content"),
					render_line!(EndsWith "sp tabs    content{DiffWhitespaceColor}%%%%"),
					render_line!(EndsWith "%%%%{DiffContextColor}sp tabs    content{DiffWhitespaceColor}%%%%"),
					render_line!(EndsWith "%#>#>%{DiffContextColor}sp tabs    content{DiffWhitespaceColor}#>%%#>")
				);
			},
		);
	});
}

#[test]
fn render_diff_show_leading_whitespace() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::Leading;
				config.diff_tab_symbol = String::from("#>");
				config.diff_space_symbol = String::from("%");
				config.diff_tab_width = 2;
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(generate_white_space_delta())
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 10,
					test_context.build_view_data(&mut module),
					render_line!(EndsWith "#>#>{DiffContextColor}sp tabs    content"),
					render_line!(EndsWith "sp tabs    content{DiffWhitespaceColor}"),
					render_line!(EndsWith "#>#>{DiffContextColor}sp tabs    content{DiffWhitespaceColor}"),
					render_line!(EndsWith "%%%%{DiffContextColor}sp tabs    content"),
					render_line!(EndsWith "sp tabs    content{DiffWhitespaceColor}"),
					render_line!(EndsWith "%%%%{DiffContextColor}sp tabs    content{DiffWhitespaceColor}"),
					render_line!(EndsWith "%#>#>%{DiffContextColor}sp tabs    content{DiffWhitespaceColor}")
				);
			},
		);
	});
}

#[test]
fn render_diff_show_no_whitespace() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				config.diff_tab_symbol = String::from("#>");
				config.diff_space_symbol = String::from("%");
				config.diff_tab_width = 2;
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(generate_white_space_delta())
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 10,
					test_context.build_view_data(&mut module),
					render_line!(EndsWith "    sp tabs    content"),
					render_line!(EndsWith "sp tabs    content"),
					render_line!(EndsWith "    sp tabs    content"),
					render_line!(EndsWith "    sp tabs    content"),
					render_line!(EndsWith "sp tabs    content"),
					render_line!(EndsWith "    sp tabs    content"),
					render_line!(EndsWith "      sp tabs    content")
				);
			},
		);
	});
}

#[test]
fn render_diff_show_whitespace_all_spaces() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|mut test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::Both;
				config.diff_tab_symbol = String::from("#>");
				config.diff_space_symbol = String::from("%");
				config.diff_tab_width = 2;
				let mut delta = Delta::new("@@ -1,7 +1,7 @@ context", 1, 1, 7, 7);
				delta.add_line(DiffLine::new(Origin::Addition, "    ", None, Some(1), false));
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.file_statuses(vec![
						FileStatusBuilder::new()
							.source_path("file.txt")
							.destination_path("file.txt")
							.status(Status::Modified)
							.push_delta(delta)
							.build(),
					])
					.build();
				let mut module = create_show_commit(&config, repo, test_context.take_todo_file());
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					Options AssertRenderOptions::INCLUDE_STYLE,
					Skip 10,
					test_context.build_view_data(&mut module),
					"{Normal}  1| {DiffWhitespaceColor}%%%%"
				);
			},
		);
	});
}

#[test]
fn handle_event_toggle_diff_to_overview() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef c1"],
			&[Event::from(MetaEvent::ShowDiff)],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module
					.diff_view_data
					.update_view_data(|updater| updater.push_line(ViewLine::from("foo")));
				module.state = ShowCommitState::Diff;
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(MetaEvent::ShowDiff))
				);
				assert!(module.diff_view_data.is_empty());
				assert_eq!(module.state, ShowCommitState::Overview);
			},
		);
	});
}

#[test]
fn handle_event_toggle_overview_to_diff() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef c1"],
			&[Event::from('d')],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module
					.overview_view_data
					.update_view_data(|updater| updater.push_line(ViewLine::from("foo")));
				module.state = ShowCommitState::Overview;
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(MetaEvent::ShowDiff))
				);
				assert!(module.diff_view_data.is_empty());
				assert_eq!(module.state, ShowCommitState::Diff);
			},
		);
	});
}

#[test]
fn handle_event_resize() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef c1"],
			&[Event::Resize(100, 100)],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::Resize(100, 100))
				);
			},
		);
	});
}

#[test]
fn render_help() {
	with_temp_repository(|repo| {
		module_test(
			&["pick aaa c1"],
			&[Event::from(StandardEvent::Help)],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				_ = test_context.handle_all_events(&mut module);
				assert_rendered_output!(
					Body test_context.build_view_data(&mut module),
					" Up      |Scroll up",
					" Down    |Scroll down",
					" PageUp  |Scroll up half a page",
					" PageDown|Scroll down half a page",
					" Home    |Scroll to the top",
					" End     |Scroll to the bottom",
					" Right   |Scroll right",
					" Left    |Scroll left",
					" d       |Show full diff",
					" ?       |Show help"
				);
			},
		);
	});
}

#[test]
fn handle_help_event_show() {
	with_temp_repository(|repo| {
		module_test(
			&["pick aaa c1"],
			&[Event::from(StandardEvent::Help)],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				_ = test_context.handle_all_events(&mut module);
				assert!(module.help.is_active());
			},
		);
	});
}
#[test]
fn handle_help_event_hide() {
	with_temp_repository(|repo| {
		module_test(
			&["pick aaa c1"],
			&[Event::from(StandardEvent::Help), Event::from('?')],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				_ = test_context.handle_all_events(&mut module);
				assert!(!module.help.is_active());
			},
		);
	});
}

#[test]
fn handle_event_other_key_from_diff() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef c1"],
			&[Event::from('a')],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.state = ShowCommitState::Diff;
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from('a'))
				);
				assert_eq!(module.state, ShowCommitState::Overview);
			},
		);
	});
}

#[test]
fn handle_event_other_key_from_overview() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef c1"],
			&[Event::from('a')],
			|mut test_context| {
				let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
				module.state = ShowCommitState::Overview;
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from('a')),
					Artifact::ChangeState(State::List)
				);
			},
		);
	});
}

#[rstest]
#[case::scroll_left(StandardEvent::ScrollLeft)]
#[case::scroll_right(StandardEvent::ScrollRight)]
#[case::scroll_down(StandardEvent::ScrollDown)]
#[case::scroll_up(StandardEvent::ScrollUp)]
#[case::scroll_jump_down(StandardEvent::ScrollJumpDown)]
#[case::scroll_jump_up(StandardEvent::ScrollJumpUp)]
fn scroll_events(#[case] event: StandardEvent) {
	with_temp_repository(|repo| {
		module_test(&[], &[Event::from(event)], |mut test_context| {
			let mut module = create_show_commit(&Config::new(), repo, test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(event))
			);
		});
	});
}
