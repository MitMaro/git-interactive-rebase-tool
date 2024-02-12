pub(crate) mod builders;
mod create_commit;
mod create_invalid_utf;
pub(crate) mod mocks;
mod shared;
mod with_git_config;
mod with_temp_bare_repository;
mod with_temp_repository;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

pub(crate) use self::{
	create_commit::{create_commit, CreateCommitOptions},
	create_invalid_utf::invalid_utf,
	with_git_config::with_git_config,
	with_temp_bare_repository::with_temp_bare_repository,
	with_temp_repository::with_temp_repository,
};
