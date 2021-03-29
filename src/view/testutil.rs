use crate::{
	display::display_color::DisplayColor,
	view::{view_data::ViewData, view_line::ViewLine},
};

// TODO change how style is passed to use a Struct
#[allow(clippy::fn_params_excessive_bools)]
fn render_style(color: DisplayColor, selected: bool, dimmed: bool, underline: bool, reversed: bool) -> String {
	let mut color_string = match color {
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
	};

	if selected {
		color_string.push_str("(selected)");
	}

	let mut style = vec![];
	if dimmed {
		style.push("Dimmed");
	}
	if underline {
		style.push("Underline");
	}
	if reversed {
		style.push("Reversed");
	}

	if style.is_empty() {
		format!("{{{}}}", color_string)
	}
	else {
		format!("{{{},{}}}", color_string, style.join(","))
	}
}

fn render_view_line(view_line: &ViewLine) -> String {
	let mut line = String::new();
	let segments = view_line.get_segments();
	for (index, segment) in segments.iter().enumerate() {
		let content = segment.get_content();
		let is_padding = index + 1 == segments.len() && content.replace(view_line.padding_character(), "").is_empty();
		// skip standard padding
		if is_padding
			&& view_line.padding_character() == " "
			&& segment.get_color() == DisplayColor::Normal
			&& !segment.is_dimmed()
			&& !segment.is_reversed()
			&& !segment.is_underlined()
		{
			continue;
		}
		line.push_str(
			render_style(
				segment.get_color(),
				view_line.get_selected(),
				segment.is_dimmed(),
				segment.is_underlined(),
				segment.is_reversed(),
			)
			.as_str(),
		);
		// only render
		if is_padding {
			line.push_str(format!("{{Pad {},{}}}", view_line.padding_character(), content.len()).as_str());
		}
		else {
			line.push_str(segment.get_content());
		}
	}
	line
}

fn render_view_data(view_data: &ViewData) -> Vec<String> {
	let mut lines = vec![];
	if view_data.show_title() {
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

	let leading_lines = view_data.get_leading_lines();
	if !leading_lines.is_empty() {
		lines.push(String::from("{LEADING}"));
		for line in leading_lines {
			lines.push(render_view_line(line));
		}
	}

	let body_lines = view_data.get_lines();
	if !body_lines.is_empty() {
		lines.push(String::from("{BODY}"));
		for line in body_lines {
			lines.push(render_view_line(line));
		}
	}

	let trailing_lines = view_data.get_trailing_lines();
	if !trailing_lines.is_empty() {
		lines.push(String::from("{TRAILING}"));
		for line in trailing_lines {
			lines.push(render_view_line(line));
		}
	}
	lines
}

pub fn _assert_rendered_output(view_data: &ViewData, expected: &[String]) {
	let output = render_view_data(view_data);
	let mut mismatch = false;
	let mut error_output = vec![
		String::from("\nUnexpected output!"),
		String::from("--- Expected"),
		String::from("+++ Actual"),
		String::from("=========="),
	];

	for (expected_line, output_line) in expected.iter().zip(output.iter()) {
		let e = expected_line.replace(" ", "·").replace("\t", "   →");
		if expected_line == output_line {
			error_output.push(format!(" {}", e));
		}
		else {
			mismatch = true;
			let o = output_line.replace(" ", "·").replace("\t", "   →");
			error_output.push(format!("-{}", e));
			error_output.push(format!("+{}", o));
		}
	}

	match expected.len() {
		a if a > output.len() => {
			mismatch = true;
			for line in expected.iter().skip(output.len()) {
				error_output.push(format!("-{}", line.replace(" ", "·").replace("\t", "   →")));
			}
		},
		a if a < output.len() => {
			mismatch = true;
			for line in output.iter().skip(expected.len()) {
				error_output.push(format!("+{}", line.replace(" ", "·").replace("\t", "   →")));
			}
		},
		_ => {},
	}

	if mismatch {
		error_output.push(String::from("==========\n"));
		panic!("{}", error_output.join("\n"));
	}
}

#[macro_export]
macro_rules! assert_rendered_output {
	($view_data:expr) => {
		let expected: Vec<String> = vec![];
		crate::view::testutil::_assert_rendered_output(&$view_data, &expected);
	};
	($view_data:expr, $($arg:expr),*) => {
		let expected = vec![$( String::from($arg), )*];
		crate::view::testutil::_assert_rendered_output(&$view_data, &expected);
	};
}
