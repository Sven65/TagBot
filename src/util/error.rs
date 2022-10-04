#[macro_export]
/// Lazy error handler
///
/// Pass a statement that might produce an error.
/// If it errors, it'll be printed to the console.
/// If not, it'll return the result of the statement.
macro_rules! handle_error {
	($e:expr, $($args: tt)*) => (match $e {
		Ok(val) => val,
		Err(err) => println!("{}: {}", $($args)*, err),
	});
}
