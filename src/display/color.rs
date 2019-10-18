use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Color {
	White,
	Black,
	Blue,
	Cyan,
	Green,
	Magenta,
	Red,
	Yellow,
	Default,
	RGB { red: i16, green: i16, blue: i16 },
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
			"transparent" | "-1" => Ok(Color::Default),
			_ => {
				let matches: Vec<&str> = s.split(',').collect();

				if matches.len() == 3 {
					let red = matches.get(0).unwrap().parse::<i16>().unwrap_or(-1);
					let green = matches.get(1).unwrap().parse::<i16>().unwrap_or(-1);
					let blue = matches.get(2).unwrap().parse::<i16>().unwrap_or(-1);

					if red > -1 && green > -1 && blue > -1 && red < 256 && green < 256 && blue < 256 {
						// values need to be mapped to 0-1000
						return Ok(Color::RGB {
							red: ((f64::from(red) / 255.0) * 1000.0) as i16,
							green: ((f64::from(green) / 255.0) * 1000.0) as i16,
							blue: ((f64::from(blue) / 255.0) * 1000.0) as i16,
						});
					}
					return Err(format!("Invalid color string: {}. Values must be within 0-255.", s));
				}
				Err(format!("Invalid color string: {}", s))
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Color;
	use std::convert::TryFrom;

	#[test]
	fn action_try_from_str_black() {
		assert_eq!(Color::try_from("black").unwrap(), Color::Black);
	}

	#[test]
	fn action_try_from_str_blue() {
		assert_eq!(Color::try_from("blue").unwrap(), Color::Blue);
	}

	#[test]
	fn action_try_from_str_cyan() {
		assert_eq!(Color::try_from("cyan").unwrap(), Color::Cyan);
	}

	#[test]
	fn action_try_from_str_green() {
		assert_eq!(Color::try_from("green").unwrap(), Color::Green);
	}

	#[test]
	fn action_try_from_str_magenta() {
		assert_eq!(Color::try_from("magenta").unwrap(), Color::Magenta);
	}

	#[test]
	fn action_try_from_str_red() {
		assert_eq!(Color::try_from("red").unwrap(), Color::Red);
	}

	#[test]
	fn action_try_from_str_white() {
		assert_eq!(Color::try_from("white").unwrap(), Color::White);
	}

	#[test]
	fn action_try_from_str_yellow() {
		assert_eq!(Color::try_from("yellow").unwrap(), Color::Yellow);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_red() {
		assert_eq!(
			Color::try_from("red,0,0").unwrap_err(),
			"Invalid color string: red,0,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_green() {
		assert_eq!(
			Color::try_from("0,green,0").unwrap_err(),
			"Invalid color string: 0,green,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_blue() {
		assert_eq!(
			Color::try_from("0,0,blue").unwrap_err(),
			"Invalid color string: 0,0,blue. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_red_lower_limit() {
		assert_eq!(
			Color::try_from("-1,0,0").unwrap_err(),
			"Invalid color string: -1,0,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_green_lower_limit() {
		assert_eq!(
			Color::try_from("0,-1,0").unwrap_err(),
			"Invalid color string: 0,-1,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_blue_lower_limit() {
		assert_eq!(
			Color::try_from("0,0,-1").unwrap_err(),
			"Invalid color string: 0,0,-1. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_red_upper_limit() {
		assert_eq!(
			Color::try_from("256,0,0").unwrap_err(),
			"Invalid color string: 256,0,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_green_upper_limit() {
		assert_eq!(
			Color::try_from("0,256,0").unwrap_err(),
			"Invalid color string: 0,256,0. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_rgb_invalid_non_number_blue_upper_limit() {
		assert_eq!(
			Color::try_from("0,0,256").unwrap_err(),
			"Invalid color string: 0,0,256. Values must be within 0-255."
		);
	}

	#[test]
	fn action_try_from_str_invalid() {
		assert_eq!(Color::try_from("invalid").unwrap_err(), "Invalid color string: invalid");
	}
}
