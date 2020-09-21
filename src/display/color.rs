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
			"black" | "light black" => Ok(Self::LightBlack),
			"blue" | "light blue" => Ok(Self::LightBlue),
			"cyan" | "light cyan" => Ok(Self::LightCyan),
			"green" | "light green" => Ok(Self::LightGreen),
			"magenta" | "light magenta" => Ok(Self::LightMagenta),
			"red" | "light red" => Ok(Self::LightRed),
			"white" | "light white" => Ok(Self::LightWhite),
			"yellow" | "light yellow" => Ok(Self::LightYellow),
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

	macro_rules! test_color_try_from {
		($name:ident, $color_string:expr, $expected:expr) => {
			concat_idents::concat_idents!(test_name = color_try_from_, $name {
				#[test]
				fn test_name() {
					assert_eq!(Color::try_from($color_string).unwrap(), $expected);
				}
			});
		}
	}

	macro_rules! test_color_try_from_invalid {
		($name:ident, $color_string:expr, $expected:expr) => {
			concat_idents::concat_idents!(test_name = color_try_from_invalid_, $name {
				#[test]
				fn test_name() {
					assert_eq!(Color::try_from($color_string).unwrap_err(), $expected);
				}
			});
		}
	}

	test_color_try_from!(named_black, "black", Color::LightBlack);
	test_color_try_from!(named_light_black, "light black", Color::LightBlack);
	test_color_try_from!(named_dark_black, "dark black", Color::DarkBlack);
	test_color_try_from!(named_blue, "blue", Color::LightBlue);
	test_color_try_from!(named_light_blue, "light blue", Color::LightBlue);
	test_color_try_from!(named_dark_blue, "dark blue", Color::DarkBlue);
	test_color_try_from!(named_cyan, "cyan", Color::LightCyan);
	test_color_try_from!(named_light_cyan, "light cyan", Color::LightCyan);
	test_color_try_from!(named_dark_cyan, "dark cyan", Color::DarkCyan);
	test_color_try_from!(named_green, "green", Color::LightGreen);
	test_color_try_from!(named_light_green, "light green", Color::LightGreen);
	test_color_try_from!(named_dark_green, "dark green", Color::DarkGreen);
	test_color_try_from!(named_magenta, "magenta", Color::LightMagenta);
	test_color_try_from!(named_light_magenta, "light magenta", Color::LightMagenta);
	test_color_try_from!(named_dark_magenta, "dark magenta", Color::DarkMagenta);
	test_color_try_from!(named_red, "red", Color::LightRed);
	test_color_try_from!(named_light_red, "light red", Color::LightRed);
	test_color_try_from!(named_dark_red, "dark red", Color::DarkRed);
	test_color_try_from!(named_white, "white", Color::LightWhite);
	test_color_try_from!(named_yellow, "yellow", Color::LightYellow);
	test_color_try_from!(named_light_yellow, "light yellow", Color::LightYellow);
	test_color_try_from!(named_dark_yellow, "dark yellow", Color::DarkYellow);
	test_color_try_from!(index_0, "0", Color::Index(0));
	test_color_try_from!(index_255, "255", Color::Index(255));
	test_color_try_from!(rgb, "100,101,102", Color::RGB {
		red: 100,
		green: 101,
		blue: 102
	});

	test_color_try_from_invalid!(
		non_number_red,
		"red,0,0",
		"Invalid color string: red,0,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_green,
		"0,green,0",
		"Invalid color string: 0,green,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_blue,
		"0,0,blue",
		"Invalid color string: 0,0,blue. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_red_lower_limit,
		"-1,0,0",
		"Invalid color string: -1,0,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_green_lower_limit,
		"0,-1,0",
		"Invalid color string: 0,-1,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_blue_lower_limit,
		"0,0,-1",
		"Invalid color string: 0,0,-1. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_red_upper_limit,
		"256,0,0",
		"Invalid color string: 256,0,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_green_upper_limit,
		"0,256,0",
		"Invalid color string: 0,256,0. Values must be within 0-255."
	);
	test_color_try_from_invalid!(
		rgb_non_number_blue_upper_limit,
		"0,0,256",
		"Invalid color string: 0,0,256. Values must be within 0-255."
	);
	test_color_try_from_invalid!(index_upper_limit, "256", "Invalid color value: 256");
	test_color_try_from_invalid!(
		index_lower_limit,
		// -1 is transparent/default and a valid value
		"-2",
		"Invalid color value: -2"
	);
	test_color_try_from_invalid!(str_single_value, "invalid", "Invalid color value: invalid");
	test_color_try_from_invalid!(
		str_multiple_value,
		"invalid,invalid",
		"Invalid color value: invalid,invalid"
	);
}
