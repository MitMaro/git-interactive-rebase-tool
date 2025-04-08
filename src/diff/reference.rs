use crate::diff::ReferenceKind;

/// Represents a pointer to an object in Git.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Reference {
	/// The object id
	hash: String,
	/// The reference full name
	name: String,
	/// The reference shorthand name
	shorthand: String,
	/// The kind of reference
	kind: ReferenceKind,
}

impl Reference {
	pub(crate) fn new(hash: String, name: String, shorthand: String, kind: ReferenceKind) -> Self {
		Self {
			hash,
			name,
			shorthand,
			kind,
		}
	}

	/// Get the oid of the reference
	#[must_use]
	#[expect(unused, reason = "Available for future use")]
	pub(crate) fn hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Get the name of the reference
	#[must_use]
	#[expect(unused, reason = "Available for future use")]
	pub(crate) fn name(&self) -> &str {
		self.name.as_str()
	}

	/// Get the shorthand name of the reference
	#[must_use]
	#[expect(unused, reason = "Available for future use")]
	pub(crate) fn shortname(&self) -> &str {
		self.shorthand.as_str()
	}

	/// Get the kind of the reference
	#[must_use]
	#[expect(unused, reason = "Available for future use")]
	pub(crate) const fn kind(&self) -> ReferenceKind {
		self.kind
	}
}

impl From<&git2::Reference<'_>> for Reference {
	fn from(reference: &git2::Reference<'_>) -> Self {
		let oid = reference
			.peel(git2::ObjectType::Any)
			.expect("Reference peel failed")
			.id();
		let kind = ReferenceKind::from(reference);
		let name = String::from(reference.name().unwrap_or("InvalidRef"));
		let shorthand = String::from(reference.shorthand().unwrap_or("InvalidRef"));

		Self::new(format!("{oid}"), name, shorthand, kind)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::with_temp_repository;

	#[test]
	fn test() {
		with_temp_repository(|repository| {
			let revision = repository.revparse_single("refs/heads/main").unwrap();
			let oid = revision.id().to_string();
			let reference = repository
				.find_reference("refs/heads/main")
				.map(|r| Reference::from(&r))
				.unwrap();

			assert_eq!(reference.hash(), format!("{oid}"));
			assert_eq!(reference.name(), "refs/heads/main");
			assert_eq!(reference.shortname(), "main");
			assert_eq!(reference.kind(), ReferenceKind::Branch);
		});
	}
}
