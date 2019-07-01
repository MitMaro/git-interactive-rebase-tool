use crate::action::Action;
use crate::window::WindowColor;

pub fn get_action_color(action: Action) -> WindowColor {
	match action {
		Action::Break => WindowColor::ActionBreak,
		Action::Drop => WindowColor::ActionDrop,
		Action::Edit => WindowColor::ActionEdit,
		Action::Exec => WindowColor::ActionExec,
		Action::Fixup => WindowColor::ActionFixup,
		Action::Pick => WindowColor::ActionPick,
		Action::Reword => WindowColor::ActionReword,
		Action::Squash => WindowColor::ActionSquash,
	}
}
