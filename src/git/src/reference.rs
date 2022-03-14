use crate::reference_kind::ReferenceKind;

/// Represents a pointer to an object in Git.
#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
	/// The object id
	pub(crate) hash: String,
	/// The reference full name
	pub(crate) name: String,
	/// The reference shorthand name
	pub(crate) shorthand: String,
	/// The kind of reference
	pub(crate) kind: ReferenceKind,
}

impl Reference {
	/// Get the oid of the reference
	#[must_use]
	#[inline]
	pub fn hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Get the name of the reference
	#[must_use]
	#[inline]
	pub fn name(&self) -> &str {
		self.name.as_str()
	}

	/// Get the shorthand name of the reference
	#[must_use]
	#[inline]
	pub fn shortname(&self) -> &str {
		self.shorthand.as_str()
	}

	/// Get the kind of the reference
	#[must_use]
	#[inline]
	pub const fn kind(&self) -> ReferenceKind {
		self.kind
	}

	pub(crate) fn from(reference: &git2::Reference<'_>) -> Self {
		let oid = reference
			.peel(git2::ObjectType::Any)
			.expect("Reference peel failed")
			.id();
		let kind = ReferenceKind::from(reference);
		let name = String::from(reference.name().unwrap_or("InvalidRef"));
		let shorthand = String::from(reference.shorthand().unwrap_or("InvalidRef"));

		Self {
			hash: format!("{}", oid),
			name,
			shorthand,
			kind,
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::Error;

	use super::*;
	use crate::testutil::{head_id, with_temp_repository};

	#[test]
	fn test() {
		with_temp_repository(|repository| {
			let oid = head_id(&repository, "main");
			let reference = repository.find_reference("refs/heads/main").map_err(Error::from)?;
			assert_eq!(reference.hash(), format!("{}", oid));
			assert_eq!(reference.name(), "refs/heads/main");
			assert_eq!(reference.shortname(), "main");
			assert_eq!(reference.kind(), ReferenceKind::Branch);
			Ok(())
		});
	}
}
