/// Asserts that expression is not empty using `is_empty`.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_not_empty!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// use testutils::assert_not_empty;
/// # fn main() {
/// let vec: Vec<usize> = vec![1];
///
/// assert_not_empty!(vec);
///
/// // With custom messages
/// assert_not_empty!(vec, "Expecting {:?} to not be empty", vec);
/// # }
/// ```
///
/// A empty value will cause a panic:
///
/// ```rust,should_panic
/// use testutils::assert_not_empty;
/// # fn main() {
/// let vec: Vec<usize> = vec![];
///
/// assert_not_empty!(vec); // Will panic
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_not_empty!`]: ./macro.debug_assert_not_empty.html
#[macro_export]
macro_rules! assert_not_empty {
	($expression:expr) => {
		assert!(!$expression.is_empty(), "assertion failed, expected {:?} to not be empty", $expression)
	};
	($expression:expr, $($arg:tt)+) => {
		assert!(
			!$expression.is_empty(),
			"assertion failed, expected {:?} to not be empty: {}",
			$expression,
			format_args!($($arg)+)
		)
	};
}

/// Asserts that expression is not empty using `is_empty`.
///
/// Like [`assert_not_empty!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_not_empty!`]: ./macro.assert_not_empty.html
#[macro_export]
macro_rules! debug_assert_not_empty {
	($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_not_empty!($($arg)*); })
}
