use crate::display::curses::Input;

pub(super) fn curses_input_to_string(input: Input) -> String {
	match input {
		Input::Character(c) => {
			if c == '\t' {
				String::from("Tab")
			}
			else {
				c.to_string()
			}
		},
		Input::KeyBackspace => String::from("Backspace"),
		Input::KeyBTab => String::from("ShiftTab"),
		Input::KeyDC => String::from("Delete"),
		Input::KeyDown => String::from("Down"),
		Input::KeyEnd => String::from("End"),
		Input::KeyEnter => String::from("Enter"),
		Input::KeyF0 => String::from("F0"),
		Input::KeyF1 => String::from("F1"),
		Input::KeyF2 => String::from("F2"),
		Input::KeyF3 => String::from("F3"),
		Input::KeyF4 => String::from("F4"),
		Input::KeyF5 => String::from("F5"),
		Input::KeyF6 => String::from("F6"),
		Input::KeyF7 => String::from("F7"),
		Input::KeyF8 => String::from("F8"),
		Input::KeyF9 => String::from("F9"),
		Input::KeyF10 => String::from("F10"),
		Input::KeyF11 => String::from("F11"),
		Input::KeyF12 => String::from("F12"),
		Input::KeyF13 => String::from("F13"),
		Input::KeyF14 => String::from("F14"),
		Input::KeyF15 => String::from("F15"),
		Input::KeyHome => String::from("Home"),
		Input::KeyIC => String::from("Insert"),
		Input::KeyLeft => String::from("Left"),
		Input::KeyNPage => String::from("PageDown"),
		Input::KeyPPage => String::from("PageUp"),
		Input::KeyResize => String::from("Resize"),
		Input::KeyRight => String::from("Right"),
		Input::KeySDC => String::from("ShiftDelete"),
		Input::KeySEnd => String::from("ShiftEnd"),
		Input::KeySF => String::from("ShiftDown"),
		Input::KeySHome => String::from("ShiftHome"),
		Input::KeySLeft => String::from("ShiftLeft"),
		Input::KeySR => String::from("ShiftUp"),
		Input::KeySRight => String::from("ShiftRight"),
		Input::KeyUp => String::from("Up"),
		_ => String::from("Other"),
	}
}

pub fn get_input_short_name(input: &str) -> String {
	match input {
		"Backspace" => String::from("bs"),
		"Delete" => String::from("dl"),
		"Down" => String::from("dn"),
		"End" => String::from("end"),
		"Enter" => String::from("ent"),
		"Home" => String::from("hm"),
		"Insert" => String::from("ins"),
		"Left" => String::from("lf"),
		"Other" => String::from("ot"),
		"PageDown" => String::from("pdn"),
		"PageUp" => String::from("pup"),
		"Resize" => String::from("rz"),
		"Right" => String::from("rt"),
		"ShiftDelete" => String::from("sdl"),
		"ShiftEnd" => String::from("sed"),
		"ShiftHome" => String::from("shm"),
		"ShiftLeft" => String::from("slf"),
		"ShiftRight" => String::from("srt"),
		"ShiftTab" => String::from("Stb"),
		"ShiftUp" => String::from("sup"),
		"Tab" => String::from("tb"),
		"Up" => String::from("up"),
		_ => String::from(input),
	}
}
