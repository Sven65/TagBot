use rlua::{Context, FromLua, Lua, Result as LuaResult, Table, ToLua, Value};

use super::types::{ConstructableFrom, ConstructableFrom2};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub fn lua_todo(ctx: Context) -> LuaResult<Value> {
	"Not yet implemented.".to_string().to_lua(ctx)
}

/// Convert a given value to a lua value for a constructable type wrapper implementing ToLua
///
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - [`rlua::Context`] to use when converting
pub fn convert_constructable<'lua, T: ConstructableFrom<V> + ToLua<'lua>, V: Clone>(
	value: V,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let converted_value = T::new(value);

	converted_value.to_lua(ctx)
}

/// Converts a given Option value to a lua value for a constructable type wrapper, or Value::Nil if the option is none.
///
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - [`rlua::Context`] to use when converting
pub fn convert_constructable_option<'lua, T: ConstructableFrom<V> + ToLua<'lua>, V: Clone>(
	value: Option<V>,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let cloned_value = value.clone();

	if cloned_value.is_none() {
		return Ok(Value::Nil);
	}

	let converted_value = T::new(value.unwrap());

	converted_value.to_lua(ctx)
}

/// Converts a given Option value to a lua value for a constructable2 type wrapper, or Value::Nil if the option is none.
///
/// # Arguments
/// * `value` - The value to convert
/// * `value2` - The second value to convert
/// * `ctx` - [`rlua::Context`] to use when converting
pub fn convert_constructable2_option<
	'lua,
	T: ConstructableFrom2<V, V2> + ToLua<'lua>,
	V: Clone,
	V2: Clone,
>(
	value: Option<V>,
	value2: Option<V2>,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let cloned_value = value.clone();

	if cloned_value.is_none() {
		return Ok(Value::Nil);
	}

	let converted_value = T::new(value.unwrap(), value2.unwrap());

	converted_value.to_lua(ctx)
}

/// Converts a given value to a lua value for a constructable2 type wrapper, or Value::Nil if the option is none.
///
/// # Arguments
/// * `value` - The value to convert
/// * `value2` - The second value to convert
/// * `ctx` - [`rlua::Context`] to use when converting
pub fn convert_constructable2<
	'lua,
	T: ConstructableFrom2<V, V2> + ToLua<'lua>,
	V: Clone,
	V2: Clone,
>(
	value: V,
	value2: V2,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let converted_value = T::new(value, value2);

	converted_value.to_lua(ctx)
}

/// Converts a value of type T into a [`LuaResult<Value>`]
///
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - [`rlua::Context`] to use for the conversion
pub fn convert_type<'lua, T: ToLua<'lua>>(value: T, ctx: Context<'lua>) -> LuaResult<Value<'lua>> {
	value.to_lua(ctx)
}

/// Converts a value of type Option<T> into a [`LuaResult<Value>`]
///
/// Returns [`Value::Nil`] if the value is None
///
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - [`rlua::Context`] to use for the conversion
pub fn convert_type_option<'lua, T: ToLua<'lua>>(
	value: Option<T>,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	if value.is_none() {
		return Ok(Value::Nil);
	}

	value.unwrap().to_lua(ctx)
}

/// Converts a [`Vec<T2>`] to a [`Vec<T>`]
///
/// # Arguments
///
/// * `value` - The value to convert
/// * `ctx` - [`rlua::Context`] to use for converting to lua
///
/// # Example
///
/// ```no_run
/// // Convert a vec of strings
///
/// let data: Vec<String> = vec!["hello", "world"];
/// convert_vec::<String, _>(data, ctx)?,
/// ```
pub fn convert_vec<'lua, T: std::convert::From<T2> + ToLua<'lua>, T2>(
	value: Vec<T2>,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let vec2: Vec<T> = value.into_iter().map(|x| x.into()).collect();

	vec2.to_lua(ctx)
}

#[rustfmt::skip]
/// Converts a [`Vec<T2>`] to a [`Vec<T>`] using T::new
///
/// # Arguments
///
/// * `value` - The value to convert
/// * `value2` - The second value to use
/// * `ctx` - [`rlua::Context`] to use for converting to lua
pub fn convert_vec_new<
	'lua,
	T: ConstructableFrom2<T2, S> + ToLua<'lua>,
	T2,
	S: Clone,
>(
	value: Vec<T2>,
	value2: S,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let vec2: Vec<T> = value.into_iter().map(|x| T::new(x, value2.clone())).collect();

	vec2.to_lua(ctx)
}

/// Converts a [`HashMap<K2, V2>`] to a [`HashMap<K, V>`]
///
/// # Arguments
/// * `from` - The map to convert from
/// * `ctx` - [`rlua::Context`] to use for converting to lua
pub fn convert_hashmap_types<
	'lua,
	K: Eq + Hash + std::convert::From<K2> + ToLua<'lua>,
	V: std::convert::From<V2> + ToLua<'lua>,
	K2,
	V2,
>(
	from: HashMap<K2, V2>,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let mut map: HashMap<K, V> = HashMap::new();

	for (k, v) in from {
		map.insert(k.into(), v.into());
	}

	let table: Table = ctx.create_table_from(map)?;

	table.to_lua(ctx)
}

/// Converts a [`HashMap<K2, V2>`] to a [`HashMap<K, V>`] using [`V::new`]
///
/// # Arguments
/// * `from` - The map to convert from
/// * `second`- The second argument to use when constructing
/// * `ctx` - [`rlua::Context`] to use for converting to lua
pub fn convert_hashmap_types_with_new<
	'lua,
	K: Eq + Hash + std::convert::From<K2> + ToLua<'lua>,
	V: ConstructableFrom2<V2, S> + ToLua<'lua>,
	S: Clone,
	K2,
	V2,
>(
	from: HashMap<K2, V2>,
	second: S,
	ctx: Context<'lua>,
) -> LuaResult<Value> {
	let mut map: HashMap<K, V> = HashMap::new();

	for (k, v) in from {
		map.insert(k.into(), V::new(v, second.clone()));
	}

	let table: Table = ctx.create_table_from(map)?;

	table.to_lua(ctx)
}

/// Retrieves an optional value from an `rlua::Table`.
///
/// This function attempts to fetch a value from the given Lua table using the provided key.
/// If the key does not exist or contains `nil`, it returns `None`. Otherwise, it attempts
/// to convert the value into type `T`.
///
/// # Type Parameters
/// - `'lua`: The Lua context lifetime.
/// - `T`: A type that implements `rlua::FromLua<'lua>`, allowing it to be converted from a Lua value.
///
/// # Parameters
/// - `table`: A reference to an `rlua::Table` from which to retrieve the value.
/// - `key`: The key associated with the value to retrieve.
///
/// # Returns
/// - `Ok(Some(T))` if the value exists and is successfully converted.
/// - `Ok(None)` if the key does not exist or is `nil`.
/// - `Err(rlua::Error)` if the conversion to `T` fails.
///
/// # Example
/// ```
/// use rlua::{Lua, Table, Result};
///
/// fn main() -> Result<()> {
///     let lua = Lua::new();
///     lua.context(|ctx| {
///         let table: Table = ctx.load("return { foo = 42, bar = nil }").eval()?;
///
///         let foo: Option<i32> = get_option(&table, "foo")?;
///         let bar: Option<i32> = get_option(&table, "bar")?;
///         let baz: Option<i32> = get_option(&table, "baz")?;
///
///         assert_eq!(foo, Some(42));
///         assert_eq!(bar, None);
///         assert_eq!(baz, None);
///
///         Ok(())
///     })
/// }
/// ```
///
/// # Errors
/// This function returns an error if the key exists but the value cannot be converted into `T`.
pub fn get_option_from_table<'lua, T: FromLua<'lua>>(
	table: &Table<'lua>,
	key: &str,
	ctx: Context<'lua>,
) -> LuaResult<Option<T>> {
	match table.clone().get::<_, Value>(key)? {
		Value::Nil => Ok(None),
		value => Ok(Some(T::from_lua(value, ctx)?)),
	}
}
