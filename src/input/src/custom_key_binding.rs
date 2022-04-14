/// A custom keybindings compatible struct.
pub trait CustomKeybinding {
	/// Create a new instance from the configuration keybindings.
	fn new(key_bindings: &config::KeyBindings) -> Self;
}
