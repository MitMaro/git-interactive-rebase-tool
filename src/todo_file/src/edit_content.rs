use super::action::Action;

/// Describes a edit context for modifying a line.
#[derive(Debug)]
pub struct EditContext {
	action: Option<Action>,
	content: Option<String>,
}

impl EditContext {
	/// Create a new empty instance.
	#[must_use]
	#[inline]
	pub const fn new() -> Self {
		Self {
			action: None,
			content: None,
		}
	}

	/// Set the action.
	#[must_use]
	#[inline]
	pub const fn action(mut self, action: Action) -> Self {
		self.action = Some(action);
		self
	}

	/// Set the content.
	#[must_use]
	#[inline]
	pub fn content(mut self, content: &str) -> Self {
		self.content = Some(String::from(content));
		self
	}

	/// Get the action.
	#[must_use]
	#[inline]
	pub const fn get_action(&self) -> Option<Action> {
		self.action
	}

	/// Get the content.
	#[must_use]
	#[inline]
	pub fn get_content(&self) -> Option<&str> {
		self.content.as_deref()
	}
}

#[cfg(test)]
mod tests {
	use claim::{assert_none, assert_some_eq};

	use super::*;

	#[test]
	fn empty() {
		let edit_context = EditContext::new();
		assert_none!(edit_context.get_action());
		assert_none!(edit_context.get_content());
	}

	#[test]
	fn with_action() {
		let edit_context = EditContext::new().action(Action::Break);
		assert_some_eq!(edit_context.get_action(), Action::Break);
		assert_none!(edit_context.get_content(),);
	}

	#[test]
	fn with_content() {
		let edit_context = EditContext::new().content("test content");
		assert_none!(edit_context.get_action());
		assert_some_eq!(edit_context.get_content(), "test content");
	}

	#[test]
	fn with_content_and_action() {
		let edit_context = EditContext::new().action(Action::Edit).content("test content");
		assert_some_eq!(edit_context.get_action(), Action::Edit);
		assert_some_eq!(edit_context.get_content(), "test content");
	}
}
