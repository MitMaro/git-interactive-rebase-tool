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
	pub(super) fn has_minimum_four_bit_color(self) -> bool {
		self == FourBit || self == EightBit || self == TrueColor
	}

	pub(super) fn has_true_color(self) -> bool {
		self == TrueColor
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
