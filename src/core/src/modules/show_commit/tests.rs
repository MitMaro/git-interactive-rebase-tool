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
use view::{assert_rendered_output, render_line, ViewLine};

use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn load_commit_during_activate() {
	with_temp_repository(|repo| {
		let oid = head_id(&repo, "main");
		let line = format!("pick {oid} comment1");
		module_test(&[line.as_str()], &[], |test_context| {
			let mut module = ShowCommit::new(&Config::new(), repo);
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
		module_test(&[line.as_str()], &[], |test_context| {
			let mut module = ShowCommit::new(&Config::new(), repo);
			// would be nice to be able to test that a second call to load_commit_diff did not happen here
			assert_results!(test_context.activate(&mut module, State::List));
			assert_results!(test_context.activate(&mut module, State::List));
		});
	});
}

#[test]
fn no_selected_line_in_activate() {
	with_temp_repository(|repo| {
		module_test(&[], &[], |test_context| {
			let mut module = ShowCommit::new(&Config::new(), repo);
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
		module_test(&["pick aaaaaaaaaa comment1"], &[], |test_context| {
			let mut module = ShowCommit::new(&Config::new(), repo);
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
			|test_context| {
				let commit = CommitBuilder::new("0123456789abcdef0123456789abcdef").build();
				let commit_date = commit.committed_date().format("%c %z").to_string();
				let diff = CommitDiffBuilder::new(commit).build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
					"{BODY}",
					format!("{{IndicatorColor}}Date: {{Normal}}{commit_date}").as_str(),
					"{Normal}",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{Normal}01234567",
					"{BODY}",
					format!("{{IndicatorColor}}D: {{Normal}}{commit_date}").as_str(),
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
			|test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.author(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{IndicatorColor}Author: {Normal}John Doe <john.doe@example.com>",
					render_line!(AnyLine 2)
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{IndicatorColor}A: {Normal}John Doe <john.doe@example.com>",
					render_line!(AnyLine 2)
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
			|test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.committer(User::new(Some("John Doe"), Some("john.doe@example.com")))
						.build(),
				)
				.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{IndicatorColor}Committer: {Normal}John Doe <john.doe@example.com>",
					render_line!(AnyLine 2)
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{IndicatorColor}C: {Normal}John Doe <john.doe@example.com>",
					render_line!(AnyLine 2)
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
			|test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.summary("Commit title")
						.build(),
				)
				.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}Commit title",
					render_line!(AnyLine 2)
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
			|test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.message("Commit body")
						.build(),
				)
				.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}Commit body",
					render_line!(AnyLine 2)
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
			|test_context| {
				let diff = CommitDiffBuilder::new(
					CommitBuilder::new("0123456789abcdef0123456789abcdef")
						.summary("Commit title")
						.message("Commit body")
						.build(),
				)
				.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}Commit title",
					"{Normal}",
					"{Normal}Commit body",
					render_line!(AnyLine 2)
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
			|test_context| {
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 6),
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 6),
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
	});
}

#[test]
fn render_overview_single_file_changed() {
	with_temp_repository(|repo| {
		module_test(
			&["pick 0123456789abcdef0123456789abcdef comment1"],
			&[],
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_files_changed(1)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 6),
					"{IndicatorColor}1{Normal} file{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_files_changed(2)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}",
					"{IndicatorColor}2{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_insertions(1)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}1{Normal} insertion{Normal} and \
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
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_insertions(2)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}2{Normal} insertions{Normal} and \
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
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_deletions(1)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
			|test_context| {
				let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build())
					.number_deletions(2)
					.build();
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 5),
					"{Normal}",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
			|test_context| {
				let mut config = Config::new();
				config.diff_show_whitespace = DiffShowWhitespaceSetting::None;
				let diff =
					CommitDiffBuilder::new(CommitBuilder::new("0123456789abcdef0123456789abcdef").build()).build();
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					"{TITLE}{HELP}",
					"{LEADING}",
					"{IndicatorColor}Commit: {Normal}0123456789abcdef0123456789abcdef",
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
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
			|test_context| {
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 3),
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor} renamed: {DiffRemoveColor}file.1b{Normal} → {DiffAddColor}file.1a",
					"{Normal}{Pad(―)}",
					"{DiffAddColor}   added: {DiffAddColor}file.2a",
					"{Normal}{Pad(―)}",
					"{DiffRemoveColor} deleted: {DiffRemoveColor}file.3a",
					"{Normal}{Pad(―)}",
					"{DiffAddColor}  copied: {Normal}file.4b{Normal} → {DiffAddColor}file.4a",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: {DiffChangeColor}file.5a",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor} changed: {DiffChangeColor}file.6a",
					"{Normal}{Pad(―)}",
					"{Normal} unknown: {Normal}file.7a"
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 3),
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line",
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 3),
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line"
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 3),
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line"
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 3),
					"{IndicatorColor}0{Normal} files{Normal} with {DiffAddColor}0{Normal} insertions{Normal} and \
					 {DiffRemoveColor}0{Normal} deletions",
					"{BODY}",
					"{Normal}{Pad(―)}",
					"{DiffChangeColor}modified: {DiffChangeColor}file.txt",
					"",
					"{Normal,Dimmed}@@{DiffContextColor} -14,0 +14,1 {Normal,Dimmed}@@{DiffContextColor} context",
					"{Normal,Dimmed}{Pad(―)}",
					"{Normal}13{Normal} {Normal}13{Normal}| {DiffContextColor}context 1",
					"{Normal}14{Normal} {Normal}  {Normal}| {DiffRemoveColor}old line",
					"{Normal}  {Normal} {Normal}14{Normal}| {DiffAddColor}new line",
					"{Normal}15{Normal} {Normal}15{Normal}| {DiffContextColor}context 2"
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 10),
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 10),
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 10),
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
			|test_context| {
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
				let mut module = ShowCommit::new(&config, repo);
				module.diff = Some(diff);
				module.state = ShowCommitState::Diff;
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					render_line!(AnyLine 10),
					"{Normal} {Normal} {Normal}1{Normal}| {DiffWhitespaceColor}%%%%"
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
				let mut module = ShowCommit::new(&Config::new(), repo);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				let _ = test_context.handle_all_events(&mut module);
				assert_rendered_output!(
					test_context.build_view_data(&mut module),
					"{TITLE}",
					"{LEADING}",
					"{Normal,Underline} Key      Action{Normal,Underline}{Pad( )}",
					"{BODY}",
					"{IndicatorColor} Up      {Normal,Dimmed}|{Normal}Scroll up",
					"{IndicatorColor} Down    {Normal,Dimmed}|{Normal}Scroll down",
					"{IndicatorColor} PageUp  {Normal,Dimmed}|{Normal}Scroll up half a page",
					"{IndicatorColor} PageDown{Normal,Dimmed}|{Normal}Scroll down half a page",
					"{IndicatorColor} Home    {Normal,Dimmed}|{Normal}Scroll to the top",
					"{IndicatorColor} End     {Normal,Dimmed}|{Normal}Scroll to the bottom",
					"{IndicatorColor} Right   {Normal,Dimmed}|{Normal}Scroll right",
					"{IndicatorColor} Left    {Normal,Dimmed}|{Normal}Scroll left",
					"{IndicatorColor} d       {Normal,Dimmed}|{Normal}Show full diff",
					"{IndicatorColor} ?       {Normal,Dimmed}|{Normal}Show help",
					"{TRAILING}",
					"{IndicatorColor}Press any key to close"
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				let _ = test_context.handle_all_events(&mut module);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
				let _ = test_context.handle_all_events(&mut module);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
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
				let mut module = ShowCommit::new(&Config::new(), repo);
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
			let mut module = ShowCommit::new(&Config::new(), repo);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(event))
			);
		});
	});
}
