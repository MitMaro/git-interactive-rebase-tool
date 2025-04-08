use git2::ErrorCode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LoadStatus {
	New,
	QuickDiff(usize, usize),
	CompleteQuickDiff,
	Diff(usize, usize),
	DiffComplete,
	Error { msg: String, code: ErrorCode },
}
