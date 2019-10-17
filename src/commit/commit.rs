use crate::commit::file_stat::FileStat;
use crate::commit::user::User;
use crate::commit::utils::load_commit_state;
use chrono::{DateTime, Local};

#[derive(Debug, PartialEq)]
pub struct Commit {
	author: User,
	body: Option<String>,
	committer: User,
	date: DateTime<Local>,
	file_stats: Option<Vec<FileStat>>,
	hash: String,
}

impl Commit {
	pub fn new(
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
