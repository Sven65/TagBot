use gag::BufferRedirect;
use rlua::{Error as LuaError, FromLuaMulti, HookTriggers, Lua, Result as LuaResult};
use serenity::{
	model::prelude::interaction::application_command::{
		ApplicationCommandInteraction, CommandData,
	},
	prelude::Context,
};
use std::io::{Error, ErrorKind, Read};

use crate::{
	services::rethinkdb::tags::Tag,
	tags::lua::lua_modules::rs_lua::types::{
		serenity::{channel_id::TBChannelId, guild_id::TBGuildId, member::TBMember, user::TBUser},
		utils::types::ConstructableFrom2,
	},
	util::command_options::FindOption,
};

use super::user_require::user_require;

const INSTRUCTION_LIMIT: Option<u32> = Some(1_000_000);
const USER_MEMORY_LIMIT: usize = 1; // MB
const SERVER_MEMORY_LIMIT: usize = 1; // MB
const MEMORY_LIMIT: Option<usize> = Some((USER_MEMORY_LIMIT + SERVER_MEMORY_LIMIT) * 1024 * 1024); // MB * 1024 kb * 1024 bytes

fn eval<'lua, T: FromLuaMulti<'lua>>(lua_ctx: rlua::Context<'lua>, script: &str) -> LuaResult<T> {
	lua_ctx.load(script).set_name("tag.lua")?.eval()
}

fn parse_pos_args(data: &CommandData) -> Vec<&str> {
	let args_opt = data.find_option("args");

	if args_opt.is_none() {
		return Vec::new();
	}

	let args = args_opt.unwrap();
	let args = args.value.as_ref().unwrap().as_str().unwrap();

	let args: Vec<&str> = args.split(' ').collect();

	args
}

fn execute_code(
	tag: Tag,
	interaction: ApplicationCommandInteraction,
	_ctx: Context,
) -> rlua::Result<String> {
	let lua = Lua::new();

	println!("interaction {:#?}", interaction);

	lua.set_memory_limit(MEMORY_LIMIT);

	let args = parse_pos_args(&interaction.data);

	// Set interaction data
	lua.context(|lua_ctx| {
		let globals = lua_ctx.globals();

		let sender = TBUser::new(interaction.clone().user);

		let member = interaction.clone().member;

		let channel_id = interaction.clone().channel_id;

		globals.set("sender", sender)?;

		if let Some(member) = member {
			let sender_member = TBMember::new(member);
			globals.set("sender_member", sender_member)?;
		}

		globals.set("channel_id", TBChannelId::new(channel_id, _ctx))?;

		let guild_id = interaction.clone().guild_id;
		if let Some(id) = guild_id {
			globals.set("guild_id", TBGuildId::new(id, _ctx))?;
		}

		Ok(())
	})?;

	lua.context(|lua_ctx| {
		let globals = lua_ctx.globals();
		globals.set("arg", args)?;

		let lua_user_require = lua_ctx.create_function(user_require)?;

		globals.set("user_require", lua_user_require)?;

		Ok(())
	})?;

	lua.set_hook(
		HookTriggers {
			every_nth_instruction: INSTRUCTION_LIMIT, // Max instructions to execute.
			..Default::default()
		},
		|_lua_context, _debug| Err(LuaError::external("Too many instructions used")),
	);

	let lua_buf = BufferRedirect::stdout();

	if lua_buf.is_err() {
		panic!("Failed to open lua buffer {:#?}", lua_buf.err());
	}

	let lua_buf = lua_buf.unwrap();
	let mut output = String::new();

	let result = lua.context(|lua_ctx| {
		let lua_script = format!(
			r#"
			local _print = print
			local sandbox = require 'sandbox'

			local env = {{ print = _print, arg = arg, require = user_require }}

			local ok, result = pcall(sandbox.run, [[{}]], {{env = env, quota = 1000}})

			if result then
				print(result)
			end

			return ''
		"#,
			&tag.content
		);

		eval::<String>(lua_ctx, lua_script.as_str())
	});

	lua_buf.into_inner().read_to_string(&mut output).unwrap();

	if result.is_err() {
		println!("Error executing lua: {:#?}", result.clone().err());

		return result;
	}

	println!("output {}", output);

	Ok(output.to_string())
}

pub async fn execute_tag(
	tag: Tag,
	interaction: ApplicationCommandInteraction,
	ctx: Context,
) -> Result<String, Error> {
	let result = execute_code(tag, interaction, ctx);

	if result.is_ok() {
		Ok(result.ok().unwrap())
	} else {
		println!("Failed to execute tag: {:#?}", result.clone().err());

		let cause = match result.clone().err().unwrap() {
			LuaError::CallbackError { traceback: _, cause } => cause.to_string(),
			_ => "".to_string(),
		};

		println!("Cause is {:#?}", cause);

		let cause = match cause.as_str() {
			"" => "".to_string(),
			_ => format!("Caused by {}", cause),
		};

		Err(Error::new(
			ErrorKind::Other,
			format!("{}\n{}", result.err().unwrap(), cause),
		))
	}
}
