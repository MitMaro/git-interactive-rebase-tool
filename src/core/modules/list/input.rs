use ::input::{Event, EventHandler, InputOptions, MetaEvent, MouseEventKind};
use lazy_static::lazy_static;

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(false).undo_redo(true).help(true);
}

#[allow(clippy::cognitive_complexity)]
pub fn get_event(event_handler: &EventHandler) -> Event {
	event_handler.read_event(&INPUT_OPTIONS, |event, key_bindings| {
		match event {
			e if key_bindings.abort.contains(&e) => Event::from(MetaEvent::Abort),
			e if key_bindings.action_break.contains(&e) => Event::from(MetaEvent::ActionBreak),
			e if key_bindings.action_drop.contains(&e) => Event::from(MetaEvent::ActionDrop),
			e if key_bindings.action_edit.contains(&e) => Event::from(MetaEvent::ActionEdit),
			e if key_bindings.action_fixup.contains(&e) => Event::from(MetaEvent::ActionFixup),
			e if key_bindings.action_pick.contains(&e) => Event::from(MetaEvent::ActionPick),
			e if key_bindings.action_reword.contains(&e) => Event::from(MetaEvent::ActionReword),
			e if key_bindings.action_squash.contains(&e) => Event::from(MetaEvent::ActionSquash),
			e if key_bindings.edit.contains(&e) => Event::from(MetaEvent::Edit),
			e if key_bindings.force_abort.contains(&e) => Event::from(MetaEvent::ForceAbort),
			e if key_bindings.force_rebase.contains(&e) => Event::from(MetaEvent::ForceRebase),
			e if key_bindings.insert_line.contains(&e) => Event::from(MetaEvent::InsertLine),
			e if key_bindings.move_down.contains(&e) => Event::from(MetaEvent::MoveCursorDown),
			e if key_bindings.move_down_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageDown),
			e if key_bindings.move_end.contains(&e) => Event::from(MetaEvent::MoveCursorEnd),
			e if key_bindings.move_home.contains(&e) => Event::from(MetaEvent::MoveCursorHome),
			e if key_bindings.move_left.contains(&e) => Event::from(MetaEvent::MoveCursorLeft),
			e if key_bindings.move_right.contains(&e) => Event::from(MetaEvent::MoveCursorRight),
			e if key_bindings.move_selection_down.contains(&e) => Event::from(MetaEvent::SwapSelectedDown),
			e if key_bindings.move_selection_up.contains(&e) => Event::from(MetaEvent::SwapSelectedUp),
			e if key_bindings.move_up.contains(&e) => Event::from(MetaEvent::MoveCursorUp),
			e if key_bindings.move_up_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageUp),
			e if key_bindings.open_in_external_editor.contains(&e) => Event::from(MetaEvent::OpenInEditor),
			e if key_bindings.rebase.contains(&e) => Event::from(MetaEvent::Rebase),
			e if key_bindings.remove_line.contains(&e) => Event::from(MetaEvent::Delete),
			e if key_bindings.show_commit.contains(&e) => Event::from(MetaEvent::ShowCommit),
			e if key_bindings.toggle_visual_mode.contains(&e) => Event::from(MetaEvent::ToggleVisualMode),
			Event::Mouse(mouse_event) => {
				match mouse_event.kind {
					MouseEventKind::ScrollDown => Event::from(MetaEvent::MoveCursorDown),
					MouseEventKind::ScrollUp => Event::from(MetaEvent::MoveCursorUp),
					_ => event,
				}
			},
			_ => event,
		}
	})
}
