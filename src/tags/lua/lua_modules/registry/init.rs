use rlua::{Context, Value};

use crate::{
	tags::lua::lua_modules::rs_lua::types::{
		serenity::{colour::TBColour, timestamp::TBTimestamp},
		Requireable,
	},
	util::paths::Paths,
};

use super::registry::LUA_MODULE_INDEX;

fn resolve_path(module_path: &str) -> String {
	let paths = Paths::new();

	let mut path = paths.prefix;

	path.push("data");
	path.push("lua");
	path.push(module_path);

	let path = path.to_str().unwrap().to_string();

	path
}

fn get_value<'lua>(key: &str, ctx: Context<'lua>) -> rlua::Value<'lua> {
	let value = ctx.globals().get::<&str, Value>(key);

	if value.is_err() {
		return rlua::Nil;
	}

	value.unwrap()
}

#[rustfmt::skip]
pub fn init_modules() {
	LUA_MODULE_INDEX.lock().unwrap().register_module_file("util", &resolve_path("util.lua"));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("variables/sender", |ctx| get_value("sender", ctx));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("variables/sender_member", |ctx| get_value("sender_member", ctx));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("variables/channel_id", |ctx| get_value("channel_id", ctx));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("timestamp", |ctx| TBTimestamp::create_module(ctx));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("colour", |ctx| TBColour::create_module(ctx));
}
