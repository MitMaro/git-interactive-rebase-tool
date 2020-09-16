use crate::display::display_color::DisplayColor;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;

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
	for segment in view_line.get_segments() {
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
		line.push_str(segment.get_content());
	}
	line
}

pub(crate) fn render_view_data(view_data: &ViewData) -> String {
	let mut lines = vec![];
	if view_data.show_title() {
		if view_data.show_help() {
			lines.push("{TITLE}{HELP}".to_string());
		}
		else {
			lines.push("{TITLE}".to_string());
		}
	}

	if let Some(prompt) = view_data.get_prompt() {
		lines.push("{PROMPT}".to_string());
		lines.push(format!("{}", prompt));
		return lines.join("\n");
	}

	lines.push("{LEADING}".to_string());
	for line in view_data.get_leading_lines() {
		lines.push(render_view_line(line));
	}

	lines.push("{BODY}".to_string());
	for line in view_data.get_lines() {
		lines.push(render_view_line(line));
	}

	lines.push("{TRAILING}".to_string());
	for line in view_data.get_trailing_lines() {
		lines.push(render_view_line(line));
	}

	return lines.join("\n");
}
