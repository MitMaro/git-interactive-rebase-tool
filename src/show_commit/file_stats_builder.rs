use crate::show_commit::delta::Delta;
use crate::show_commit::diff_line::DiffLine;
use crate::show_commit::file_stat::FileStat;
use crate::show_commit::status::Status;

#[derive(Debug, Clone)]
pub(super) struct FileStatsBuilder {
	delta: Option<Delta>,
	file_stat: Option<FileStat>,
	file_stats: Vec<FileStat>,
}

impl FileStatsBuilder {
	pub(crate) fn new() -> Self {
		Self {
			delta: None,
			file_stat: None,
			file_stats: vec![],
		}
	}

	fn close_delta(&mut self) {
		if let Some(d) = &self.delta {
			match &mut self.file_stat {
				Some(fs) => fs.add_delta(d.clone()),
				None => panic!("add_file_stat must be called once before adding a delta"),
			}
		}
	}

	fn close_file_stat(&mut self) {
		if let Some(fs) = &self.file_stat {
			self.file_stats.push(fs.clone());
		}
	}

	pub(crate) fn add_file_stat(&mut self, from_name: String, to_name: String, status: Status) {
		self.close_delta();
		self.close_file_stat();
		self.delta = None;
		self.file_stat = Some(FileStat::new(from_name, to_name, status));
	}

	pub(crate) fn add_delta(&mut self, header: &str, old_start: u32, new_start: u32, old_lines: u32, new_lines: u32) {
		self.close_delta();
		self.delta = Some(Delta::new(header, old_start, new_start, old_lines, new_lines));
	}

	pub(crate) fn add_diff_line(&mut self, diff_line: DiffLine) {
		match &mut self.delta {
			Some(d) => d.add_line(diff_line),
			None => panic!("add_delta must be called once before adding a diff line"),
		}
	}

	pub(crate) fn build(mut self) -> Vec<FileStat> {
		self.close_delta();
		self.close_file_stat();
		self.file_stats
	}
}
