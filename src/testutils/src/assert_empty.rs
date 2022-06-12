/// Asserts that expression is empty using `is_empty`.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_empty!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// use testutils::assert_empty;
/// # fn main() {
/// let vec: Vec<usize> = vec![];
///
/// assert_empty!(vec);
///
/// // With custom messages
/// assert_empty!(vec, "Expecting {:?} to be empty", vec);
/// # }
/// ```
///
/// A non-empty value will cause a panic:
///
/// ```rust,should_panic
/// use testutils::assert_empty;
/// # fn main() {
/// let vec: Vec<usize> = vec![1];
///
/// assert_empty!(vec); // Will panic
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_empty!`]: ./macro.debug_assert_empty.html
#[macro_export]
macro_rules! assert_empty {
	($expression:expr) => {
		assert!($expression.is_empty(), "assertion failed, expected {:?} to be empty", $expression)
	};
	($expression:expr, $($arg:tt)+) => {
		assert!(
			$expression.is_empty(),
			"assertion failed, expected {:?} to be empty: {}",
			$expression,
			format_args!($($arg)+)
		)
	};
}

/// Asserts that expression is empty using `is_empty`.
///
/// Like [`assert_empty!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_empty!`]: ./macro.assert_empty.html
#[macro_export]
macro_rules! debug_assert_empty {
	($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_empty!($($arg)*); })
}
