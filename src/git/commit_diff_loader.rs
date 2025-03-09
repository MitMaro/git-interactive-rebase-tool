use std::{
	path::PathBuf,
	sync::{Arc, LazyLock},
};

use git2::{DiffFindOptions, DiffOptions, Oid, Repository};
use parking_lot::{Mutex, MutexGuard};

use crate::git::{
	Commit,
	CommitDiff,
	CommitDiffLoaderOptions,
	Delta,
	DiffLine,
	FileMode,
	FileStatus,
	FileStatusBuilder,
	Status,
};

static UNKNOWN_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("unknown"));

pub(crate) struct CommitDiffLoader<'options> {
	config: &'options CommitDiffLoaderOptions,
	repo: Arc<Mutex<Repository>>,
}

impl<'options> CommitDiffLoader<'options> {
	pub(crate) const fn new(repo: Arc<Mutex<Repository>>, config: &'options CommitDiffLoaderOptions) -> Self {
		Self { config, repo }
	}

	pub(crate) fn load_from_hash(&self, oid: Oid) -> Result<Vec<CommitDiff>, git2::Error> {
		let repo = self.repo.lock();
		let commit = repo.find_commit(oid)?;
		let no_parents = commit.parent_ids().count() == 0;

		// some commits do not have parents, and can't have file stats
		let diffs = if no_parents {
			vec![self.load_diff(&repo, None, &commit)?]
		}
		else {
			//
			let mut diffs = vec![];
			for parent in commit.parents() {
				diffs.push(self.load_diff(&repo, Some(&parent), &commit)?);
			}
			diffs
		};
		Ok(diffs)
	}

	#[allow(clippy::as_conversions, clippy::unwrap_in_result)]
	fn load_diff(
		&self,
		repo: &MutexGuard<'_, Repository>,
		parent: Option<&git2::Commit<'_>>,
		commit: &git2::Commit<'_>,
	) -> Result<CommitDiff, git2::Error> {
		let mut diff_options = DiffOptions::new();
		// include_unmodified added to find copies from unmodified files
		_ = diff_options
			.context_lines(self.config.context_lines)
			.ignore_filemode(false)
			.ignore_whitespace(self.config.ignore_whitespace)
			.ignore_whitespace_change(self.config.ignore_whitespace_change)
			.ignore_blank_lines(self.config.ignore_blank_lines)
			.include_typechange(true)
			.include_typechange_trees(true)
			.include_unmodified(self.config.copies)
			.indent_heuristic(true)
			.interhunk_lines(self.config.interhunk_context)
			.minimal(true);

		let mut diff_find_options = DiffFindOptions::new();
		_ = diff_find_options
			.rename_limit(self.config.rename_limit as usize)
			.renames(self.config.renames)
			.renames_from_rewrites(self.config.renames)
			.rewrites(self.config.renames)
			.copies(self.config.copies)
			.copies_from_unmodified(self.config.copies);

		let mut diff = if let Some(p) = parent {
			repo.diff_tree_to_tree(Some(&p.tree()?), Some(&commit.tree()?), Some(&mut diff_options))?
		}
		else {
			repo.diff_tree_to_tree(None, Some(&commit.tree()?), Some(&mut diff_options))?
		};

		diff.find_similar(Some(&mut diff_find_options))?;

		let mut unmodified_file_count: usize = 0;

		let file_stats_builder = Mutex::new(FileStatusBuilder::new());

		diff.foreach(
			&mut |diff_delta, _| {
				// unmodified files are included for copy detection, so ignore
				if diff_delta.status() == git2::Delta::Unmodified {
					unmodified_file_count += 1;
					return true;
				}

				let mut fsb = file_stats_builder.lock();

				let source_file = diff_delta.old_file();
				let source_file_mode = FileMode::from(source_file.mode());
				let source_file_path = source_file.path().unwrap_or(UNKNOWN_PATH.as_path());

				let destination_file = diff_delta.new_file();
				let destination_file_mode = FileMode::from(destination_file.mode());
				let destination_file_path = destination_file.path().unwrap_or(UNKNOWN_PATH.as_path());

				fsb.add_file_stat(FileStatus::new(
					source_file_path,
					source_file_mode,
					source_file.is_binary(),
					destination_file_path,
					destination_file_mode,
					destination_file.is_binary(),
					Status::from(diff_delta.status()),
				));

				true
			},
			None,
			Some(&mut |_, diff_hunk| {
				let mut fsb = file_stats_builder.lock();
				fsb.add_delta(Delta::from(&diff_hunk));
				true
			}),
			Some(&mut |_, _, diff_line| {
				let mut fsb = file_stats_builder.lock();
				fsb.add_diff_line(DiffLine::from(&diff_line));
				true
			}),
		)
		.expect("diff.foreach failed. Please report this as a bug.");

		let stats = diff.stats()?;
		let number_files_changed = stats.files_changed() - unmodified_file_count;
		let number_insertions = stats.insertions();
		let number_deletions = stats.deletions();

		let fsb = file_stats_builder.into_inner();

		Ok(CommitDiff::new(
			Commit::from(commit),
			parent.map(Commit::from),
			fsb.build(),
			number_files_changed,
			number_insertions,
			number_deletions,
		))
	}
}

#[cfg(all(unix, test))]
mod tests {
	use std::{
		fs::{File, remove_file},
		io::Write,
		os::unix::fs::symlink,
	};

	use super::*;
	use crate::{git::Origin, test_helpers::with_temp_repository};

	#[cfg(not(tarpaulin_include))]
	fn _format_status(status: &FileStatus) -> String {
		let s = match status.status() {
			Status::Added => "Added",
			Status::Deleted => "Deleted",
			Status::Modified => "Modified",
			Status::Renamed => "Renamed",
			Status::Copied => "Copied",
			Status::Typechange => "Typechange",
			Status::Other => "Other",
		};

		format!("Status {s}")
	}

	#[cfg(not(tarpaulin_include))]
	fn _format_file_mode(mode: FileMode) -> String {
		String::from(match mode {
			FileMode::Normal => "n",
			FileMode::Executable => "x",
			FileMode::Link => "l",
			FileMode::Other => "o",
		})
	}

	#[cfg(not(tarpaulin_include))]
	fn _format_paths(status: &FileStatus) -> String {
		let source_mode = _format_file_mode(status.source_mode());
		let source_binary = if status.source_is_binary() { ",b" } else { "" };

		if status.source_path() == status.destination_path()
			&& status.source_mode() == status.destination_mode()
			&& status.source_is_binary() == status.destination_is_binary()
		{
			format!("{} ({source_mode}{source_binary})", status.source_path().display())
		}
		else {
			let destination_binary = if status.destination_is_binary() { ",b" } else { "" };
			format!(
				"{} ({source_mode}{source_binary}) > {} ({}{destination_binary})",
				status.source_path().display(),
				status.destination_path().display(),
				_format_file_mode(status.destination_mode()),
			)
		}
	}

	#[cfg(not(tarpaulin_include))]
	#[allow(clippy::string_slice)]
	fn _format_diff_line(line: &DiffLine) -> String {
		let origin = match line.origin() {
			Origin::Addition => "+",
			Origin::Binary => "B",
			Origin::Context => " ",
			Origin::Deletion => "-",
			Origin::Header => "H",
		};
		if line.end_of_file() && line.line() != "\n" {
			String::from("\\ No newline at end of file")
		}
		else {
			format!(
				"{origin}{} {}| {}",
				line.old_line_number()
					.map_or_else(|| String::from(" "), |v| v.to_string()),
				line.new_line_number()
					.map_or_else(|| String::from(" "), |v| v.to_string()),
				if line.line().ends_with('\n') {
					&line.line()[..line.line().len() - 1]
				}
				else {
					line.line()
				},
			)
		}
	}

	#[cfg(not(tarpaulin_include))]
	fn _assert_commit_diff(diff: &CommitDiff, expected: &[String]) {
		let mut actual = vec![];
		for status in diff.file_statuses() {
			actual.push(_format_paths(status));
			actual.push(_format_status(status));
			for delta in status.deltas() {
				actual.push(format!(
					"@@ -{},{} +{},{} @@{}",
					delta.old_lines_start(),
					delta.old_number_lines(),
					delta.new_lines_start(),
					delta.new_number_lines(),
					if delta.context().is_empty() {
						String::new()
					}
					else {
						format!(" {}", delta.context())
					},
				));
				for line in delta.lines() {
					actual.push(_format_diff_line(line));
				}
			}
		}
		pretty_assertions::assert_eq!(actual, expected);
	}

	macro_rules! assert_commit_diff {
		($diff:expr, $($arg:expr),*) => {
			let expected = vec![$( String::from($arg), )*];
			_assert_commit_diff($diff, &expected);
		};
	}

	#[cfg(not(tarpaulin_include))]
	fn write_normal_file(repository: &crate::git::Repository, name: &str, contents: &[&str]) {
		let root = repository.repo_path().parent().unwrap().to_path_buf();

		let file_path = root.join(name);
		let mut file = File::create(file_path.as_path()).unwrap();
		if !contents.is_empty() {
			writeln!(file, "{}", contents.join("\n")).unwrap();
		}
		repository.add_path_to_index(PathBuf::from(name).as_path()).unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn remove_path(repository: &crate::git::Repository, name: &str) {
		let root = repository.repo_path().parent().unwrap().to_path_buf();

		let file_path = root.join(name);
		_ = remove_file(file_path);

		repository
			.remove_path_from_index(PathBuf::from(name).as_path())
			.unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn create_commit(repository: &crate::git::Repository) {
		let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(1_609_459_200, 0)).unwrap();
		repository
			.create_commit_on_index("refs/heads/main", &sig, &sig, "title")
			.unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn diff_from_head(repository: &crate::git::Repository, options: &CommitDiffLoaderOptions) -> CommitDiff {
		let id = repository.commit_id_from_ref("refs/heads/main").unwrap();
		let loader = CommitDiffLoader::new(repository.repository(), options);
		loader.load_from_hash(id).unwrap().remove(0)
	}

	#[test]
	fn load_from_hash_commit_no_parents() {
		with_temp_repository(|repo| {
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 0);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
		});
	}

	#[test]
	fn load_from_hash_added_file() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line1"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (o) > a (n)", "Status Added", "@@ -0,0 +1,1 @@", "+  1| line1");
		});
	}

	#[test]
	fn load_from_hash_removed_file() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line1"]);
			create_commit(&repo);
			remove_path(&repo, "a");
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 1);
			assert_commit_diff!(
				&diff,
				"a (n) > a (o)",
				"Status Deleted",
				"@@ -1,1 +0,0 @@",
				"-1  | line1"
			);
		});
	}

	#[test]
	fn load_from_hash_modified_file() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line1"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line2"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 1);
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -1,1 +1,1 @@",
				"-1  | line1",
				"+  1| line2"
			);
		});
	}

	#[test]
	fn load_from_hash_with_context() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0", "line1", "line2", "line3", "line4", "line5"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line0", "line1", "line2", "line3-m", "line4", "line5"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().context_lines(2));
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -2,5 +2,5 @@ line0",
				" 2 2| line1",
				" 3 3| line2",
				"-4  | line3",
				"+  4| line3-m",
				" 5 5| line4",
				" 6 6| line5"
			);
		});
	}

	#[test]
	fn load_from_hash_ignore_white_space_change() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &[" line0", "line1"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["  line0", " line1-m"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().ignore_whitespace_change(true));
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -2,1 +2,1 @@",
				"-2  | line1",
				"+  2|  line1-m"
			);
		});
	}

	#[test]
	fn load_from_hash_ignore_white_space() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0", "line1"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["  line0", " line1-m"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().ignore_whitespace(true));
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -2,1 +2,1 @@ line0",
				"-2  | line1",
				"+  2|  line1-m"
			);
		});
	}

	#[test]
	fn load_from_hash_copies() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0"]);
			create_commit(&repo);
			write_normal_file(&repo, "b", &["line0"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().copies(true));
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > b (n)", "Status Copied");
		});
	}

	#[test]
	fn load_from_hash_copies_modified_source() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line0", "a"]);
			write_normal_file(&repo, "b", &["line0"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().copies(true));
			assert_eq!(diff.number_files_changed(), 2);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -1,0 +2,1 @@ line0",
				"+  2| a",
				"a (n) > b (n)",
				"Status Copied"
			);
		});
	}

	#[test]
	fn load_from_hash_interhunk_context() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0", "line1", "line2", "line3", "line4", "line5"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line0", "line1-m", "line2", "line3", "line4-m", "line5"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().interhunk_context(2));
			assert_commit_diff!(
				&diff,
				"a (n)",
				"Status Modified",
				"@@ -2,4 +2,4 @@ line0",
				"-2  | line1",
				"+  2| line1-m",
				" 3 3| line2",
				" 4 4| line3",
				"-5  | line4",
				"+  5| line4-m"
			);
		});
	}

	#[test]
	fn load_from_hash_rename_source_not_modified() {
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0"]);
			create_commit(&repo);
			remove_path(&repo, "a");
			write_normal_file(&repo, "b", &["line0"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().renames(true, 100));
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > b (n)", "Status Renamed");
		});
	}

	#[test]
	fn load_from_hash_rename_source_modified() {
		// this test can be confusing to follow, here is how it is created:
		// - starting with am existing tracked file "a"
		// - move "a" and call it "b"
		// - create a new file "a" with different contents
		// this creates a situation where git detects the rename from the original unmodified
		// version of "a" before a new file called "a" was created
		with_temp_repository(|repo| {
			write_normal_file(&repo, "a", &["line0"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["other0"]);
			write_normal_file(&repo, "b", &["line0"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().renames(true, 100));
			assert_eq!(diff.number_files_changed(), 2);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(
				&diff,
				"a (o) > a (n)",
				"Status Added",
				"@@ -0,0 +1,1 @@",
				"+  1| other0",
				"a (n) > b (n)",
				"Status Renamed"
			);
		});
	}

	#[cfg(unix)]
	#[test]
	fn load_from_hash_file_mode_executable() {
		with_temp_repository(|repo| {
			use std::os::unix::fs::PermissionsExt;
			let root = repo.repo_path().parent().unwrap().to_path_buf();

			write_normal_file(&repo, "a", &["line0"]);
			create_commit(&repo);
			let file = File::open(root.join("a")).unwrap();
			let mut permissions = file.metadata().unwrap().permissions();
			permissions.set_mode(0o755);
			file.set_permissions(permissions).unwrap();
			repo.add_path_to_index(PathBuf::from("a").as_path()).unwrap();
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new().renames(true, 100));
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > a (x)", "Status Modified");
		});
	}

	#[cfg(unix)]
	#[test]
	fn load_from_hash_type_changed() {
		with_temp_repository(|repo| {
			let root = repo.repo_path().parent().unwrap().to_path_buf();

			write_normal_file(&repo, "a", &["line0"]);
			write_normal_file(&repo, "b", &["line0"]);
			create_commit(&repo);
			remove_path(&repo, "a");
			symlink(root.join("b"), root.join("a")).unwrap();
			repo.add_path_to_index(PathBuf::from("a").as_path()).unwrap();
			repo.add_path_to_index(PathBuf::from("b").as_path()).unwrap();
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > a (l)", "Status Typechange");
		});
	}

	#[test]
	fn load_from_hash_binary_added_file() {
		with_temp_repository(|repo| {
			// treat all files as binary
			write_normal_file(&repo, ".gitattributes", &["a binary"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line1"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (o,b) > a (n,b)", "Status Added");
		});
	}

	#[test]
	fn load_from_hash_binary_modified_file() {
		with_temp_repository(|repo| {
			// treat all files as binary
			write_normal_file(&repo, ".gitattributes", &["a binary"]);
			write_normal_file(&repo, "a", &["line1"]);
			create_commit(&repo);
			write_normal_file(&repo, "a", &["line2"]);
			create_commit(&repo);
			let diff = diff_from_head(&repo, &CommitDiffLoaderOptions::new());
			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n,b)", "Status Modified");
		});
	}
}
