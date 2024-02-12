use chrono::{DateTime, Local, TimeZone};

use crate::git::{GitError, Reference, User};

/// Represents a commit.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Commit {
	pub(crate) hash: String,
	pub(crate) reference: Option<Reference>,
	pub(crate) author: User,
	pub(crate) authored_date: Option<DateTime<Local>>,
	pub(crate) message: Option<String>,
	pub(crate) committer: Option<User>,
	pub(crate) committed_date: DateTime<Local>,
	pub(crate) summary: Option<String>,
}

impl Commit {
	/// Get the hash of the commit
	#[must_use]
	pub(crate) fn hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Get the reference to the commit
	#[must_use]
	pub(crate) const fn reference(&self) -> &Option<Reference> {
		&self.reference
	}

	/// Get the author of the commit.
	#[must_use]
	pub(crate) const fn author(&self) -> &User {
		&self.author
	}

	/// Get the author of the commit.
	#[must_use]
	pub(crate) const fn authored_date(&self) -> &Option<DateTime<Local>> {
		&self.authored_date
	}

	/// Get the committer of the commit.
	#[must_use]
	pub(crate) const fn committer(&self) -> &Option<User> {
		&self.committer
	}

	/// Get the committed date of the commit.
	#[must_use]
	pub(crate) const fn committed_date(&self) -> &DateTime<Local> {
		&self.committed_date
	}

	/// Get the commit message summary.
	#[must_use]
	pub(crate) fn summary(&self) -> Option<&str> {
		self.summary.as_deref()
	}

	/// Get the full commit message.
	#[must_use]
	pub(crate) fn message(&self) -> Option<&str> {
		self.message.as_deref()
	}

	fn new(commit: &git2::Commit<'_>, reference: Option<&git2::Reference<'_>>) -> Self {
		let author = User::new(commit.author().name(), commit.author().email());
		let message = commit.message().map(String::from);
		let summary = commit.summary().map(String::from);
		// this should never panic, since msecs is always zero
		let committed_date = Local.timestamp_opt(commit.time().seconds(), 0).unwrap();
		let authored_date = Local
			.timestamp_opt(commit.author().when().seconds(), 0)
			.single()
			.unwrap_or(committed_date);

		let try_committer = User::new(commit.committer().name(), commit.committer().email());
		let committer = (try_committer.is_some() && try_committer != author).then_some(try_committer);

		Self {
			hash: format!("{}", commit.id()),
			reference: reference.map(Reference::from),
			author,
			authored_date: if authored_date == committed_date {
				None
			}
			else {
				Some(authored_date)
			},
			message,
			committer,
			committed_date,
			summary,
		}
	}
}

impl TryFrom<&git2::Reference<'_>> for Commit {
	type Error = GitError;

	fn try_from(reference: &git2::Reference<'_>) -> Result<Self, Self::Error> {
		let commit = reference
			.peel_to_commit()
			.map_err(|e| GitError::CommitLoad { cause: e })?;
		Ok(Self::new(&commit, Some(reference)))
	}
}

impl From<&git2::Commit<'_>> for Commit {
	fn from(commit: &git2::Commit<'_>) -> Self {
		Self::new(commit, None)
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};
	use testutils::assert_err_eq;

	use super::*;
	use crate::test_helpers::{
		builders::{CommitBuilder, ReferenceBuilder},
		create_commit,
		with_temp_repository,
		CreateCommitOptions,
		JAN_2021_EPOCH,
	};

	#[test]
	fn hash() {
		let commit = CommitBuilder::new("0123456789ABCDEF").build();
		assert_eq!(commit.hash(), "0123456789ABCDEF");
	}

	#[test]
	fn reference() {
		let commit = CommitBuilder::new("0123456789ABCDEF")
			.reference(ReferenceBuilder::new("0123456789ABCDEF").build())
			.build();

		assert_some_eq!(commit.reference(), &ReferenceBuilder::new("0123456789ABCDEF").build());
	}

	#[test]
	fn author() {
		let commit = CommitBuilder::new("0123456789ABCDEF")
			.author(User::new(Some("name"), Some("name@example.com")))
			.build();
		assert_eq!(commit.author(), &User::new(Some("name"), Some("name@example.com")));
	}

	#[test]
	fn committed_date() {
		let commit = CommitBuilder::new("0123456789ABCDEF")
			.commit_time(JAN_2021_EPOCH)
			.build();
		assert_eq!(
			commit.committed_date(),
			&DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap()
		);
	}

	#[test]
	fn summary() {
		let commit = CommitBuilder::new("0123456789ABCDEF").summary("title").build();
		assert_some_eq!(commit.summary(), "title");
	}

	#[test]
	fn message() {
		let commit = CommitBuilder::new("0123456789ABCDEF").message("title\n\nbody").build();
		assert_some_eq!(commit.message(), "title\n\nbody");
	}

	#[test]
	fn new_authored_date_same_committed_date() {
		with_temp_repository(|repository| {
			create_commit(
				&repository,
				Some(CreateCommitOptions::new().author_time(JAN_2021_EPOCH)),
			);
			let commit = repository.find_commit("refs/heads/main").unwrap();
			assert_none!(commit.authored_date());
		});
	}

	#[test]
	fn new_authored_date_different_than_committed() {
		with_temp_repository(|repository| {
			create_commit(
				&repository,
				Some(
					CreateCommitOptions::new()
						.commit_time(JAN_2021_EPOCH)
						.author_time(JAN_2021_EPOCH + 1),
				),
			);
			let commit = repository.find_commit("refs/heads/main").unwrap();
			assert_some_eq!(
				commit.authored_date(),
				&DateTime::parse_from_rfc3339("2021-01-01T00:00:01Z").unwrap()
			);
		});
	}

	#[test]
	fn new_committer_different_than_author() {
		with_temp_repository(|repository| {
			create_commit(&repository, Some(CreateCommitOptions::new().committer("Committer")));
			let commit = repository.find_commit("refs/heads/main").unwrap();
			assert_some_eq!(
				commit.committer(),
				&User::new(Some("Committer"), Some("committer@example.com"))
			);
		});
	}

	#[test]
	fn new_committer_same_as_author() {
		with_temp_repository(|repository| {
			let commit = repository.find_commit("refs/heads/main").unwrap();
			assert_none!(commit.committer());
		});
	}

	#[test]
	fn try_from_success() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let repo_lock = repo.lock();
			let reference = repo_lock.find_reference("refs/heads/main").unwrap();
			let commit = Commit::try_from(&reference).unwrap();

			assert_eq!(commit.reference.unwrap().shortname(), "main");
		});
	}

	#[test]
	fn try_from_error() {
		with_temp_repository(|repository| {
			let repo = repository.repository();
			let repo_lock = repo.lock();
			let blob = repo_lock.blob(b"foo").unwrap();
			_ = repo_lock.reference("refs/blob", blob, false, "blob").unwrap();

			let reference = repo_lock.find_reference("refs/blob").unwrap();
			assert_err_eq!(Commit::try_from(&reference), GitError::CommitLoad {
				cause: git2::Error::new(
					git2::ErrorCode::InvalidSpec,
					git2::ErrorClass::Object,
					format!(
						"the git_object of id '{blob}' can not be successfully peeled into a commit (git_object_t=1).",
					)
				)
			});
		});
	}
}
