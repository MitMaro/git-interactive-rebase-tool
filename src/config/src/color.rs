use anyhow::{anyhow, Error};

/// Represents a color.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum Color {
	/// The default teminal color.
	Default,
	/// The standard white color.
	LightWhite,
	/// The standard black color.
	LightBlack,
	/// The standard blue color.
	LightBlue,
	/// The standard cyan color.
	LightCyan,
	/// The standard green color.
	LightGreen,
	/// The standard magenta color.
	LightMagenta,
	/// The standard red color.
	LightRed,
	/// The standard yellow color.
	LightYellow,
	/// The standard grey color.
	LightGrey,
	/// The dimmed white color.
	DarkWhite,
	/// The dimmed black color.
	DarkBlack,
	/// The dimmed blue color.
	DarkBlue,
	/// The dimmed cyan color.
	DarkCyan,
	/// The dimmed green color.
	DarkGreen,
	/// The dimmed magenta color.
	DarkMagenta,
	/// The dimmed red color.
	DarkRed,
	/// The dimmed yellow color.
	DarkYellow,
	/// The dimmed grey color.
	DarkGrey,
	/// An ANSI indexed color.
	Index(u8),
	/// A RGB color triple.
	Rgb {
		/// The red amount of the triple.
		red: u8,
		/// The green amount of the triple.
		green: u8,
		/// The blue amount of the triple.
		blue: u8,
	},
}

impl TryFrom<&str> for Color {
	type Error = Error;

	#[allow(clippy::indexing_slicing)]
	#[inline]
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
			"grey" | "light grey" => Ok(Self::LightGrey),
			"dark black" => Ok(Self::DarkBlack),
			"dark blue" => Ok(Self::DarkBlue),
			"dark cyan" => Ok(Self::DarkCyan),
			"dark green" => Ok(Self::DarkGreen),
			"dark magenta" => Ok(Self::DarkMagenta),
			"dark red" => Ok(Self::DarkRed),
			"dark white" => Ok(Self::DarkWhite),
			"dark yellow" => Ok(Self::DarkYellow),
			"dark grey" => Ok(Self::DarkGrey),
			"transparent" | "-1" => Ok(Self::Default),
			_ => {
				let matches: Vec<&str> = s.split(',').collect();

				match matches.len() {
					1 => {
						let color_index = s.parse::<u8>();
						match color_index {
							Ok(i) if (0..=255).contains(&i) => Ok(Self::Index(i)),
							_ => {
								Err(anyhow!(
									"\"{}\" is not a valid color index. Index must be between 0-255.",
									s
								))
							},
						}
					},
					3 => {
						let red = matches[0].parse::<i16>().unwrap_or(-1);
						let green = matches[1].parse::<i16>().unwrap_or(-1);
						let blue = matches[2].parse::<i16>().unwrap_or(-1);

						if red >= 0 && green >= 0 && blue >= 0 && red < 256 && green < 256 && blue < 256 {
							return Ok(Self::Rgb {
								red: red
									.try_into()
									.map_err(|e| anyhow!("Invalid red color value").context(e))?,
								green: green
									.try_into()
									.map_err(|e| anyhow!("Invalid green color value").context(e))?,
								blue: blue
									.try_into()
									.map_err(|e| anyhow!("Invalid blue color value").context(e))?,
							});
						}
						Err(anyhow!(
							"\"{}\" is not a valid color triple. Values must be between 0-255.",
							s
						))
					},
					_ => Err(anyhow!("\"{}\" is not a valid color value.", s)),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::named_black("black", Color::LightBlack)]
	#[case::named_light_black("light black", Color::LightBlack)]
	#[case::named_dark_black("dark black", Color::DarkBlack)]
	#[case::named_blue("blue", Color::LightBlue)]
	#[case::named_light_blue("light blue", Color::LightBlue)]
	#[case::named_dark_blue("dark blue", Color::DarkBlue)]
	#[case::named_cyan("cyan", Color::LightCyan)]
	#[case::named_light_cyan("light cyan", Color::LightCyan)]
	#[case::named_dark_cyan("dark cyan", Color::DarkCyan)]
	#[case::named_green("green", Color::LightGreen)]
	#[case::named_light_green("light green", Color::LightGreen)]
	#[case::named_dark_green("dark green", Color::DarkGreen)]
	#[case::named_magenta("magenta", Color::LightMagenta)]
	#[case::named_light_magenta("light magenta", Color::LightMagenta)]
	#[case::named_dark_magenta("dark magenta", Color::DarkMagenta)]
	#[case::named_red("red", Color::LightRed)]
	#[case::named_light_red("light red", Color::LightRed)]
	#[case::named_dark_red("dark red", Color::DarkRed)]
	#[case::named_white("white", Color::LightWhite)]
	#[case::named_yellow("yellow", Color::LightYellow)]
	#[case::named_light_yellow("light yellow", Color::LightYellow)]
	#[case::named_dark_yellow("dark yellow", Color::DarkYellow)]
	#[case::index_0("0", Color::Index(0))]
	#[case::index_255("255", Color::Index(255))]
	#[case::rgb("100,101,102", Color::Rgb {
		red: 100,
		green: 101,
		blue: 102
	})]
	fn try_from(#[case] color_string: &str, #[case] expected: Color) {
		assert_eq!(Color::try_from(color_string).unwrap(), expected);
	}

	#[rstest]
	#[case::non_number_red("red,0,0", "\"red,0,0\" is not a valid color triple. Values must be between 0-255.")]
	#[case::rgb_non_number_green(
		"0,green,0",
		"\"0,green,0\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_blue(
		"0,0,blue",
		"\"0,0,blue\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_red_lower_limit(
		"-1,0,0",
		"\"-1,0,0\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_green_lower_limit(
		"0,-1,0",
		"\"0,-1,0\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_blue_lower_limit(
		"0,0,-1",
		"\"0,0,-1\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_red_upper_limit(
		"256,0,0",
		"\"256,0,0\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_green_upper_limit(
		"0,256,0",
		"\"0,256,0\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::rgb_non_number_blue_upper_limit(
		"0,0,256",
		"\"0,0,256\" is not a valid color triple. Values must be between 0-255."
	)]
	#[case::index_upper_limit("256", "\"256\" is not a valid color index. Index must be between 0-255.")]
	#[case::index_lower_limit("-2", "\"-2\" is not a valid color index. Index must be between 0-255.")]
	#[case::str_single_value("invalid", "\"invalid\" is not a valid color index. Index must be between 0-255.")]
	#[case::str_multiple_value("invalid,invalid", "\"invalid,invalid\" is not a valid color value.")]
	fn color_try_from_fail(#[case] color_string: &str, #[case] expected: &str) {
		assert_eq!(Color::try_from(color_string).unwrap_err().to_string(), expected);
	}
}
