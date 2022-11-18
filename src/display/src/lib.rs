// LINT-REPLACE-START
// This section is autogenerated, do not modify directly
// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![cfg_attr(allow_unknown_lints, allow(renamed_and_removed_lints))]
// enable all rustc's built-in lints
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	rust_2021_incompatible_closure_captures,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
	rust_2021_prelude_collisions,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unstable_features,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![cfg_attr(include_nightly_lints, deny(clippy::nursery))]
#![allow(
	clippy::arithmetic_side_effects,
	clippy::blanket_clippy_restriction_lints,
	clippy::bool_to_int_with_if,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::indexing_slicing,
	clippy::integer_arithmetic,
	clippy::map_err_ignore,
	clippy::missing_docs_in_private_items,
	clippy::mod_module_files,
	clippy::module_name_repetitions,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::option_if_let_else,
	clippy::pub_use,
	clippy::redundant_pub_crate,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
	clippy::tabs_in_doc_comments,
	clippy::too_many_lines,
	clippy::unwrap_used
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
// allow some things in tests
#![cfg_attr(
	test,
	allow(
		clippy::cognitive_complexity,
		clippy::let_underscore_drop,
		clippy::let_underscore_must_use,
		clippy::needless_pass_by_value,
		clippy::panic,
		clippy::shadow_reuse,
		clippy::shadow_unrelated,
		clippy::undocumented_unsafe_blocks,
		clippy::unimplemented,
		clippy::unreachable
	)
)]
// allowable upcoming nightly lints
#![cfg_attr(include_nightly_lints, allow(clippy::missing_trait_methods))]
// LINT-REPLACE-END

//! Git Interactive Rebase Tool - Display Module
//!
//! # Description
//! This module is used to handle working with the terminal display.
//!
//! ```
//! use config::Theme;
//! use display::{CrossTerm, Display, DisplayColor};
//! let theme = Theme::new();
//! let tui = CrossTerm::new();
//! let mut display = Display::new(tui, &theme);
//!
//! display.start();
//! display.clear();
//! display.draw_str("Hello world!");
//! display.color(DisplayColor::IndicatorColor, false);
//! display.set_style(false, true, false);
//! display.draw_str("Hello colorful, underlined world!");
//! display.refresh();
//! display.end();
//! ```
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod color_mode;
#[cfg(not(tarpaulin_include))]
mod crossterm;
mod display_color;
mod error;
mod size;
#[cfg(not(tarpaulin_include))]
pub mod testutil;
mod tui;
mod utils;

use ::crossterm::style::{Color, Colors};
use config::Theme;

use self::utils::register_selectable_color_pairs;
pub use self::{
	color_mode::ColorMode,
	crossterm::CrossTerm,
	display_color::DisplayColor,
	error::DisplayError,
	size::Size,
	tui::Tui,
};

/// A high level interface to the terminal display.
#[derive(Debug)]
pub struct Display<T: Tui> {
	action_break: (Colors, Colors),
	action_drop: (Colors, Colors),
	action_edit: (Colors, Colors),
	action_exec: (Colors, Colors),
	action_fixup: (Colors, Colors),
	action_label: (Colors, Colors),
	action_merge: (Colors, Colors),
	action_pick: (Colors, Colors),
	action_reset: (Colors, Colors),
	action_reword: (Colors, Colors),
	action_squash: (Colors, Colors),
	action_update_ref: (Colors, Colors),
	tui: T,
	diff_add: (Colors, Colors),
	diff_change: (Colors, Colors),
	diff_context: (Colors, Colors),
	diff_remove: (Colors, Colors),
	diff_whitespace: (Colors, Colors),
	indicator: (Colors, Colors),
	normal: (Colors, Colors),
}

impl<T: Tui> Display<T> {
	/// Create a new display instance.
	#[inline]
	pub fn new(tui: T, theme: &Theme) -> Self {
		let color_mode = tui.get_color_mode();
		let normal = register_selectable_color_pairs(
			color_mode,
			theme.color_foreground,
			theme.color_background,
			theme.color_selected_background,
		);
		let indicator = register_selectable_color_pairs(
			color_mode,
			theme.color_indicator,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_break = register_selectable_color_pairs(
			color_mode,
			theme.color_action_break,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_drop = register_selectable_color_pairs(
			color_mode,
			theme.color_action_drop,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_edit = register_selectable_color_pairs(
			color_mode,
			theme.color_action_edit,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_exec = register_selectable_color_pairs(
			color_mode,
			theme.color_action_exec,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_fixup = register_selectable_color_pairs(
			color_mode,
			theme.color_action_fixup,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_pick = register_selectable_color_pairs(
			color_mode,
			theme.color_action_pick,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_reword = register_selectable_color_pairs(
			color_mode,
			theme.color_action_reword,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_squash = register_selectable_color_pairs(
			color_mode,
			theme.color_action_squash,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_label = register_selectable_color_pairs(
			color_mode,
			theme.color_action_label,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_reset = register_selectable_color_pairs(
			color_mode,
			theme.color_action_reset,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_merge = register_selectable_color_pairs(
			color_mode,
			theme.color_action_merge,
			theme.color_background,
			theme.color_selected_background,
		);
		let action_update_ref = register_selectable_color_pairs(
			color_mode,
			theme.color_action_update_ref,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_add = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_add,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_change = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_change,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_remove = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_remove,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_context = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_context,
			theme.color_background,
			theme.color_selected_background,
		);
		let diff_whitespace = register_selectable_color_pairs(
			color_mode,
			theme.color_diff_whitespace,
			theme.color_background,
			theme.color_selected_background,
		);

		Self {
			action_break,
			action_drop,
			action_edit,
			action_exec,
			action_fixup,
			action_label,
			action_merge,
			action_pick,
			action_reset,
			action_reword,
			action_squash,
			action_update_ref,
			tui,
			diff_add,
			diff_change,
			diff_context,
			diff_remove,
			diff_whitespace,
			indicator,
			normal,
		}
	}

	/// Draws a string of text to the terminal interface.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn draw_str(&mut self, s: &str) -> Result<(), DisplayError> {
		self.tui.print(s)
	}

	/// Clear the terminal interface and reset any style and color attributes.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn clear(&mut self) -> Result<(), DisplayError> {
		self.color(DisplayColor::Normal, false)?;
		self.set_style(false, false, false)?;
		self.tui.reset()
	}

	/// Force a refresh of the terminal interface. This normally should be called after after all
	/// text has been drawn to the terminal interface. This is considered a slow operation, so
	/// should be called only as needed.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn refresh(&mut self) -> Result<(), DisplayError> {
		self.tui.flush()
	}

	/// Set the color of text drawn to the terminal interface. This will only change text drawn to
	/// the terminal after this function call.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn color(&mut self, color: DisplayColor, selected: bool) -> Result<(), DisplayError> {
		self.tui.set_color(
			if selected {
				match color {
					DisplayColor::ActionBreak => self.action_break.1,
					DisplayColor::ActionDrop => self.action_drop.1,
					DisplayColor::ActionEdit => self.action_edit.1,
					DisplayColor::ActionExec => self.action_exec.1,
					DisplayColor::ActionFixup => self.action_fixup.1,
					DisplayColor::ActionPick => self.action_pick.1,
					DisplayColor::ActionReword => self.action_reword.1,
					DisplayColor::ActionSquash => self.action_squash.1,
					DisplayColor::ActionLabel => self.action_label.1,
					DisplayColor::ActionReset => self.action_reset.1,
					DisplayColor::ActionMerge => self.action_merge.1,
					DisplayColor::ActionUpdateRef => self.action_update_ref.1,
					DisplayColor::Normal => self.normal.1,
					DisplayColor::IndicatorColor => self.indicator.1,
					DisplayColor::DiffAddColor => self.diff_add.1,
					DisplayColor::DiffRemoveColor => self.diff_remove.1,
					DisplayColor::DiffChangeColor => self.diff_change.1,
					DisplayColor::DiffContextColor => self.diff_context.1,
					DisplayColor::DiffWhitespaceColor => self.diff_whitespace.1,
				}
			}
			else {
				match color {
					DisplayColor::ActionBreak => self.action_break.0,
					DisplayColor::ActionDrop => self.action_drop.0,
					DisplayColor::ActionEdit => self.action_edit.0,
					DisplayColor::ActionExec => self.action_exec.0,
					DisplayColor::ActionFixup => self.action_fixup.0,
					DisplayColor::ActionPick => self.action_pick.0,
					DisplayColor::ActionReword => self.action_reword.0,
					DisplayColor::ActionSquash => self.action_squash.0,
					DisplayColor::ActionLabel => self.action_label.0,
					DisplayColor::ActionReset => self.action_reset.0,
					DisplayColor::ActionMerge => self.action_merge.0,
					DisplayColor::ActionUpdateRef => self.action_update_ref.0,
					DisplayColor::Normal => self.normal.0,
					DisplayColor::IndicatorColor => self.indicator.0,
					DisplayColor::DiffAddColor => self.diff_add.0,
					DisplayColor::DiffRemoveColor => self.diff_remove.0,
					DisplayColor::DiffChangeColor => self.diff_change.0,
					DisplayColor::DiffContextColor => self.diff_context.0,
					DisplayColor::DiffWhitespaceColor => self.diff_whitespace.0,
				}
			},
		)
	}

	/// Set the style attributes of text drawn to the terminal interface. This will only change text
	/// drawn to the terminal after this function call.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn set_style(&mut self, dim: bool, underline: bool, reverse: bool) -> Result<(), DisplayError> {
		self.set_dim(dim)?;
		self.set_underline(underline)?;
		self.set_reverse(reverse)
	}

	/// Get the width and height of the terminal interface. This can be a slow operation, so should
	/// not be called unless absolutely needed.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn get_window_size(&self) -> Size {
		self.tui.get_size()
	}

	/// Reset the cursor position to the start of the line.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn ensure_at_line_start(&mut self) -> Result<(), DisplayError> {
		self.tui.move_to_column(0)
	}

	/// Move the cursor position `right` characters from the end of the line.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn move_from_end_of_line(&mut self, right: u16) -> Result<(), DisplayError> {
		let width = self.get_window_size().width().try_into().unwrap_or(u16::MAX);
		self.tui.move_to_column(width - right)
	}

	/// Move the cursor to the next line.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn next_line(&mut self) -> Result<(), DisplayError> {
		self.tui.move_next_line()
	}

	/// Start the terminal interface interactions. This should be called before any terminal
	/// interactions are performed.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn start(&mut self) -> Result<(), DisplayError> {
		self.tui.start()?;
		self.tui.flush()
	}

	/// End the terminal interface interactions. This should be called after all terminal
	/// interactions are complete. This resets the terminal interface to the default state, and
	/// should be called on program exit.
	///
	/// # Errors
	/// Will error if the underlying terminal interface is in an error state.
	#[inline]
	pub fn end(&mut self) -> Result<(), DisplayError> {
		self.tui.end()?;
		self.tui.flush()
	}

	fn set_dim(&mut self, on: bool) -> Result<(), DisplayError> {
		self.tui.set_dim(on)
	}

	fn set_underline(&mut self, on: bool) -> Result<(), DisplayError> {
		self.tui.set_underline(on)
	}

	fn set_reverse(&mut self, on: bool) -> Result<(), DisplayError> {
		self.tui.set_reverse(on)
	}
}

#[cfg(test)]
mod tests {
	use ::crossterm::style::Color as CrosstermColor;
	use rstest::rstest;

	use super::{testutil::CrossTerm, *};
	use crate::testutil::State;

	#[test]
	fn draw_str() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.draw_str("Test String").unwrap();
		assert_eq!(display.tui.get_output(), &["Test String"]);
	}

	#[test]
	fn clear() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.draw_str("Test String").unwrap();
		display.set_dim(true).unwrap();
		display.set_reverse(true).unwrap();
		display.set_underline(true).unwrap();
		display.clear().unwrap();
		assert!(display.tui.get_output().is_empty());
		assert!(!display.tui.is_dimmed());
		assert!(!display.tui.is_reverse());
		assert!(!display.tui.is_underline());
	}

	#[test]
	fn refresh() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.refresh().unwrap();
		assert!(!display.tui.is_dirty());
	}

	#[rstest]
	#[case::action_break(DisplayColor::ActionBreak, false, CrosstermColor::White, CrosstermColor::Reset)]
	#[case::action_break_selected(
		DisplayColor::ActionBreak,
		true,
		CrosstermColor::White,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_drop(DisplayColor::ActionDrop, false, CrosstermColor::Red, CrosstermColor::Reset)]
	#[case::action_drop_selected(DisplayColor::ActionDrop, true, CrosstermColor::Red, CrosstermColor::AnsiValue(237))]
	#[case::action_edit(DisplayColor::ActionEdit, false, CrosstermColor::Blue, CrosstermColor::Reset)]
	#[case::action_edit_selected(DisplayColor::ActionEdit, true, CrosstermColor::Blue, CrosstermColor::AnsiValue(237))]
	#[case::action_exec(DisplayColor::ActionExec, false, CrosstermColor::White, CrosstermColor::Reset)]
	#[case::action_exec_selected(
		DisplayColor::ActionExec,
		true,
		CrosstermColor::White,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_fixup(DisplayColor::ActionFixup, false, CrosstermColor::Magenta, CrosstermColor::Reset)]
	#[case::action_fixup_selected(
		DisplayColor::ActionFixup,
		true,
		CrosstermColor::Magenta,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_pick(DisplayColor::ActionPick, false, CrosstermColor::Green, CrosstermColor::Reset)]
	#[case::action_pick_selected(
		DisplayColor::ActionPick,
		true,
		CrosstermColor::Green,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_reword(DisplayColor::ActionReword, false, CrosstermColor::Yellow, CrosstermColor::Reset)]
	#[case::action_reword_selected(
		DisplayColor::ActionReword,
		true,
		CrosstermColor::Yellow,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_squash(DisplayColor::ActionSquash, false, CrosstermColor::Cyan, CrosstermColor::Reset)]
	#[case::action_squash_selected(
		DisplayColor::ActionSquash,
		true,
		CrosstermColor::Cyan,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_label(DisplayColor::ActionLabel, false, CrosstermColor::DarkYellow, CrosstermColor::Reset)]
	#[case::action_label_selected(
		DisplayColor::ActionLabel,
		true,
		CrosstermColor::DarkYellow,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_reset(DisplayColor::ActionReset, false, CrosstermColor::DarkYellow, CrosstermColor::Reset)]
	#[case::action_reset_selected(
		DisplayColor::ActionReset,
		true,
		CrosstermColor::DarkYellow,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_merge(DisplayColor::ActionMerge, false, CrosstermColor::DarkYellow, CrosstermColor::Reset)]
	#[case::action_merge_selected(
		DisplayColor::ActionMerge,
		true,
		CrosstermColor::DarkYellow,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::action_update_ref(
		DisplayColor::ActionUpdateRef,
		false,
		CrosstermColor::DarkMagenta,
		CrosstermColor::Reset
	)]
	#[case::action_update_ref_selected(
		DisplayColor::ActionUpdateRef,
		true,
		CrosstermColor::DarkMagenta,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::normal(DisplayColor::Normal, false, CrosstermColor::Reset, CrosstermColor::Reset)]
	#[case::normal_selected(DisplayColor::Normal, true, CrosstermColor::Reset, CrosstermColor::AnsiValue(237))]
	#[case::indicator(DisplayColor::IndicatorColor, false, CrosstermColor::Cyan, CrosstermColor::Reset)]
	#[case::indicator_selected(
		DisplayColor::IndicatorColor,
		true,
		CrosstermColor::Cyan,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::diff_add(DisplayColor::DiffAddColor, false, CrosstermColor::Green, CrosstermColor::Reset)]
	#[case::diff_add_selected(
		DisplayColor::DiffAddColor,
		true,
		CrosstermColor::Green,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::diff_remove(DisplayColor::DiffRemoveColor, false, CrosstermColor::Red, CrosstermColor::Reset)]
	#[case::diff_remove_selected(
		DisplayColor::DiffRemoveColor,
		true,
		CrosstermColor::Red,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::diff_change(DisplayColor::DiffChangeColor, false, CrosstermColor::Yellow, CrosstermColor::Reset)]
	#[case::diff_change_selected(
		DisplayColor::DiffChangeColor,
		true,
		CrosstermColor::Yellow,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::diff_context(DisplayColor::DiffContextColor, false, CrosstermColor::White, CrosstermColor::Reset)]
	#[case::diff_context_selected(
		DisplayColor::DiffContextColor,
		true,
		CrosstermColor::White,
		CrosstermColor::AnsiValue(237)
	)]
	#[case::diff_whitespace(
		DisplayColor::DiffWhitespaceColor,
		false,
		CrosstermColor::DarkGrey,
		CrosstermColor::Reset
	)]
	#[case::diff_whitespace_selected(
		DisplayColor::DiffWhitespaceColor,
		true,
		CrosstermColor::DarkGrey,
		CrosstermColor::AnsiValue(237)
	)]
	fn color(
		#[case] display_color: DisplayColor,
		#[case] selected: bool,
		#[case] expected_foreground: CrosstermColor,
		#[case] expected_background: CrosstermColor,
	) {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.color(display_color, selected).unwrap();
		assert!(
			display
				.tui
				.is_colors_enabled(Colors::new(expected_foreground, expected_background))
		);
	}

	#[rstest]
	#[case::all_off(false, false, false)]
	#[case::reverse(false, false, true)]
	#[case::underline(false, true, false)]
	#[case::underline_reverse(false, true, true)]
	#[case::dim(true, false, false)]
	#[case::dim_reverse(true, false, true)]
	#[case::dim_underline(true, true, false)]
	#[case::all_on(true, true, true)]
	fn style(#[case] dim: bool, #[case] underline: bool, #[case] reverse: bool) {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.set_style(dim, underline, reverse).unwrap();
		assert_eq!(display.tui.is_dimmed(), dim);
		assert_eq!(display.tui.is_underline(), underline);
		assert_eq!(display.tui.is_reverse(), reverse);
	}

	#[test]
	fn get_window_size() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.tui.set_size(Size::new(12, 10));
		assert_eq!(display.get_window_size(), Size::new(12, 10));
	}

	#[test]
	fn ensure_at_line_start() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.ensure_at_line_start().unwrap();
		assert_eq!(display.tui.get_position(), (0, 0));
	}

	#[test]
	fn move_from_end_of_line() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.tui.set_size(Size::new(20, 10));
		display.move_from_end_of_line(5).unwrap();
		// character after the 15th character (0-indexed)
		assert_eq!(display.tui.get_position(), (15, 0));
	}

	#[test]
	fn start() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.start().unwrap();
		assert_eq!(display.tui.get_state(), State::Normal);
	}

	#[test]
	fn end() {
		let mut display = Display::new(CrossTerm::new(), &Theme::new());
		display.end().unwrap();
		assert_eq!(display.tui.get_state(), State::Ended);
	}
}
