//! Utilities for writing tests that interact with input events.
use std::time::Duration;

use bitflags::bitflags;
use crossbeam_channel::{unbounded, Receiver};
use display::DisplayColor;

use super::{action::ViewAction, render_slice::RenderAction, view_data::ViewData, view_line::ViewLine, ViewSender};

const STARTS_WITH: &str = "{{StartsWith}}";
const ENDS_WITH: &str = "{{EndsWith}}";
const ANY_LINE: &str = "{{Any}}";
const VISIBLE_SPACE_REPLACEMENT: &str = "\u{b7}"; // "·"
const VISIBLE_TAB_REPLACEMENT: &str = "   \u{2192}"; // "   →"

/// Assert the rendered output from a `ViewData`.
#[macro_export]
macro_rules! render_line {
	(AnyLine) => {{
		concat!("{{Any}}")
	}};
	(AnyLine  $count:expr) => {{
		concat!("{{Any(", $count, ")}}")
	}};
	(StartsWith $line:expr) => {{
		concat!("{{StartsWith}}", $line)
	}};
	(EndsWith $line:expr) => {{
		concat!("{{EndsWith}}", $line)
	}};
}

bitflags! {
	/// Options for the `assert_rendered_output!` macro
	pub struct AssertRenderOptions: u8 {
		/// The default assertion options
		const DEFAULT = 0b0000_0000;
		/// Ignore trailing whitespace
		const INCLUDE_TRAILING_WHITESPACE = 0b0000_0001;
	}
}

fn replace_invisibles(line: &str) -> String {
	line.replace(' ', VISIBLE_SPACE_REPLACEMENT)
		.replace('\t', VISIBLE_TAB_REPLACEMENT)
}

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

/// Render a `ViewLine` to a `String` using similar logic that is used in the `View`.
#[must_use]
#[inline]
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

#[allow(clippy::panic)]
fn expand_expected(expected: &[String]) -> Vec<String> {
	expected
		.iter()
		.flat_map(|f| {
			if f.starts_with("{{Any(") && f.ends_with(")}}") {
				let lines = f
					.replace("{{Any(", "")
					.replace(")}}", "")
					.as_str()
					.parse::<u32>()
					.unwrap_or_else(|_| panic!("Expected {} to have integer line count", f));
				vec![String::from(ANY_LINE); lines as usize]
			}
			else {
				vec![f.clone()]
			}
		})
		.collect::<Vec<String>>()
}

#[allow(clippy::indexing_slicing, clippy::string_slice)]
pub(crate) fn _assert_rendered_output(options: AssertRenderOptions, actual: &[String], expected: &[String]) {
	let mut mismatch = false;
	let mut error_output = vec![
		String::from("\nUnexpected output!"),
		String::from("--- Expected"),
		String::from("+++ Actual"),
		String::from("=========="),
	];

	for (expected_line, output_line) in expected.iter().zip(actual.iter()) {
		let output = if options.contains(AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE) {
			output_line.as_str()
		}
		else {
			output_line.trim_end()
		};

		let mut e = replace_invisibles(expected_line);
		let o = replace_invisibles(output);

		if expected_line == ANY_LINE {
			error_output.push(format!(" {}", o));
			continue;
		}

		if expected_line.starts_with(ENDS_WITH) {
			e = e.replace(ENDS_WITH, "");
			if output.ends_with(&expected_line.replace(ENDS_WITH, "")) {
				error_output.push(format!(" {}", o));
			}
			else {
				mismatch = true;
				error_output.push(format!("-EndsWith {}", e));
				error_output.push(format!("+         {}", &o[o.len() - e.len() + 2..]));
			}
			continue;
		}

		if expected_line.starts_with(STARTS_WITH) {
			e = e.replace(STARTS_WITH, "");
			if output.starts_with(&expected_line.replace(STARTS_WITH, "")) {
				error_output.push(format!(" {}", o));
			}
			else {
				mismatch = true;
				error_output.push(format!("-StartsWith {}", e));
				error_output.push(format!("+           {}", &o[0..=e.len()]));
			}
			continue;
		}

		if expected_line == output {
			error_output.push(format!(" {}", e));
		}
		else {
			mismatch = true;
			error_output.push(format!("-{}", e));
			error_output.push(format!("+{}", o));
		}
	}

	match expected.len() {
		a if a > actual.len() => {
			mismatch = true;
			for line in expected.iter().skip(actual.len()) {
				error_output.push(format!("-{}", replace_invisibles(line)));
			}
		},
		a if a < actual.len() => {
			mismatch = true;
			for line in actual.iter().skip(expected.len()) {
				error_output.push(format!("+{}", replace_invisibles(line)));
			}
		},
		_ => {},
	}

	if mismatch {
		error_output.push(String::from("==========\n"));
		panic!("{}", error_output.join("\n"));
	}
}

/// Assert the rendered output from a `ViewData`. Generally this function is not used directly,
/// instead use the `assert_rendered_output!` macro.
#[inline]
pub fn _assert_rendered_output_from_view_data(view_data: &ViewData, expected: &[String], options: AssertRenderOptions) {
	let output = render_view_data(view_data);

	_assert_rendered_output(options, &output, &expand_expected(expected));
}

/// Assert the rendered output from a `ViewData`.
#[macro_export]
macro_rules! assert_rendered_output {
	($view_data:expr) => {
		use view::testutil::{_assert_rendered_output_from_view_data, AssertRenderOptions};
		let expected: Vec<String> = vec![];
		_assert_rendered_output_from_view_data($view_data, &expected, AssertRenderOptions::DEFAULT);
	};
	($view_data:expr, $($arg:expr),*) => {
		use view::testutil::{_assert_rendered_output_from_view_data, AssertRenderOptions};
		let expected = vec![$( String::from($arg), )*];
		_assert_rendered_output_from_view_data($view_data, &expected, AssertRenderOptions::DEFAULT);
	};
	(Options $options:expr, $view_data:expr, $($arg:expr),*) => {
		use view::testutil::{_assert_rendered_output_from_view_data, AssertRenderOptions};
		let expected = vec![$( String::from($arg), )*];
		_assert_rendered_output_from_view_data($view_data, &expected, $options);
	};
}

fn assert_view_sender_actions(view_sender: &ViewSender, expected_actions: &[String]) {
	let actions = view_sender
		.clone_render_slice()
		.lock()
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

fn action_to_string(action: ViewAction) -> String {
	String::from(match action {
		ViewAction::Stop => "Stop",
		ViewAction::Refresh => "Refresh",
		ViewAction::Render => "Render",
		ViewAction::Start => "Start",
		ViewAction::End => "End",
	})
}

/// Context for a `ViewSender` test.
#[derive(Debug)]
#[non_exhaustive]
pub struct TestContext {
	/// The sender instance.
	pub sender: ViewSender,
	/// The receiver instance.
	pub receiver: Receiver<ViewAction>,
}

impl TestContext {
	/// Drop the receiver, useful for testing disconnect error handling.
	#[inline]
	pub fn drop_receiver(&mut self) {
		let (_, receiver) = unbounded();
		self.receiver = receiver;
	}

	/// Assert that render actions were sent.
	#[inline]
	pub fn assert_render_action(&self, actions: &[&str]) {
		assert_view_sender_actions(
			&self.sender,
			actions
				.iter()
				.map(|s| String::from(*s))
				.collect::<Vec<String>>()
				.as_slice(),
		);
	}

	/// Assert that certain messages were sent by the `ViewSender`.
	#[inline]
	#[allow(clippy::missing_panics_doc)]
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
				let action_name = action_to_string(action);
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
			error_output.push(format!("+{}", action_to_string(action)));
		}

		if mismatch {
			error_output.push(String::from("==========\n"));
			panic!("{}", error_output.join("\n"));
		}
	}
}

/// Provide an `ViewSender` instance for use within a test.
#[inline]
pub fn with_view_sender<C>(callback: C)
where C: FnOnce(TestContext) {
	let (sender, receiver) = unbounded();
	let view_sender = ViewSender::new(sender);

	callback(TestContext {
		sender: view_sender,
		receiver,
	});
}
