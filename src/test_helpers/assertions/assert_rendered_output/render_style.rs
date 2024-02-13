use crate::{display::DisplayColor, view::LineSegment};

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
