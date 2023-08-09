/// Options for `TodoFile`
#[derive(Debug, Clone)]
pub struct TodoFileOptions {
	pub(crate) undo_limit: u32,
	pub(crate) comment_prefix: String,
}

impl TodoFileOptions {
	/// Create a new instance of `TodoFileOptions`
	#[must_use]
	#[inline]
	pub fn new(undo_limit: u32, comment_prefix: &str) -> Self {
		Self {
			undo_limit,
			comment_prefix: String::from(comment_prefix),
		}
	}
}
