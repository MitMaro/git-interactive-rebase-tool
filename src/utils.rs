macro_rules! print_err {
	($($arg:tt)*) => (
		{
			use std::io::prelude::*;
			if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
				panic!(
					"Failed to write to stderr.\n\
					Original error output: {}\n\
					Secondary error writing to stderr: {}", format!($($arg)*), e
				);
			}
		}
	)
}
