use std::{
	fmt::{Debug, Formatter},
	path::PathBuf,
	sync::{Arc, LazyLock},
	time::{Duration, Instant},
};

use git2::{Diff, DiffFindOptions, DiffOptions, Repository};
use parking_lot::{Mutex, RwLock};

use crate::{
	diff::{
		Commit,
		CommitDiff,
		CommitDiffLoaderOptions,
		Delta,
		DiffLine,
		FileMode,
		FileStatus,
		FileStatusBuilder,
		Status,
		thread::LoadStatus,
	},
	git::GitError,
};

static UNKNOWN_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("unknown"));

pub(crate) trait DiffUpdateHandlerFn: Fn(LoadStatus) -> bool + Sync + Send {}

impl<FN: Fn(LoadStatus) -> bool + Sync + Send> DiffUpdateHandlerFn for FN {}

fn create_status_update(quick: bool, processed_files: usize, total_files: usize) -> LoadStatus {
	if quick {
		LoadStatus::QuickDiff(processed_files, total_files)
	}
	else {
		LoadStatus::Diff(processed_files, total_files)
	}
}

pub(crate) struct CommitDiffLoader {
	config: CommitDiffLoaderOptions,
	repository: Repository,
	commit_diff: Arc<RwLock<CommitDiff>>,
}

impl CommitDiffLoader {
	pub(crate) fn new(repository: Repository, config: CommitDiffLoaderOptions) -> Self {
		Self {
			repository,
			config,
			commit_diff: Arc::new(RwLock::new(CommitDiff::new())),
		}
	}

	pub(crate) fn reset(&mut self) {
		self.commit_diff.write().clear();
	}

	pub(crate) fn commit_diff(&self) -> Arc<RwLock<CommitDiff>> {
		Arc::clone(&self.commit_diff)
	}

	fn diff<'repo>(
		repository: &'repo Repository,
		config: &CommitDiffLoaderOptions,
		commit: &git2::Commit<'_>,
		diff_options: &mut DiffOptions,
	) -> Result<Diff<'repo>, GitError> {
		_ = diff_options
			.context_lines(config.context_lines)
			.ignore_filemode(false)
			.ignore_whitespace(config.ignore_whitespace)
			.ignore_whitespace_change(config.ignore_whitespace_change)
			.ignore_blank_lines(config.ignore_blank_lines)
			.include_typechange(true)
			.include_typechange_trees(true)
			.indent_heuristic(true)
			.interhunk_lines(config.interhunk_context)
			.minimal(true);

		let commit_tree = commit.tree().map_err(|e| GitError::DiffLoad { cause: e })?;

		if let Some(p) = commit.parents().next() {
			let parent_tree = p.tree().map_err(|e| GitError::DiffLoad { cause: e })?;
			repository.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(diff_options))
		}
		else {
			repository.diff_tree_to_tree(None, Some(&commit_tree), Some(diff_options))
		}
		.map_err(|e| GitError::DiffLoad { cause: e })
	}

	pub(crate) fn load_diff(&mut self, hash: &str, update_notifier: impl DiffUpdateHandlerFn) -> Result<(), GitError> {
		let oid = self
			.repository
			.revparse_single(hash)
			.map_err(|e| GitError::DiffLoad { cause: e })?
			.id();
		let commit = self
			.repository
			.find_commit(oid)
			.map_err(|e| GitError::DiffLoad { cause: e })?;

		{
			// only the first parent matter for things like diffs, the second parent, if it exists,
			// is only used for conflict resolution, and has no use
			let parent = commit.parents().next().map(|c| Commit::from(&c));
			let mut commit_diff = self.commit_diff.write();
			commit_diff.reset(Commit::from(&commit), parent);
			if update_notifier(LoadStatus::New) {
				return Ok(());
			}
		}

		// when a diff contains a lot of untracked files, collecting the diff information can take
		// upwards of a minute. This performs a quicker diff, that does not detect copies and
		// renames against unmodified files.
		if self.config.copies {
			let should_continue = self.collect(
				&Self::diff(&self.repository, &self.config, &commit, &mut DiffOptions::new())?,
				&update_notifier,
				true,
			)?;

			if !should_continue || update_notifier(LoadStatus::CompleteQuickDiff) {
				return Ok(());
			}
		}

		let mut diff_options = DiffOptions::new();
		// include_unmodified added to find copies from unmodified files
		_ = diff_options.include_unmodified(self.config.copies);
		let mut diff = Self::diff(&self.repository, &self.config, &commit, &mut diff_options)?;

		let mut diff_find_options = DiffFindOptions::new();
		_ = diff_find_options
			.rename_limit(self.config.rename_limit as usize)
			.renames(self.config.renames)
			.renames_from_rewrites(self.config.renames)
			.rewrites(self.config.renames)
			.copies(self.config.copies)
			.copies_from_unmodified(self.config.copies);

		diff.find_similar(Some(&mut diff_find_options))
			.map_err(|e| GitError::DiffLoad { cause: e })?;
		let should_continue = self.collect(&diff, &update_notifier, false)?;

		if should_continue {
			_ = update_notifier(LoadStatus::DiffComplete);
			return Ok(());
		}
		Ok(())
	}

	pub(crate) fn collect(
		&self,
		diff: &Diff<'_>,
		update_handler: &impl DiffUpdateHandlerFn,
		quick: bool,
	) -> Result<bool, GitError> {
		let file_stats_builder = Mutex::new(FileStatusBuilder::new());
		let mut unmodified_file_count: usize = 0;
		let mut change_count: usize = 0;

		let stats = diff.stats().map_err(|e| GitError::DiffLoad { cause: e })?;
		let total_files_changed = stats.files_changed();

		if update_handler(create_status_update(quick, 0, total_files_changed)) {
			return Ok(false);
		}
		let mut time = Instant::now();

		let collect_result = diff.foreach(
			&mut |diff_delta, _| {
				change_count += 1;

				#[cfg(test)]
				{
					// this is needed to test timing in tests, the other option would be to mock
					// Instant, but that's more effort than is worth the value.
					// Since this adds 10ms of delay for each delta, each file added to the diff
					// will add ~10ms of delay.
					//
					// this may be flaky, due to the Diff progress being based on time. However,
					// the added thread sleep during tests should make the diff progress very
					// stable, as the diff processing can process the files much faster than a
					// fraction of a millisecond.
					std::thread::sleep(Duration::from_millis(10));
				}
				if time.elapsed() > Duration::from_millis(25) {
					if update_handler(create_status_update(quick, change_count, total_files_changed)) {
						return false;
					}
					time = Instant::now();
				}

				// unmodified files are included for copy detection, so ignore
				if diff_delta.status() == git2::Delta::Unmodified {
					unmodified_file_count += 1;
					return true;
				}

				let source_file = diff_delta.old_file();
				let source_file_mode = FileMode::from(source_file.mode());
				let source_file_path = source_file.path().unwrap_or(UNKNOWN_PATH.as_path());

				let destination_file = diff_delta.new_file();
				let destination_file_mode = FileMode::from(destination_file.mode());
				let destination_file_path = destination_file.path().unwrap_or(UNKNOWN_PATH.as_path());

				let mut fsb = file_stats_builder.lock();
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
		);

		// error caused by early return
		if collect_result.is_err() {
			return Ok(false);
		}

		let mut commit_diff = self.commit_diff.write();

		let number_files_changed = total_files_changed - unmodified_file_count;
		let number_insertions = stats.insertions();
		let number_deletions = stats.deletions();

		let fsb = file_stats_builder.into_inner();
		commit_diff.update(fsb.build(), number_files_changed, number_insertions, number_deletions);
		Ok(true)
	}
}

impl Debug for CommitDiffLoader {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CommitDiffLoader")
			.field(
				"repository",
				&format!("Repository({})", &self.repository.path().display()),
			)
			.finish_non_exhaustive()
	}
}

#[cfg(all(unix, test))]
mod tests {
	use std::{
		fs::{File, remove_file},
		io::Write as _,
		os::unix::fs::symlink,
	};

	use git2::Index;

	use super::*;
	use crate::{diff::Origin, test_helpers::with_temp_repository};

	impl CommitDiffLoader {
		fn take_diff(mut self) -> CommitDiff {
			let diff = std::mem::replace(&mut self.commit_diff, Arc::new(RwLock::new(CommitDiff::new())));
			Arc::try_unwrap(diff).unwrap().into_inner()
		}
	}

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
	#[expect(clippy::string_slice, reason = "Slice on safe range.")]
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
	fn index(repository: &Repository) -> Index {
		repository.index().unwrap()
	}

	#[cfg(not(tarpaulin_include))]
	fn root_path(repository: &Repository) -> PathBuf {
		repository.path().to_path_buf().parent().unwrap().to_path_buf()
	}

	#[cfg(not(tarpaulin_include))]
	fn commit_from_ref<'repo>(repository: &'repo Repository, reference: &str) -> git2::Commit<'repo> {
		repository.find_reference(reference).unwrap().peel_to_commit().unwrap()
	}

	#[cfg(not(tarpaulin_include))]
	fn add_path(repository: &Repository, name: &str) {
		index(repository).add_path(PathBuf::from(name).as_path()).unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn write_normal_file(repository: &Repository, name: &str, contents: &[&str]) {
		let file_path = root_path(repository).join(name);
		let mut file = File::create(file_path.as_path()).unwrap();
		if !contents.is_empty() {
			writeln!(file, "{}", contents.join("\n")).unwrap();
		}

		index(repository).add_path(PathBuf::from(name).as_path()).unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn remove_path(repository: &Repository, name: &str) {
		let file_path = root_path(repository).join(name);
		_ = remove_file(file_path.as_path());

		index(repository).remove_path(PathBuf::from(name).as_path()).unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn create_commit(repository: &Repository) {
		let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(1_609_459_200, 0)).unwrap();
		let tree = repository.find_tree(index(repository).write_tree().unwrap()).unwrap();
		let head = commit_from_ref(repository, "refs/heads/main");
		_ = repository
			.commit(Some("HEAD"), &sig, &sig, "title", &tree, &[&head])
			.unwrap();
	}

	#[cfg(not(tarpaulin_include))]
	fn diff_from_head(repository: Repository, options: CommitDiffLoaderOptions) -> Result<CommitDiffLoader, GitError> {
		let commit = commit_from_ref(&repository, "refs/heads/main");
		let hash = commit.id().to_string();
		drop(commit);
		let mut loader = CommitDiffLoader::new(repository, options);
		loader.load_diff(hash.as_str(), |_| false)?;
		Ok(loader)
	}

	#[cfg(not(tarpaulin_include))]
	fn diff_with_notifier(
		repository: Repository,
		options: CommitDiffLoaderOptions,
		update_notifier: impl DiffUpdateHandlerFn,
	) -> Result<CommitDiffLoader, GitError> {
		let commit = commit_from_ref(&repository, "refs/heads/main");
		let hash = commit.id().to_string();
		drop(commit);
		let mut loader = CommitDiffLoader::new(repository, options);
		loader.load_diff(hash.as_str(), update_notifier)?;
		Ok(loader)
	}

	#[test]
	fn load_from_hash_commit_no_parents() {
		with_temp_repository(|repository| {
			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 0);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
		});
	}

	#[test]
	fn load_from_hash_added_file() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);
			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (o) > a (n)", "Status Added", "@@ -0,0 +1,1 @@", "+  1| line1");
		});
	}

	#[test]
	fn load_from_hash_removed_file() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);
			remove_path(&repository, "a");
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["line2"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &[
				"line0", "line1", "line2", "line3", "line4", "line5",
			]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &[
				"line0",
				"line1",
				"line2",
				"line3-m",
				"line4",
				"line5",
			]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().context_lines(2)).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &[" line0", "line1"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["  line0", " line1-m"]);
			create_commit(&repository);

			let loader = diff_from_head(
				repository,
				CommitDiffLoaderOptions::new().ignore_whitespace_change(true),
			)
			.unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0", "line1"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["  line0", " line1-m"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().ignore_whitespace(true)).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0"]);
			create_commit(&repository);
			write_normal_file(&repository, "b", &["line0"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().copies(true)).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > b (n)", "Status Copied");
		});
	}

	#[test]
	fn load_from_hash_copies_modified_source() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["line0", "a"]);
			write_normal_file(&repository, "b", &["line0"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().copies(true)).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &[
				"line0", "line1", "line2", "line3", "line4", "line5",
			]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &[
				"line0",
				"line1-m",
				"line2",
				"line3",
				"line4-m",
				"line5",
			]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().interhunk_context(2)).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0"]);
			create_commit(&repository);
			remove_path(&repository, "a");
			write_normal_file(&repository, "b", &["line0"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().renames(true, 100)).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > b (n)", "Status Renamed");
		});
	}

	#[test]
	fn load_from_hash_rename_source_modified() {
		// this test can be confusing to follow, here is how it is created:
		// - starting with an existing tracked file "a"
		// - move "a" and call it "b"
		// - create a new file "a" with different contents
		// this creates a situation where git detects the rename from the original unmodified
		// version of "a" before a new file called "a" was created
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["other0"]);
			write_normal_file(&repository, "b", &["line0"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().renames(true, 100)).unwrap();
			let diff = loader.take_diff();

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
		with_temp_repository(|repository| {
			use std::os::unix::fs::PermissionsExt as _;

			let root = root_path(&repository);

			write_normal_file(&repository, "a", &["line0"]);
			create_commit(&repository);
			let file = File::open(root.join("a")).unwrap();
			let mut permissions = file.metadata().unwrap().permissions();
			permissions.set_mode(0o755);
			file.set_permissions(permissions).unwrap();

			add_path(&repository, "a");
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new().renames(true, 100)).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > a (x)", "Status Modified");
		});
	}

	#[cfg(unix)]
	#[test]
	fn load_from_hash_type_changed() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line0"]);
			write_normal_file(&repository, "b", &["line0"]);
			create_commit(&repository);
			remove_path(&repository, "a");
			let root = root_path(&repository);
			symlink(root.join("b"), root.join("a")).unwrap();
			add_path(&repository, "a");
			add_path(&repository, "b");
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n) > a (l)", "Status Typechange");
		});
	}

	#[test]
	fn load_from_hash_binary_added_file() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 1);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (o) > a (n)", "Status Added", "@@ -0,0 +1,1 @@", "+  1| line1");
		});
	}

	#[test]
	fn load_from_hash_binary_modified_file() {
		with_temp_repository(|repository| {
			// treat all files as binary
			write_normal_file(&repository, ".gitattributes", &["a binary"]);
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);
			write_normal_file(&repository, "a", &["line2"]);
			create_commit(&repository);

			let loader = diff_from_head(repository, CommitDiffLoaderOptions::new()).unwrap();
			let diff = loader.take_diff();

			assert_eq!(diff.number_files_changed(), 1);
			assert_eq!(diff.number_insertions(), 0);
			assert_eq!(diff.number_deletions(), 0);
			assert_commit_diff!(&diff, "a (n,b)", "Status Modified");
		});
	}

	#[test]
	fn diff_notifier() {
		with_temp_repository(|repository| {
			for i in 0..10 {
				write_normal_file(&repository, format!("a-{i}").as_str(), &["line"]);
			}
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				false
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new(), notifier).unwrap();

			let c = calls.lock();
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
			assert_eq!(c.get(1).unwrap(), &LoadStatus::Diff(0, 10));
			assert!(matches!(c.get(2).unwrap(), &LoadStatus::Diff(_, 10)));
			assert!(matches!(c.last().unwrap(), &LoadStatus::DiffComplete));
		});
	}

	#[test]
	fn diff_notifier_with_copies() {
		with_temp_repository(|repository| {
			for i in 0..10 {
				write_normal_file(&repository, format!("a-{i}").as_str(), &["line"]);
			}
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				false
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new().copies(true), notifier).unwrap();

			// Since the exact emitted statues are based on time, this matches a dynamic pattern of:
			// 		- New
			// 		- QuickDiff(0, 10)
			// 		- QuickDiff(>0, 10)
			// 		- CompleteQuickDiff
			// 		- Diff(0, 10)
			// 		- Diff(>0, 10)
			// 		- DiffComplete
			let mut pass = false;
			let mut expected = LoadStatus::New;
			for c in calls.lock().clone() {
				match (&expected, c) {
					(&LoadStatus::New, LoadStatus::New) => {
						expected = LoadStatus::QuickDiff(0, 10);
					},
					(&LoadStatus::QuickDiff(0, 10), LoadStatus::QuickDiff(0, 10)) => {
						expected = LoadStatus::QuickDiff(1, 10);
					},
					(&LoadStatus::QuickDiff(1, 10), LoadStatus::QuickDiff(p, 10)) => {
						assert!(p > 0);
						expected = LoadStatus::CompleteQuickDiff;
					},
					(&LoadStatus::CompleteQuickDiff, LoadStatus::CompleteQuickDiff) => {
						expected = LoadStatus::Diff(0, 10);
					},
					(&LoadStatus::Diff(0, 10), LoadStatus::Diff(0, 10)) => {
						expected = LoadStatus::Diff(1, 10);
					},
					(&LoadStatus::Diff(1, 10), LoadStatus::Diff(p, 10)) => {
						assert!(p > 0);
						expected = LoadStatus::DiffComplete;
					},
					(&LoadStatus::DiffComplete, LoadStatus::DiffComplete) => {
						pass = true;
					},
					(..) => {},
				}
			}

			assert!(pass);
		});
	}

	#[test]
	fn cancel_diff_after_setting_commit() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				true
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new(), notifier).unwrap();

			let c = calls.lock();
			assert_eq!(c.len(), 1);
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
		});
	}

	#[test]
	fn cancel_diff_after_collect_load_stats() {
		with_temp_repository(|repository| {
			write_normal_file(&repository, "a", &["line1"]);
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				c.len() == 2
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new(), notifier).unwrap();

			let c = calls.lock();
			assert_eq!(c.len(), 2);
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
			assert_eq!(c.get(1).unwrap(), &LoadStatus::Diff(0, 1));
		});
	}

	#[test]
	fn cancel_diff_during_diff_collect() {
		with_temp_repository(|repository| {
			for i in 0..10 {
				write_normal_file(&repository, format!("a-{i}").as_str(), &["line"]);
			}
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				c.len() == 4
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new(), notifier).unwrap();
			let c = calls.lock();
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
			assert_eq!(c.get(1).unwrap(), &LoadStatus::Diff(0, 10));
			assert!(matches!(c.last().unwrap(), &LoadStatus::Diff(_, 10)));
		});
	}

	#[test]
	fn cancel_diff_during_quick_diff_collect() {
		with_temp_repository(|repository| {
			for i in 0..10 {
				write_normal_file(&repository, format!("a-{i}").as_str(), &["line"]);
			}
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				c.push(status);
				c.len() == 3
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new().copies(true), notifier).unwrap();
			let c = calls.lock();
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
			assert_eq!(c.get(1).unwrap(), &LoadStatus::QuickDiff(0, 10));
			assert!(matches!(c.last().unwrap(), &LoadStatus::QuickDiff(_, 10)));
		});
	}

	#[test]
	fn cancel_diff_during_quick_diff_complete() {
		with_temp_repository(|repository| {
			for i in 0..10 {
				write_normal_file(&repository, format!("a-{i}").as_str(), &["line"]);
			}
			create_commit(&repository);

			let calls = Arc::new(Mutex::new(Vec::new()));
			let notifier_calls = Arc::clone(&calls);
			let notifier = move |status| {
				let mut c = notifier_calls.lock();
				let rtn = status == LoadStatus::CompleteQuickDiff;
				c.push(status);
				rtn
			};

			_ = diff_with_notifier(repository, CommitDiffLoaderOptions::new().copies(true), notifier).unwrap();
			let c = calls.lock();
			assert_eq!(c.first().unwrap(), &LoadStatus::New);
			assert_eq!(c.get(1).unwrap(), &LoadStatus::QuickDiff(0, 10));
			assert!(matches!(c.get(2).unwrap(), &LoadStatus::QuickDiff(_, 10)));
			assert_eq!(c.last().unwrap(), &LoadStatus::CompleteQuickDiff);
		});
	}
}
