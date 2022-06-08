use chrono::{DateTime, Local, TimeZone};

use crate::{reference::Reference, user::User};

/// Represents a commit.
#[derive(Debug, PartialEq, Eq)]
pub struct Commit {
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
	#[inline]
	pub fn hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Get the reference to the commit
	#[must_use]
	#[inline]
	pub const fn reference(&self) -> &Option<Reference> {
		&self.reference
	}

	/// Get the author of the commit.
	#[must_use]
	#[inline]
	pub const fn author(&self) -> &User {
		&self.author
	}

	/// Get the author of the commit.
	#[must_use]
	#[inline]
	pub const fn authored_date(&self) -> &Option<DateTime<Local>> {
		&self.authored_date
	}

	/// Get the committer of the commit.
	#[must_use]
	#[inline]
	pub const fn committer(&self) -> &Option<User> {
		&self.committer
	}

	/// Get the committed date of the commit.
	#[must_use]
	#[inline]
	pub const fn committed_date(&self) -> &DateTime<Local> {
		&self.committed_date
	}

	/// Get the commit message summary.
	#[must_use]
	#[inline]
	pub const fn summary(&self) -> &Option<String> {
		&self.summary
	}

	/// Get the full commit message.
	#[must_use]
	#[inline]
	pub const fn message(&self) -> &Option<String> {
		&self.message
	}

	fn new(commit: &git2::Commit<'_>, reference: Option<&git2::Reference<'_>>) -> Self {
		let author = User::new(commit.author().name(), commit.author().email());
		let authored_date = Local.timestamp(commit.author().when().seconds(), 0);
		let message = commit.message().map(String::from);
		let summary = commit.summary().map(String::from);
		let committed_date = Local.timestamp(commit.time().seconds(), 0);

		let try_committer = User::new(commit.committer().name(), commit.committer().email());
		let committer = (try_committer.is_some() && try_committer != author).then(|| try_committer);

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
	type Error = anyhow::Error;

	#[inline]
	fn try_from(reference: &git2::Reference<'_>) -> Result<Self, Self::Error> {
		let commit = reference.peel_to_commit()?;
		Ok(Self::new(&commit, Some(reference)))
	}
}

impl From<&git2::Commit<'_>> for Commit {
	#[inline]
	fn from(commit: &git2::Commit<'_>) -> Self {
		Self::new(commit, None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil::{
		create_commit,
		with_temp_repository,
		CommitBuilder,
		CreateCommitOptions,
		ReferenceBuilder,
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

		assert_eq!(
			commit.reference().as_ref().unwrap(),
			&ReferenceBuilder::new("0123456789ABCDEF").build()
		);
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
		assert_eq!(commit.summary().as_ref().unwrap(), "title");
	}

	#[test]
	fn message() {
		let commit = CommitBuilder::new("0123456789ABCDEF").message("title\n\nbody").build();
		assert_eq!(commit.message().as_ref().unwrap(), "title\n\nbody");
	}

	#[test]
	fn new_authored_date_same_committed_date() {
		with_temp_repository(|repository| {
			create_commit(
				&repository,
				Some(CreateCommitOptions::new().author_time(JAN_2021_EPOCH)),
			);
			let commit = repository.find_commit("refs/heads/main")?;
			assert!(commit.authored_date().is_none());
			Ok(())
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
			let commit = repository.find_commit("refs/heads/main")?;
			assert_eq!(
				commit.authored_date().as_ref().unwrap(),
				&DateTime::parse_from_rfc3339("2021-01-01T00:00:01Z").unwrap()
			);
			Ok(())
		});
	}

	#[test]
	fn new_committer_different_than_author() {
		with_temp_repository(|repository| {
			create_commit(&repository, Some(CreateCommitOptions::new().committer("Committer")));
			let commit = repository.find_commit("refs/heads/main")?;
			assert_eq!(
				commit.committer().as_ref().unwrap(),
				&User::new(Some("Committer"), Some("committer@example.com"))
			);
			Ok(())
		});
	}

	#[test]
	fn new_committer_same_as_author() {
		with_temp_repository(|repository| {
			let commit = repository.find_commit("refs/heads/main")?;
			assert!(commit.committer().is_none());
			Ok(())
		});
	}
}
