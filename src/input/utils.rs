use pancurses::Input;

pub(super) fn curses_input_to_string(input: Input) -> String {
	match input {
		Input::Character(c) => c.to_string(),
		Input::KeyLeft => String::from("Left"),
		Input::KeyRight => String::from("Right"),
		Input::KeyDown => String::from("Down"),
		Input::KeyUp => String::from("Up"),
		Input::KeyPPage => String::from("PageUp"),
		Input::KeyNPage => String::from("PageDown"),
		Input::KeyResize => String::from("Resize"),
		_ => String::from("Other"),
	}
}
