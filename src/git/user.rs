/// Represents a user within a commit with a name and email address
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct User {
	name: Option<String>,
	email: Option<String>,
}

impl User {
	/// Creates a new user
	#[must_use]
	pub(crate) fn new(name: Option<&str>, email: Option<&str>) -> Self {
		Self {
			email: email.map(String::from),
			name: name.map(String::from),
		}
	}

	/// Get the optional name of the user
	#[must_use]
	pub(crate) fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	/// Get the optional email of the user
	#[must_use]
	pub(crate) fn email(&self) -> Option<&str> {
		self.email.as_deref()
	}

	/// Returns `true` if one of name or email is a `Some` value.
	#[must_use]
	pub(crate) const fn is_some(&self) -> bool {
		self.name.is_some() || self.email.is_some()
	}

	/// Returns `true` if both name and email is a `None` value.
	#[must_use]
	pub(crate) const fn is_none(&self) -> bool {
		self.name.is_none() && self.email.is_none()
	}
}

impl ToString for User {
	/// Creates a formatted string of the user
	///
	/// The user if formatted with "Name &lt;Email&gt;", which matches the Git CLI. If name or email are
	/// `None` then they are omitted from the result. If neither are set, and empty is returned.
	fn to_string(&self) -> String {
		if let Some(name) = self.name.as_ref() {
			if let Some(email) = self.email.as_ref() {
				format!("{name} <{email}>")
			}
			else {
				String::from(name)
			}
		}
		else if let Some(email) = self.email.as_ref() {
			format!("<{email}>")
		}
		else {
			String::new()
		}
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_some_eq;
	use rstest::rstest;

	use super::*;

	#[test]
	fn name() {
		let user = User::new(Some("name"), None);
		assert_some_eq!(user.name(), "name");
	}

	#[test]
	fn email() {
		let user = User::new(None, Some("email"));
		assert_some_eq!(user.email(), "email");
	}

	#[rstest]
	#[case(Some("name"), None)]
	#[case(None, Some("email"))]
	#[case(Some("email"), Some("email"))]
	fn is_some_none_when_some(#[case] name: Option<&str>, #[case] email: Option<&str>) {
		let user = User::new(name, email);
		assert!(user.is_some());
		assert!(!user.is_none());
	}

	#[test]
	fn is_some_none_when_none() {
		let user = User::new(None, None);
		assert!(!user.is_some());
		assert!(user.is_none());
	}

	#[test]
	fn to_string_with_none_name_and_none_email() {
		let user = User::new(None, None);
		assert_eq!(user.to_string(), "");
	}

	#[test]
	fn to_string_with_none_name_and_some_email() {
		let user = User::new(None, Some("me@example.com"));
		assert_eq!(user.to_string(), "<me@example.com>");
	}

	#[test]
	fn to_string_with_some_name_and_none_email() {
		let user = User::new(Some("Tim Oram"), None);
		assert_eq!(user.to_string(), "Tim Oram");
	}

	#[test]
	fn to_string_with_some_name_and_some_email() {
		let user = User::new(Some("Tim Oram"), Some("me@example.com"));
		assert_eq!(user.to_string(), "Tim Oram <me@example.com>");
	}
}
