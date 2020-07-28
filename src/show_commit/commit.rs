use crate::show_commit::diff_line::{DiffLine, Origin};
use crate::show_commit::file_stat::FileStat;
use crate::show_commit::file_stats_builder::FileStatsBuilder;
use crate::show_commit::status::Status;
use crate::show_commit::user::User;
use chrono::{DateTime, Local, TimeZone};
use git2::{DiffFindOptions, DiffOptions, Error, Repository};
use std::sync::Mutex;

pub(super) struct LoadCommitDiffOptions {
	pub(super) context_lines: u32,
	pub(super) copies: bool,
	pub(super) ignore_whitespace: bool,
	pub(super) ignore_whitespace_change: bool,
	pub(super) interhunk_lines: u32,
	pub(super) rename_limit: u32,
	pub(super) renames: bool,
}

#[derive(Debug)]
pub(crate) struct Commit {
	author: User,
	body: Option<String>,
	committer: User,
	date: DateTime<Local>,
	file_stats: Vec<FileStat>,
	hash: String,
	number_files_changed: usize,
	insertions: usize,
	deletions: usize,
}

fn load_commit_state(hash: &str, config: LoadCommitDiffOptions) -> Result<Commit, Error> {
	let repo = Repository::open_from_env()?;
	let commit = repo.find_commit(repo.revparse_single(hash)?.id())?;

	let full_hash = commit.id().to_string();
	let date = Local.timestamp(commit.time().seconds(), 0);
	let body = commit.message().map(String::from);
	let author = User::new(commit.author().name(), commit.author().email());
	let committer = User::new(commit.committer().name(), commit.committer().email());
	let committer = if committer == author {
		User::new(None, None)
	}
	else {
		committer
	};
	let mut number_files_changed = 0;
	let mut insertions = 0;
	let mut deletions = 0;

	let mut diff_options = DiffOptions::new();

	// include_unmodified added to find copies from unmodified files
	let diff_options = diff_options
		.context_lines(config.context_lines)
		.ignore_filemode(false)
		.ignore_whitespace(config.ignore_whitespace)
		.ignore_whitespace_change(config.ignore_whitespace_change)
		.include_typechange(true)
		.include_typechange_trees(true)
		.include_unmodified(config.copies)
		.indent_heuristic(true)
		.interhunk_lines(config.interhunk_lines)
		.minimal(true);

	let mut diff_find_options = DiffFindOptions::new();
	let diff_find_options = diff_find_options
		.renames(config.renames)
		.renames_from_rewrites(config.renames)
		.rewrites(config.renames)
		.rename_limit(config.rename_limit as usize)
		.copies(config.copies)
		.copies_from_unmodified(config.copies);

	// some commits do not have parents, and can't have file stats
	let file_stats = match commit.parent_ids().count() {
		0 => vec![],
		_ => {
			let mut diff = repo.diff_tree_to_tree(
				// parent exists from check above
				Some(&commit.parent(0)?.tree()?),
				Some(&commit.tree()?),
				Some(diff_options),
			)?;

			diff.find_similar(Some(diff_find_options))?;

			let mut unmodified_file_count: usize = 0;

			let file_stats_builder = Mutex::new(FileStatsBuilder::new());

			// TODO trace file mode change and binary files
			diff.foreach(
				&mut |diff_delta, _| {
					// unmodified files are included for copy detection, so ignore
					if diff_delta.status() == git2::Delta::Unmodified {
						unmodified_file_count += 1;
						return true;
					}

					let mut fsb = file_stats_builder.lock().unwrap();

					let from_file_path = diff_delta
						.old_file()
						.path()
						.map(|p| String::from(p.to_str().unwrap()))
						.unwrap_or_else(|| String::from("unknown"));
					let to_file_path = diff_delta
						.new_file()
						.path()
						.map(|p| String::from(p.to_str().unwrap()))
						.unwrap_or_else(|| String::from("unknown"));

					fsb.add_file_stat(
						from_file_path,
						to_file_path,
						Status::new_from_git_delta(diff_delta.status()),
					);

					true
				},
				None,
				Some(&mut |_, diff_hunk| {
					let mut fsb = file_stats_builder.lock().unwrap();

					let header = std::str::from_utf8(diff_hunk.header()).unwrap();

					fsb.add_delta(
						header,
						diff_hunk.old_start(),
						diff_hunk.new_start(),
						diff_hunk.old_lines(),
						diff_hunk.new_lines(),
					);
					true
				}),
				Some(&mut |_, _, diff_line| {
					let mut fsb = file_stats_builder.lock().unwrap();
					fsb.add_diff_line(DiffLine::new(
						Origin::from_chr(diff_line.origin()),
						std::str::from_utf8(diff_line.content()).unwrap(),
						diff_line.old_lineno(),
						diff_line.new_lineno(),
						diff_line.origin() == '=' || diff_line.origin() == '>' || diff_line.origin() == '<',
					));
					true
				}),
			)
			.unwrap();

			if let Ok(stats) = diff.stats() {
				number_files_changed = stats.files_changed() - unmodified_file_count;
				insertions = stats.insertions();
				deletions = stats.deletions();
			}

			let fsb = file_stats_builder.into_inner().unwrap();

			fsb.build()
		},
	};

	Ok(Commit {
		hash: full_hash,
		author,
		committer,
		date,
		file_stats,
		body,
		number_files_changed,
		insertions,
		deletions,
	})
}

impl Commit {
	/// Load commit information from a commit hash.
	pub(super) fn new_from_hash(hash: &str, config: LoadCommitDiffOptions) -> Result<Self, String> {
		load_commit_state(hash, config).map_err(|e| String::from(e.message()))
	}

	pub(super) fn get_author(&self) -> &User {
		&self.author
	}

	pub(super) fn get_committer(&self) -> &User {
		&self.committer
	}

	pub(super) fn get_date(&self) -> &DateTime<Local> {
		&self.date
	}

	pub(crate) fn get_hash(&self) -> &str {
		&self.hash
	}

	pub(super) fn get_body(&self) -> &Option<String> {
		&self.body
	}

	pub(crate) fn get_file_stats(&self) -> &Vec<FileStat> {
		&self.file_stats
	}

	pub(crate) fn get_number_files_changed(&self) -> usize {
		self.number_files_changed
	}

	pub(crate) fn get_number_insertions(&self) -> usize {
		self.insertions
	}

	pub(crate) fn get_number_deletions(&self) -> usize {
		self.deletions
	}
}

#[cfg(test)]
mod tests {
	// some of this file is difficult to test because it would require a non-standard git repo, so
	// we test what is possible
	use crate::show_commit::commit::{Commit, LoadCommitDiffOptions};
	use crate::show_commit::status::Status;
	use serial_test::serial;
	use std::env::set_var;
	use std::path::Path;

	fn set_git_dir(fixture: &str) {
		set_var(
			"GIT_DIR",
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("fixtures")
				.join(fixture)
				.to_str()
				.unwrap(),
		);
	}

	fn load_commit_state(hash: &str) -> Result<Commit, String> {
		Commit::new_from_hash(hash, LoadCommitDiffOptions {
			context_lines: 3,
			copies: true,
			ignore_whitespace: false,
			ignore_whitespace_change: false,
			interhunk_lines: 3,
			rename_limit: 200,
			renames: true,
		})
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_hash() {
		set_git_dir("simple");
		let commit = load_commit_state("18d82dcc4c36cade807d7cf79700b6bbad8080b9").unwrap();
		assert_eq!(commit.get_hash(), "18d82dcc4c36cade807d7cf79700b6bbad8080b9");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_author() {
		set_git_dir("simple");
		let commit = load_commit_state("18d82dcc4c36cade807d7cf79700b6bbad8080b9").unwrap();
		assert_eq!(commit.get_author().to_string().unwrap(), "Tim Oram <dev@mitmaro.ca>");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_date() {
		set_git_dir("simple");
		let commit = load_commit_state("18d82dcc4c36cade807d7cf79700b6bbad8080b9").unwrap();
		assert_eq!(commit.get_date().timestamp(), 1580172067);
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_body() {
		set_git_dir("simple");
		let commit = load_commit_state("18d82dcc4c36cade807d7cf79700b6bbad8080b9").unwrap();
		assert_eq!(
			commit.get_body().as_ref().unwrap(),
			"Empty commit title\n\nEmpty commit body\n"
		);
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_committer_match_author() {
		set_git_dir("simple");
		let commit = load_commit_state("ac950e31a96660e55d8034948b5d9b985c97692d").unwrap();
		assert!(commit.get_committer().to_string().is_none());
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_committer_not_match_author() {
		set_git_dir("simple");
		let commit = load_commit_state("2836dcdcbd040f9157652dd3db0d584a44d4793d").unwrap();
		assert_eq!(
			commit.get_committer().to_string().unwrap(),
			"Not Tim Oram <not-dev@mitmaro.ca>"
		);
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_modified_file() {
		set_git_dir("simple");
		let commit = load_commit_state("1cc0456637cb220155e957c641f483e60724c581").unwrap();
		let file_stat = commit.get_file_stats().first().unwrap();
		assert_eq!(*file_stat.get_status(), Status::Modified);
		assert_eq!(file_stat.get_from_name(), "a");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_added_file() {
		set_git_dir("simple");
		let commit = load_commit_state("c1ac7f2c32f9e00012f409572d223c9457ae497b").unwrap();
		let file_stat = commit.get_file_stats().first().unwrap();
		assert_eq!(*file_stat.get_status(), Status::Added);
		assert_eq!(file_stat.get_from_name(), "e");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_deleted_file() {
		set_git_dir("simple");
		let commit = load_commit_state("d85479638307e4db37e1f1f2c3c807f7ff36a0ff").unwrap();
		let file_stat = commit.get_file_stats().first().unwrap();
		assert_eq!(*file_stat.get_status(), Status::Deleted);
		assert_eq!(file_stat.get_from_name(), "b");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_renamed_file() {
		set_git_dir("simple");
		let commit = load_commit_state("aed0fd1db3e73c0e568677ae8903a11c5fbc5659").unwrap();
		let file_stat = commit.get_file_stats().first().unwrap();
		assert_eq!(*file_stat.get_status(), Status::Renamed);
		assert_eq!(file_stat.get_from_name(), "c");
		assert_eq!(file_stat.get_to_name(), "f");
	}

	#[test]
	#[serial]
	fn commit_utils_load_commit_state_load_copied_file() {
		set_git_dir("simple");
		let commit = load_commit_state("c028f42bdb2a5a9f80adea23d95eb240b994a6c2").unwrap();
		let file_stat = commit.get_file_stats().first().unwrap();
		assert_eq!(*file_stat.get_status(), Status::Copied);
		assert_eq!(file_stat.get_from_name(), "d");
		assert_eq!(file_stat.get_to_name(), "g");
	}
}
