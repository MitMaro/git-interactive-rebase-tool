#[derive(Debug, Eq, PartialEq)]
pub struct User {
	name: Option<String>,
	email: Option<String>,
}

impl User {
	pub fn new(name: Option<&str>, email: Option<&str>) -> Self {
		User {
			email: email.map(String::from),
			name: name.map(String::from),
		}
	}

	pub fn to_string(&self) -> Option<String> {
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
