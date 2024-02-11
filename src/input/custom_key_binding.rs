/// A custom keybindings compatible struct.
pub(crate) trait CustomKeybinding {
	/// Create a new instance from the configuration keybindings.
	fn new(key_bindings: &crate::config::KeyBindings) -> Self;
}
