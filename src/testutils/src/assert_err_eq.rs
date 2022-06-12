// Taken and modified from https://github.com/svartalf/rust-claim/commit/815fb7da4965117917406941bc4f3a6b076d1404
/// Asserts that expression returns [`Err(E)`] variant
/// and its value of `E` type equals to the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_err_eq!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// use testutils::assert_err_eq;
/// # fn main() {
/// let res: Result<(), i32> = Err(1);
///
/// assert_err_eq!(res, 1);
///
/// // With custom messages
/// assert_err_eq!(res, 1, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// Value of `E` type from `Err(E)` will be returned from the macro call:
///
/// ```rust
/// use testutils::assert_err_eq;
/// # fn main() {
/// let res: Result<(), i32> = Err(1);
///
/// let value = assert_err_eq!(res, 1);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// `Ok(..)` variant will cause panic:
///
/// ```rust,should_panic
/// use testutils::assert_err_eq;
/// # fn main() {
/// let res: Result<(), i32> = Ok(());
///
/// assert_err_eq!(res, 1); // Will panic
/// # }
/// ```
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_err_eq!`]: ./macro.debug_assert_err_eq.html
#[macro_export]
macro_rules! assert_err_eq {
    ($cond:expr, $expected:expr,) => {
        $crate::assert_err_eq!($cond, $expected);
    };
    ($cond:expr, $expected:expr) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}", ok);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(..), got {:?}: {}", ok, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that expression returns [`Err(E)`] variant in runtime.
///
/// Like [`assert_err_eq!`], this macro also has a second version,
/// where a custom panic message can be provided.
///
/// ## Uses
///
/// See [`debug_assert!`] documentation for possible use cases.
/// The same applies to this macro.
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert!`]: https://doc.rust-lang.org/std/macro.debug_assert.html
/// [`assert_err_eq!`]: ./macro.assert_err_eq.html
#[macro_export]
macro_rules! debug_assert_err_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err_eq!($($arg)*); })
}
