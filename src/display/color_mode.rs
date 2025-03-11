/// Represents the color mode of a terminal interface.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ColorMode {
	/// Supports 2 colors.
	TwoTone,
	/// Supports 8 colors.
	ThreeBit,
	/// Supports 16 colors.
	FourBit,
	/// Supports 256 colors.
	EightBit,
	/// Supports 24 bits of color.
	TrueColor,
}

impl ColorMode {
	/// Supports 4 bit or more of color.
	#[must_use]
	pub(crate) fn has_minimum_four_bit_color(self) -> bool {
		self == Self::FourBit || self == Self::EightBit || self == Self::TrueColor
	}

	/// Has true color support.
	#[must_use]
	pub(crate) fn has_true_color(self) -> bool {
		self == Self::TrueColor
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn color_mode_has_minimum_four_bit_color_two_tone() {
		assert!(!ColorMode::TwoTone.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_three_bit() {
		assert!(!ColorMode::ThreeBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_four_bit() {
		assert!(ColorMode::FourBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_eight_bit() {
		assert!(ColorMode::EightBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_true_color() {
		assert!(ColorMode::TrueColor.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_true_color_two_tone() {
		assert!(!ColorMode::TwoTone.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_three_bit() {
		assert!(!ColorMode::ThreeBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_four_bit() {
		assert!(!ColorMode::FourBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_eight_bit() {
		assert!(!ColorMode::EightBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_true_color() {
		assert!(ColorMode::TrueColor.has_true_color());
	}
}
