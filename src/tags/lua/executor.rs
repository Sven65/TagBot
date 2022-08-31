use rlua::{Lua, Result, FromLuaMulti};
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};
use std::{io::Read};
use gag::{BufferRedirect};

use crate::{services::rethinkdb::tags::Tag};


fn eval<'lua, T: FromLuaMulti<'lua>>(lua_ctx: rlua::Context<'lua>, script: &str) -> Result<T> {
    lua_ctx.load(script).set_name("cock.lua")?.eval()
}

fn execute_code(tag: Tag, _interaction: ApplicationCommandInteraction, _ctx: Context) -> rlua::Result<String> {

	let lua = Lua::new();

	// MB * 1024 kb * 1024 bytes
	lua.set_memory_limit(Some(1 * 1024 * 1024));


	lua.context(|lua_ctx| {
		let globals = lua_ctx.globals();
		globals.set("glob_test", "Hello World!")?;

		Ok(())
	})?;

	let lua_buf = BufferRedirect::stdout();

	if lua_buf.is_err() {
		panic!("Failed to open lua buffer {:#?}", lua_buf.err());
	}

	let lua_buf = lua_buf.unwrap();
	let mut output = String::new();

	
	let result = lua.context(|lua_ctx| {
		let lua_script = format!("
			local _print = print
			local sandbox = require 'sandbox'


			local env = {{ print = _print }}

			local ok, result = pcall(sandbox.run, [[{}]], {{env = env, quota = 10000}})

			if result then
				print(result)
			end

			return ''
		", &tag.content);

		eval::<String>(
			lua_ctx, 
			lua_script.as_str()
		)
	});

	lua_buf.into_inner().read_to_string(&mut output).unwrap();

	if result.is_err() {
		println!("Error executing lua: {:#?}", result.err());
	}


	return Ok(output.to_string());
}

pub async fn execute_tag(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let result = execute_code(tag, interaction, ctx);

	if result.is_ok() {
		return result.ok().unwrap();
	} else {
		return format!("Failed to execute tag: {:#?}", result.err())
	}
}