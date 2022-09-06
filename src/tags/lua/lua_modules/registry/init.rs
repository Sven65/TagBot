use rlua::Value;

use crate::util::{paths::Paths, interactions::to_lua::TBUser};

use super::registry::LUA_MODULE_INDEX;

fn resolve_path(module_path: &str) -> String {
	let paths = Paths::new();

	let mut path = paths.prefix.clone();

	path.push("data");
	path.push("lua");
	path.push(module_path);

	// println!("Path {:?}", path.clone());

	let path = path.to_str().unwrap().to_string();

	return path;
}

pub fn init_modules() {
	LUA_MODULE_INDEX.lock().unwrap().register_module_file("util", &resolve_path("util.lua"));
	LUA_MODULE_INDEX.lock().unwrap().register_rust_module("variables/sender", |ctx| {
		let user = ctx.globals().get::<&str, Value>("user");

		if user.is_err() {
			return rlua::Nil;
		}

		let user = user.unwrap();

		return user;
	});
}

