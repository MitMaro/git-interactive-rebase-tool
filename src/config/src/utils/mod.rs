mod get_bool;
mod get_diff_ignore_whitespace;
mod get_diff_rename;
mod get_diff_show_whitespace;
mod get_input;
mod get_string;
mod get_unsigned_integer;

pub(crate) use self::{
	get_bool::get_bool,
	get_diff_ignore_whitespace::get_diff_ignore_whitespace,
	get_diff_rename::git_diff_renames,
	get_diff_show_whitespace::get_diff_show_whitespace,
	get_input::get_input,
	get_string::{get_optional_string, get_string},
	get_unsigned_integer::get_unsigned_integer,
};
