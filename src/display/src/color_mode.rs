use super::color_mode::ColorMode::{EightBit, FourBit, TrueColor};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorMode {
	TwoTone,
	ThreeBit,
	FourBit,
	EightBit,
	TrueColor,
}

impl ColorMode {
	pub fn has_minimum_four_bit_color(self) -> bool {
		self == FourBit || self == EightBit || self == TrueColor
	}

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
