#[macro_export]
macro_rules! select {
	(default $default: expr, $first: expr, $($arg:expr),*) => {
		if let Some(value) = $first() {
			value
		}
		$(else if let Some(value) = $arg() {
			value
		})*
		else {
			$default()
		}
	};
}
