pub(crate) mod assertions;
pub(crate) mod builders;
mod create_commit;
mod create_default_test_module_handler;
mod create_event_reader;
mod create_invalid_utf;
mod create_test_keybindings;
mod create_test_module_handler;
pub(crate) mod mocks;
mod set_git_directory;
mod shared;
pub(crate) mod testers;
mod with_event_handler;
mod with_git_config;
mod with_search;
mod with_temp_bare_repository;
mod with_temp_repository;
mod with_todo_file;
mod with_view_state;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

pub(crate) use self::{
	create_commit::{create_commit, CreateCommitOptions},
	create_default_test_module_handler::{create_default_test_module_handler, DefaultTestModule},
	create_event_reader::create_event_reader,
	create_invalid_utf::invalid_utf,
	create_test_keybindings::create_test_keybindings,
	create_test_module_handler::create_test_module_handler,
	set_git_directory::set_git_directory,
	shared::TestModuleProvider,
	with_event_handler::{with_event_handler, EventHandlerTestContext},
	with_git_config::with_git_config,
	with_search::{with_search, SearchTestContext},
	with_temp_bare_repository::with_temp_bare_repository,
	with_temp_repository::with_temp_repository,
	with_todo_file::{with_todo_file, TodoFileTestContext},
	with_view_state::{with_view_state, ViewStateTestContext},
};
