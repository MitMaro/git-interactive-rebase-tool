use std::{slice::Iter, vec};

use crate::view::ViewLine;

/// Represents a line in the view.
#[derive(Debug)]
pub(crate) struct ViewLines {
	lines: Vec<ViewLine>,
}

impl ViewLines {
	pub(crate) fn new() -> Self {
		Self { lines: vec![] }
	}

	#[expect(
		clippy::cast_possible_truncation,
		reason = "Number of lines will be below maximum of 16-bit."
	)]
	pub(crate) fn count(&self) -> u16 {
		self.lines.len() as u16
	}

	pub(crate) fn iter(&self) -> Iter<'_, ViewLine> {
		self.lines.iter()
	}

	pub(crate) fn clear(&mut self) {
		self.lines.clear();
	}

	pub(crate) fn push(&mut self, view_line: ViewLine) {
		self.lines.push(view_line);
	}
}

impl<const N: usize> From<[ViewLine; N]> for ViewLines {
	fn from(values: [ViewLine; N]) -> Self {
		Self {
			lines: Vec::from(values),
		}
	}
}

impl IntoIterator for ViewLines {
	type IntoIter = vec::IntoIter<Self::Item>;
	type Item = ViewLine;

	fn into_iter(self) -> Self::IntoIter {
		self.lines.into_iter()
	}
}

impl FromIterator<ViewLine> for ViewLines {
	fn from_iter<T: IntoIterator<Item = ViewLine>>(iter: T) -> Self {
		Self {
			lines: Vec::from_iter(iter),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		assert!(ViewLines::new().lines.is_empty());
	}

	#[test]
	fn push() {
		let mut view_lines = ViewLines::new();
		view_lines.push(ViewLine::new_empty_line());
		assert_eq!(view_lines.count(), 1);
	}

	#[test]
	fn clear() {
		let mut view_lines = ViewLines::new();
		view_lines.push(ViewLine::new_empty_line());
		view_lines.clear();
		assert!(view_lines.lines.is_empty());
	}

	#[test]
	fn iter() {
		let mut view_lines = ViewLines::new();
		view_lines.push(ViewLine::new_empty_line());
		view_lines.push(ViewLine::new_empty_line());
		assert_eq!(view_lines.iter().len(), 2);
	}

	#[test]
	fn into_iter() {
		let mut view_lines = ViewLines::new();
		view_lines.push(ViewLine::new_empty_line());
		view_lines.push(ViewLine::new_empty_line());
		assert_eq!(view_lines.into_iter().len(), 2);
	}

	#[test]
	fn from_slice() {
		let view_lines = ViewLines::from([ViewLine::new_empty_line()]);
		assert_eq!(view_lines.iter().len(), 1);
	}

	#[test]
	fn from_iter() {
		let view_lines = ViewLines::from_iter([ViewLine::new_empty_line()]);
		assert_eq!(view_lines.iter().len(), 1);
	}
}
