use rlua::{ToLua, Context, Value, Result as LuaResult, Table};

use super::types::{ConstructableFrom, ConstructableFrom2};
use std::{collections::HashMap, hash::Hash, fmt::Debug};



/// Convert a given value to a lua value for a constructable type wrapper implementing ToLua
/// 
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - Lua context to use when converting
pub fn convert_constructable<'lua, T: ConstructableFrom<V> + ToLua<'lua>, V: Clone>(value: V, ctx: Context<'lua>) -> LuaResult<Value> {
	let converted_value = T::new(value);

	Ok(converted_value.to_lua(ctx)?)
}

/// Converts a given Option value to a lua value for a constructable type wrapper, or Value::Nil if the option is none.
/// 
/// # Arguments
/// * `value` - The value to convert
/// * `ctx` - Lua context to use when converting
pub fn convert_constructable_option<'lua, T: ConstructableFrom<V> + ToLua<'lua>, V: Clone>(value: Option<V>, ctx: Context<'lua>) -> LuaResult<Value> {
	let cloned_value = value.clone();

	if cloned_value.is_none() {
		return Ok(Value::Nil);
	}

	let converted_value = T::new(value.unwrap());

	Ok(converted_value.to_lua(ctx)?)
}

/// Converts a given Option value to a lua value for a constructable2 type wrapper, or Value::Nil if the option is none.
/// 
/// # Arguments
/// * `value` - The value to convert
/// * `value2` - The second value to convert
/// * `ctx` - Lua context to use when converting
pub fn convert_constructable2_option<'lua, T: ConstructableFrom2<V, V2> + ToLua<'lua>, V: Clone, V2: Clone>(value: Option<V>, value2: Option<V2>, ctx: Context<'lua>) -> LuaResult<Value> {
	let cloned_value = value.clone();

	if cloned_value.is_none() {
		return Ok(Value::Nil);
	}

	let converted_value = T::new(value.unwrap(), value2.unwrap());

	Ok(converted_value.to_lua(ctx)?)
}

/// Converts a given value to a lua value for a constructable2 type wrapper, or Value::Nil if the option is none.
/// 
/// # Arguments
/// * `value` - The value to convert
/// * `value2` - The second value to convert
/// * `ctx` - Lua context to use when converting
pub fn convert_constructable2<'lua, T: ConstructableFrom2<V, V2> + ToLua<'lua>, V: Clone, V2: Clone>(value: V, value2: V2, ctx: Context<'lua>) -> LuaResult<Value> {
	let converted_value = T::new(value, value2);

	Ok(converted_value.to_lua(ctx)?)
}

pub fn convert_type<'lua, T: ToLua<'lua>>(value: T, ctx: Context<'lua>) -> LuaResult<Value> {
	Ok(value.to_lua(ctx)?)
}

pub fn convert_type_option<'lua, T: ToLua<'lua>>(value: Option<T>, ctx: Context<'lua>) -> LuaResult<Value> {
	if value.is_none() {
		return Ok(Value::Nil);
	}

	Ok(value.unwrap().to_lua(ctx)?)
}


// from, to
pub fn convert_hashmap_types<'lua,
	K: Eq + Hash + std::convert::From<K2> + ToLua<'lua>,
	V: std::convert::From<V2> + ToLua<'lua>,
	K2, V2
>(from: HashMap<K2, V2>, ctx: Context<'lua>) -> LuaResult<Value> {
	let mut map: HashMap<K, V> = HashMap::new();

	for (k, v) in from {
		map.insert(k.into(), v.into());
	};

	let table: Table = ctx.create_table_from(map)?;

	Ok(table.to_lua(ctx)?)
}