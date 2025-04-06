use crate::diff::{Delta, DiffLine, FileStatus};

#[derive(Debug, Clone)]
pub(crate) struct FileStatusBuilder {
	delta: Option<Delta>,
	file_stat: Option<FileStatus>,
	file_stats: Vec<FileStatus>,
}

impl FileStatusBuilder {
	pub(crate) const fn new() -> Self {
		Self {
			delta: None,
			file_stat: None,
			file_stats: vec![],
		}
	}

	fn close_delta(&mut self) {
		if let Some(d) = self.delta.take() {
			self.file_stat
				.as_mut()
				.expect("add_file_stat must be called once before adding a delta")
				.add_delta(d);
		}
	}

	fn close_file_stat(&mut self) {
		if let Some(fs) = self.file_stat.take() {
			self.file_stats.push(fs);
		}
	}

	pub(crate) fn add_file_stat(&mut self, file_stat: FileStatus) {
		self.close_delta();
		self.close_file_stat();
		self.delta = None;
		self.file_stat = Some(file_stat);
	}

	pub(crate) fn add_delta(&mut self, delta: Delta) {
		self.close_delta();
		self.delta = Some(delta);
	}

	pub(crate) fn add_diff_line(&mut self, diff_line: DiffLine) {
		self.delta
			.as_mut()
			.expect("add_delta must be called once before adding a diff line")
			.add_line(diff_line);
	}

	pub(crate) fn build(mut self) -> Vec<FileStatus> {
		self.close_delta();
		self.close_file_stat();
		self.file_stats
	}
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use super::*;
	use crate::diff::{FileMode, Origin, Status};

	#[test]
	fn build_file_stat_with_file_stat_without_delta() {
		let file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta() {
		let mut file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stat_1.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_without_delta_followed_by_file_stat_with_delta() {
		let file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let mut file_stat_2 = FileStatus::new(
			PathBuf::from("from2").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to2").as_path(),
			FileMode::Normal,
			false,
			Status::Deleted,
		);
		let delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_file_stat(file_stat_2.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stat_2.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1, file_stat_2]);
	}

	#[test]
	fn build_file_stat_with_file_stat_without_delta_followed_by_file_stat_with_delta_followed_by_file_stat() {
		let file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let mut file_stat_2 = FileStatus::new(
			PathBuf::from("from2").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to2").as_path(),
			FileMode::Normal,
			false,
			Status::Deleted,
		);
		let delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_file_stat(file_stat_2.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stat_2.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![
			file_stat_1.clone(),
			file_stat_2,
			file_stat_1
		]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta_with_diff_line() {
		let mut file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let mut delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stats_builder.add_diff_line(diff_line_1.clone());
		delta_1.add_line(diff_line_1);
		file_stat_1.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta_with_diff_line_followed_by_file_stat() {
		let mut file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let file_stat_2 = FileStatus::new(
			PathBuf::from("from2").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to2").as_path(),
			FileMode::Normal,
			false,
			Status::Deleted,
		);
		let mut delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stats_builder.add_diff_line(diff_line_1.clone());
		file_stats_builder.add_file_stat(file_stat_2.clone());
		delta_1.add_line(diff_line_1);
		file_stat_1.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1, file_stat_2]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta_with_diff_line_followed_by_delta() {
		let mut file_stat_1 = FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		);
		let mut delta_1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let delta_2 = Delta::new("@ path/to/file.rs:56 @ impl Delta2 {", 11, 10, 9, 8);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stats_builder.add_diff_line(diff_line_1.clone());
		file_stats_builder.add_delta(delta_2.clone());
		delta_1.add_line(diff_line_1);
		file_stat_1.add_delta(delta_1);
		file_stat_1.add_delta(delta_2);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	#[should_panic(expected = "add_file_stat must be called once before adding a delta")]
	fn add_delta_without_file_stat() {
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4));
		_ = file_stats_builder.build();
	}

	#[test]
	#[should_panic(expected = "add_delta must be called once before adding a diff line")]
	fn add_diff_line_before_delta() {
		let mut file_stats_builder = FileStatusBuilder::new();
		file_stats_builder.add_file_stat(FileStatus::new(
			PathBuf::from("from").as_path(),
			FileMode::Normal,
			false,
			PathBuf::from("to").as_path(),
			FileMode::Normal,
			false,
			Status::Added,
		));
		file_stats_builder.add_diff_line(DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false));
		_ = file_stats_builder.build();
	}
}
