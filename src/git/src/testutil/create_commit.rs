#![cfg(not(tarpaulin_include))]

use lazy_static::lazy_static;

use crate::{testutil::JAN_2021_EPOCH, Repository};

lazy_static! {
	static ref DEFAULT_COMMIT_OPTIONS: CreateCommitOptions = CreateCommitOptions::default();
}

/// Options for creating a new commit.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct CreateCommitOptions {
	author_name: String,
	author_email: String,
	author_time: Option<i64>,
	committer_name: Option<String>,
	committer_email: Option<String>,
	committer_time: i64,
	head_name: String,
	message: String,
}

impl CreateCommitOptions {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Set the author name and related email address.
	#[inline]
	pub fn author(&mut self, name: &str) -> &mut Self {
		self.author_name = String::from(name);
		self.author_email = format!("{}@example.com", name.to_lowercase());
		self
	}

	/// Set the author name.
	#[inline]
	pub fn author_name(&mut self, name: &str) -> &mut Self {
		self.author_name = String::from(name);
		self
	}

	/// Set the author email address.
	#[inline]
	pub fn author_email(&mut self, email: &str) -> &mut Self {
		self.author_email = String::from(email);
		self
	}

	/// Set the authored commit time.
	#[inline]
	pub fn author_time(&mut self, time: i64) -> &mut Self {
		self.author_time = Some(time);
		self
	}

	/// Set the committer name and related email address.
	#[inline]
	pub fn committer(&mut self, name: &str) -> &mut Self {
		self.committer_name = Some(String::from(name));
		self.committer_email = Some(format!("{}@example.com", name.to_lowercase()));
		self
	}

	/// Set the committer name.
	#[inline]
	pub fn committer_name(&mut self, name: &str) -> &mut Self {
		self.committer_name = Some(String::from(name));
		self
	}

	/// Set the committer email.
	#[inline]
	pub fn committer_email(&mut self, email: &str) -> &mut Self {
		self.committer_email = Some(String::from(email));
		self
	}

	/// Set the committed commit time.
	#[inline]
	pub fn commit_time(&mut self, time: i64) -> &mut Self {
		self.committer_time = time;
		self
	}

	/// Set the head name.
	#[inline]
	pub fn head_name(&mut self, name: &str) -> &mut Self {
		self.head_name = String::from(name);
		self
	}

	/// Set the commit message.
	#[inline]
	pub fn message(&mut self, message: &str) -> &mut Self {
		self.message = String::from(message);
		self
	}
}

impl Default for CreateCommitOptions {
	#[inline]
	fn default() -> Self {
		Self {
			author_name: String::from("Author"),
			author_email: String::from("author@example.com"),
			author_time: None,
			committer_name: None,
			committer_email: None,
			committer_time: JAN_2021_EPOCH,
			head_name: String::from("main"),
			message: String::from("title\n\nbody"),
		}
	}
}

/// Create a commit based on the provided options. If `options` is not provided, will create a
/// commit using the default options. This function does not add modified or new files to the stage
/// before creating a commit.
///
/// # Errors
///
/// If the commit cannot be created for any reason, this function will error.
#[inline]
pub fn create_commit(repository: &Repository, options: Option<&CreateCommitOptions>) -> Result<(), git2::Error> {
	let repo = repository.git2_repository();
	let opts = options.unwrap_or(&DEFAULT_COMMIT_OPTIONS);
	let id = repo.index()?.write_tree()?;
	let tree = repo.find_tree(id)?;
	let author_sig = git2::Signature::new(
		opts.author_name.as_str(),
		opts.author_email.as_str(),
		&git2::Time::new(opts.author_time.unwrap_or(opts.committer_time), 0),
	)?;
	let committer_sig = git2::Signature::new(
		opts.committer_name.as_ref().unwrap_or(&opts.author_name).as_str(),
		opts.committer_email.as_ref().unwrap_or(&opts.author_email).as_str(),
		&git2::Time::new(opts.committer_time, 0),
	)?;
	let main = repo.find_reference(format!("refs/heads/{}", opts.head_name).as_str())?;
	let head = main.peel_to_commit()?;
	let _ = repo.commit(
		Some("HEAD"),
		&author_sig,
		&committer_sig,
		opts.message.as_str(),
		&tree,
		&[&head],
	)?;
	Ok(())
}
