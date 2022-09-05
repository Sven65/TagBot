use rlua::{Result as LuaResult, Error as LuaError, Context, FromLuaMulti};

use crate::tags::lua::lua_modules::registry::registry::LUA_MODULE_INDEX;


/// Custom replacement for luas `require` function
/// 
/// Takes the `name` argument and tries to load and execute it from the LUA_MODULE_INDEX variable
/// 
/// # Arguments
/// 
/// * `ctx` - The lua context to load on
/// * `name` - The name of the module to load
pub fn user_require<'lua, T: FromLuaMulti<'lua>> (ctx: Context<'lua>, name: String) -> LuaResult<T> {
	let index = LUA_MODULE_INDEX.lock().unwrap();
	
	if !index.has_module(&name) {
		return Err(LuaError::external(format!("Module {} not found.", name)));
	}

	let content = index.load_module_to_string(&name);

	if content.is_err() {
		return Err(LuaError::external(content.err().unwrap()));
	}

	let source = content.ok().unwrap();

	let chunk = ctx.load(source.as_str());

	Ok(chunk.eval()?)
}