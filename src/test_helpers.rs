#![allow(dead_code)]
pub(crate) mod assertions;
pub(crate) mod builders;
mod create_commit;
mod create_default_test_module_handler;
mod create_event_reader;
mod create_invalid_utf;
mod create_test_module_handler;
pub(crate) mod mocks;
mod shared;
pub(crate) mod testers;
mod with_env_var;
mod with_event_handler;
mod with_git_config;
mod with_git_directory;
mod with_search;
mod with_temp_bare_repository;
mod with_temp_repository;
mod with_todo_file;
mod with_view_state;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

pub(crate) use self::{
	create_commit::{CreateCommitOptions, create_commit},
	create_default_test_module_handler::{DefaultTestModule, create_default_test_module_handler},
	create_event_reader::create_event_reader,
	create_invalid_utf::invalid_utf,
	create_test_module_handler::create_test_module_handler,
	shared::TestModuleProvider,
	with_env_var::{EnvVarAction, with_env_var},
	with_event_handler::{EventHandlerTestContext, with_event_handler},
	with_git_config::with_git_config,
	with_git_directory::with_git_directory,
	with_search::with_search,
	with_temp_bare_repository::with_temp_bare_repository,
	with_temp_repository::with_temp_repository,
	with_todo_file::with_todo_file,
	with_view_state::{ViewStateTestContext, with_view_state},
};
