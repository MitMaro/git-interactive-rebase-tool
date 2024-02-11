use crate::git::{Reference, ReferenceKind};

/// Builder for creating a new reference.
#[derive(Debug)]
pub(crate) struct ReferenceBuilder {
	reference: Reference,
}

impl ReferenceBuilder {
	/// Create a new instance of the builder with the provided hash. The new instance will default
	/// to a branch kind and a name of "main".
	#[must_use]
	pub(crate) fn new(hash: &str) -> Self {
		Self {
			reference: Reference {
				hash: String::from(hash),
				name: String::from("refs/heads/main"),
				shorthand: String::from("main"),
				kind: ReferenceKind::Branch,
			},
		}
	}

	/// Set the hash.
	pub(crate) fn hash(&mut self, hash: &str) -> &mut Self {
		self.reference.hash = String::from(hash);
		self
	}

	/// Set the name.
	pub(crate) fn name(&mut self, name: &str) -> &mut Self {
		self.reference.name = String::from(name);
		self
	}

	/// Set the shortname.
	pub(crate) fn shorthand(&mut self, shorthand: &str) -> &mut Self {
		self.reference.shorthand = String::from(shorthand);
		self
	}

	/// Set the kind.
	pub(crate) fn kind(&mut self, kind: ReferenceKind) -> &mut Self {
		self.reference.kind = kind;
		self
	}

	/// Build the `Reference`.
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub(crate) fn build(self) -> Reference {
		self.reference
	}
}
