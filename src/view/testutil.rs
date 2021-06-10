use std::{
	sync::{mpsc, mpsc::Receiver},
	time::Duration,
};

use crate::{
	display::display_color::DisplayColor,
	view::{action::ViewAction, render_slice::RenderAction, view_data::ViewData, view_line::ViewLine, ViewSender},
};

fn render_style(color: DisplayColor, dimmed: bool, underline: bool, reversed: bool) -> String {
	let color_string = match color {
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

pub fn render_view_line(view_line: &ViewLine) -> String {
	let mut line = String::new();

	if view_line.get_selected() {
		line.push_str("{Selected}");
	}

	for segment in view_line.get_segments() {
		line.push_str(
			render_style(
				segment.get_color(),
				segment.is_dimmed(),
				segment.is_underlined(),
				segment.is_reversed(),
			)
			.as_str(),
		);
		line.push_str(segment.get_content());
	}
	if let Some(padding) = view_line.get_padding().as_ref() {
		line.push_str(
			render_style(
				padding.get_color(),
				padding.is_dimmed(),
				padding.is_underlined(),
				padding.is_reversed(),
			)
			.as_str(),
		);
		line.push_str(format!("{{Pad({})}}", padding.get_content()).as_str());
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

pub fn _assert_rendered_output(output: &[String], expected: &[String]) {
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

pub fn _assert_rendered_output_from_view_data(view_data: &ViewData, expected: &[String]) {
	let output = render_view_data(view_data);
	_assert_rendered_output(&output, expected);
}

#[macro_export]
macro_rules! assert_rendered_output {
	($view_data:expr) => {
		let expected: Vec<String> = vec![];
		crate::view::testutil::_assert_rendered_output_from_view_data($view_data, &expected);
	};
	($view_data:expr, $($arg:expr),*) => {
		let expected = vec![$( String::from($arg), )*];
		crate::view::testutil::_assert_rendered_output_from_view_data($view_data, &expected);
	};
}

pub fn _assert_view_sender_actions(view_sender: &ViewSender, expected_actions: &[String]) {
	let actions = view_sender
		.clone_render_slice()
		.lock()
		.unwrap()
		.get_actions()
		.iter()
		.map(|a| {
			match *a {
				RenderAction::ScrollDown => String::from("ScrollDown"),
				RenderAction::ScrollUp => String::from("ScrollUp"),
				RenderAction::ScrollRight => String::from("ScrollRight"),
				RenderAction::ScrollLeft => String::from("ScrollLeft"),
				RenderAction::PageUp => String::from("PageUp"),
				RenderAction::PageDown => String::from("PageDown"),
				RenderAction::Resize(width, height) => format!("Resize({}, {})", width, height),
			}
		})
		.collect::<Vec<String>>();

	let mut mismatch = false;
	let mut error_output = vec![
		String::from("\nUnexpected actions!"),
		String::from("--- Expected"),
		String::from("+++ Actual"),
		String::from("=========="),
	];

	for (expected_action, actual_action) in expected_actions.iter().zip(actions.iter()) {
		if expected_action == actual_action {
			error_output.push(format!(" {}", expected_action));
		}
		else {
			mismatch = true;
			error_output.push(format!("-{}", expected_action));
			error_output.push(format!("+{}", actual_action));
		}
	}

	match expected_actions.len() {
		a if a > actions.len() => {
			mismatch = true;
			for action in expected_actions.iter().skip(actions.len()) {
				error_output.push(format!("-{}", action));
			}
		},
		a if a < actions.len() => {
			mismatch = true;
			for action in actions.iter().skip(expected_actions.len()) {
				error_output.push(format!("+{}", action));
			}
		},
		_ => {},
	}

	if mismatch {
		error_output.push(String::from("==========\n"));
		panic!("{}", error_output.join("\n"));
	}
}

fn action_to_string(action: &ViewAction) -> String {
	String::from(match *action {
		ViewAction::Stop => "Stop",
		ViewAction::Refresh => "Refresh",
		ViewAction::Render => "Render",
		ViewAction::Start => "Start",
		ViewAction::End => "End",
	})
}

#[macro_export]
macro_rules! assert_render_action {
	($sender:expr, $($arg:expr),*) => {
		let expected = vec![$( String::from($arg), )*];
		crate::view::testutil::_assert_view_sender_actions($sender, &expected);
	};
}

pub struct TestContext {
	pub view_sender: ViewSender,
	pub receiver: Receiver<ViewAction>,
}

impl TestContext {
	pub fn drop_receiver(&mut self) {
		let (_, receiver) = mpsc::channel();
		self.receiver = receiver;
	}

	pub fn assert_render_action(&self, actions: &[&str]) {
		_assert_view_sender_actions(
			&self.view_sender,
			actions
				.iter()
				.map(|s| String::from(*s))
				.collect::<Vec<String>>()
				.as_slice(),
		);
	}

	pub fn assert_sent_messages(&self, messages: Vec<&str>) {
		let mut mismatch = false;
		let mut error_output = vec![
			String::from("\nUnexpected messages!"),
			String::from("--- Expected"),
			String::from("+++ Actual"),
			String::from("=========="),
		];

		for message in messages {
			if let Ok(action) = self.receiver.recv_timeout(Duration::new(1, 0)) {
				let action_name = action_to_string(&action);
				if message == action_name {
					error_output.push(format!(" {}", message));
				}
				else {
					mismatch = true;
					error_output.push(format!("-{}", message));
					error_output.push(format!("+{}", action_name));
				}
			}
			else {
				error_output.push(format!("-{}", message));
			}
		}

		// wait some time for any other actions that were sent that should have not been
		while let Ok(action) = self.receiver.recv_timeout(Duration::new(0, 10000)) {
			mismatch = true;
			error_output.push(format!("+{}", action_to_string(&action)));
		}

		if mismatch {
			error_output.push(String::from("==========\n"));
			panic!("{}", error_output.join("\n"));
		}
	}
}

pub fn with_view_sender<C>(callback: C)
where C: FnOnce(TestContext) {
	let (sender, receiver) = mpsc::channel();
	let view_sender = ViewSender::new(sender);

	callback(TestContext { view_sender, receiver });
}
