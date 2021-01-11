use super::delta::Delta;
use super::diff_line::DiffLine;
use super::file_stat::FileStat;

#[derive(Debug, Clone)]
pub(super) struct FileStatsBuilder {
	delta: Option<Delta>,
	file_stat: Option<FileStat>,
	file_stats: Vec<FileStat>,
}

impl FileStatsBuilder {
	pub(crate) const fn new() -> Self {
		Self {
			delta: None,
			file_stat: None,
			file_stats: vec![],
		}
	}

	fn close_delta(&mut self) {
		if let Some(ref d) = self.delta {
			match self.file_stat {
				Some(ref mut fs) => fs.add_delta(d.clone()),
				None => panic!("add_file_stat must be called once before adding a delta"),
			}
		}
	}

	fn close_file_stat(&mut self) {
		if let Some(ref fs) = self.file_stat {
			self.file_stats.push(fs.clone());
		}
	}

	pub(crate) fn add_file_stat(&mut self, file_stat: FileStat) {
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
		match self.delta {
			Some(ref mut d) => d.add_line(diff_line),
			None => panic!("add_delta must be called once before adding a diff line"),
		}
	}

	pub(crate) fn build(mut self) -> Vec<FileStat> {
		self.close_delta();
		self.close_file_stat();
		self.file_stats
	}
}

#[cfg(test)]
mod tests {
	use super::origin::Origin;
	use super::status::Status;
	use super::*;

	#[test]
	fn build_file_stat_with_file_stat_without_delta() {
		let file_stat_1 = FileStat::new("from", "to", Status::Added);
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta() {
		let mut file_stat_1 = FileStat::new("from", "to", Status::Added);
		let delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stat_1.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_without_delta_followed_by_file_stat_with_delta() {
		let file_stat_1 = FileStat::new("from", "to", Status::Added);
		let mut file_stat_2 = FileStat::new("from2", "to2", Status::Deleted);
		let delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_file_stat(file_stat_2.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stat_2.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1, file_stat_2]);
	}

	#[test]
	fn build_file_stat_with_file_stat_without_delta_followed_by_file_stat_with_delta_followed_by_file_stat() {
		let file_stat_1 = FileStat::new("from", "to", Status::Added);
		let mut file_stat_2 = FileStat::new("from2", "to2", Status::Deleted);
		let delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let mut file_stats_builder = FileStatsBuilder::new();
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
		let mut file_stat_1 = FileStat::new("from", "to", Status::Added);
		let mut delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_file_stat(file_stat_1.clone());
		file_stats_builder.add_delta(delta_1.clone());
		file_stats_builder.add_diff_line(diff_line_1.clone());
		delta_1.add_line(diff_line_1);
		file_stat_1.add_delta(delta_1);
		assert_eq!(file_stats_builder.build(), vec![file_stat_1]);
	}

	#[test]
	fn build_file_stat_with_file_stat_with_delta_with_diff_line_followed_by_file_stat() {
		let mut file_stat_1 = FileStat::new("from", "to", Status::Added);
		let file_stat_2 = FileStat::new("from2", "to2", Status::Deleted);
		let mut delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatsBuilder::new();
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
		let mut file_stat_1 = FileStat::new("from", "to", Status::Added);
		let mut delta_1 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let delta_2 = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta2 {", 11, 10, 9, 8);
		let diff_line_1 = DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false);
		let mut file_stats_builder = FileStatsBuilder::new();
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
	#[should_panic]
	fn add_delta_without_file_stat() {
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4));
		file_stats_builder.build();
	}
	#[test]
	#[should_panic]
	fn add_diff_line_before_delta() {
		let mut file_stats_builder = FileStatsBuilder::new();
		file_stats_builder.add_file_stat(FileStat::new("from", "to", Status::Added));
		file_stats_builder.add_diff_line(DiffLine::new(Origin::Addition, "My Line", Some(1), Some(2), false));
		file_stats_builder.build();
	}
}
