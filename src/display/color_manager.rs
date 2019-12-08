use crate::config::Theme;
use crate::display::display_color::DisplayColor;
use crate::display::Curses;
use pancurses::chtype;

pub(in crate::display) struct ColorManager {
	action_break: (chtype, chtype),
	action_drop: (chtype, chtype),
	action_edit: (chtype, chtype),
	action_exec: (chtype, chtype),
	action_fixup: (chtype, chtype),
	action_pick: (chtype, chtype),
	action_reword: (chtype, chtype),
	action_squash: (chtype, chtype),
	diff_add: (chtype, chtype),
	diff_change: (chtype, chtype),
	diff_remove: (chtype, chtype),
	indicator: (chtype, chtype),
	normal: (chtype, chtype),
}

impl ColorManager {
	pub fn new(theme: &Theme, curses: &mut Curses) -> Self {
		Self {
			normal: curses.register_selectable_color_pairs(
				theme.color_foreground,
				theme.color_background,
				theme.color_selected_background,
			),
			indicator: curses.register_selectable_color_pairs(
				theme.color_indicator,
				theme.color_background,
				theme.color_selected_background,
			),
			action_break: curses.register_selectable_color_pairs(
				theme.color_action_break,
				theme.color_background,
				theme.color_selected_background,
			),
			action_drop: curses.register_selectable_color_pairs(
				theme.color_action_drop,
				theme.color_background,
				theme.color_selected_background,
			),
			action_edit: curses.register_selectable_color_pairs(
				theme.color_action_edit,
				theme.color_background,
				theme.color_selected_background,
			),
			action_exec: curses.register_selectable_color_pairs(
				theme.color_action_exec,
				theme.color_background,
				theme.color_selected_background,
			),
			action_fixup: curses.register_selectable_color_pairs(
				theme.color_action_fixup,
				theme.color_background,
				theme.color_selected_background,
			),
			action_pick: curses.register_selectable_color_pairs(
				theme.color_action_pick,
				theme.color_background,
				theme.color_selected_background,
			),
			action_reword: curses.register_selectable_color_pairs(
				theme.color_action_reword,
				theme.color_background,
				theme.color_selected_background,
			),
			action_squash: curses.register_selectable_color_pairs(
				theme.color_action_squash,
				theme.color_background,
				theme.color_selected_background,
			),
			diff_add: curses.register_selectable_color_pairs(
				theme.color_diff_add,
				theme.color_background,
				theme.color_selected_background,
			),
			diff_change: curses.register_selectable_color_pairs(
				theme.color_diff_change,
				theme.color_background,
				theme.color_selected_background,
			),
			diff_remove: curses.register_selectable_color_pairs(
				theme.color_diff_remove,
				theme.color_background,
				theme.color_selected_background,
			),
		}
	}

	pub fn get_color(&self, color: DisplayColor, selected: bool) -> chtype {
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
				DisplayColor::Normal => self.normal.1,
				DisplayColor::IndicatorColor => self.indicator.1,
				DisplayColor::DiffAddColor => self.diff_add.1,
				DisplayColor::DiffRemoveColor => self.diff_remove.1,
				DisplayColor::DiffChangeColor => self.diff_change.1,
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
				DisplayColor::Normal => self.normal.0,
				DisplayColor::IndicatorColor => self.indicator.0,
				DisplayColor::DiffAddColor => self.diff_add.0,
				DisplayColor::DiffRemoveColor => self.diff_remove.0,
				DisplayColor::DiffChangeColor => self.diff_change.0,
			}
		}
	}
}
