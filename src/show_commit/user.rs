/// Represents a user within a commit with a name and email address
#[derive(Debug, Eq, PartialEq)]
pub(super) struct User {
	name: Option<String>,
	email: Option<String>,
}

impl User {
	/// Creates a new user
	pub(super) fn new(name: Option<&str>, email: Option<&str>) -> Self {
		User {
			email: email.map(String::from),
			name: name.map(String::from),
		}
	}

	/// Creates a formatted string of the user
	///
	/// The user if formatted with "Name \<Email\>", which matches the Git CLI. If name or email are
	/// `None` then they are omitted from the result. If neither are set, `None` is returned.
	pub(super) fn to_string(&self) -> Option<String> {
		let name = &self.name;
		let email = &self.email;
		match name {
			Some(n) => {
				match email {
					Some(e) => Some(format!("{} <{}>", *n, *e)),
					None => Some(n.to_string()),
				}
			},
			None => {
				match email {
					Some(e) => Some(format!("<{}>", *e)),
					None => None,
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::show_commit::user::User;

	#[test]
	fn commit_user_with_none_name_email() {
		let user = User::new(None, None);
		assert!(user.to_string().is_none());
	}

	#[test]
	fn commit_user_with_none_name_with_email() {
		let user = User::new(None, Some("me@example.com"));
		assert_eq!(user.to_string().unwrap(), "<me@example.com>");
	}

	#[test]
	fn commit_user_with_name_none_email() {
		let user = User::new(Some("Tim Oram"), None);
		assert_eq!(user.to_string().unwrap(), "Tim Oram");
	}

	#[test]
	fn commit_user_with_name_email() {
		let user = User::new(Some("Tim Oram"), Some("me@example.com"));
		assert_eq!(user.to_string().unwrap(), "Tim Oram <me@example.com>");
	}

	#[test]
	fn commit_user_compare_users_matching_name_email() {
		assert_eq!(
			User::new(Some("Tim Oram"), Some("me@example.com")),
			User::new(Some("Tim Oram"), Some("me@example.com"))
		);
	}

	#[test]
	fn commit_user_compare_users_matching_name_only() {
		assert_ne!(
			User::new(Some("Tim Oram"), Some("me1@example.com")),
			User::new(Some("Tim Oram"), Some("me2@example.com"))
		);
	}

	#[test]
	fn commit_user_compare_users_matching_email_only() {
		assert_ne!(
			User::new(Some("Tim Oram 1"), Some("me@example.com")),
			User::new(Some("Tim Oram 2"), Some("me@example.com"))
		);
	}

	#[test]
	fn commit_user_compare_users_matching_none() {
		assert_eq!(User::new(None, None), User::new(None, None));
	}
}
