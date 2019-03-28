use chrono::{DateTime, Local, TimeZone};
use git2::{Delta, DiffFindOptions, DiffOptions, Error, Repository};

#[derive(Debug, Eq, PartialEq)]
pub struct User {
	name: Option<String>,
	email: Option<String>,
}

impl User {
	pub fn to_string(&self) -> Option<String> {
		let name = &self.name;
		let email = &self.email;
		match name {
			Some(n) => {
				match email {
					Some(e) => Some(format!("{} <{}>", *n, *e)),
					None => Some(n.to_string()),
				}
			},
			None => {
				match email {
					Some(e) => Some(format!("<{}>", *e)),
					None => None,
				}
			},
		}
	}
}

#[derive(PartialEq, Debug)]
pub struct FileStat {
	status: Delta,
	to_name: String,
	from_name: String,
}

#[derive(PartialEq, Debug)]
pub struct Commit {
	author: User,
	body: Option<String>,
	committer: User,
	date: DateTime<Local>,
	file_stats: Option<Vec<FileStat>>,
	hash: String,
}

fn load_commit_state(hash: &str) -> Result<Commit, Error> {
	let repo = Repository::open_from_env()?;
	let commit = repo.find_commit(repo.revparse_single(hash)?.id())?;

	let full_hash = commit.id().to_string();
	let author_name = commit.author().name().map(String::from);
	let author_email = commit.author().email().map(String::from);
	let committer_name = commit.committer().name().map(String::from);
	let committer_email = commit.committer().email().map(String::from);
	let date = Local.timestamp(commit.time().seconds(), 0);
	let body = commit.message().map(String::from);

	let author = User {
		email: author_email,
		name: author_name,
	};

	let committer = User {
		email: committer_email,
		name: committer_name,
	};

	let committer = if committer != author {
		committer
	}
	else {
		User {
			email: None,
			name: None,
		}
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

			Some(
				diff.deltas()
					.map(|d| {
						FileStat {
							status: d.status(),
							from_name: d
								.old_file()
								.path()
								.map(|p| String::from(p.to_str().unwrap()))
								.unwrap_or_else(|| String::from("unknown")),
							to_name: d
								.new_file()
								.path()
								.map(|p| String::from(p.to_str().unwrap()))
								.unwrap_or_else(|| String::from("unknown")),
						}
					})
					// unmodified isn't being correctly removed
					.filter(|d| d.status != Delta::Unmodified)
					.collect::<Vec<FileStat>>(),
			)
		},
	};

	Ok(Commit {
		author,
		body,
		committer,
		date,
		file_stats,
		hash: full_hash,
	})
}

impl FileStat {
	pub fn get_status(&self) -> &Delta {
		&self.status
	}

	pub fn get_to_name(&self) -> &String {
		&self.to_name
	}

	pub fn get_from_name(&self) -> &String {
		&self.from_name
	}
}

impl Commit {
	pub fn from_commit_hash(hash: &str) -> Result<Self, String> {
		load_commit_state(hash).map_err(|e| String::from(e.message()))
	}

	pub fn get_author(&self) -> &User {
		&self.author
	}

	pub fn get_committer(&self) -> &User {
		&self.committer
	}

	pub fn get_date(&self) -> &DateTime<Local> {
		&self.date
	}

	pub fn get_hash(&self) -> &String {
		&self.hash
	}

	pub fn get_body(&self) -> &Option<String> {
		&self.body
	}

	pub fn get_file_stats(&self) -> &Option<Vec<FileStat>> {
		&self.file_stats
	}

	pub fn get_file_stats_length(&self) -> usize {
		match &self.file_stats {
			Some(s) => s.len(),
			None => 0,
		}
	}
}
