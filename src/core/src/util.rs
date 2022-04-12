#[macro_export]
macro_rules! select {
	(default $default: expr, $first: expr) => {
		if let Some(value) = $first() {
			value
		}
		else {
			$default()
		}
	};
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

#[macro_export]
macro_rules! first {
	($first: expr, $($arg:expr),*) => {
		if $first().is_some() {
		}
		$(else if $arg().is_some() {
		})*
	};
}
