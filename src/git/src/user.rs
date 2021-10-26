/// Represents a user within a commit with a name and email address
#[derive(Debug, Eq, PartialEq)]
pub struct User {
	name: Option<String>,
	email: Option<String>,
}

impl User {
	/// Creates a new user
	#[inline]
	#[must_use]
	pub fn new(name: Option<&str>, email: Option<&str>) -> Self {
		Self {
			email: email.map(String::from),
			name: name.map(String::from),
		}
	}

	/// Get the optional name of the user
	#[inline]
	#[must_use]
	pub const fn name(&self) -> &Option<String> {
		&self.name
	}

	/// Get the optional email of the user
	#[inline]
	#[must_use]
	pub const fn email(&self) -> &Option<String> {
		&self.email
	}

	/// Returns `true` if one of name or email is a `Some` value.
	#[inline]
	#[must_use]
	pub const fn is_some(&self) -> bool {
		self.name.is_some() || self.email.is_some()
	}

	/// Returns `true` if both name and email is a `None` value.
	#[inline]
	#[must_use]
	pub const fn is_none(&self) -> bool {
		self.name.is_none() && self.email.is_none()
	}
}

impl ToString for User {
	/// Creates a formatted string of the user
	///
	/// The user if formatted with "Name &lt;Email&gt;", which matches the Git CLI. If name or email are
	/// `None` then they are omitted from the result. If neither are set, and empty is returned.
	#[inline]
	fn to_string(&self) -> String {
		if let Some(name) = self.name.as_ref() {
			if let Some(email) = self.email.as_ref() {
				format!("{} <{}>", name, email)
			}
			else {
				String::from(name)
			}
		}
		else if let Some(email) = self.email.as_ref() {
			format!("<{}>", email)
		}
		else {
			String::from("")
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[test]
	fn name() {
		let user = User::new(Some("name"), None);
		assert_eq!(user.name(), &Some(String::from("name")));
	}

	#[test]
	fn email() {
		let user = User::new(None, Some("email"));
		assert_eq!(user.email(), &Some(String::from("email")));
	}

	#[rstest]
	#[case(None, None, false)]
	#[case(Some("name"), None, true)]
	#[case(None, Some("email"), true)]
	#[case(Some("email"), Some("email"), true)]
	fn is_some(#[case] name: Option<&str>, #[case] email: Option<&str>, #[case] expected: bool) {
		assert_eq!(User::new(name, email).is_some(), expected);
	}

	#[rstest]
	#[case(None, None, true)]
	#[case(Some("name"), None, false)]
	#[case(None, Some("email"), false)]
	#[case(Some("email"), Some("email"), false)]
	fn is_none(#[case] name: Option<&str>, #[case] email: Option<&str>, #[case] expected: bool) {
		assert_eq!(User::new(name, email).is_none(), expected);
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
