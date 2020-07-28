use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
	LightWhite,
	LightBlack,
	LightBlue,
	LightCyan,
	LightGreen,
	LightMagenta,
	LightRed,
	LightYellow,
	DarkWhite,
	DarkBlack,
	DarkBlue,
	DarkCyan,
	DarkGreen,
	DarkMagenta,
	DarkRed,
	DarkYellow,
	Default,
	Index(i16),
	RGB { red: i16, green: i16, blue: i16 },
}

impl TryFrom<&str> for Color {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"black" => Ok(Self::LightBlack),
			"blue" => Ok(Self::LightBlue),
			"cyan" => Ok(Self::LightCyan),
			"green" => Ok(Self::LightGreen),
			"magenta" => Ok(Self::LightMagenta),
			"red" => Ok(Self::LightRed),
			"white" => Ok(Self::LightWhite),
			"yellow" => Ok(Self::LightYellow),
			"light black" => Ok(Self::LightBlack),
			"light blue" => Ok(Self::LightBlue),
			"light cyan" => Ok(Self::LightCyan),
			"light green" => Ok(Self::LightGreen),
			"light magenta" => Ok(Self::LightMagenta),
			"light red" => Ok(Self::LightRed),
			"light white" => Ok(Self::LightWhite),
			"light yellow" => Ok(Self::LightYellow),
			"dark black" => Ok(Self::DarkBlack),
			"dark blue" => Ok(Self::DarkBlue),
			"dark cyan" => Ok(Self::DarkCyan),
			"dark green" => Ok(Self::DarkGreen),
			"dark magenta" => Ok(Self::DarkMagenta),
			"dark red" => Ok(Self::DarkRed),
			"dark white" => Ok(Self::DarkWhite),
			"dark yellow" => Ok(Self::DarkYellow),
			"transparent" | "-1" => Ok(Self::Default),
			_ => {
				let matches: Vec<&str> = s.split(',').collect();

				match matches.len() {
					1 => {
						let color_index = s.parse::<i16>();
						match color_index {
							Ok(i) if i >= 0 && i < 256 => Ok(Self::Index(i)),
							_ => Err(format!("Invalid color value: {}", s)),
						}
					},
					3 => {
						let red = matches[0].parse::<i16>().unwrap_or(-1);
						let green = matches[1].parse::<i16>().unwrap_or(-1);
						let blue = matches[2].parse::<i16>().unwrap_or(-1);

						if red > -1 && green > -1 && blue > -1 && red < 256 && green < 256 && blue < 256 {
							return Ok(Self::RGB { red, green, blue });
						}
						Err(format!("Invalid color string: {}. Values must be within 0-255.", s))
					},
					_ => Err(format!("Invalid color value: {}", s)),
				}
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
		assert_eq!(Color::try_from("black").unwrap(), Color::LightBlack);
	}

	#[test]
	fn action_try_from_str_light_black() {
		assert_eq!(Color::try_from("light black").unwrap(), Color::LightBlack);
	}

	#[test]
	fn action_try_from_str_dark_black() {
		assert_eq!(Color::try_from("dark black").unwrap(), Color::DarkBlack);
	}

	#[test]
	fn action_try_from_str_blue() {
		assert_eq!(Color::try_from("blue").unwrap(), Color::LightBlue);
	}

	#[test]
	fn action_try_from_str_light_blue() {
		assert_eq!(Color::try_from("light blue").unwrap(), Color::LightBlue);
	}

	#[test]
	fn action_try_from_str_dark_blue() {
		assert_eq!(Color::try_from("dark blue").unwrap(), Color::DarkBlue);
	}

	#[test]
	fn action_try_from_str_cyan() {
		assert_eq!(Color::try_from("cyan").unwrap(), Color::LightCyan);
	}

	#[test]
	fn action_try_from_str_light_cyan() {
		assert_eq!(Color::try_from("light cyan").unwrap(), Color::LightCyan);
	}

	#[test]
	fn action_try_from_str_dark_cyan() {
		assert_eq!(Color::try_from("dark cyan").unwrap(), Color::DarkCyan);
	}

	#[test]
	fn action_try_from_str_green() {
		assert_eq!(Color::try_from("green").unwrap(), Color::LightGreen);
	}

	#[test]
	fn action_try_from_str_light_green() {
		assert_eq!(Color::try_from("light green").unwrap(), Color::LightGreen);
	}

	#[test]
	fn action_try_from_str_dark_green() {
		assert_eq!(Color::try_from("dark green").unwrap(), Color::DarkGreen);
	}

	#[test]
	fn action_try_from_str_magenta() {
		assert_eq!(Color::try_from("magenta").unwrap(), Color::LightMagenta);
	}

	#[test]
	fn action_try_from_str_light_magenta() {
		assert_eq!(Color::try_from("light magenta").unwrap(), Color::LightMagenta);
	}

	#[test]
	fn action_try_from_str_dark_magenta() {
		assert_eq!(Color::try_from("dark magenta").unwrap(), Color::DarkMagenta);
	}

	#[test]
	fn action_try_from_str_red() {
		assert_eq!(Color::try_from("red").unwrap(), Color::LightRed);
	}

	#[test]
	fn action_try_from_str_light_red() {
		assert_eq!(Color::try_from("light red").unwrap(), Color::LightRed);
	}

	#[test]
	fn action_try_from_str_dark_red() {
		assert_eq!(Color::try_from("dark red").unwrap(), Color::DarkRed);
	}

	#[test]
	fn action_try_from_str_white() {
		assert_eq!(Color::try_from("white").unwrap(), Color::LightWhite);
	}

	#[test]
	fn action_try_from_str_yellow() {
		assert_eq!(Color::try_from("yellow").unwrap(), Color::LightYellow);
	}

	#[test]
	fn action_try_from_str_light_yellow() {
		assert_eq!(Color::try_from("light yellow").unwrap(), Color::LightYellow);
	}

	#[test]
	fn action_try_from_str_dark_yellow() {
		assert_eq!(Color::try_from("dark yellow").unwrap(), Color::DarkYellow);
	}

	#[test]
	fn action_try_from_color_index_minimum() {
		assert_eq!(Color::try_from("0").unwrap(), Color::Index(0));
	}

	#[test]
	fn action_try_from_color_index_maximum() {
		assert_eq!(Color::try_from("255").unwrap(), Color::Index(255));
	}

	#[test]
	fn action_try_from_color_rgb_color() {
		assert_eq!(Color::try_from("100,101,102").unwrap(), Color::RGB {
			red: 100,
			green: 101,
			blue: 102
		});
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
	fn action_try_from_color_index_invalid_upper_limit() {
		assert_eq!(Color::try_from("256").unwrap_err(), "Invalid color value: 256");
	}

	#[test]
	fn action_try_from_color_index_invalid_lower_limit() {
		// -1 is transparent/default and a valid value
		assert_eq!(Color::try_from("-2").unwrap_err(), "Invalid color value: -2");
	}

	#[test]
	fn action_try_from_str_invalid_single_value() {
		assert_eq!(Color::try_from("invalid").unwrap_err(), "Invalid color value: invalid");
	}

	#[test]
	fn action_try_from_str_invalid_multiple_value() {
		assert_eq!(
			Color::try_from("invalid,invalid").unwrap_err(),
			"Invalid color value: invalid,invalid"
		);
	}
}
