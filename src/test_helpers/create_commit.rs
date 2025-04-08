use std::sync::LazyLock;

use git2::{Repository, Signature};

use crate::{diff::Commit, test_helpers::JAN_2021_EPOCH};

static DEFAULT_COMMIT_OPTIONS: LazyLock<CreateCommitOptions> = LazyLock::new(CreateCommitOptions::new);

/// Options for creating a new commit.
#[derive(Debug)]
pub(crate) struct CreateCommitOptions {
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
	#[must_use]
	pub(crate) fn new() -> Self {
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

	/// Set the author name and related email address.
	pub(crate) fn author(&mut self, name: &str) -> &mut Self {
		self.author_name = String::from(name);
		self.author_email = format!("{}@example.com", name.to_lowercase());
		self
	}

	/// Set the author name.
	pub(crate) fn author_name(&mut self, name: &str) -> &mut Self {
		self.author_name = String::from(name);
		self
	}

	/// Set the author email address.
	pub(crate) fn author_email(&mut self, email: &str) -> &mut Self {
		self.author_email = String::from(email);
		self
	}

	/// Set the authored commit time.
	pub(crate) fn author_time(&mut self, time: i64) -> &mut Self {
		self.author_time = Some(time);
		self
	}

	/// Set the committer name and related email address.
	pub(crate) fn committer(&mut self, name: &str) -> &mut Self {
		self.committer_name = Some(String::from(name));
		self.committer_email = Some(format!("{}@example.com", name.to_lowercase()));
		self
	}

	/// Set the committer name.
	pub(crate) fn committer_name(&mut self, name: &str) -> &mut Self {
		self.committer_name = Some(String::from(name));
		self
	}

	/// Set the committer email.
	pub(crate) fn committer_email(&mut self, email: &str) -> &mut Self {
		self.committer_email = Some(String::from(email));
		self
	}

	/// Set the committed commit time.
	pub(crate) fn commit_time(&mut self, time: i64) -> &mut Self {
		self.committer_time = time;
		self
	}

	/// Set the head name.
	pub(crate) fn head_name(&mut self, name: &str) -> &mut Self {
		self.head_name = String::from(name);
		self
	}

	/// Set the commit message.
	pub(crate) fn message(&mut self, message: &str) -> &mut Self {
		self.message = String::from(message);
		self
	}
}

fn create_commit_on_index(
	repository: &Repository,
	reference: &str,
	author: &Signature<'_>,
	committer: &Signature<'_>,
	message: &str,
) -> Result<Commit, git2::Error> {
	let tree = repository.find_tree(repository.index()?.write_tree()?)?;
	let head = repository.find_reference(reference)?.peel_to_commit()?;
	_ = repository.commit(Some("HEAD"), author, committer, message, &tree, &[&head])?;

	Ok(Commit::from(&repository.find_reference(reference)?.peel_to_commit()?))
}

/// Create a commit based on the provided options. If `options` is not provided, will create a
/// commit using the default options. This function does not add modified or new files to the stage
/// before creating a commit.
///
/// # Panics
/// If any Git operation cannot be performed.
pub(crate) fn create_commit(repository: &Repository, options: Option<&CreateCommitOptions>) -> Commit {
	let opts = options.unwrap_or(&DEFAULT_COMMIT_OPTIONS);
	let author_sig = Signature::new(
		opts.author_name.as_str(),
		opts.author_email.as_str(),
		&git2::Time::new(opts.author_time.unwrap_or(opts.committer_time), 0),
	)
	.unwrap();
	let committer_sig = Signature::new(
		opts.committer_name.as_ref().unwrap_or(&opts.author_name).as_str(),
		opts.committer_email.as_ref().unwrap_or(&opts.author_email).as_str(),
		&git2::Time::new(opts.committer_time, 0),
	)
	.unwrap();
	let ref_name = format!("refs/heads/{}", opts.head_name);

	create_commit_on_index(
		repository,
		ref_name.as_str(),
		&author_sig,
		&committer_sig,
		opts.message.as_str(),
	)
	.unwrap()
}
