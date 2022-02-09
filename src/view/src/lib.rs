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
	semicolon_in_expressions_from_macros,
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
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::blanket_clippy_restriction_lints,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::implicit_return,
	clippy::integer_arithmetic,
	clippy::missing_docs_in_private_items,
	clippy::mod_module_files,
	clippy::option_if_let_else,
	clippy::redundant_pub_crate,
	clippy::tabs_in_doc_comments,
	clippy::too_many_lines
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
// LINT-REPLACE-END
#![allow(clippy::as_conversions, clippy::integer_division, clippy::module_name_repetitions)]

//! Git Interactive Rebase Tool - View Module
//!
//! # Description
//! This module is used to handle working with the view.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod action;
mod line_segment;
mod render_context;
mod render_slice;
mod scroll_position;
mod sender;
#[cfg(not(tarpaulin_include))]
pub mod testutil;
mod thread;
mod util;
mod view_data;
mod view_data_updater;
mod view_line;

#[cfg(test)]
mod tests;

use anyhow::Result;
use display::{Display, DisplayColor, Size, Tui};

use self::render_slice::RenderSlice;
pub use self::{
	action::ViewAction,
	line_segment::LineSegment,
	render_context::RenderContext,
	sender::Sender as ViewSender,
	thread::spawn_view_thread,
	util::handle_view_data_scroll,
	view_data::ViewData,
	view_data_updater::ViewDataUpdater,
	view_line::ViewLine,
};

const TITLE: &str = "Git Interactive Rebase Tool";
const TITLE_SHORT: &str = "Git Rebase";
const TITLE_HELP_INDICATOR_LABEL: &str = "Help: ";
const SCROLLBAR_INDICATOR_CHARACTER: &str = "\u{2588}"; // "█"

/// Represents a view.
#[derive(Debug)]
pub struct View<C: Tui> {
	character_vertical_spacing: String,
	display: Display<C>,
	help_indicator_key: String,
	last_render_version: u32,
}

impl<C: Tui> View<C> {
	/// Create a new instance of the view.
	#[inline]
	pub fn new(display: Display<C>, character_vertical_spacing: &str, help_indicator_key: &str) -> Self {
		Self {
			character_vertical_spacing: String::from(character_vertical_spacing),
			display,
			help_indicator_key: String::from(help_indicator_key),
			last_render_version: u32::MAX,
		}
	}

	/// End processing of the view.
	///
	/// # Errors
	/// Results in an error if the terminal cannot be started.
	#[inline]
	pub fn start(&mut self) -> Result<()> {
		self.display.start()
	}

	/// End the view processing.
	///
	/// # Errors
	/// Results in an error if the terminal cannot be ended.
	#[inline]
	pub fn end(&mut self) -> Result<()> {
		self.display.end()
	}

	/// Get the size of the view.
	#[inline]
	#[deprecated = "This leaks internals of the Display and will eventually be removed"]
	pub fn get_view_size(&self) -> Size {
		self.display.get_window_size()
	}

	/// Render a slice.
	///
	/// # Errors
	/// Results in an error if there are errors with interacting with the terminal.
	#[inline]
	pub fn render(&mut self, render_slice: &RenderSlice) -> Result<()> {
		let current_render_version = render_slice.get_version();
		if self.last_render_version == current_render_version {
			return Ok(());
		}
		self.last_render_version = current_render_version;
		let view_size = self.display.get_window_size();
		let window_height = view_size.height();

		self.display.clear()?;

		self.display.ensure_at_line_start()?;
		if render_slice.show_title() {
			self.display.ensure_at_line_start()?;
			self.draw_title(render_slice.show_help())?;
			self.display.next_line()?;
		}

		let lines = render_slice.get_lines();
		let leading_line_count = render_slice.get_leading_lines_count();
		let trailing_line_count = render_slice.get_trailing_lines_count();
		let lines_count = lines.len() - leading_line_count - trailing_line_count;
		let show_scroll_bar = render_slice.should_show_scroll_bar();
		let scroll_indicator_index = render_slice.get_scroll_index();
		let view_height = window_height - leading_line_count - trailing_line_count;

		let leading_lines_iter = lines.iter().take(leading_line_count);
		let lines_iter = lines.iter().skip(leading_line_count).take(lines_count);
		let trailing_lines_iter = lines.iter().skip(leading_line_count + lines_count);

		for line in leading_lines_iter {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			self.display.next_line()?;
		}

		for (index, line) in lines_iter.enumerate() {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			if show_scroll_bar {
				self.display.move_from_end_of_line(1)?;
				self.display.color(DisplayColor::Normal, true)?;
				self.display.draw_str(
					if scroll_indicator_index == index {
						SCROLLBAR_INDICATOR_CHARACTER
					}
					else {
						" "
					},
				)?;
			}
			self.display.color(DisplayColor::Normal, false)?;
			self.display.set_style(false, false, false)?;
			self.display.next_line()?;
		}

		if view_height > lines_count {
			self.display.color(DisplayColor::Normal, false)?;
			self.display.set_style(false, false, false)?;
			let draw_height = view_height - lines_count - if render_slice.show_title() { 1 } else { 0 };
			self.display.ensure_at_line_start()?;
			for _x in 0..draw_height {
				self.display.draw_str(self.character_vertical_spacing.as_str())?;
				self.display.next_line()?;
			}
		}

		for line in trailing_lines_iter {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			self.display.next_line()?;
		}
		self.display.refresh()?;
		Ok(())
	}

	fn draw_view_line(&mut self, line: &ViewLine) -> Result<()> {
		for segment in line.get_segments() {
			self.display.color(segment.get_color(), line.get_selected())?;
			self.display
				.set_style(segment.is_dimmed(), segment.is_underlined(), segment.is_reversed())?;
			self.display.draw_str(segment.get_content())?;
		}

		// reset style
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, false, false)?;
		Ok(())
	}

	fn draw_title(&mut self, show_help: bool) -> Result<()> {
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, true, false)?;
		let window_width = self.display.get_window_size().width();

		let title_help_indicator_total_length = TITLE_HELP_INDICATOR_LABEL.len() + self.help_indicator_key.len();

		if window_width >= TITLE.len() {
			self.display.draw_str(TITLE)?;
			// only draw help if there is room
			if window_width > TITLE.len() + title_help_indicator_total_length {
				if (window_width - TITLE.len() - title_help_indicator_total_length) > 0 {
					let padding = " ".repeat(window_width - TITLE.len() - title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
				if show_help {
					self.display
						.draw_str(format!("{}{}", TITLE_HELP_INDICATOR_LABEL, self.help_indicator_key).as_str())?;
				}
				else {
					let padding = " ".repeat(title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
			}
			else if (window_width - TITLE.len()) > 0 {
				let padding = " ".repeat(window_width - TITLE.len());
				self.display.draw_str(padding.as_str())?;
			}
		}
		else {
			self.display.draw_str(TITLE_SHORT)?;
			if (window_width - TITLE_SHORT.len()) > 0 {
				let padding = " ".repeat(window_width - TITLE_SHORT.len());
				self.display.draw_str(padding.as_str())?;
			}
		}

		// reset style
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, false, false)?;
		Ok(())
	}
}
