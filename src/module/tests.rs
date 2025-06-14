use anyhow::anyhow;

use crate::{
	input::KeyBindings,
	module::{Event, InputOptions, Module, State},
	test_helpers::testers,
};

struct TestModule;

impl Module for TestModule {}

#[test]
fn default_trait_method_activate() {
	let mut module = TestModule {};
	assert!(module.activate(State::List).artifact().is_none());
}

#[test]
fn default_trait_method_deactivate() {
	let mut module = TestModule {};
	assert!(module.deactivate().artifact().is_none());
}

#[test]
fn default_trait_method_build_view_data() {
	testers::module(&[], &[], |context| {
		let mut module = TestModule {};
		let view_data = module.build_view_data(&context.render_context);
		assert!(!view_data.get_name().is_empty());
	});
}

#[test]
fn default_trait_method_input_options() {
	let module = TestModule {};
	assert_eq!(module.input_options(), &InputOptions::RESIZE);
}

#[test]
fn default_trait_method_read_event() {
	let module = TestModule {};
	assert_eq!(module.read_event(Event::from('a'), &KeyBindings::default()), Event::from('a'));
}

#[test]
fn default_trait_method_handle_event() {
	let mut module = TestModule {};
	let mut result = module.handle_event(Event::from('a'));
	assert!(result.artifact().is_none());
}

#[test]
fn default_trait_method_handle_error() {
	let mut module = TestModule {};
	assert!(module.handle_error(&anyhow!("Error")).artifact().is_none());
}
