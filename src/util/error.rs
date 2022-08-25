#[macro_export]
macro_rules! handle_error {
	($e:expr, $($args: tt)*) => (match $e {
		Ok(val) => val,
		Err(err) => println!("{}: {}", $($args)*, err),
	});
}
