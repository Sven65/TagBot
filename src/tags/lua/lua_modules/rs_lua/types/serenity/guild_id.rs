use rlua::{Error as LuaError, IntoLua, MetaMethod, UserData, Value};
use serenity::model::prelude::{GuildId, PartialGuild};
use serenity::{prelude::Context as SerenityContext, Error};
use tagbot_macros::lua_document;
use tokio::runtime::Handle;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::partial_guild::TBPartialGuild;

async fn get_guild(guild_id: GuildId, s_ctx: SerenityContext) -> Result<PartialGuild, Error> {
	let guild = guild_id.to_partial_guild(&s_ctx.http).await.unwrap();

	Ok(guild)
}

/// Wrapper for [`serenity::model::prelude::GuildId`]
#[derive(Clone)]
#[lua_document("TBGuildId", class)]
pub struct TBGuildId(pub GuildId, pub SerenityContext);

impl ConstructableFrom2<GuildId, SerenityContext> for TBGuildId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `guild_id` - The serenity GuildId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	fn new(guild_id: GuildId, s_ctx: SerenityContext) -> TBGuildId {
		TBGuildId(guild_id, s_ctx)
	}
}

impl UserData for TBGuildId {
	#[lua_document("TBGuildId", parse_comments)]
	#[allow(unused_doc_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		/// @desc Resolves the ID to a guild.
		/// @method
		/// @return {TBGuild} A discord guild
		methods.add_method("resolve", |ctx, this, _: Value| {
			let guild_id = this.0;
			let s_ctx = this.1.clone();

			let guild = tokio::task::block_in_place(move || {
				Handle::current().block_on(async move { get_guild(guild_id, s_ctx).await })
			});

			if guild.is_err() {
				return Err(LuaError::external("Failed to get guild."));
			}

			TBPartialGuild(guild.unwrap(), this.1.clone()).into_lua(ctx)
		});
	}
}
