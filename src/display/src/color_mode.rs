use super::color_mode::ColorMode::{EightBit, FourBit, TrueColor};

/// Represents the color mode of a terminal interface.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum ColorMode {
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
	#[inline]
	#[must_use]
	pub fn has_minimum_four_bit_color(self) -> bool {
		self == FourBit || self == EightBit || self == TrueColor
	}

	/// Has true color support.
	#[inline]
	#[must_use]
	pub fn has_true_color(self) -> bool {
		self == TrueColor
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::color_mode::ColorMode::*;

	#[test]
	fn color_mode_has_minimum_four_bit_color_two_tone() {
		assert!(!TwoTone.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_three_bit() {
		assert!(!ThreeBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_four_bit() {
		assert!(FourBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_eight_bit() {
		assert!(EightBit.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_minimum_four_bit_color_true_color() {
		assert!(TrueColor.has_minimum_four_bit_color());
	}

	#[test]
	fn color_mode_has_true_color_two_tone() {
		assert!(!TwoTone.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_three_bit() {
		assert!(!ThreeBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_four_bit() {
		assert!(!FourBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_eight_bit() {
		assert!(!EightBit.has_true_color());
	}

	#[test]
	fn color_mode_has_true_color_true_color() {
		assert!(TrueColor.has_true_color());
	}
}
