pub(crate) mod builders;
mod create_commit;
mod create_event_reader;
mod create_invalid_utf;
mod create_test_keybindings;
pub(crate) mod mocks;
mod shared;
mod with_event_handler;
mod with_git_config;
mod with_temp_bare_repository;
mod with_temp_repository;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

pub(crate) use self::{
	create_commit::{create_commit, CreateCommitOptions},
	create_event_reader::create_event_reader,
	create_invalid_utf::invalid_utf,
	create_test_keybindings::create_test_keybindings,
	with_event_handler::{with_event_handler, EventHandlerTestContext},
	with_git_config::with_git_config,
	with_temp_bare_repository::with_temp_bare_repository,
	with_temp_repository::with_temp_repository,
};
