use crate::commit::file_stat::FileStat;
use crate::commit::user::User;
use crate::commit::Commit;
use chrono::{Local, TimeZone};
use git2::{Delta, DiffFindOptions, DiffOptions, Error, Repository};

pub(super) fn load_commit_state(hash: &str) -> Result<Commit, Error> {
	let repo = Repository::open_from_env()?;
	let commit = repo.find_commit(repo.revparse_single(hash)?.id())?;

	let full_hash = commit.id().to_string();
	let date = Local.timestamp(commit.time().seconds(), 0);
	let body = commit.message().map(String::from);

	let author = User::new(commit.author().name(), commit.author().email());

	let committer = User::new(commit.committer().name(), commit.committer().email());

	let committer = if committer != author {
		committer
	}
	else {
		User::new(None, None)
	};

	let mut diff_options = DiffOptions::new();
	let diff_options = diff_options
		.include_unmodified(true)
		.show_unmodified(true)
		.ignore_filemode(false)
		.include_typechange_trees(true)
		.include_typechange(true);

	let mut diff_find_options = DiffFindOptions::new();
	let diff_find_options = diff_find_options
		.renames(true)
		.copies_from_unmodified(true)
		.copies(true)
		.remove_unmodified(true); // this doesn't seem to work

	// some commits do not have parents, and can't have file stats
	let file_stats = match commit.parent_ids().count() {
		0 => None,
		_ => {
			let mut diff = repo.diff_tree_to_tree(
				// parent exists from check aboe
				Some(&commit.parent(0)?.tree()?),
				Some(&commit.tree()?),
				Some(diff_options),
			)?;

			diff.find_similar(Some(diff_find_options))?;

			// filter unmodified isn't being correctly removed
			Some(
				diff.deltas()
					.map(|d| {
						FileStat::new(
							d.old_file()
								.path()
								.map(|p| String::from(p.to_str().unwrap()))
								.unwrap_or_else(|| String::from("unknown")),
							d.new_file()
								.path()
								.map(|p| String::from(p.to_str().unwrap()))
								.unwrap_or_else(|| String::from("unknown")),
							d.status(),
						)
					})
					.filter(|d| *d.get_status() != Delta::Unmodified)
					.collect::<Vec<FileStat>>(),
			)
		},
	};

	Ok(Commit::new(full_hash, author, committer, date, file_stats, body))
}
