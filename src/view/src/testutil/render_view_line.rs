use bitflags::bitflags;
use display::DisplayColor;

use crate::{LineSegment, ViewData, ViewLine};

bitflags! {
	/// Options for the `assert_rendered_output!` macro
	#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
	pub struct AssertRenderOptions: u8 {
		/// Ignore trailing whitespace
		const INCLUDE_TRAILING_WHITESPACE = 0b0000_0001;
		/// Ignore pinned indicator
		const INCLUDE_PINNED = 0b0000_0010;
		/// Don't include style information
		const INCLUDE_STYLE = 0b0000_0100;
		/// Only render the body, in this mode {BODY} is also not rendered
		const BODY_ONLY = 0b0000_1000;
	}
}

pub(crate) fn render_view_data(view_data: &ViewData, options: AssertRenderOptions) -> Vec<String> {
	let mut lines = vec![];
	let body_only = options.contains(AssertRenderOptions::BODY_ONLY);
	if !body_only && view_data.show_title() {
		if view_data.show_help() {
			lines.push(String::from("{TITLE}{HELP}"));
		}
		else {
			lines.push(String::from("{TITLE}"));
		}
	}

	if view_data.is_empty() {
		lines.push(String::from("{EMPTY}"));
	}

	if !body_only {
		let leading_lines = view_data.get_leading_lines();
		if !leading_lines.is_empty() {
			lines.push(String::from("{LEADING}"));
			for line in leading_lines {
				lines.push(render_view_line(line, Some(options)));
			}
		}
	}

	let body_lines = view_data.get_lines();
	if !body_lines.is_empty() {
		if !body_only {
			lines.push(String::from("{BODY}"));
		}
		for line in body_lines {
			lines.push(render_view_line(line, Some(options)));
		}
	}

	if !body_only {
		let trailing_lines = view_data.get_trailing_lines();
		if !trailing_lines.is_empty() {
			lines.push(String::from("{TRAILING}"));
			for line in trailing_lines {
				lines.push(render_view_line(line, Some(options)));
			}
		}
	}
	lines
}

pub(crate) fn render_style(line_segment: &LineSegment) -> String {
	let color_string = match line_segment.get_color() {
		DisplayColor::ActionBreak => String::from("ActionBreak"),
		DisplayColor::ActionDrop => String::from("ActionDrop"),
		DisplayColor::ActionEdit => String::from("ActionEdit"),
		DisplayColor::ActionExec => String::from("ActionExec"),
		DisplayColor::ActionFixup => String::from("ActionFixup"),
		DisplayColor::ActionPick => String::from("ActionPick"),
		DisplayColor::ActionReword => String::from("ActionReword"),
		DisplayColor::ActionSquash => String::from("ActionSquash"),
		DisplayColor::DiffAddColor => String::from("DiffAddColor"),
		DisplayColor::DiffChangeColor => String::from("DiffChangeColor"),
		DisplayColor::DiffRemoveColor => String::from("DiffRemoveColor"),
		DisplayColor::DiffContextColor => String::from("DiffContextColor"),
		DisplayColor::DiffWhitespaceColor => String::from("DiffWhitespaceColor"),
		DisplayColor::IndicatorColor => String::from("IndicatorColor"),
		DisplayColor::Normal => String::from("Normal"),
		DisplayColor::ActionLabel => String::from("ActionLabel"),
		DisplayColor::ActionReset => String::from("ActionReset"),
		DisplayColor::ActionMerge => String::from("ActionMerge"),
		DisplayColor::ActionUpdateRef => String::from("ActionUpdateRef"),
	};

	let mut style = vec![];
	if line_segment.is_dimmed() {
		style.push("Dimmed");
	}
	if line_segment.is_underlined() {
		style.push("Underline");
	}
	if line_segment.is_reversed() {
		style.push("Reversed");
	}

	if style.is_empty() {
		format!("{{{color_string}}}")
	}
	else {
		format!("{{{color_string},{}}}", style.join(","))
	}
}

/// Render a `ViewLine` to a `String` using similar logic that is used in the `View`.
#[must_use]
#[inline]
pub fn render_view_line(view_line: &ViewLine, options: Option<AssertRenderOptions>) -> String {
	let mut line = String::new();

	let opts = options.unwrap_or_default();

	if opts.contains(AssertRenderOptions::INCLUDE_PINNED) {
		let pinned = view_line.get_number_of_pinned_segment();
		if pinned > 0 {
			line.push_str(format!("{{Pin({pinned})}}").as_str());
		}
	}

	if view_line.get_selected() {
		line.push_str("{Selected}");
	}

	let mut last_style = String::new();
	for segment in view_line.get_segments() {
		if opts.contains(AssertRenderOptions::INCLUDE_STYLE) {
			let style = render_style(segment);
			if style != last_style {
				line.push_str(style.as_str());
				last_style = style;
			}
		}
		line.push_str(segment.get_content());
	}
	if let Some(padding) = view_line.get_padding().as_ref() {
		if opts.contains(AssertRenderOptions::INCLUDE_STYLE) {
			let style = render_style(padding);
			if style != last_style {
				line.push_str(style.as_str());
			}
		}
		line.push_str(format!("{{Pad({})}}", padding.get_content()).as_str());
	}
	line
}
