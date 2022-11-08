use anyhow::anyhow;

use crate::{
	module::{Event, InputOptions, Module, State},
	testutil::{create_test_keybindings, module_test},
};

struct TestModule;

impl Module for TestModule {}

#[test]
fn default_trait_method_activate() {
	module_test(&[], &[], |context| {
		let mut module = TestModule {};
		assert!(
			module
				.activate(context.todo_file_context.todo_file(), State::List)
				.artifact()
				.is_none()
		);
	});
}

#[test]
fn default_trait_method_deactivate() {
	let mut module = TestModule {};
	assert!(module.deactivate().artifact().is_none());
}

#[test]
fn default_trait_method_build_view_data() {
	module_test(&[], &[], |context| {
		let mut module = TestModule {};
		let view_data = module.build_view_data(&context.render_context, context.todo_file_context.todo_file());
		assert!(view_data.is_empty());
	});
}

#[test]
fn default_trait_method_input_options() {
	let module = TestModule {};
	assert_eq!(module.input_options(), &InputOptions::RESIZE);
}

#[test]
fn default_trait_method_read_event() {
	let key_bindings = create_test_keybindings();
	let module = TestModule {};
	assert_eq!(module.read_event(Event::from('a'), &key_bindings), Event::from('a'));
}

#[test]
fn default_trait_method_handle_event() {
	module_test(&[], &[], |mut context| {
		let mut module = TestModule {};
		let mut result = module.handle_event(
			Event::from('a'),
			&context.view_context.state,
			context.todo_file_context.todo_file_mut(),
		);
		assert!(result.artifact().is_none());
	});
}

#[test]
fn default_trait_method_handle_error() {
	let mut module = TestModule {};
	assert!(module.handle_error(&anyhow!("Error")).artifact().is_none());
}
