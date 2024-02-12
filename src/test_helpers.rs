pub(crate) mod builders;
mod create_invalid_utf;
pub(crate) mod mocks;
mod with_git_config;

pub(crate) static JAN_2021_EPOCH: i64 = 1_609_459_200;

pub(crate) use self::{create_invalid_utf::invalid_utf, with_git_config::with_git_config};
