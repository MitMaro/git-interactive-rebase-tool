use crate::exit::Exit;

const LICENSE_MESSAGE: &str = r#"
Sequence Editor for Git Interactive Rebase

Copyright (C) 2017-2020 Tim Oram and Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

A list of open source software and the license terms can be found at
<https://gitrebasetool.mitmaro.ca/licenses.html>
"#;

pub(crate) fn run() -> Exit {
	Exit::from(LICENSE_MESSAGE)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_success() {
		assert!(
			run()
				.get_message()
				.unwrap()
				.contains("Sequence Editor for Git Interactive Rebase")
		);
	}
}
