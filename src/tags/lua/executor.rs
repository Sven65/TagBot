use include_lua::include_lua;
use rlua::{Lua, Result, HookTriggers, FromLuaMulti, MultiValue};
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};
use std::{env, path::{PathBuf, Path}};

use crate::{services::rethinkdb::tags::Tag, handle_error};

fn get_current_dir() -> PathBuf {
	let current_exe = env::current_exe();

	if current_exe.is_err() {
		panic!("Failed to get current exe path");
	}

	let current_exe = current_exe.ok().unwrap();

	return current_exe;
}

fn eval<'lua, T: FromLuaMulti<'lua>>(lua_ctx: rlua::Context<'lua>, script: &str) -> Result<T> {
    lua_ctx.load(script).eval()
}

fn execute_code(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> rlua::Result<String> {
	let lua = Lua::new();

	// MB * 1024 kb * 1024 bytes
	lua.set_memory_limit(Some(1 * 1024 * 1024));

	lua.context(|lua_ctx| {
		let globals = lua_ctx.globals();
		globals.set("glob_test", "Hello World!")?;

		Ok(())
	})?;

	// lua.context(|lua_ctx| {
	// 	let print = lua_ctx.create_function(func)

	// 	Ok(())
	// })?;

	let data = lua.context(|lua_ctx| {
		eval::<String>(
			lua_ctx, 
			format!("
				local sandbox = require 'sandbox'


				local ok, result = pcall(sandbox.run, {})

				print(ok, result)
			", &tag.content).as_str()
		)
	});

	println!("Data is {:#?}", data);

	return Ok("exec done".to_string());
}

pub async fn execute_tag(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let result = execute_code(tag, interaction, ctx);

	if result.is_ok() {
		return result.ok().unwrap();
	} else {
		return format!("Failed to execute tag: {:#?}", result.err())
	}
}