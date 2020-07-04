use crate::show_commit::file_stat::FileStat;
use crate::show_commit::user::User;
use crate::show_commit::util::load_commit_state;
use chrono::{DateTime, Local};

#[derive(Debug, PartialEq)]
pub(super) struct Commit {
	author: User,
	body: Option<String>,
	committer: User,
	date: DateTime<Local>,
	file_stats: Option<Vec<FileStat>>,
	hash: String,
}

impl Commit {
	pub(super) fn new(
		hash: String,
		author: User,
		committer: User,
		date: DateTime<Local>,
		file_stats: Option<Vec<FileStat>>,
		body: Option<String>,
	) -> Self
	{
		Commit {
			author,
			body,
			committer,
			date,
			file_stats,
			hash,
		}
	}

	pub(super) fn from_commit_hash(hash: &str) -> Result<Self, String> {
		load_commit_state(hash).map_err(|e| String::from(e.message()))
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

	pub(super) fn get_hash(&self) -> &String {
		&self.hash
	}

	pub(super) fn get_body(&self) -> &Option<String> {
		&self.body
	}

	pub(super) fn get_file_stats(&self) -> &Option<Vec<FileStat>> {
		&self.file_stats
	}
}
