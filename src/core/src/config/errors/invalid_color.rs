use thiserror::Error;

/// A invalid color error.
#[derive(Error, Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum InvalidColorError {
	/// The indexed color is invalid.
	#[error("Index must be between 0-255")]
	Indexed,
	/// The red color is invalid.
	#[error("Red color value must be between 0-255")]
	Red,
	/// The green color is invalid.
	#[error("Green color value must be between 0-255")]
	Green,
	/// The blue color is invalid.
	#[error("Blue color value must be between 0-255")]
	Blue,
	/// An unknown color was used.
	#[error("Unknown color value")]
	Invalid,
}
