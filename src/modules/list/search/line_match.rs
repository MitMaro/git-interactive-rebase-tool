#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct LineMatch {
	index: usize,
	hash: bool,
	content: bool,
}

impl LineMatch {
	pub(crate) const fn new(index: usize, hash: bool, content: bool) -> Self {
		Self { index, hash, content }
	}

	pub(crate) const fn index(&self) -> usize {
		self.index
	}

	pub(crate) const fn hash(&self) -> bool {
		self.hash
	}

	pub(crate) const fn content(&self) -> bool {
		self.content
	}
}
