use crate::{Reference, ReferenceKind};

/// Builder for creating a new reference.
#[derive(Debug)]
pub struct ReferenceBuilder {
	reference: Reference,
}

impl ReferenceBuilder {
	/// Create a new instance of the builder with the provided hash. The new instance will default
	/// to a branch kind and a name of "main".
	#[inline]
	#[must_use]
	pub fn new(hash: &str) -> Self {
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
	#[inline]
	pub fn hash(&mut self, hash: &str) -> &mut Self {
		self.reference.hash = String::from(hash);
		self
	}

	/// Set the name.
	#[inline]
	pub fn name(&mut self, name: &str) -> &mut Self {
		self.reference.name = String::from(name);
		self
	}

	/// Set the shortname.
	#[inline]
	pub fn shorthand(&mut self, shorthand: &str) -> &mut Self {
		self.reference.shorthand = String::from(shorthand);
		self
	}

	/// Set the kind.
	#[inline]
	pub fn kind(&mut self, kind: ReferenceKind) -> &mut Self {
		self.reference.kind = kind;
		self
	}

	/// Build the `Reference`.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn build(self) -> Reference {
		self.reference
	}
}
