use crate::module::ExitStatus;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Exit {
	message: Option<String>,
	status: ExitStatus,
}

impl Exit {
	pub(crate) fn new(status: ExitStatus, message: &str) -> Self {
		Self {
			message: Some(String::from(message)),
			status,
		}
	}

	pub(crate) const fn get_message(&self) -> &Option<String> {
		&self.message
	}

	pub(crate) const fn get_status(&self) -> &ExitStatus {
		&self.status
	}
}

impl From<ExitStatus> for Exit {
	fn from(status: ExitStatus) -> Self {
		Self { message: None, status }
	}
}

impl From<String> for Exit {
	fn from(msg: String) -> Self {
		Self {
			message: Some(msg),
			status: ExitStatus::Good,
		}
	}
}

impl From<&str> for Exit {
	fn from(msg: &str) -> Self {
		Self::from(String::from(msg))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn exit_new() {
		let exit = Exit::new(ExitStatus::StateError, "This is an error");
		assert_eq!(exit.get_message(), &Some(String::from("This is an error")));
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
	}

	#[test]
	fn exit_from_exit_status() {
		let exit = Exit::from(ExitStatus::Kill);
		assert_eq!(exit.get_message(), &None);
		assert_eq!(exit.get_status(), &ExitStatus::Kill);
	}
}
