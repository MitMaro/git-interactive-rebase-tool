use ::input::KeyCode;
use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn normal_mode_help() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			module.state = ListState::Normal;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{Normal,Underline} Key      Action{Pad( )}",
				"{BODY}",
				"{IndicatorColor} Up      {Normal,Dimmed}|{Normal}Move selection up",
				"{IndicatorColor} Down    {Normal,Dimmed}|{Normal}Move selection down",
				"{IndicatorColor} PageUp  {Normal,Dimmed}|{Normal}Move selection up half a page",
				"{IndicatorColor} PageDown{Normal,Dimmed}|{Normal}Move selection down half a page",
				"{IndicatorColor} Home    {Normal,Dimmed}|{Normal}Move selection to top of the list",
				"{IndicatorColor} End     {Normal,Dimmed}|{Normal}Move selection to end of the list",
				"{IndicatorColor} Left    {Normal,Dimmed}|{Normal}Scroll content to the left",
				"{IndicatorColor} Right   {Normal,Dimmed}|{Normal}Scroll content to the right",
				"{IndicatorColor} q       {Normal,Dimmed}|{Normal}Abort interactive rebase",
				"{IndicatorColor} Q       {Normal,Dimmed}|{Normal}Immediately abort interactive rebase",
				"{IndicatorColor} w       {Normal,Dimmed}|{Normal}Write interactive rebase file",
				"{IndicatorColor} W       {Normal,Dimmed}|{Normal}Immediately write interactive rebase file",
				"{IndicatorColor} ?       {Normal,Dimmed}|{Normal}Show help",
				"{IndicatorColor} j       {Normal,Dimmed}|{Normal}Move selected lines down",
				"{IndicatorColor} k       {Normal,Dimmed}|{Normal}Move selected lines up",
				"{IndicatorColor} c       {Normal,Dimmed}|{Normal}Show commit information",
				"{IndicatorColor} b       {Normal,Dimmed}|{Normal}Toggle break action",
				"{IndicatorColor} p       {Normal,Dimmed}|{Normal}Set selected commits to be picked",
				"{IndicatorColor} r       {Normal,Dimmed}|{Normal}Set selected commits to be reworded",
				"{IndicatorColor} e       {Normal,Dimmed}|{Normal}Set selected commits to be edited",
				"{IndicatorColor} s       {Normal,Dimmed}|{Normal}Set selected commits to be squashed",
				"{IndicatorColor} f       {Normal,Dimmed}|{Normal}Set selected commits to be fixed-up",
				"{IndicatorColor} d       {Normal,Dimmed}|{Normal}Set selected commits to be dropped",
				"{IndicatorColor} E       {Normal,Dimmed}|{Normal}Edit an exec, label, reset or merge action's content",
				"{IndicatorColor} I       {Normal,Dimmed}|{Normal}Insert a new line",
				"{IndicatorColor} Delete  {Normal,Dimmed}|{Normal}Completely remove the selected lines",
				"{IndicatorColor} Controlz{Normal,Dimmed}|{Normal}Undo the last change",
				"{IndicatorColor} Controly{Normal,Dimmed}|{Normal}Redo the previous undone change",
				"{IndicatorColor} !       {Normal,Dimmed}|{Normal}Open the todo file in the default editor",
				"{IndicatorColor} v       {Normal,Dimmed}|{Normal}Enter visual mode",
				"{TRAILING}",
				"{IndicatorColor}Press any key to close"
			);
		},
	);
}

#[test]
fn normal_mode_help_event() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help), Event::from(KeyCode::Enter)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			module.state = ListState::Normal;
			_ = test_context.handle_all_events(&mut module);
			assert!(!module.normal_mode_help.is_active());
		},
	);
}

#[test]
fn visual_mode_help() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			module.state = ListState::Visual;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{Normal,Underline} Key      Action{Pad( )}",
				"{BODY}",
				"{IndicatorColor} Up      {Normal,Dimmed}|{Normal}Move selection up",
				"{IndicatorColor} Down    {Normal,Dimmed}|{Normal}Move selection down",
				"{IndicatorColor} PageUp  {Normal,Dimmed}|{Normal}Move selection up half a page",
				"{IndicatorColor} PageDown{Normal,Dimmed}|{Normal}Move selection down half a page",
				"{IndicatorColor} Home    {Normal,Dimmed}|{Normal}Move selection to top of the list",
				"{IndicatorColor} End     {Normal,Dimmed}|{Normal}Move selection to end of the list",
				"{IndicatorColor} Left    {Normal,Dimmed}|{Normal}Scroll content to the left",
				"{IndicatorColor} Right   {Normal,Dimmed}|{Normal}Scroll content to the right",
				"{IndicatorColor} q       {Normal,Dimmed}|{Normal}Abort interactive rebase",
				"{IndicatorColor} Q       {Normal,Dimmed}|{Normal}Immediately abort interactive rebase",
				"{IndicatorColor} w       {Normal,Dimmed}|{Normal}Write interactive rebase file",
				"{IndicatorColor} W       {Normal,Dimmed}|{Normal}Immediately write interactive rebase file",
				"{IndicatorColor} ?       {Normal,Dimmed}|{Normal}Show help",
				"{IndicatorColor} j       {Normal,Dimmed}|{Normal}Move selected lines down",
				"{IndicatorColor} k       {Normal,Dimmed}|{Normal}Move selected lines up",
				"{IndicatorColor} p       {Normal,Dimmed}|{Normal}Set selected commits to be picked",
				"{IndicatorColor} r       {Normal,Dimmed}|{Normal}Set selected commits to be reworded",
				"{IndicatorColor} e       {Normal,Dimmed}|{Normal}Set selected commits to be edited",
				"{IndicatorColor} s       {Normal,Dimmed}|{Normal}Set selected commits to be squashed",
				"{IndicatorColor} f       {Normal,Dimmed}|{Normal}Set selected commits to be fixed-up",
				"{IndicatorColor} d       {Normal,Dimmed}|{Normal}Set selected commits to be dropped",
				"{IndicatorColor} Delete  {Normal,Dimmed}|{Normal}Completely remove the selected lines",
				"{IndicatorColor} Controlz{Normal,Dimmed}|{Normal}Undo the last change",
				"{IndicatorColor} Controly{Normal,Dimmed}|{Normal}Redo the previous undone change",
				"{IndicatorColor} !       {Normal,Dimmed}|{Normal}Open the todo file in the default editor",
				"{IndicatorColor} v       {Normal,Dimmed}|{Normal}Exit visual mode",
				"{TRAILING}",
				"{IndicatorColor}Press any key to close"
			);
		},
	);
}

#[test]
fn visual_mode_help_event() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help), Event::from(KeyCode::Enter)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			module.state = ListState::Visual;
			_ = test_context.handle_all_events(&mut module);
			assert!(!module.visual_mode_help.is_active());
		},
	);
}
