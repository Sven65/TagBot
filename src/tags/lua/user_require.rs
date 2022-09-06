use rlua::{Result as LuaResult, Error as LuaError, Context, Value};

use crate::tags::lua::lua_modules::registry::registry::LUA_MODULE_INDEX;


/// Custom replacement for luas `require` function
/// 
/// Takes the `name` argument and tries to load and execute it from the LUA_MODULE_INDEX variable
/// 
/// # Arguments
/// 
/// * `ctx` - The lua context to load on
/// * `name` - The name of the module to load
pub fn user_require<'lua> (ctx: Context<'lua>, name: String) -> LuaResult<Value> {
	let index = LUA_MODULE_INDEX.lock().unwrap();

	let result = index.load_module(name.as_str(), ctx);

	if result.is_err() {
		return Err(LuaError::external(result.err().unwrap()));
	}

	let final_result = result.unwrap();

	Ok(final_result)
}