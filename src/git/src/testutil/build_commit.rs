use chrono::{Local, TimeZone};

use crate::{testutil::JAN_2021_EPOCH, Commit, Reference, User};

/// Builder for creating a new commit.
#[derive(Debug)]
pub struct CommitBuilder {
	commit: Commit,
}

impl CommitBuilder {
	/// Create a new instance of the builder with the provided hash. The new instance will default
	/// to a committed date of Jan 1, 2021 UTC. All other fields are `None`.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn new(hash: &str) -> Self {
		Self {
			commit: Commit {
				hash: String::from(hash),
				reference: None,
				author: User::new(None, None),
				authored_date: None,
				committed_date: Local.timestamp_opt(JAN_2021_EPOCH, 0).unwrap(),
				committer: None,
				message: None,
				summary: None,
			},
		}
	}

	/// Set the hash.
	#[inline]
	#[must_use]
	pub fn hash(mut self, hash: &str) -> Self {
		self.commit.hash = String::from(hash);
		self
	}

	/// Set the reference, use `create::testutil::ReferenceBuilder` to build a `Reference`.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn reference(mut self, reference: Reference) -> Self {
		self.commit.reference = Some(reference);
		self
	}

	/// Set the author name and related email address.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn author(mut self, author: User) -> Self {
		self.commit.author = author;
		self
	}

	/// Set the authored commit time from number of seconds since unix epoch.
	#[inline]
	#[must_use]
	pub fn authored_time(mut self, time: i64) -> Self {
		self.commit.authored_date = Some(Local.timestamp_opt(time, 0).unwrap());
		self
	}

	/// Set the committer name and related email address.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn committer(mut self, committer: User) -> Self {
		self.commit.committer = Some(committer);
		self
	}

	/// Set the committed commit time from number of seconds since unix epoch.
	#[inline]
	#[must_use]
	pub fn commit_time(mut self, time: i64) -> Self {
		self.commit.committed_date = Local.timestamp_opt(time, 0).unwrap();
		self
	}

	/// Set the commit summary.
	#[inline]
	#[must_use]
	pub fn summary(mut self, summary: &str) -> Self {
		self.commit.summary = Some(String::from(summary));
		self
	}

	/// Set the commit message.
	#[inline]
	#[must_use]
	pub fn message(mut self, message: &str) -> Self {
		self.commit.message = Some(String::from(message));
		self
	}

	/// Build the `Commit`.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn build(self) -> Commit {
		self.commit
	}
}
