use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
	White,
	Black,
	Blue,
	Cyan,
	Green,
	Magenta,
	Red,
	Yellow,
}

impl TryFrom<&str> for Color {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"black" => Ok(Color::Black),
			"blue" => Ok(Color::Blue),
			"cyan" => Ok(Color::Cyan),
			"green" => Ok(Color::Green),
			"magenta" => Ok(Color::Magenta),
			"red" => Ok(Color::Red),
			"white" => Ok(Color::White),
			"yellow" => Ok(Color::Yellow),
			_ => Err(format!("Invalid color string: {}", s)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Color;
	use super::TryFrom;

	#[test]
	fn action_from_str_black() {
		assert_eq!(Color::try_from("black").unwrap(), Color::Black);
	}

	#[test]
	fn action_from_str_blue() {
		assert_eq!(Color::try_from("blue").unwrap(), Color::Blue);
	}

	#[test]
	fn action_from_str_cyan() {
		assert_eq!(Color::try_from("cyan").unwrap(), Color::Cyan);
	}

	#[test]
	fn action_from_str_green() {
		assert_eq!(Color::try_from("green").unwrap(), Color::Green);
	}

	#[test]
	fn action_from_str_magenta() {
		assert_eq!(Color::try_from("magenta").unwrap(), Color::Magenta);
	}

	#[test]
	fn action_from_str_red() {
		assert_eq!(Color::try_from("red").unwrap(), Color::Red);
	}

	#[test]
	fn action_from_str_yellow() {
		assert_eq!(Color::try_from("yellow").unwrap(), Color::Yellow);
	}
}
