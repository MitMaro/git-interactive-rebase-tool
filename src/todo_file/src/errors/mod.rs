//! Git Interactive Rebase Tool - Todo File Module Errors
//!
//! # Description
//! This module contains error types used in the Todo File Module.

mod io;
mod parse;

pub use self::{
	io::{FileReadErrorCause, IoError},
	parse::ParseError,
};
