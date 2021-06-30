use super::action::Action;

#[derive(Debug)]
pub struct EditContext {
	action: Option<Action>,
	content: Option<String>,
}

impl EditContext {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			action: None,
			content: None,
		}
	}

	#[must_use]
	pub const fn action(mut self, action: Action) -> Self {
		self.action = Some(action);
		self
	}

	#[must_use]
	pub fn content(mut self, content: &str) -> Self {
		self.content = Some(content.to_owned());
		self
	}

	#[must_use]
	pub const fn get_action(&self) -> &Option<Action> {
		&self.action
	}

	#[must_use]
	pub const fn get_content(&self) -> &Option<String> {
		&self.content
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let edit_context = EditContext::new();
		assert_eq!(edit_context.get_action(), &None);
		assert_eq!(edit_context.get_content(), &None);
	}

	#[test]
	fn with_action() {
		let edit_context = EditContext::new().action(Action::Break);
		assert_eq!(edit_context.get_action(), &Some(Action::Break));
		assert_eq!(edit_context.get_content(), &None);
	}

	#[test]
	fn with_content() {
		let edit_context = EditContext::new().content("test content");
		assert_eq!(edit_context.get_action(), &None);
		assert_eq!(edit_context.get_content(), &Some(String::from("test content")));
	}

	#[test]
	fn with_content_and_action() {
		let edit_context = EditContext::new().action(Action::Edit).content("test content");
		assert_eq!(edit_context.get_action(), &Some(Action::Edit));
		assert_eq!(edit_context.get_content(), &Some(String::from("test content")));
	}
}
