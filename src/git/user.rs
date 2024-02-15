use std::fmt::Display;

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
}

impl Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(name) = self.name() {
			if let Some(email) = self.email() {
				write!(f, "{name} <{email}>")
			}
			else {
				write!(f, "{name}")
			}
		}
		else if let Some(email) = self.email() {
			write!(f, "<{email}>")
		}
		else {
			write!(f, "")
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
	}

	#[test]
	fn is_some_none_when_none() {
		let user = User::new(None, None);
		assert!(!user.is_some());
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
