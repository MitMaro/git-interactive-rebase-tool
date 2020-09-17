#[macro_export]
macro_rules! build_trace {
	($name:expr) => {{
		let args: Vec<String> = vec![];
		(String::from($name), args)
	}};
	($name:expr, $($arg:expr),*) => {{
		let mut args = vec![];
		$( args.push(format!("{}", $arg)); )*
		(String::from($name), args)
	}};
}

pub fn panic_trace_error(e: &(String, Vec<String>), trace: &[(String, Vec<String>)]) {
	panic!(vec![
		"\n==========",
		"Missing function call in trace",
		format!("Call: {}({})", e.0, e.1.join(", ")).as_str(),
		"Trace:",
		trace
			.iter()
			.map(|(f, args): &(String, Vec<String>)| {
				format!(
					"\t{}({})",
					f,
					args.iter()
						.map(|v| v.replace("\n", "\\n"))
						.collect::<Vec<String>>()
						.join(", ")
				)
			})
			.collect::<Vec<String>>()
			.join("\n")
			.as_str(),
		"==========\n"
	]
	.join("\n"));
}

pub fn compare_trace(actual: &[(String, Vec<String>)], expected: &[(String, Vec<String>)]) {
	let mut e_iter = expected.iter();
	let mut a_iter = actual.iter();
	'trace: while let Some(e) = e_iter.next() {
		loop {
			if let Some(a) = a_iter.next() {
				// function name and argument length must match
				if !a.0.eq(&e.0) || a.1.len() != e.1.len() {
					continue;
				}
				if a.1.iter().zip(&e.1).all(|(a, e)| e.eq("*") || a.eq(e)) {
					continue 'trace;
				}
			}
			else {
				panic_trace_error(e, actual);
			}
		}
	}

	if let Some(e) = e_iter.next() {
		panic_trace_error(e, actual);
	}
}
