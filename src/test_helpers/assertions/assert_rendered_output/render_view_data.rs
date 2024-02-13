use crate::{
	test_helpers::assertions::assert_rendered_output::{render_view_line, AssertRenderOptions},
	view::ViewData,
};

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
