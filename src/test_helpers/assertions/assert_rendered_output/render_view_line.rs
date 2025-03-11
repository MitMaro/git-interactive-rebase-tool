use crate::{
	test_helpers::assertions::assert_rendered_output::{AssertRenderOptions, render_style},
	view::ViewLine,
};

/// Render a `ViewLine` to a `String` using similar logic that is used in the `View`.
#[must_use]
pub(crate) fn render_view_line(view_line: &ViewLine, options: Option<AssertRenderOptions>) -> String {
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
	if let Some(padding) = view_line.get_padding() {
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
