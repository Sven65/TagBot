use rlua::{Lua, Result as LuaResult, FromLuaMulti, HookTriggers, Error as LuaError, Value};
use serenity::{model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandData}, prelude::Context};
use std::{io::{Read, ErrorKind, Error}};
use gag::{BufferRedirect};

use crate::{services::rethinkdb::tags::Tag, util::command_options::FindOption};

use super::user_require::user_require;

fn eval<'lua, T: FromLuaMulti<'lua>>(lua_ctx: rlua::Context<'lua>, script: &str) -> LuaResult<T> {
    lua_ctx.load(script).set_name("cock.lua")?.eval()
}

fn parse_pos_args(data: &CommandData) -> Vec<&str> {
	let args_opt = data.find_option("args");

	if args_opt.is_none() {
		return Vec::new();
	}

	let args = args_opt.unwrap();
	let args = args.value.as_ref().unwrap().as_str().unwrap();

	let args: Vec<&str> = args.split(" ").collect();

	return args;
}

fn execute_code(tag: Tag, interaction: ApplicationCommandInteraction, _ctx: Context) -> rlua::Result<String> {

	let lua = Lua::new();

	// MB * 1024 kb * 1024 bytes
	lua.set_memory_limit(Some(1 * 1024 * 1024));

	let args = parse_pos_args(&interaction.data);

	lua.context(|lua_ctx| {
		let globals = lua_ctx.globals();
		globals.set("glob_test", "Hello World!")?;

		globals.set("arg", args)?;

		let lua_user_require = lua_ctx.create_function(|ctx, name| {
			return user_require::<Value>(ctx, name);
		})?;

		globals.set("user_require", lua_user_require)?;

		Ok(())
	})?;


	lua.set_hook(HookTriggers {
		every_nth_instruction: Some(100000), // Max instructions to execute.
		..Default::default()
	}, |_lua_context, _debug| {
		Err(LuaError::external("Too many instructions used"))
	});

	let lua_buf = BufferRedirect::stdout();

	if lua_buf.is_err() {
		panic!("Failed to open lua buffer {:#?}", lua_buf.err());
	}

	let lua_buf = lua_buf.unwrap();
	let mut output = String::new();

	
	let result = lua.context(|lua_ctx| {
		let lua_script = format!(r#"
			local _print = print
			local sandbox = require 'sandbox'

			local env = {{ print = _print, arg = arg, require = user_require }}

			local ok, result = pcall(sandbox.run, [[{}]], {{env = env, quota = 1000}})

			if result then
				print(result)
			end

			return ''
		"#, &tag.content);

		eval::<String>(
			lua_ctx, 
			lua_script.as_str()
		)
	});

	lua_buf.into_inner().read_to_string(&mut output).unwrap();

	if result.is_err() {
		println!("Error executing lua: {:#?}", result.clone().err());


		return result;
	}


	return Ok(output.to_string());
}

pub async fn execute_tag(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> Result<String, Error> {
	let result = execute_code(tag, interaction, ctx);

	if result.is_ok() {
		return Ok(result.ok().unwrap());
	} else {
		println!("Failed to execute tag: {:#?}", result.clone().err());

		let cause = match result.clone().err().unwrap() {
			LuaError::CallbackError { traceback: _, cause } => cause.to_string(),
			_ => "".to_string(),
		};

		println!("Cause is {:#?}", cause);

		let cause = match cause.as_str() {
			"" => "".to_string(),
			_ => format!("Caused by {}", cause).to_string(),
		};

		return Err(
			Error::new(
				ErrorKind::Other, 
				format!("{}\n{}", result.err().unwrap().to_string(), cause)
			)
		);
	}
}