use rlua::{Context, Result as LuaResult, Table, ToLua, Value};

use super::types::{ConstructableFrom, ConstructableFrom2};
use std::{collections::HashMap, hash::Hash};

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
