use crate::display::color::Color;

#[derive(Clone, Debug)]
pub(crate) struct Theme {
	pub(crate) color_foreground: Color,
	pub(crate) color_background: Color,
	pub(crate) color_selected_background: Color,
	pub(crate) color_indicator: Color,
	pub(crate) color_action_break: Color,
	pub(crate) color_action_drop: Color,
	pub(crate) color_action_edit: Color,
	pub(crate) color_action_exec: Color,
	pub(crate) color_action_fixup: Color,
	pub(crate) color_action_pick: Color,
	pub(crate) color_action_reword: Color,
	pub(crate) color_action_squash: Color,
	pub(crate) color_diff_add: Color,
	pub(crate) color_diff_change: Color,
	pub(crate) color_diff_remove: Color,
	pub(crate) character_vertical_spacing: String,
}
