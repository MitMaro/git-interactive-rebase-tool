use std::sync::Arc;

use anyhow::Result;
use git::{CommitDiff, CommitDiffLoaderOptions, Repository};
use parking_lot::Mutex;

#[derive(Debug, Clone)]
pub(crate) struct Git {
	repository: Arc<Mutex<Repository>>,
}

impl Git {
	pub(crate) fn new(repository: Repository) -> Self {
		Self {
			repository: Arc::new(Mutex::new(repository)),
		}
	}

	pub(crate) fn load_commit_diff(&self, hash: &str, config: &CommitDiffLoaderOptions) -> Result<CommitDiff> {
		self.repository.lock().load_commit_diff(hash, config)
	}
}
