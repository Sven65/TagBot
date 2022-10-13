/// Trait for constructing struct from a given type
pub trait ConstructableFrom<T> {
	/// Creates a new wrapper
	fn new(value: T) -> Self;
}

/// Trait for constructing struct from two given types
pub trait ConstructableFrom2<T, T2> {
	/// Creates a new wrapper
	fn new(value: T, value2: T2) -> Self;
}
