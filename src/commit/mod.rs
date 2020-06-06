mod file_stat;
pub(crate) mod status;
mod user;
mod utils;

use crate::commit::file_stat::FileStat;
use crate::commit::user::User;
use crate::commit::utils::load_commit_state;
use chrono::{DateTime, Local};

#[derive(Debug, PartialEq)]
pub(crate) struct Commit {
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

	pub(crate) fn from_commit_hash(hash: &str) -> Result<Self, String> {
		load_commit_state(hash).map_err(|e| String::from(e.message()))
	}

	pub(crate) fn get_author(&self) -> &User {
		&self.author
	}

	pub(crate) fn get_committer(&self) -> &User {
		&self.committer
	}

	pub(crate) fn get_date(&self) -> &DateTime<Local> {
		&self.date
	}

	pub(crate) fn get_hash(&self) -> &String {
		&self.hash
	}

	pub(crate) fn get_body(&self) -> &Option<String> {
		&self.body
	}

	pub(crate) fn get_file_stats(&self) -> &Option<Vec<FileStat>> {
		&self.file_stats
	}
}
