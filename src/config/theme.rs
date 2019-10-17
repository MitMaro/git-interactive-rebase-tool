use crate::display::Color;

#[derive(Clone, Debug)]
pub struct Theme {
	pub color_foreground: Color,
	pub color_background: Color,
	pub color_selected_background: Color,
	pub color_indicator: Color,
	pub color_action_break: Color,
	pub color_action_drop: Color,
	pub color_action_edit: Color,
	pub color_action_exec: Color,
	pub color_action_fixup: Color,
	pub color_action_pick: Color,
	pub color_action_reword: Color,
	pub color_action_squash: Color,
	pub color_diff_add: Color,
	pub color_diff_change: Color,
	pub color_diff_remove: Color,
	pub character_vertical_spacing: String,
}
