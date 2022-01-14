#![cfg(not(tarpaulin_include))]

//! Utilities for writing tests that interact with Git.
mod with_temp_repository;

pub use self::with_temp_repository::{with_temp_bare_repository, with_temp_repository};
