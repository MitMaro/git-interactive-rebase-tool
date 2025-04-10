use crate::diff::{Reference, ReferenceKind};

/// Builder for creating a new reference.
#[derive(Debug)]
pub(crate) struct ReferenceBuilder {
	hash: String,
	name: String,
	shorthand: String,
	kind: ReferenceKind,
}

impl ReferenceBuilder {
	/// Create a new instance of the builder with the provided hash. The new instance will default
	/// to a branch kind and a name of "main".
	#[must_use]
	pub(crate) fn new(hash: &str) -> Self {
		Self {
			hash: String::from(hash),
			name: String::from("refs/heads/main"),
			shorthand: String::from("main"),
			kind: ReferenceKind::Branch,
		}
	}

	/// Set the hash.
	pub(crate) fn hash(&mut self, hash: &str) -> &mut Self {
		self.hash = String::from(hash);
		self
	}

	/// Set the name.
	pub(crate) fn name(&mut self, name: &str) -> &mut Self {
		self.name = String::from(name);
		self
	}

	/// Set the shortname.
	pub(crate) fn shorthand(&mut self, shorthand: &str) -> &mut Self {
		self.shorthand = String::from(shorthand);
		self
	}

	/// Set the kind.
	pub(crate) fn kind(&mut self, kind: ReferenceKind) -> &mut Self {
		self.kind = kind;
		self
	}

	/// Build the `Reference`.
	#[must_use]
	pub(crate) fn build(self) -> Reference {
		Reference::new(self.hash, self.name, self.shorthand, self.kind)
	}
}
