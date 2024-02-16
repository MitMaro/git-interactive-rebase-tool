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

#[macro_export]
macro_rules! debug_assert_not_empty {
	($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_not_empty!($($arg)*); })
}
